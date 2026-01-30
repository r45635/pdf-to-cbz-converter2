use pdfium_render::prelude::*;
use std::path::PathBuf;
use std::env;

/// Initialize Pdfium with explicit library path and logging
pub fn bind_pdfium() -> Result<Pdfium, PdfiumError> {
    // Determine library filename based on platform
    let lib_filename = if cfg!(target_os = "macos") {
        "libpdfium.dylib"
    } else if cfg!(target_os = "windows") {
        "pdfium.dll"
    } else {
        "libpdfium.so"
    };

    // Try multiple locations in order of preference:
    // 1. Bundled resources directory (production)
    // 2. Current executable directory
    // 3. Project root (development)
    let mut lib_path: Option<PathBuf> = None;

    // Try 1: Bundled resources (Tauri resource path)
    if let Ok(exe_path) = env::current_exe() {
        if let Some(exe_dir) = exe_path.parent() {
            // macOS: .app/Contents/MacOS/../Resources/
            let bundled_path = if cfg!(target_os = "macos") {
                exe_dir.join("../Resources").join(lib_filename)
            } else {
                exe_dir.join(lib_filename)
            };
            
            if bundled_path.exists() {
                eprintln!("[PDFIUM] Found library in bundle: {}", bundled_path.display());
                lib_path = Some(bundled_path);
            }
        }
    }

    // Try 2: Development - project root
    if lib_path.is_none() {
        let dev_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .join(lib_filename);
        
        if dev_path.exists() {
            eprintln!("[PDFIUM] Found library in dev path: {}", dev_path.display());
            lib_path = Some(dev_path);
        }
    }

    // If no library found, try system library as fallback
    if lib_path.is_none() {
        eprintln!("[PDFIUM] Library not found in bundle or dev paths, trying system library");
        return Ok(Pdfium::new(
            Pdfium::bind_to_system_library()
                .or_else(|_| Pdfium::bind_to_library(Pdfium::pdfium_platform_library_name_at_path("./")))?
        ));
    }

    let final_path = lib_path.unwrap();

    let final_path = lib_path.unwrap();
    eprintln!("[PDFIUM] Attempting to load library from: {}", final_path.display());

    let pdfium = Pdfium::new(
        Pdfium::bind_to_library(Pdfium::pdfium_platform_library_name_at_path(&final_path))
            .or_else(|e| {
                eprintln!("[PDFIUM ERROR] Failed to bind to library: {:?}", e);
                Pdfium::bind_to_system_library()
            })?,
    );

    eprintln!("[PDFIUM] Successfully loaded library from: {}", final_path.display());

    Ok(pdfium)
}
