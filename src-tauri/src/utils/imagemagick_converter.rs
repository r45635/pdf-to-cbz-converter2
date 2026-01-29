use std::process::Command;
use std::path::Path;
use crate::models::ImageFormat;

/// Convert PDF to images using ImageMagick (simplest & best approach)
/// ImageMagick automatically determines optimal resolution for quality
pub fn convert_pdf_with_imagemagick(
    pdf_path: &str,
    output_dir: &str,
    dpi: u32,
) -> Result<Vec<(String, Vec<u8>)>, String> {
    let path = Path::new(pdf_path);
    if !path.exists() {
        return Err(format!("PDF file not found: {}", pdf_path));
    }

    // Check if ImageMagick is available
    let check_convert = Command::new("convert")
        .arg("--version")
        .output();

    if check_convert.is_err() {
        return Err("ImageMagick 'convert' not found. Install with: brew install imagemagick".to_string());
    }

    eprintln!("[IMG] Converting PDF with ImageMagick at {} DPI...", dpi);

    // Use ImageMagick to convert PDF to PNG images
    // ImageMagick's 'convert' handles DPI/density automatically
    let output = Command::new("convert")
        .arg("-density").arg(dpi.to_string())  // Set output density
        .arg("-quality").arg("98")             // Quality for intermediate processing
        .arg(pdf_path)                          // Input PDF
        .arg(format!("{}/%04d.png", output_dir)) // Output pattern (PNG lossless)
        .output()
        .map_err(|e| format!("ImageMagick conversion failed: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("ImageMagick error: {}", stderr));
    }

    // Read all generated PNG files
    let mut images = Vec::new();
    let mut page_num = 1;

    loop {
        let img_path = format!("{}/{:04}.png", output_dir, page_num);
        let img_file = Path::new(&img_path);

        if !img_file.exists() {
            break;
        }

        let img_data = std::fs::read(&img_path)
            .map_err(|e| format!("Failed to read image {}: {}", page_num, e))?;

        let img_size_kb = img_data.len() as f64 / 1024.0;
        eprintln!("[IMG] Page {}: {:.1}KB", page_num, img_size_kb);

        let filename = format!("page_{:04}.png", page_num);
        images.push((filename, img_data));
        page_num += 1;
    }

    if images.is_empty() {
        return Err("No images generated from PDF".to_string());
    }

    eprintln!("[IMG] ✓ Converted {} pages with ImageMagick", images.len());
    Ok(images)
}

/// Simple check if ImageMagick is available
pub fn is_imagemagick_available() -> bool {
    Command::new("convert")
        .arg("--version")
        .output()
        .is_ok()
}
