use anyhow::Result;
use image::GenericImageView;
use std::io::Cursor;

/// Get image dimensions without full decoding (header-only reading)
pub fn get_dimensions(image_data: &[u8]) -> Result<(u32, u32)> {
    if let Ok(size) = imagesize::blob_size(image_data) {
        Ok((size.width as u32, size.height as u32))
    } else {
        // Fallback: decode image
        let img = image::ImageReader::new(Cursor::new(image_data))
            .with_guessed_format()?
            .decode()?;
        Ok(img.dimensions())
    }
}
