use anyhow::{Context, Result};
use pdfium_render::prelude::*;
use std::io::Cursor;
use rayon::prelude::*;
use image::{ImageEncoder, GenericImageView};
use pdf_conversion_lib::{bind_pdfium, find_best_image_candidate, log_page_diagnostic, extract_image_bytes, extract_image_bytes_as_jpeg};

/// Extract images from PDF with PNG lossless encoding at specified DPI
/// Uses Direct Extract pipeline: high-quality image extraction if available,
/// otherwise falls back to full-page rendering at the specified DPI as PNG
pub fn extract_images_lossless_at_dpi(pdf_data: &[u8], dpi: u32, max_pages: u32) -> Result<Vec<(String, Vec<u8>)>> {
    let effective_dpi = if dpi == 0 { 300 } else { dpi };

    let pdfium = bind_pdfium()
        .context("Failed to initialize Pdfium")?;
    let document = pdfium
        .load_pdf_from_byte_vec(pdf_data.to_vec(), None)
        .context("Failed to load PDF")?;

    let page_count = document.pages().len() as u32;
    if page_count == 0 {
        anyhow::bail!("PDF has no pages");
    }

    let pages_to_process = if max_pages > 0 && max_pages < page_count {
        max_pages
    } else {
        page_count
    };

    let mut images = Vec::new();

    // Process each page
    for page_num in 1..=pages_to_process {
        let page = document
            .pages()
            .get((page_num - 1) as u16)
            .context(format!("Failed to get page {}", page_num))?;

        // Try Direct Extract pipeline: find best image candidate
        let (best_candidate, crop_bounds) = find_best_image_candidate(&page)?;

        if let Some(candidate) = &best_candidate {
            // Try to extract the image
            match extract_image_bytes(&page, candidate.object_index) {
                Ok(image_bytes) => {
                    log_page_diagnostic(page_num, &best_candidate, crop_bounds, false);
                    let filename = format!("page_{:04}.png", page_num);
                    images.push((filename, image_bytes));
                    continue;
                }
                Err(e) => {
                    eprintln!("[EXTRACT] Failed to extract image: {}", e);
                    eprintln!("[EXTRACT] Falling back to full-page render");
                }
            }
        }

        // Fallback: render entire page at specified DPI as PNG
        log_page_diagnostic(page_num, &None, crop_bounds, true);

        // Render at specified DPI for lossless mode
        let width_pt = page.width().value as f64;
        let height_pt = page.height().value as f64;

        let scale = effective_dpi as f64 / 72.0;
        let target_width_px = (width_pt * scale).round() as i32;
        let target_height_px = (height_pt * scale).round() as i32;

        let config = PdfRenderConfig::new()
            .set_target_width(target_width_px.max(1))
            .set_target_height(target_height_px.max(1));

        let bitmap = page
            .render_with_config(&config)
            .context(format!("Failed to render page {}", page_num))?;

        let image = bitmap.as_image();

        // Encode as PNG (lossless)
        let rgb_image = image.to_rgb8();
        let mut png_data = Vec::new();
        let encoder = image::codecs::png::PngEncoder::new(&mut png_data);
        encoder.write_image(
            rgb_image.as_raw(),
            rgb_image.width(),
            rgb_image.height(),
            image::ExtendedColorType::Rgb8,
        ).context(format!("Failed to encode rendered page {} as PNG", page_num))?;

        let filename = format!("page_{:04}.png", page_num);
        images.push((filename, png_data));
    }

    if images.is_empty() {
        anyhow::bail!("No images could be extracted from PDF");
    }

    Ok(images)
}

/// Extract original images from PDF (legacy lossless mode)
/// DEPRECATED: Use extract_images_lossless_at_dpi instead
#[allow(dead_code)]
pub fn extract_images_lossless(pdf_data: &[u8]) -> Result<Vec<(String, Vec<u8>)>> {
    let pdfium = bind_pdfium()
        .context("Failed to initialize Pdfium")?;
    let document = pdfium
        .load_pdf_from_byte_vec(pdf_data.to_vec(), None)
        .context("Failed to load PDF")?;

    let page_count = document.pages().len();
    if page_count == 0 {
        anyhow::bail!("PDF has no pages");
    }

    let mut images = Vec::new();

    // Process each page
    for page_num in 1..=page_count as u32 {
        let page = document
            .pages()
            .get((page_num - 1) as u16)
            .context(format!("Failed to get page {}", page_num))?;

        // Try Direct Extract pipeline: find best image candidate
        let (best_candidate, crop_bounds) = find_best_image_candidate(&page)?;

        if let Some(candidate) = &best_candidate {
            // Try to extract the image
            match extract_image_bytes(&page, candidate.object_index) {
                Ok(image_bytes) => {
                    log_page_diagnostic(page_num, &best_candidate, crop_bounds, false);
                    let filename = format!("page_{:04}.png", page_num);
                    images.push((filename, image_bytes));
                    continue;
                }
                Err(e) => {
                    eprintln!("[EXTRACT] Failed to extract image: {}", e);
                    eprintln!("[EXTRACT] Falling back to full-page render");
                }
            }
        }

        // Fallback: render entire page at native resolution
        log_page_diagnostic(page_num, &None, crop_bounds, true);

        // Render at 72 DPI (native PDF resolution) for true lossless
        let width_pt = page.width().value as i32;
        let height_pt = page.height().value as i32;

        let config = PdfRenderConfig::new()
            .set_target_width(width_pt)
            .set_target_height(height_pt);

        let bitmap = page
            .render_with_config(&config)
            .context(format!("Failed to render page {}", page_num))?;

        let image = bitmap.as_image();

        // Encode as PNG (lossless)
        let rgb_image = image.to_rgb8();
        let mut png_data = Vec::new();
        let encoder = image::codecs::png::PngEncoder::new(&mut png_data);
        encoder.write_image(
            rgb_image.as_raw(),
            rgb_image.width(),
            rgb_image.height(),
            image::ExtendedColorType::Rgb8,
        ).context(format!("Failed to encode rendered page {} as PNG", page_num))?;

        let filename = format!("page_{:04}.png", page_num);
        images.push((filename, png_data));
    }

    if images.is_empty() {
        anyhow::bail!("No images could be extracted from PDF");
    }

    Ok(images)
}

/// Convert PDF bytes to images at specified DPI with quality control
pub fn convert_pdf_to_images(pdf_data: &[u8], dpi: u32, quality: u8) -> Result<Vec<(String, Vec<u8>)>> {
    let effective_dpi = if dpi == 0 { 300 } else { dpi };

    let pdfium = bind_pdfium()
        .context("Failed to initialize Pdfium")?;
    let document = pdfium
        .load_pdf_from_byte_vec(pdf_data.to_vec(), None)
        .context("Failed to load PDF")?;

    let page_count = document.pages().len();
    if page_count == 0 {
        anyhow::bail!("PDF has no pages");
    }

    let mut images = Vec::new();

    // Render each page sequentially
    for page_num in 1..=page_count as u32 {
        let page = document
            .pages()
            .get((page_num - 1) as u16)
            .context(format!("Failed to get page {}", page_num))?;

        // Get page dimensions
        let width_pt = page.width().value as f64;
        let height_pt = page.height().value as f64;

        // Render at native 72 DPI first
        let native_width_px = width_pt.round() as i32;
        let native_height_px = height_pt.round() as i32;

        let config = PdfRenderConfig::new()
            .set_target_width(native_width_px)
            .set_target_height(native_height_px);

        let bitmap = page
            .render_with_config(&config)
            .context(format!("Failed to render page {}", page_num))?;

        let mut image = bitmap.as_image().clone();

        // Scale to target DPI if needed
        // Use CatmullRom for better performance (~2-3x faster than Lanczos3)
        if effective_dpi != 72 {
            let scale = effective_dpi as f64 / 72.0;
            let width_px = (width_pt * scale).round() as u32;
            let height_px = (height_pt * scale).round() as u32;

            image = image::DynamicImage::ImageRgba8(image::imageops::resize(
                &image,
                width_px,
                height_px,
                image::imageops::FilterType::CatmullRom,
            ));
        }

        // Encode as JPEG with specified quality
        let rgb_image = image.to_rgb8();
        let mut jpeg_data = Vec::new();
        
        let mut encoder = image::codecs::jpeg::JpegEncoder::new_with_quality(
            &mut jpeg_data,
            quality
        );
        
        encoder
            .encode(
                rgb_image.as_raw(),
                rgb_image.width(),
                rgb_image.height(),
                image::ExtendedColorType::Rgb8,
            )
            .context(format!("Failed to encode page {} as JPEG", page_num))?;

        let filename = format!("page_{:04}.jpg", page_num);
        images.push((filename, jpeg_data));
    }

    Ok(images)
}

/// Convert PDF bytes to images at specified DPI with quality control (PARALLEL VERSION)
pub fn convert_pdf_to_images_parallel(pdf_data: &[u8], dpi: u32, quality: u8, max_pages: u32) -> Result<Vec<(String, Vec<u8>)>> {
    let effective_dpi = if dpi == 0 { 300 } else { dpi };

    let pdfium = bind_pdfium()
        .context("Failed to initialize Pdfium")?;
    let document = pdfium
        .load_pdf_from_byte_vec(pdf_data.to_vec(), None)
        .context("Failed to load PDF")?;

    let page_count = document.pages().len();
    if page_count == 0 {
        anyhow::bail!("PDF has no pages");
    }

    let pages_to_process = if max_pages > 0 && max_pages < page_count as u32 {
        max_pages
    } else {
        page_count as u32
    };

    // Process pages sequentially (pdfium operations must be sequential)
    // First try direct extraction (JPEG direct, no PNG intermediate!), then fallback to render
    let mut extracted_pages = Vec::new(); // Direct JPEG bytes (fast path)
    let mut pages_to_render = Vec::new();  // Pages that need rendering

    for page_num in 1..=pages_to_process {
        let page = document
            .pages()
            .get((page_num - 1) as u16)
            .context(format!("Failed to get page {}", page_num))?;

        // Get page dimensions
        let width_pt = page.width().value as f64;
        let height_pt = page.height().value as f64;

        // Try direct extraction first
        let (best_candidate, _crop_bounds) = find_best_image_candidate(&page)?;

        let extraction_success = if let Some(candidate) = best_candidate {
            // Try to extract the image DIRECTLY as JPEG (OPTIMIZED - no PNG intermediate!)
            match extract_image_bytes_as_jpeg(&page, candidate.object_index, quality) {
                Ok(jpeg_bytes) => {
                    // Successfully extracted as JPEG - add to fast path
                    let filename = format!("page_{:04}.jpg", page_num);
                    extracted_pages.push((filename, jpeg_bytes));
                    true
                }
                Err(_) => false // Fall through to render
            }
        } else {
            false // Fall through to render
        };

        // If extraction failed, queue for rendering
        if !extraction_success {
            pages_to_render.push((page_num, width_pt, height_pt));
        }
    }

    // Render pages that didn't have extractable images
    let mut rendered_images = Vec::new();
    for (page_num, width_pt, height_pt) in pages_to_render {
        let page = document
            .pages()
            .get((page_num - 1) as u16)
            .context(format!("Failed to get page {}", page_num))?;

        let native_width_px = width_pt.round() as i32;
        let native_height_px = height_pt.round() as i32;
        let config = PdfRenderConfig::new()
            .set_target_width(native_width_px)
            .set_target_height(native_height_px);
        let bitmap = page
            .render_with_config(&config)
            .context(format!("Failed to render page {}", page_num))?;

        let image = bitmap.as_image().clone();
        rendered_images.push((page_num, width_pt, height_pt, image));
    }

    // Process rendered images in parallel (scaling + encoding)
    let rendered_results: Vec<_> = rendered_images
        .into_par_iter()
        .map(|(page_num, width_pt, height_pt, mut image)| {
            // Scale to target DPI if needed
            // Use CatmullRom for better performance (~2-3x faster than Lanczos3)
            if effective_dpi != 72 {
                let scale = effective_dpi as f64 / 72.0;
                let width_px = (width_pt * scale).round() as u32;
                let height_px = (height_pt * scale).round() as u32;

                image = image::DynamicImage::ImageRgba8(image::imageops::resize(
                    &image,
                    width_px,
                    height_px,
                    image::imageops::FilterType::CatmullRom,
                ));
            }

            // Encode as JPEG with specified quality
            let rgb_image = image.to_rgb8();
            let mut jpeg_data = Vec::new();

            let mut encoder = image::codecs::jpeg::JpegEncoder::new_with_quality(
                &mut jpeg_data,
                quality
            );

            encoder
                .encode(
                    rgb_image.as_raw(),
                    rgb_image.width(),
                    rgb_image.height(),
                    image::ExtendedColorType::Rgb8,
                )
                .context(format!("Failed to encode page {} as JPEG", page_num))?;

            let filename = format!("page_{:04}.jpg", page_num);
            Ok::<_, anyhow::Error>((page_num, filename, jpeg_data))
        })
        .collect();

    // Combine extracted pages + rendered pages, maintaining order
    let mut all_pages: Vec<(u32, String, Vec<u8>)> = Vec::new();

    // Add extracted pages
    for (filename, jpeg_data) in extracted_pages {
        // Parse page number from filename
        let page_num: u32 = filename.split('_').nth(1)
            .and_then(|s| s.split('.').next())
            .and_then(|s| s.parse().ok())
            .unwrap_or(0);
        all_pages.push((page_num, filename, jpeg_data));
    }

    // Add rendered pages
    for result in rendered_results {
        let (page_num, filename, jpeg_data) = result?;
        all_pages.push((page_num, filename, jpeg_data));
    }

    // Sort by page number to maintain order
    all_pages.sort_by_key(|k| k.0);

    // Extract final image data
    let mut images = Vec::new();
    for (_, filename, jpeg_data) in all_pages {
        images.push((filename, jpeg_data));
    }

    Ok(images)
}

/// Create PDF from image bytes
pub fn create_pdf_from_images(images: Vec<(String, Vec<u8>)>) -> Result<Vec<u8>> {
    use printpdf::{Mm, PdfDocument};

    if images.is_empty() {
        anyhow::bail!("No images to convert");
    }

    let (document, page1, layer1) =
        PdfDocument::new("CBZ to PDF", Mm(210.0), Mm(297.0), "Layer 1");

    // Add first image
    if let Some((_, image_data)) = images.first() {
        add_image_to_page(&document, page1, layer1, image_data)?;
    }

    // Add remaining images
    for (_, image_data) in images.iter().skip(1) {
        let (page, layer) = document.add_page(Mm(210.0), Mm(297.0), "Page");
        add_image_to_page(&document, page, layer, image_data)?;
    }

    // Serialize to bytes
    let pdf_bytes = document
        .save_to_bytes()
        .map_err(|e| anyhow::anyhow!("Failed to serialize PDF: {:?}", e))?;

    Ok(pdf_bytes)
}

/// Add image to PDF page (A4: 210×297mm)
fn add_image_to_page(
    document: &printpdf::PdfDocumentReference,
    page_id: printpdf::PdfPageIndex,
    layer_id: printpdf::PdfLayerIndex,
    image_data: &[u8],
) -> Result<()> {
    use image::ImageFormat;
    use printpdf::{Image, ImageTransform, Mm};

    // Fast dimension reading
    let (img_width, img_height) = if let Ok(size) = imagesize::blob_size(image_data) {
        (size.width as u32, size.height as u32)
    } else {
        // Fallback: decode image to get dimensions
        let reader = image::ImageReader::new(Cursor::new(image_data));
        let reader_with_format = reader.with_guessed_format()
            .context("Failed to detect image format")?;
        let img = reader_with_format.decode()
            .context("Failed to decode image")?;
        img.dimensions()
    };

    // Get layer
    let layer = document.get_page(page_id).get_layer(layer_id);

    // Try direct JPEG insertion (no decode)
    if let Ok(size) = imagesize::blob_size(image_data) {
        let reader = image::ImageReader::new(Cursor::new(image_data));
        if let Ok(reader_with_format) = reader.with_guessed_format() {
            if matches!(reader_with_format.format(), Some(ImageFormat::Jpeg)) {
                // Direct JPEG insertion (fast!)
                let image_xobject = printpdf::ImageXObject {
                    width: printpdf::Px(size.width),
                    height: printpdf::Px(size.height),
                    color_space: printpdf::ColorSpace::Rgb,
                    bits_per_component: printpdf::ColorBits::Bit8,
                    image_data: image_data.to_vec(),
                    image_filter: Some(printpdf::ImageFilter::DCT),
                    interpolate: true,
                    smask: None,
                    clipping_bbox: None,
                };

                let image = Image::from(image_xobject);
                let dpi = calculate_dpi(size.width as f32, size.height as f32);

                let transform = ImageTransform {
                    translate_x: Some(Mm(0.0)),
                    translate_y: Some(Mm(0.0)),
                    dpi: Some(dpi),
                    rotate: None,
                    ..Default::default()
                };

                image.add_to_layer(layer, transform);
                return Ok(());
            }
        }
    }

    // Fallback: decode and convert
    let reader = image::ImageReader::new(Cursor::new(image_data));
    let reader_with_format = reader
        .with_guessed_format()
        .context("Failed to detect image format")?;
    let img = reader_with_format
        .decode()
        .context("Failed to decode image")?;

    let rgb_img = img.to_rgb8();
    let image_xobject = printpdf::ImageXObject {
        width: printpdf::Px(img_width as usize),
        height: printpdf::Px(img_height as usize),
        color_space: printpdf::ColorSpace::Rgb,
        bits_per_component: printpdf::ColorBits::Bit8,
        image_data: rgb_img.to_vec(),
        image_filter: None,
        interpolate: true,
        smask: None,
        clipping_bbox: None,
    };

    let image = Image::from(image_xobject);
    let dpi = calculate_dpi(img_width as f32, img_height as f32);

    let transform = ImageTransform {
        translate_x: Some(Mm(0.0)),
        translate_y: Some(Mm(0.0)),
        dpi: Some(dpi),
        rotate: None,
        ..Default::default()
    };

    image.add_to_layer(layer, transform);
    Ok(())
}

/// Ultra-fast conversion using Ghostscript for rendering + direct extraction
/// Ghostscript can render 270 pages in ~5s instead of 150s with PDFium
pub fn convert_pdf_optimized_ghostscript(pdf_data: &[u8], dpi: u32, quality: u8, max_pages: u32) -> Result<Vec<(String, Vec<u8>)>> {
    use std::fs;
    use std::process::Command;
    use tempfile::TempDir;

    let effective_dpi = if dpi == 0 { 300 } else { dpi };
    eprintln!("[GS-FAST] Starting optimized Ghostscript rendering at {} DPI", effective_dpi);

    // Create temp directory for Ghostscript output
    let temp_dir = TempDir::new().context("Failed to create temp directory")?;
    let temp_path = temp_dir.path();

    // Write PDF to temp file
    let pdf_path = temp_path.join("input.pdf");
    fs::write(&pdf_path, pdf_data).context("Failed to write PDF to temp file")?;

    // Render ALL pages in one batch command with Ghostscript (ULTRA FAST!)
    let output_template = temp_path.join("page_%04d.jpg");
    let output_str = output_template.to_str().context("Invalid temp path")?;

    eprintln!("[GS-FAST] Rendering PDF with Ghostscript at {} DPI...", effective_dpi);
    let gs_quality = ((quality as f32 / 100.0) * 100.0) as i32;

    let start = std::time::Instant::now();
    let output = Command::new("gs")
        .args(&[
            "-q",                           // Quiet mode
            "-dNOPAUSE",                     // Don't pause between pages
            "-dBATCH",                       // Exit after processing
            "-dSAFER",                       // Security restrictions
            "-sDEVICE=jpeg",                 // JPEG output
            &format!("-dJPEGQ={}", gs_quality), // JPEG quality
            &format!("-r{}x{}", effective_dpi, effective_dpi), // DPI
            "-dTextAlphaBits=4",
            "-dGraphicsAlphaBits=4",
            &format!("-sOutputFile={}", output_str),
            pdf_path.to_str().context("Invalid PDF path")?,
        ])
        .output()
        .context("Failed to run Ghostscript")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!("[GS-FAST] Ghostscript stderr: {}", stderr);
        anyhow::bail!("Ghostscript rendering failed");
    }

    let gs_time = start.elapsed().as_secs_f64();
    eprintln!("[GS-FAST] Ghostscript rendered {} pages in {:.2}s", max_pages, gs_time);

    // Load rendered pages from Ghostscript output
    let mut images = Vec::new();
    let effective_max = if max_pages == 0 { 9999 } else { max_pages };

    for page_num in 1..=effective_max as u32 {
        let page_file = temp_path.join(format!("page_{:04}.jpg", page_num));
        if !page_file.exists() {
            break; // No more pages
        }

        let jpeg_data = fs::read(&page_file)
            .context(format!("Failed to read rendered page {}", page_num))?;

        let filename = format!("page_{:04}.jpg", page_num);
        images.push((filename, jpeg_data));
    }

    eprintln!("[GS-FAST] ✓ Loaded {} pages from Ghostscript", images.len());
    Ok(images)
}

/// Calculate optimal DPI to fit image on A4 page (210×297mm)
fn calculate_dpi(img_width: f32, img_height: f32) -> f32 {
    // A4 dimensions: 210mm × 297mm = 8.27" × 11.69"
    let page_width_inch = 210.0 / 25.4;
    let page_height_inch = 297.0 / 25.4;

    let dpi_w = img_width / page_width_inch;
    let dpi_h = img_height / page_height_inch;

    dpi_w.max(dpi_h)
}
