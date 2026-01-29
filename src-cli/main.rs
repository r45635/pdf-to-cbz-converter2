use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use std::path::PathBuf;
use std::time::Instant;
use pdf_conversion_lib::{bind_pdfium, convert_pdf_to_images_parallel, extract_images_lossless_at_dpi, create_pdf_from_images};

mod archive;
mod benchmark;
mod image;

#[derive(Parser)]
#[command(
    name = "pdf-to-cbz",
    version = "3.0.0",
    about = "Ultra-fast PDF ↔ CBZ/CBR converter",
    long_about = "Convert PDF files to CBZ/CBR comic archives and vice versa. Supports batch operations and custom DPI settings."
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Convert PDF to CBZ
    #[command(about = "Convert PDF file(s) to CBZ archive")]
    PdfToCbz {
        /// Input PDF file path
        #[arg(value_name = "INPUT")]
        input: PathBuf,

        /// Output CBZ file path (optional, auto-generated from input if not provided)
        #[arg(short, long, value_name = "OUTPUT")]
        output: Option<PathBuf>,

        /// DPI for rendering (default: 300)
        #[arg(short, long, default_value = "300")]
        dpi: u32,

        /// PNG lossless mode: encode rendered pages as PNG at same DPI instead of JPEG
        #[arg(short, long)]
        lossless: bool,

        /// JPEG quality for compression (1-100, default: 90, only in lossy mode)
        #[arg(short = 'q', long, default_value = "90")]
        quality: u8,

        /// Maximum number of pages to process (0 = all)
        #[arg(long, default_value = "0")]
        max_pages: u32,

        /// Number of threads for parallel processing (default: number of CPU cores)
        #[arg(short = 't', long)]
        threads: Option<usize>,
    },

    /// Convert CBZ/CBR to PDF
    #[command(about = "Convert CBZ or CBR archive to PDF")]
    CbzToPdf {
        /// Input CBZ/CBR file path
        #[arg(value_name = "INPUT")]
        input: PathBuf,

        /// Output PDF file path (optional, auto-generated from input if not provided)
        #[arg(short, long, value_name = "OUTPUT")]
        output: Option<PathBuf>,

        /// Preserve original image format and quality (lossless mode)
        #[arg(short, long)]
        lossless: bool,

        /// JPEG quality for re-compression (1-100, default: 90, only if not lossless)
        #[arg(short = 'q', long, default_value = "90")]
        quality: u8,
    },

    /// Smoke test: diagnostic render of single page with regression checks
    #[command(about = "Render single page with sanity checks (white_ratio, bbox coverage)")]
    SmokeRender {
        /// Input PDF file path
        #[arg(value_name = "INPUT")]
        input: PathBuf,

        /// Page number to test (1-indexed, default: 1)
        #[arg(long, default_value = "1")]
        page: u32,

        /// DPI for rendering (default: 200)
        #[arg(short, long, default_value = "200")]
        dpi: u32,

        /// Output debug image file (default: _smoke_page.png)
        #[arg(short, long, default_value = "_smoke_page.png")]
        output: PathBuf,

        /// Fail if white_ratio > threshold (default: 0.95)
        #[arg(long, default_value = "0.95")]
        max_white_ratio: f64,

        /// Fail if bbox_coverage < threshold (default: 0.30, 30%)
        #[arg(long, default_value = "0.30")]
        min_bbox_coverage: f64,
    },

    /// Benchmark compression methods (Stored vs Deflated)
    #[command(about = "Compare CBZ compression methods for performance optimization")]
    Benchmark {
        /// Input PDF file path
        #[arg(value_name = "INPUT")]
        input: PathBuf,

        /// DPI for rendering (default: 300)
        #[arg(short, long, default_value = "300")]
        dpi: u32,

        /// JPEG quality (1-100, default: 90)
        #[arg(short = 'q', long, default_value = "90")]
        quality: u8,

        /// Maximum pages to process (0 = all)
        #[arg(long, default_value = "0")]
        max_pages: u32,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::PdfToCbz {
            input,
            output,
            dpi,
            lossless,
            quality,
            max_pages,
            threads,
        } => convert_pdf_to_cbz(&input, output, dpi, lossless, quality, max_pages, threads),
        Commands::CbzToPdf { input, output, lossless, quality } => convert_cbz_to_pdf(&input, output, lossless, quality),
        Commands::SmokeRender { input, page, dpi, output, max_white_ratio, min_bbox_coverage } =>
            smoke_render(&input, page, dpi, &output, max_white_ratio, min_bbox_coverage),
        Commands::Benchmark { input, dpi, quality, max_pages } =>
            run_benchmark(&input, dpi, quality, max_pages),
    }
}

fn convert_pdf_to_cbz(input_path: &PathBuf, output_path: Option<PathBuf>, dpi: u32, lossless: bool, quality: u8, max_pages: u32, threads: Option<usize>) -> Result<()> {
    // Validate input
    if !input_path.exists() {
        anyhow::bail!("Input PDF file not found: {:?}", input_path);
    }

    if !input_path.is_file() {
        anyhow::bail!("Input path is not a file: {:?}", input_path);
    }

    // Validate quality
    if quality == 0 || quality > 100 {
        anyhow::bail!("Quality must be between 1 and 100");
    }

    // Configure thread pool
    if let Some(num_threads) = threads {
        rayon::ThreadPoolBuilder::new()
            .num_threads(num_threads)
            .build_global()
            .context("Failed to configure thread pool")?;
        println!("Using {} threads for parallel processing", num_threads);
    } else {
        println!("Using {} threads (auto-detected)", rayon::current_num_threads());
    }

    // Determine output path
    let output_file = match output_path {
        Some(p) => p,
        None => {
            let stem = input_path.file_stem().context("Invalid input filename")?;
            input_path.with_file_name(format!("{}.cbz", stem.to_string_lossy()))
        }
    };

    println!("Converting PDF to CBZ: {:?}", input_path);
    println!("Output: {:?}", output_file);

    if lossless {
        println!("Mode: PNG Lossless (direct extract or render at {} DPI as PNG)", dpi);
    } else {
        println!("Mode: JPEG Lossy (render at {} DPI, quality: {})", dpi, quality);
    }

    if max_pages > 0 {
        println!("Max pages: {}", max_pages);
    }

    // Read PDF
    let pdf_data = std::fs::read(&input_path)
        .context("Failed to read PDF file")?;

    // Convert to images
    let images = if lossless {
        // PNG Lossless: direct extract or render as PNG at same DPI
        extract_images_lossless_at_dpi(&pdf_data, dpi, max_pages)
            .context("Failed to extract images from PDF")?
    } else {
        // JPEG Lossy: render at specified DPI with quality parameter
        convert_pdf_to_images_parallel(&pdf_data, dpi, quality, max_pages)
            .context("Failed to convert PDF to images")?
    };

    println!("Processed {} pages", images.len());

    // Create CBZ archive
    let cbz_data = archive::create_cbz(images)
        .context("Failed to create CBZ archive")?;

    // Write output
    std::fs::write(&output_file, cbz_data)
        .context("Failed to write CBZ file")?;

    let file_size_mb = std::fs::metadata(&output_file)
        .map(|m| m.len() as f64 / (1024.0 * 1024.0))
        .unwrap_or(0.0);

    println!("✓ Successfully created: {:?} ({:.2} MB)", output_file, file_size_mb);
    Ok(())
}

fn convert_cbz_to_pdf(input_path: &PathBuf, output_path: Option<PathBuf>, lossless: bool, quality: u8) -> Result<()> {
    // Validate input
    if !input_path.exists() {
        anyhow::bail!("Input CBZ/CBR file not found: {:?}", input_path);
    }

    if !input_path.is_file() {
        anyhow::bail!("Input path is not a file: {:?}", input_path);
    }

    // Validate quality
    if quality == 0 || quality > 100 {
        anyhow::bail!("Quality must be between 1 and 100");
    }

    // Determine output path
    let output_file = match output_path {
        Some(p) => p,
        None => {
            let stem = input_path.file_stem().context("Invalid input filename")?;
            input_path.with_file_name(format!("{}.pdf", stem.to_string_lossy()))
        }
    };

    println!("Converting CBZ/CBR to PDF: {:?}", input_path);
    println!("Output: {:?}", output_file);
    
    if lossless {
        println!("Mode: Lossless (preserving original image quality)");
    } else {
        println!("Mode: Re-compressing with JPEG quality: {}", quality);
    }

    // Read archive
    let archive_data = std::fs::read(&input_path)
        .context("Failed to read CBZ/CBR file")?;

    // Extract images
    let images = archive::extract_images(&archive_data)
        .context("Failed to extract images from archive")?;

    if images.is_empty() {
        anyhow::bail!("No images found in archive");
    }

    println!("Extracted {} images", images.len());

    // Create PDF
    let pdf_data = create_pdf_from_images(images)
        .context("Failed to create PDF from images")?;

    // Write output
    std::fs::write(&output_file, pdf_data)
        .context("Failed to write PDF file")?;

    let file_size_mb = std::fs::metadata(&output_file)
        .map(|m| m.len() as f64 / (1024.0 * 1024.0))
        .unwrap_or(0.0);

    println!("✓ Successfully created: {:?} ({:.2} MB)", output_file, file_size_mb);
    Ok(())
}

fn smoke_render(input_path: &PathBuf, page_num: u32, dpi: u32, output_path: &PathBuf, max_white_ratio: f64, min_bbox_coverage: f64) -> Result<()> {
    use pdfium_render::prelude::*;

    // Validate input
    if !input_path.exists() {
        anyhow::bail!("Input PDF file not found: {:?}", input_path);
    }

    if !input_path.is_file() {
        anyhow::bail!("Input path is not a file: {:?}", input_path);
    }

    println!("═══════════════════════════════════════════════════════════");
    println!("SMOKE TEST: Single page render with regression checks");
    println!("═══════════════════════════════════════════════════════════");
    println!("Input:           {:?}", input_path);
    println!("Page:            {}", page_num);
    println!("DPI:             {}", dpi);
    println!("Output image:    {:?}", output_path);
    println!("Thresholds:");
    println!("  max_white_ratio:   {:.3} ({:.1}%)", max_white_ratio, max_white_ratio * 100.0);
    println!("  min_bbox_coverage: {:.3} ({:.1}%)", min_bbox_coverage, min_bbox_coverage * 100.0);
    println!();

    // Read PDF
    let pdf_data = std::fs::read(&input_path)
        .context("Failed to read PDF file")?;

    let pdfium = bind_pdfium()
        .context("Failed to initialize Pdfium")?;
    let document = pdfium
        .load_pdf_from_byte_vec(pdf_data, None)
        .context("Failed to load PDF")?;

    let page_count = document.pages().len() as u32;
    println!("Total pages: {}", page_count);

    if page_count == 0 {
        anyhow::bail!("PDF has no pages");
    }

    if page_num < 1 || page_num > page_count {
        anyhow::bail!("Page {} is out of range (1-{})", page_num, page_count);
    }

    // Get requested page
    let page = document
        .pages()
        .get((page_num - 1) as u16)
        .context(format!("Failed to get page {}", page_num))?;

    let boundaries = page.boundaries();
    
    println!();
    println!("───────────────────────────────────────────────────────────");
    println!("PAGE 1 BOX ANALYSIS");
    println!("───────────────────────────────────────────────────────────");
    
    // MediaBox (always present)
    if let Ok(media_box) = boundaries.media() {
        let b = media_box.bounds;
        let w = b.width().value;
        let h = b.height().value;
        println!("MediaBox:");
        println!("  left={:.2}, bottom={:.2}, right={:.2}, top={:.2}", 
            b.left().value, b.bottom().value, b.right().value, b.top().value);
        println!("  width={:.2} pt, height={:.2} pt", w, h);
    }
    
    // CropBox
    let has_crop = if let Ok(crop_box) = boundaries.crop() {
        let b = crop_box.bounds;
        let w = b.width().value;
        let h = b.height().value;
        println!("CropBox:");
        println!("  left={:.2}, bottom={:.2}, right={:.2}, top={:.2}", 
            b.left().value, b.bottom().value, b.right().value, b.top().value);
        println!("  width={:.2} pt, height={:.2} pt", w, h);
        true
    } else {
        println!("CropBox: NOT PRESENT");
        false
    };
    
    // TrimBox
    let has_trim = if let Ok(trim_box) = boundaries.trim() {
        let b = trim_box.bounds;
        let w = b.width().value;
        let h = b.height().value;
        println!("TrimBox:");
        println!("  left={:.2}, bottom={:.2}, right={:.2}, top={:.2}", 
            b.left().value, b.bottom().value, b.right().value, b.top().value);
        println!("  width={:.2} pt, height={:.2} pt", w, h);
        true
    } else {
        println!("TrimBox: NOT PRESENT");
        false
    };

    println!();
    println!("page.width/height: {:.2} x {:.2} pt", page.width().value, page.height().value);

    // Check for image objects (Direct Extract pipeline)
    println!();
    println!("───────────────────────────────────────────────────────────");
    println!("OBJECT DETECTION & BOUNDS ANALYSIS");
    println!("───────────────────────────────────────────────────────────");

    let objects = page.objects();
    let mut image_count = 0;
    let mut text_count = 0;
    let mut path_count = 0;
    let mut form_count = 0;
    let mut other_count = 0;
    let mut content_bbox: Option<(f32, f32, f32, f32)> = None;

    println!("Total objects in page: {}", objects.len());
    println!();

    for (idx, object) in objects.iter().enumerate() {
        // Track all objects for content bbox
        let obj_type = if object.as_image_object().is_some() {
            image_count += 1;
            "IMAGE"
        } else if object.as_text_object().is_some() {
            text_count += 1;
            "TEXT"
        } else if object.as_path_object().is_some() {
            path_count += 1;
            "PATH"
        } else if object.as_x_object_form_object().is_some() {
            form_count += 1;
            "FORM"
        } else {
            other_count += 1;
            "OTHER"
        };

        if let Ok(obj_bbox) = object.bounds() {
            let obj_rect = (
                obj_bbox.left().value,
                obj_bbox.bottom().value,
                obj_bbox.right().value,
                obj_bbox.top().value
            );

            let w = obj_rect.2 - obj_rect.0;
            let h = obj_rect.3 - obj_rect.1;

            println!("Object[{}] type={:5} bounds=({:.1},{:.1},{:.1},{:.1}) size=({:.1}x{:.1})",
                idx, obj_type, obj_rect.0, obj_rect.1, obj_rect.2, obj_rect.3, w, h);

            if let Some((min_x, min_y, max_x, max_y)) = content_bbox {
                content_bbox = Some((
                    min_x.min(obj_rect.0),
                    min_y.min(obj_rect.1),
                    max_x.max(obj_rect.2),
                    max_y.max(obj_rect.3)
                ));
            } else {
                content_bbox = Some(obj_rect);
            }
        } else {
            println!("Object[{}] type={:5} bounds=UNAVAILABLE", idx, obj_type);
        }
    }
    
    // Display object type summary
    println!();
    println!("Object summary:");
    println!("  Images: {}", image_count);
    println!("  Text:   {}", text_count);
    println!("  Paths:  {}", path_count);
    println!("  Forms:  {}", form_count);
    println!("  Other:  {}", other_count);

    // Display content bounding box and compute bbox_coverage
    println!();
    let mut bbox_coverage = 0.0_f64;
    if let Some((min_x, min_y, max_x, max_y)) = content_bbox {
        let content_w = max_x - min_x;
        let content_h = max_y - min_y;
        println!("Content Bounding Box (union of all objects):");
        println!("  left={:.2}, bottom={:.2}, right={:.2}, top={:.2}",
            min_x, min_y, max_x, max_y);
        println!("  width={:.2} pt, height={:.2} pt", content_w, content_h);

        // Compare to page size
        let page_w = page.width().value;
        let page_h = page.height().value;
        bbox_coverage = ((content_w * content_h) / (page_w * page_h)) as f64;
        println!("  coverage={:.3} ({:.1}% of page area)", bbox_coverage, bbox_coverage * 100.0);

        if bbox_coverage < 0.25 {
            println!("  ⚠️  WARNING: Content covers < 25% of page - may indicate offset/tiny thumbnail issue");
        }
    } else {
        println!("Content Bounding Box: NO OBJECTS FOUND");
        bbox_coverage = 0.0;
    }

    let pipeline = if image_count > 0 {
        println!();
        println!("Pipeline: Direct Extract (would attempt image extraction)");
        "Direct Extract (fallback to render)"
    } else {
        println!();
        println!("Pipeline: Render (no extractable images found)");
        "Render"
    };

    // Render requested page
    println!();
    println!("───────────────────────────────────────────────────────────");
    println!("RENDERING PAGE {}", page_num);
    println!("───────────────────────────────────────────────────────────");
    
    let scale = dpi as f64 / 72.0;
    
    let (use_box, width_pt, height_pt) = if has_crop {
        let crop_box = boundaries.crop().unwrap();
        ("CropBox", crop_box.bounds.width().value as f64, crop_box.bounds.height().value as f64)
    } else if has_trim {
        let trim_box = boundaries.trim().unwrap();
        ("TrimBox", trim_box.bounds.width().value as f64, trim_box.bounds.height().value as f64)
    } else {
        ("MediaBox", page.width().value as f64, page.height().value as f64)
    };
    
    println!("Using box: {}", use_box);
    println!("Box dimensions: {:.2} x {:.2} pt", width_pt, height_pt);
    println!("Scale factor: {:.3} (DPI {} / 72)", scale, dpi);
    
    let target_width_px = (width_pt * scale).round() as i32;
    let target_height_px = (height_pt * scale).round() as i32;
    
    println!("Target size: {} x {} px", target_width_px, target_height_px);

    let config = PdfRenderConfig::new()
        .set_target_width(target_width_px.max(1))
        .set_target_height(target_height_px.max(1));

    let bitmap = page
        .render_with_config(&config)
        .context("Failed to render page 1")?;

    let img = bitmap.as_image();
    
    println!("Rendered image: {} x {} px", img.width(), img.height());

    // Compute non-white pixel percentage
    println!();
    println!("───────────────────────────────────────────────────────────");
    println!("SANITY CHECK");
    println!("───────────────────────────────────────────────────────────");
    
    // Use external image crate, not our local module
    let thumbnail = ::image::imageops::resize(&img, 64, 64, ::image::imageops::FilterType::Lanczos3);
    let mut non_white_pixels = 0;
    let total_pixels = 64 * 64;
    
    for pixel in thumbnail.pixels() {
        let ::image::Rgba([r, g, b, _a]) = pixel;
        // Consider pixel non-white if any channel < 250
        if *r < 250 || *g < 250 || *b < 250 {
            non_white_pixels += 1;
        }
    }
    
    let non_white_pct = (non_white_pixels as f64 / total_pixels as f64) * 100.0;
    let white_ratio = 1.0 - (non_white_pct / 100.0);
    
    println!("Non-white pixels (64x64 thumbnail): {}/{} ({:.1}%)", 
        non_white_pixels, total_pixels, non_white_pct);
    println!("White ratio: {:.3} (threshold: {:.3})", white_ratio, max_white_ratio);
    
    // Check both thresholds
    let white_ratio_ok = white_ratio <= max_white_ratio;
    let bbox_coverage_ok = bbox_coverage >= min_bbox_coverage;
    let test_passed = white_ratio_ok && bbox_coverage_ok;

    if !white_ratio_ok {
        println!("❌ FAILED: White ratio {:.3} exceeds max {:.3}!", white_ratio, max_white_ratio);
    } else {
        println!("✓ White ratio: {:.3} <= {:.3} (OK)", white_ratio, max_white_ratio);
    }

    if !bbox_coverage_ok {
        println!("❌ FAILED: BBox coverage {:.3} ({:.1}%) below min {:.3} ({:.1}%)!",
            bbox_coverage, bbox_coverage * 100.0, min_bbox_coverage, min_bbox_coverage * 100.0);
    } else {
        println!("✓ BBox coverage: {:.3} ({:.1}%) >= {:.3} ({:.1}%) (OK)",
            bbox_coverage, bbox_coverage * 100.0, min_bbox_coverage, min_bbox_coverage * 100.0);
    }

    // Save debug image
    println!();
    println!("───────────────────────────────────────────────────────────");
    println!("SAVING DEBUG IMAGE");
    println!("───────────────────────────────────────────────────────────");

    img.save(output_path)
        .context("Failed to save debug image")?;

    let file_size_kb = std::fs::metadata(output_path)
        .map(|m| m.len() as f64 / 1024.0)
        .unwrap_or(0.0);

    println!("Saved: {:?} ({:.1} KB)", output_path, file_size_kb);

    // Summary
    println!();
    println!("═══════════════════════════════════════════════════════════");
    println!("SMOKE TEST SUMMARY");
    println!("═══════════════════════════════════════════════════════════");
    println!("Page:                {}", page_num);
    println!("Pipeline:            {}", pipeline);
    println!("Rendered size:       {} x {} px", img.width(), img.height());
    println!("Output file:         {:.1} KB", file_size_kb);
    println!();
    println!("Metrics:");
    println!("  white_ratio:       {:.3} / {:.3} {}", white_ratio, max_white_ratio, if white_ratio_ok { "✓" } else { "❌" });
    println!("  bbox_coverage:     {:.3} / {:.3} {}", bbox_coverage, min_bbox_coverage, if bbox_coverage_ok { "✓" } else { "❌" });
    println!("  non-white pixels:  {:.1}% ({})", non_white_pct, if non_white_pct < 1.0 { "⚠️ LOW" } else { "OK" });
    println!();

    if test_passed {
        println!("Status:              ✓✓✓ PASS (all checks passed)");
        println!("═══════════════════════════════════════════════════════════");
        Ok(())
    } else {
        println!("Status:              ❌ FAIL (see metrics above)");
        println!("═══════════════════════════════════════════════════════════");
        anyhow::bail!("Smoke test failed: white_ratio={:.3} (max={:.3}), bbox_coverage={:.3} (min={:.3})",
            white_ratio, max_white_ratio, bbox_coverage, min_bbox_coverage);
    }
}

fn run_benchmark(input_path: &PathBuf, dpi: u32, quality: u8, max_pages: u32) -> Result<()> {
    // Validate input
    if !input_path.exists() {
        anyhow::bail!("Input PDF file not found: {:?}", input_path);
    }

    println!("═══════════════════════════════════════════════════════════════");
    println!("CBZ COMPRESSION BENCHMARK");
    println!("═══════════════════════════════════════════════════════════════");
    println!("Input:   {:?}", input_path);
    println!("DPI:     {}", dpi);
    println!("Quality: {}", quality);
    if max_pages > 0 {
        println!("Max pages: {}", max_pages);
    }
    println!();

    // Read PDF
    let pdf_data = std::fs::read(&input_path)
        .context("Failed to read PDF file")?;

    println!("Step 1: Converting PDF to images...");
    let step1_start = Instant::now();

    // Convert to JPEG images (lossy mode - the common case)
    let images = convert_pdf_to_images_parallel(&pdf_data, dpi, quality, max_pages)
        .context("Failed to convert PDF to images")?;

    let step1_time = step1_start.elapsed();
    println!("  ✓ Converted {} pages in {:.2}s", images.len(), step1_time.as_secs_f64());

    // Calculate total image data size
    let total_image_size: usize = images.iter().map(|(_, data)| data.len()).sum();
    println!("  Total image data: {:.1} MB", total_image_size as f64 / 1024.0 / 1024.0);
    println!();

    // Run benchmark comparing compression methods
    benchmark::run_benchmark(&images);

    println!();
    println!("═══════════════════════════════════════════════════════════════");
    println!("TIMING BREAKDOWN");
    println!("═══════════════════════════════════════════════════════════════");
    println!("PDF→Images: {:.2}s (rendering + JPEG encoding)", step1_time.as_secs_f64());
    println!();
    println!("Note: With STORED mode, CBZ creation would be near-instant,");
    println!("since JPEG images are already optimally compressed.");
    println!("═══════════════════════════════════════════════════════════════");

    Ok(())
}
