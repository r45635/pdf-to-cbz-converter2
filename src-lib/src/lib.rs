// Shared PDF conversion library for CLI and Tauri

pub mod pdfium_loader;
pub mod direct_extract;
pub mod conversion;

// Re-export main types and functions for convenience
pub use pdfium_loader::bind_pdfium;
pub use direct_extract::{
    ImageCandidate,
    find_best_image_candidate,
    extract_image_bytes,
    extract_image_bytes_as_jpeg,
    log_page_diagnostic,
    MIN_COVERAGE_FOR_DIRECT_EXTRACT,
};
pub use conversion::{
    convert_pdf_to_images_parallel,
    extract_images_lossless_at_dpi,
    create_pdf_from_images,
};

// Re-export pdfium_render types that are part of the public API
pub use pdfium_render::prelude::Pdfium;
