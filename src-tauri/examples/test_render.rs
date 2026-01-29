use pdfium_render::prelude::*;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pdfium = Pdfium::default();

    let pdf_data = fs::read("../samples/Vers_les_Etoiles_BD.pdf")?;
    let document = pdfium.load_pdf_from_byte_vec(pdf_data, None)?;

    let page = document.pages().get(0)?;

    let width_pt = page.width().value as f64;
    let height_pt = page.height().value as f64;

    println!("Page dimensions: {:.1} pt x {:.1} pt", width_pt, height_pt);

    // Test rendering at 72 DPI (native)
    let native_width_px = width_pt.round() as i32;
    let native_height_px = height_pt.round() as i32;

    println!("Native size (72 DPI): {} x {} px", native_width_px, native_height_px);

    let config = PdfRenderConfig::new()
        .set_target_width(native_width_px)
        .set_target_height(native_height_px);

    let bitmap = page.render_with_config(&config)?;
    let image = bitmap.as_image();

    println!("Rendered bitmap size: {} x {} px", image.width(), image.height());
    println!("Image matches target size: {}", image.width() as i32 == native_width_px && image.height() as i32 == native_height_px);

    // Save first page to check
    image.save("/tmp/test_page_0_72dpi.png")?;
    println!("Saved test page (72 DPI) to /tmp/test_page_0_72dpi.png");

    // Now test at 150 DPI with resizing
    let dpi = 150;
    let scale = dpi as f64 / 72.0;
    let width_px_150 = (width_pt * scale).round() as u32;
    let height_px_150 = (height_pt * scale).round() as u32;

    println!("\nAt {} DPI: {} x {} px (target)", dpi, width_px_150, height_px_150);

    let resized = image::imageops::resize(
        &image,
        width_px_150,
        height_px_150,
        image::imageops::FilterType::Lanczos3,
    );

    println!("Resized image: {} x {}", resized.width(), resized.height());

    let dyn_image = image::DynamicImage::ImageRgba8(resized);
    dyn_image.save("/tmp/test_page_0_150dpi.png")?;
    println!("Saved resized page (150 DPI) to /tmp/test_page_0_150dpi.png");

    Ok(())
}
