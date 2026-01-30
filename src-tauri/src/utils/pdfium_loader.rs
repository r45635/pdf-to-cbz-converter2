use pdfium_render::prelude::*;
use std::path::PathBuf;
use std::env;
use std::fs;

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

    eprintln!("[PDFIUM] Starting initialization, looking for: {}", lib_filename);

    // Try multiple locations in order of preference:
    let mut lib_path: Option<PathBuf> = None;
    let mut search_paths = Vec::new();

    // Try 1: Bundled resources (Tauri resource path)
    if let Ok(exe_path) = env::current_exe() {
        eprintln!("[PDFIUM] Executable path: {}", exe_path.display());
        
        if let Some(exe_dir) = exe_path.parent() {
            eprintln!("[PDFIUM] Executable directory: {}", exe_dir.display());
            
            // Different paths for different platforms
            let candidates = if cfg!(target_os = "macos") {
                vec![
                    exe_dir.join("../Resources").join(lib_filename),
                    exe_dir.join("Resources").join(lib_filename),
                ]
            } else if cfg!(target_os = "windows") {
                vec![
                    exe_dir.join(lib_filename),                    // Same dir as exe
                    exe_dir.join("resources").join(lib_filename),   // resources subdir
                    exe_dir.join("..").join(lib_filename),          // Parent dir
                ]
            } else {
                vec![
                    exe_dir.join(lib_filename),
                    exe_dir.join("resources").join(lib_filename),
                ]
            };

            for candidate in candidates {
                search_paths.push(candidate.clone());
                if candidate.exists() {
                    eprintln!("[PDFIUM] ✅ Found library at: {}", candidate.display());
                    lib_path = Some(candidate);
                    break;
                } else {
                    eprintln!("[PDFIUM] ❌ Not found: {}", candidate.display());
                }
            }
        }
    }

    // Try 2: List directory contents for debugging
    if lib_path.is_none() {
        if let Ok(exe_path) = env::current_exe() {
            if let Some(exe_dir) = exe_path.parent() {
                eprintln!("[PDFIUM] Listing directory contents:");
                if let Ok(entries) = fs::read_dir(exe_dir) {
                    for entry in entries.flatten() {
                        eprintln!("[PDFIUM]   - {}", entry.path().display());
                    }
                }
            }
        }
    }

    // Try 3: Development - project root
    if lib_path.is_none() {
        let dev_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .join(lib_filename);
        
        search_paths.push(dev_path.clone());
        if dev_path.exists() {
            eprintln!("[PDFIUM] ✅ Found library in dev path: {}", dev_path.display());
            lib_path = Some(dev_path);
        } else {
            eprintln!("[PDFIUM] ❌ Not found in dev path: {}", dev_path.display());
        }
    }

    // If no library found, try system library as fallback
    if lib_path.is_none() {
        eprintln!("[PDFIUM] ⚠️  Library not found in any of these locations:");
        for path in &search_paths {
            eprintln!("[PDFIUM]      - {}", path.display());
        }
        eprintln!("[PDFIUM] Trying system library as last resort...");
        
        return Ok(Pdfium::new(
            Pdfium::bind_to_system_library()
                .or_else(|e| {
                    eprintln!("[PDFIUM ERROR] System library also failed: {:?}", e);
                    Pdfium::bind_to_library(Pdfium::pdfium_platform_library_name_at_path("./"))
                })?
        ));
    }

    let final_path = lib_path.unwrap();
    eprintln!("[PDFIUM] 🎯 Attempting to load library from: {}", final_path.display());

    let pdfium = Pdfium::new(
        Pdfium::bind_to_library(Pdfium::pdfium_platform_library_name_at_path(&final_path))
            .or_else(|e| {
                eprintln!("[PDFIUM ERROR] ❌ Failed to bind to library at {}: {:?}", final_path.display(), e);
                eprintln!("[PDFIUM] Trying system library as fallback...");
                Pdfium::bind_to_system_library()
            })?,
    );

    eprintln!("[PDFIUM] ✅ Successfully loaded PDFium from: {}", final_path.display());

    Ok(pdfium)
}
            })?,
    );

    eprintln!("[PDFIUM] Successfully loaded library from: {}", final_path.display());

    Ok(pdfium)
}
