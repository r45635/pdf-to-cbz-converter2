use std::time::Instant;
use pdf_to_cbz_converter::*;
use std::fs;

fn main() {
    println!("=== PDF to CBZ Conversion Performance Test ===\n");

    // Create or find a test PDF
    let test_pdf_path = "test_sample.pdf";

    if !std::path::Path::new(test_pdf_path).exists() {
        println!("Test PDF not found at: {}", test_pdf_path);
        println!("Please provide a test PDF file at: {}", test_pdf_path);
        return;
    }

    let pdf_data = match fs::read(test_pdf_path) {
        Ok(data) => {
            println!("✓ Loaded test PDF: {} bytes\n", data.len());
            data
        }
        Err(e) => {
            eprintln!("✗ Failed to read PDF: {}", e);
            return;
        }
    };

    // Test different DPI levels
    let dpi_levels = vec![150, 200, 300];

    for dpi in dpi_levels {
        println!("Testing conversion at {} DPI...", dpi);

        let start = Instant::now();

        match convert_pdf_to_images(&pdf_data, dpi) {
            Ok(images) => {
                let elapsed = start.elapsed();
                println!("  ✓ Converted {} pages", images.len());
                println!("  ⏱ Time: {:.2}s", elapsed.as_secs_f64());

                let total_size: u64 = images.iter().map(|(_, data)| data.len() as u64).sum();
                println!("  📊 Total size: {:.2} MB", total_size as f64 / (1024.0 * 1024.0));

                if images.len() > 0 {
                    let avg_size = total_size / images.len() as u64;
                    println!("  📄 Avg per page: {:.2} KB", avg_size as f64 / 1024.0);
                }
                println!("  ⚡ Throughput: {:.1} pages/sec\n",
                    images.len() as f64 / elapsed.as_secs_f64());
            }
            Err(e) => {
                println!("  ✗ Conversion failed: {}\n", e);
            }
        }
    }

    println!("=== Performance Test Complete ===");
}

fn convert_pdf_to_images(pdf_data: &[u8], dpi: u32) -> Result<Vec<(String, Vec<u8>)>, String> {
    use crate::models::ImageFormat;

    let pdfium = pdfium_render::prelude::Pdfium::default();
    let document = pdfium
        .load_pdf_from_byte_vec(pdf_data.to_vec(), None)
        .map_err(|e| format!("Failed to load PDF: {}", e))?;

    let page_count = document.pages().len();
    if page_count == 0 {
        return Err("PDF has no pages".to_string());
    }

    let mut images = Vec::new();

    for page_num in 1..=page_count as u32 {
        match render_pdf_page_sync(pdf_data, page_num, dpi, ImageFormat::Png, 95) {
            Ok(img_data) => {
                let filename = format!("page_{:04}.png", page_num);
                images.push((filename, img_data));
            }
            Err(e) => {
                return Err(format!("Failed to render page {}: {}", page_num, e));
            }
        }
    }

    Ok(images)
}
