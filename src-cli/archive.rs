use anyhow::{Context, Result};
use std::io::{Cursor, Read, Write};
use zip::{ZipArchive, ZipWriter};
use zip::write::SimpleFileOptions;

/// Create CBZ (ZIP) archive from images
/// Uses STORED (no compression) because JPEG images are already optimally compressed
/// This is 50x+ faster with only ~4% larger files
pub fn create_cbz(images: Vec<(String, Vec<u8>)>) -> Result<Vec<u8>> {
    let buffer = Cursor::new(Vec::new());
    let mut zip = ZipWriter::new(buffer);

    // STORED = no compression - optimal for JPEG images which are already compressed
    // Benchmark: 53x faster, only 3.8% larger files
    let options = SimpleFileOptions::default()
        .compression_method(zip::CompressionMethod::Stored);

    for (filename, data) in images {
        zip.start_file(&filename, options)
            .context(format!("Failed to add file {}", filename))?;
        zip.write_all(&data)
            .context("Failed to write file data")?;
    }

    let buffer = zip.finish().context("Failed to finalize ZIP archive")?;
    Ok(buffer.into_inner())
}

/// Extract images from CBZ/CBR archive
pub fn extract_images(archive_data: &[u8]) -> Result<Vec<(String, Vec<u8>)>> {
    // Check for RAR magic bytes (supports both RAR 4.x and RAR 5.x)
    let is_rar = if archive_data.len() >= 8 {
        // RAR 5.x: Rar!\x1a\x07\x01\x00
        &archive_data[0..8] == b"Rar!\x1a\x07\x01\x00" ||
        // RAR 4.x: Rar!\x1a\x07\x00
        &archive_data[0..7] == b"Rar!\x1a\x07\x00"
    } else {
        false
    };

    if is_rar {
        extract_from_rar(archive_data)
    } else {
        extract_from_zip(archive_data)
    }
}

/// Extract images from ZIP archive (CBZ format)
fn extract_from_zip(archive_data: &[u8]) -> Result<Vec<(String, Vec<u8>)>> {
    let cursor = Cursor::new(archive_data);
    let mut archive = ZipArchive::new(cursor)
        .context("Failed to open ZIP archive")?;

    let mut images = Vec::new();

    for i in 0..archive.len() {
        let mut file = archive
            .by_index(i)
            .context(format!("Failed to read file at index {}", i))?;

        let file_name = file.name().to_string();

        // Skip non-image files
        if !is_image_file(&file_name) {
            continue;
        }

        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)
            .context("Failed to read file contents")?;

        images.push((file_name, buffer));
    }

    // Sort by filename to maintain page order
    images.sort_by(|a, b| a.0.cmp(&b.0));

    Ok(images)
}

/// Extract images from RAR archive (CBR format)
fn extract_from_rar(archive_data: &[u8]) -> Result<Vec<(String, Vec<u8>)>> {
    use std::process::Command;

    // RAR extraction requires external `unar` tool
    let temp_dir = std::env::temp_dir();
    let temp_cbr = temp_dir.join(format!("temp_{}.cbr", uuid::Uuid::new_v4()));
    let temp_extract_dir = temp_dir.join(format!("cbr_extract_{}", uuid::Uuid::new_v4()));

    // Write CBR to temporary file
    std::fs::write(&temp_cbr, archive_data)
        .context("Failed to write temporary CBR file")?;

    // Create extraction directory
    std::fs::create_dir_all(&temp_extract_dir)
        .context("Failed to create extraction directory")?;

    // Extract using unar
    let output = Command::new("unar")
        .arg("-o")
        .arg(&temp_extract_dir)
        .arg(&temp_cbr)
        .output()
        .context("Failed to execute unar command. Install with: brew install unar")?;

    if !output.status.success() {
        let error_msg = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("Failed to extract RAR archive: {}", error_msg);
    }

    // Read extracted images
    let mut images = Vec::new();

    // Recursively read all files from extraction directory
    fn read_images_recursive(dir: &std::path::Path, images: &mut Vec<(String, Vec<u8>)>) -> Result<()> {
        if let Ok(entries) = std::fs::read_dir(dir) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if path.is_dir() {
                        // Recurse into subdirectories
                        read_images_recursive(&path, images)?;
                    } else if path.is_file() {
                        let file_name = path
                            .file_name()
                            .context("Failed to get filename")?
                            .to_string_lossy()
                            .to_string();

                        if is_image_file(&file_name) {
                            let buffer = std::fs::read(&path)
                                .context(format!("Failed to read extracted file: {}", file_name))?;
                            images.push((file_name, buffer));
                        }
                    }
                }
            }
        }
        Ok(())
    }

    read_images_recursive(&temp_extract_dir, &mut images)?;

    // Cleanup
    let _ = std::fs::remove_file(&temp_cbr);
    let _ = std::fs::remove_dir_all(&temp_extract_dir);

    // Sort by filename
    images.sort_by(|a, b| a.0.cmp(&b.0));

    Ok(images)
}

/// Check if filename is an image file
fn is_image_file(filename: &str) -> bool {
    let lower = filename.to_lowercase();
    lower.ends_with(".jpg")
        || lower.ends_with(".jpeg")
        || lower.ends_with(".png")
        || lower.ends_with(".webp")
        || lower.ends_with(".gif")
}

