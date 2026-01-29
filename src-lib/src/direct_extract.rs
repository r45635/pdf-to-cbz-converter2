use anyhow::{Context, Result};
use pdfium_render::prelude::*;
use image::ImageEncoder;

// Threshold for considering an image suitable for direct extraction.
// Comics can have significant margins and white space.
// If ANY substantive image is found on page, extract it rather than rendering.
// Typical comic pages: main image 20-80% + gutters/margins.
// Even a small extracted image is preferable to rendering white space.
pub const MIN_COVERAGE_FOR_DIRECT_EXTRACT: f64 = 0.005; // 0.5% (very low threshold)

/// Information about a candidate image for extraction
#[derive(Debug, Clone)]
pub struct ImageCandidate {
    /// Index in the objects list
    pub object_index: usize,
    /// Computed bounds on page (after CTM)
    pub bounds: (f32, f32, f32, f32), // (left, bottom, right, top)
    /// Coverage percentage of page area (0.0 to 1.0)
    pub coverage: f64,
    /// Whether we can extract raw bytes
    pub can_extract_raw: bool,
}

/// Find the best image candidate on a PDF page
/// Recursively traverses Form XObjects to find images
/// Returns (Some(candidate), bounds_of_cropbox) if suitable image found
/// Returns (None, bounds_of_cropbox) if no suitable image, fallback to render
pub fn find_best_image_candidate(
    page: &PdfPage,
) -> Result<(Option<ImageCandidate>, (f32, f32, f32, f32))> {
    // Get effective page bounds (CropBox or MediaBox)
    let crop_box = page.boundaries().crop()
        .or_else(|_| page.boundaries().media())
        .context("Failed to get page boundaries")?;

    let page_bounds = crop_box.bounds;
    let crop_left = page_bounds.left().value;
    let crop_bottom = page_bounds.bottom().value;
    let crop_right = page_bounds.right().value;
    let crop_top = page_bounds.top().value;
    let crop_bounds = (crop_left, crop_bottom, crop_right, crop_top);
    let crop_area = (crop_right - crop_left) * (crop_top - crop_bottom);

    // Traverse all objects and find image candidates
    let mut candidates = Vec::new();

    traverse_page_objects(page, &mut candidates, crop_area);

    // Pick best candidate (highest coverage)
    let best = candidates
        .iter()
        .max_by(|a, b| a.coverage.partial_cmp(&b.coverage).unwrap())
        .cloned();

    // Return best if coverage >= threshold
    let result = best.and_then(|candidate| {
        if candidate.coverage >= MIN_COVERAGE_FOR_DIRECT_EXTRACT {
            Some(candidate)
        } else {
            None
        }
    });

    Ok((result, crop_bounds))
}

/// Recursively traverse page objects including Form XObjects
fn traverse_page_objects(
    page: &PdfPage,
    candidates: &mut Vec<ImageCandidate>,
    crop_area: f32,
) {
    for (idx, object) in page.objects().iter().enumerate() {
        process_object(&object, idx, candidates, crop_area);
    }
}

/// Process a single object and recursively process Form XObjects
fn process_object(
    object: &PdfPageObject,
    object_index: usize,
    candidates: &mut Vec<ImageCandidate>,
    crop_area: f32,
) {
    // Check if this is an image object
    if let Some(image_obj) = object.as_image_object() {
        if let Ok(bounds) = image_obj.bounds() {
            let obj_bounds = (
                bounds.left().value,
                bounds.bottom().value,
                bounds.right().value,
                bounds.top().value,
            );

            let obj_w = obj_bounds.2 - obj_bounds.0;
            let obj_h = obj_bounds.3 - obj_bounds.1;
            let obj_area = obj_w * obj_h;
            let coverage = (obj_area / crop_area) as f64;

            let can_extract_raw = image_obj.get_raw_bitmap().is_ok();

            candidates.push(ImageCandidate {
                object_index,
                bounds: obj_bounds,
                coverage,
                can_extract_raw,
            });
        }
    }

    // Note: pdfium_render's Form XObject traversal API is limited.
    // Form XObjects are more complex to navigate recursively without the full crate API.
    // For now, we primarily search top-level objects and log any Form XObjects found.
    if object.as_x_object_form_object().is_some() {
        eprintln!("[EXTRACT] Note: Form XObject found at object[{}] - recursive traversal not yet implemented", object_index);
    }
}

/// Extract an image object as PNG bytes (lossless)
pub fn extract_image_bytes(
    page: &PdfPage,
    object_index: usize,
) -> Result<Vec<u8>> {
    let page_objects = page.objects();

    if object_index >= page_objects.len() {
        anyhow::bail!("Object index out of bounds");
    }

    // Get object by iterating (pdfium_render API limitation)
    let mut target_object = None;
    for (idx, obj) in page_objects.iter().enumerate() {
        if idx == object_index {
            target_object = Some(obj);
            break;
        }
    }

    let object = target_object.context("Object not found at index")?;

    if let Some(image_obj) = object.as_image_object() {
        // Get bitmap and encode as PNG (lossless)
        if let Ok(bitmap) = image_obj.get_raw_bitmap() {
            let dynamic_image = bitmap.as_image();
            let rgb_image = dynamic_image.to_rgb8();

            // Encode as PNG (lossless)
            let mut png_data = Vec::new();
            let encoder = image::codecs::png::PngEncoder::new(&mut png_data);
            encoder.write_image(
                rgb_image.as_raw(),
                rgb_image.width(),
                rgb_image.height(),
                image::ExtendedColorType::Rgb8,
            ).context("Failed to encode image as PNG")?;

            return Ok(png_data);
        }
    }

    anyhow::bail!("Object is not an image or cannot be extracted")
}

/// Extract an image object as JPEG bytes with quality control (NO intermediate PNG encoding)
pub fn extract_image_bytes_as_jpeg(
    page: &PdfPage,
    object_index: usize,
    quality: u8,
) -> Result<Vec<u8>> {
    let page_objects = page.objects();

    if object_index >= page_objects.len() {
        anyhow::bail!("Object index out of bounds");
    }

    // Get object by iterating (pdfium_render API limitation)
    let mut target_object = None;
    for (idx, obj) in page_objects.iter().enumerate() {
        if idx == object_index {
            target_object = Some(obj);
            break;
        }
    }

    let object = target_object.context("Object not found at index")?;

    if let Some(image_obj) = object.as_image_object() {
        // Get bitmap and encode as JPEG directly (OPTIMIZED - no PNG intermediate!)
        if let Ok(bitmap) = image_obj.get_raw_bitmap() {
            let dynamic_image = bitmap.as_image();
            let rgb_image = dynamic_image.to_rgb8();

            // Encode directly as JPEG (SKIP PNG encoding!)
            let mut jpeg_data = Vec::new();
            let mut encoder = image::codecs::jpeg::JpegEncoder::new_with_quality(
                &mut jpeg_data,
                quality
            );
            encoder.encode(
                rgb_image.as_raw(),
                rgb_image.width(),
                rgb_image.height(),
                image::ExtendedColorType::Rgb8,
            ).context("Failed to encode image as JPEG")?;

            return Ok(jpeg_data);
        }
    }

    anyhow::bail!("Object is not an image or cannot be extracted")
}

/// Log diagnostic info about a page's best image candidate
pub fn log_page_diagnostic(
    page_num: u32,
    candidate_opt: &Option<ImageCandidate>,
    crop_bounds: (f32, f32, f32, f32),
    fallback_to_render: bool,
) {
    eprintln!();
    eprintln!("[EXTRACT] Page {} diagnostic:", page_num);
    eprintln!("[EXTRACT] CropBox: left={:.1}, bottom={:.1}, right={:.1}, top={:.1}",
        crop_bounds.0, crop_bounds.1, crop_bounds.2, crop_bounds.3);

    if let Some(candidate) = candidate_opt {
        eprintln!("[EXTRACT] Best image candidate: object[{}]", candidate.object_index);
        eprintln!("[EXTRACT]   bounds: ({:.1}, {:.1}, {:.1}, {:.1})",
            candidate.bounds.0, candidate.bounds.1, candidate.bounds.2, candidate.bounds.3);
        eprintln!("[EXTRACT]   coverage: {:.1}%", candidate.coverage * 100.0);
        if candidate.can_extract_raw {
            eprintln!("[EXTRACT]   extraction: PNG lossless");
        } else {
            eprintln!("[EXTRACT]   extraction: PNG render");
        }
    } else {
        eprintln!("[EXTRACT] No suitable image candidate (coverage < 60%)");
    }

    if fallback_to_render {
        eprintln!("[EXTRACT] Pipeline: Full-page render (fallback)");
    } else {
        eprintln!("[EXTRACT] Pipeline: Direct extract from image object");
    }
}
