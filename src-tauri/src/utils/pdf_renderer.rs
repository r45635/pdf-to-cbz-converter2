use anyhow::{Context, Result};
use pdfium_render::prelude::*;
use crate::models::ImageFormat;

/// Render a single PDF page to image bytes in target format
pub async fn render_pdf_page(
    pdf_path: &str,
    page_num: u32,
    dpi: u32,
    format: ImageFormat,
    quality: u8,
) -> Result<Vec<u8>> {
    let pdf_data = tokio::fs::read(pdf_path)
        .await
        .context("Failed to read PDF file")?;

    render_page_from_bytes(&pdf_data, page_num, dpi, format, quality).await
}

/// Render a PDF page from bytes - async wrapper
pub async fn render_page_from_bytes(
    pdf_data: &[u8],
    page_num: u32,
    dpi: u32,
    format: ImageFormat,
    quality: u8,
) -> Result<Vec<u8>> {
    // pdfium-render is synchronous, so we use spawn_blocking
    let pdf_data = pdf_data.to_vec();

    tokio::task::spawn_blocking(move || {
        render_pdf_page_sync(&pdf_data, page_num, dpi, format, quality)
    })
    .await
    .context("Task join error")?
}

/// Synchronous PDF page rendering for parallel processing
/// Uses pdfium-render
pub fn render_pdf_page_sync(
    pdf_data: &[u8],
    page_num: u32,
    dpi: u32,
    format: ImageFormat,
    _quality: u8,
) -> Result<Vec<u8>> {
    let pdfium = crate::utils::bind_pdfium()?;

    let document = pdfium
        .load_pdf_from_byte_vec(pdf_data.to_vec(), None)
        .context("Failed to load PDF document")?;

    // Get page (page_num is 1-indexed, but pdfium uses 0-indexed)
    let page = document
        .pages()
        .get((page_num - 1) as u16)
        .context(format!("Page {} not found", page_num))?;

    // Calculate pixel dimensions based on DPI
    // PDFium uses 72 DPI by default in its coordinate system
    let width_pt = page.width().value as f64;
    let height_pt = page.height().value as f64;

    // First render at 72 DPI (native, no scaling)
    // This ensures we get the full page content
    let native_width_px = width_pt.round() as i32;
    let native_height_px = height_pt.round() as i32;

    let config = PdfRenderConfig::new()
        .set_target_width(native_width_px)
        .set_target_height(native_height_px);

    let bitmap = page
        .render_with_config(&config)
        .context("Failed to render page")?;

    // Convert bitmap to image
    let image = bitmap.as_image();

    // Now scale to the target DPI if different from 72
    // Use CatmullRom for better performance (~2-3x faster than Lanczos3)
    let final_image = if dpi != 72 {
        let scale = dpi as f64 / 72.0;
        let width_px = (width_pt * scale).round() as u32;
        let height_px = (height_pt * scale).round() as u32;

        image::DynamicImage::ImageRgba8(image::imageops::resize(
            &image,
            width_px,
            height_px,
            image::imageops::FilterType::CatmullRom,
        ))
    } else {
        image.clone()
    };

    let mut image_data = Vec::new();

    // Encode to target format
    if matches!(format, ImageFormat::Jpeg) {
        final_image
            .write_to(&mut std::io::Cursor::new(&mut image_data), image::ImageFormat::Jpeg)
            .context("Failed to encode JPEG")?;
    } else {
        // PNG encoding
        final_image
            .write_to(&mut std::io::Cursor::new(&mut image_data), image::ImageFormat::Png)
            .context("Failed to encode PNG")?;
    }

    Ok(image_data)
}
