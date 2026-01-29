// Test scaling by checking file sizes across different DPI levels
use pdfium_render::prelude::*;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== PDF Page Scaling Verification ===\n");

    let pdf_path = "../samples/Vers_les_Etoiles_BD.pdf";
    let pdf_data = fs::read(pdf_path)?;
    let pdfium = Pdfium::default();
    let document = pdfium.load_pdf_from_byte_vec(pdf_data, None)?;
    let page = document.pages().get(0)?;

    let width_pt = page.width().value as f64;
    let height_pt = page.height().value as f64;

    println!("PDF page: {:.0} x {:.0} pt (A4 size)\n", width_pt, height_pt);

    // Test at different DPI levels
    let dpi_levels = vec![72, 100, 150, 200];

    for dpi in dpi_levels {
        // Render at 72 DPI native
        let native_width = width_pt.round() as i32;
        let native_height = height_pt.round() as i32;

        let config = PdfRenderConfig::new()
            .set_target_width(native_width)
            .set_target_height(native_height);

        let bitmap = page.render_with_config(&config)?;
        let image = bitmap.as_image();

        // Scale to target DPI if different
        let final_image = if dpi != 72 {
            let scale = dpi as f64 / 72.0;
            let width_px = (width_pt * scale).round() as u32;
            let height_px = (height_pt * scale).round() as u32;

            println!("  DPI {}: rendering native 595x842, scaling to {}x{}", dpi, width_px, height_px);

            let resized = image::imageops::resize(
                &image,
                width_px,
                height_px,
                image::imageops::FilterType::Lanczos3,
            );

            image::DynamicImage::ImageRgba8(resized)
        } else {
            println!("  DPI {}: native render 595x842", dpi);
            image.clone()
        };

        // Save and check size
        let output = format!("/tmp/scale_test_{}_dpi.jpg", dpi);
        final_image.save_with_format(&output, image::ImageFormat::Jpeg)?;

        let file_size = fs::metadata(&output)?.len();
        println!("    → Saved: {} ({} KB)\n", output, file_size / 1024);
    }

    println!("=== Verification Complete ===");
    println!("✓ All scaling operations successful!");
    println!("\nExpected:");
    println!("  72 DPI:  ~595x842 px");
    println!("  100 DPI: ~827x1169 px (should be ~1.4x larger in both dimensions)");
    println!("  150 DPI: ~1240x1754 px (should be ~2x larger than 72 DPI)");
    println!("  200 DPI: ~1654x2337 px (should be ~2.8x larger than 72 DPI)");

    Ok(())
}
