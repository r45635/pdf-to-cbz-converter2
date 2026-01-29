use pdfium_render::prelude::*;
use std::path::PathBuf;

/// Initialize Pdfium with explicit library path and logging
pub fn bind_pdfium() -> Result<Pdfium, PdfiumError> {
    // Determine library path based on platform
    let lib_path = if cfg!(target_os = "macos") {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .join("libpdfium.dylib")
    } else if cfg!(target_os = "windows") {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .join("pdfium.dll")
    } else {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .join("libpdfium.so")
    };

    eprintln!("[PDFIUM] Attempting to load library from: {}", lib_path.display());
    
    if !lib_path.exists() {
        eprintln!("[PDFIUM ERROR] Library file not found at: {}", lib_path.display());
        eprintln!("[PDFIUM] Falling back to Pdfium::default()");
        return Ok(Pdfium::default());
    }

    let pdfium = Pdfium::new(
        Pdfium::bind_to_library(Pdfium::pdfium_platform_library_name_at_path(&lib_path))
            .or_else(|_| Pdfium::bind_to_system_library())?,
    );

    eprintln!("[PDFIUM] Successfully loaded library");
    eprintln!("[PDFIUM] Library path: {}", lib_path.display());

    Ok(pdfium)
}
