use pdf_to_cbz_lib::utils;
use std::fs;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== PDF to CBZ Conversion Test ===\n");

    let pdf_path = "../samples/Vers_les_Etoiles_BD.pdf";

    if !Path::new(pdf_path).exists() {
        eprintln!("Error: PDF file not found at {}", pdf_path);
        return Err("PDF not found".into());
    }

    println!("Testing with: {}", pdf_path);

    // Read PDF and test rendering at 72 DPI (native)
    let pdf_data = fs::read(pdf_path)?;
    println!("PDF size: {} KB\n", pdf_data.len() / 1024);

    println!("Rendering page 1 at 72 DPI...");
    let page1_72dpi = utils::render_pdf_page_sync(
        &pdf_data,
        1,
        72,
        pdf_to_cbz_lib::models::ImageFormat::Jpeg,
        85,
    )?;
    println!("✓ Page 1 at 72 DPI: {} KB", page1_72dpi.len() / 1024);

    println!("\nRendering page 1 at 100 DPI...");
    let page1_100dpi = utils::render_pdf_page_sync(
        &pdf_data,
        1,
        100,
        pdf_to_cbz_lib::models::ImageFormat::Jpeg,
        85,
    )?;
    println!("✓ Page 1 at 100 DPI: {} KB", page1_100dpi.len() / 1024);

    println!("\nRendering page 1 at 150 DPI...");
    let page1_150dpi = utils::render_pdf_page_sync(
        &pdf_data,
        1,
        150,
        pdf_to_cbz_lib::models::ImageFormat::Jpeg,
        85,
    )?;
    println!("✓ Page 1 at 150 DPI: {} KB", page1_150dpi.len() / 1024);

    println!("\nRendering page 2 at 150 DPI...");
    let page2_150dpi = utils::render_pdf_page_sync(
        &pdf_data,
        2,
        150,
        pdf_to_cbz_lib::models::ImageFormat::Jpeg,
        85,
    )?;
    println!("✓ Page 2 at 150 DPI: {} KB", page2_150dpi.len() / 1024);

    // Create a minimal CBZ with these pages
    println!("\nCreating test CBZ with 2 pages at 150 DPI...");
    let images = vec![
        ("page_0001.jpg".to_string(), page1_150dpi),
        ("page_0002.jpg".to_string(), page2_150dpi),
    ];

    let cbz_data = utils::create_cbz(images)?;
    println!("✓ CBZ created: {} KB", cbz_data.len() / 1024);

    // Save for inspection
    let output_path = "/tmp/test_conversion.cbz";
    fs::write(output_path, &cbz_data)?;
    println!("\nSaved to: {}", output_path);

    // Extract and verify dimensions
    println!("\nExtracting and verifying images...");
    let extracted = utils::extract_images_from_cbz(&cbz_data)?;
    println!("✓ Extracted {} images", extracted.len());

    for (i, img_data) in extracted.iter().enumerate() {
        println!("  Image {}: {} bytes", i + 1, img_data.len());
    }

    println!("\n=== Test Complete ===");
    println!("If images are ~2-3x larger at 150 DPI vs 72 DPI, scaling is working correctly!");

    Ok(())
}
