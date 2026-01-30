use pdfium_render::prelude::*;
use std::path::PathBuf;
use std::env;

/// Initialize Pdfium with explicit library path, env var support, and comprehensive logging
pub fn bind_pdfium() -> Result<Pdfium, PdfiumError> {
    // Log system information
    log_system_info();

    // Step 1: Determine library path (with priority)
    let lib_path = determine_lib_path();

    eprintln!("[PDFIUM] Attempting to load library from: {}", lib_path.display());

    if !lib_path.exists() {
        eprintln!("[PDFIUM ERROR] Library file not found at: {}", lib_path.display());
        eprintln!("[PDFIUM] Trying system library as fallback...");
        // Fallback: try system library
        match Pdfium::bind_to_system_library() {
            Ok(lib) => {
                let pdfium = Pdfium::new(lib);
                log_pdfium_info(&pdfium);
                return Ok(pdfium);
            }
            Err(e) => {
                eprintln!("[PDFIUM ERROR] Failed to load system library: {:?}", e);
                return Err(e);
            }
        }
    }

    eprintln!("[PDFIUM] File exists, size: {} bytes", lib_path.metadata().map(|m| m.len()).unwrap_or(0));

    // Step 2: Load from explicit path
    // Try absolute path first, then try just the filename as fallback
    let pdfium = match Pdfium::bind_to_library(lib_path.to_string_lossy().to_string()) {
        Ok(lib) => {
            eprintln!("[PDFIUM] Successfully bound to library via path");
            Pdfium::new(lib)
        }
        Err(e) => {
            eprintln!("[PDFIUM ERROR] Failed to bind to library via path: {:?}", e);
            eprintln!("[PDFIUM] Trying alternative binding methods...");
            // Fallback: try just the filename
            match Pdfium::bind_to_library(Pdfium::pdfium_platform_library_name_at_path(&lib_path)) {
                Ok(lib) => {
                    eprintln!("[PDFIUM] Successfully bound via filename");
                    Pdfium::new(lib)
                }
                Err(e2) => {
                    eprintln!("[PDFIUM ERROR] Failed to bind via filename: {:?}", e2);
                    eprintln!("[PDFIUM] Trying system library as last resort...");
                    match Pdfium::bind_to_system_library() {
                        Ok(lib) => {
                            eprintln!("[PDFIUM] Successfully bound to system library");
                            Pdfium::new(lib)
                        }
                        Err(e3) => {
                            eprintln!("[PDFIUM ERROR] All binding methods failed");
                            eprintln!("[PDFIUM ERROR] Path error: {:?}", e);
                            eprintln!("[PDFIUM ERROR] Filename error: {:?}", e2);
                            eprintln!("[PDFIUM ERROR] System error: {:?}", e3);
                            return Err(e3);
                        }
                    }
                }
            }
        }
    };

    eprintln!("[PDFIUM] Successfully loaded library from: {}", lib_path.display());
    log_pdfium_info(&pdfium);

    Ok(pdfium)
}

/// Determine library path with priority:
/// 1. PDFIUM_LIB_DIR env var
/// 2. ./libpdfium.dylib (macOS dev)
/// 3. ./pdfium.dll (Windows dev)
/// 4. ./libpdfium.so (Linux dev)
/// 5. Tauri resource dir (release)
fn determine_lib_path() -> PathBuf {
    // Check PDFIUM_LIB_DIR env var
    if let Ok(env_path) = env::var("PDFIUM_LIB_DIR") {
        let p = PathBuf::from(&env_path);
        eprintln!("[PDFIUM] Found env PDFIUM_LIB_DIR: {}", p.display());
        return p;
    }

    // Check project root ./libpdfium.* (dev)
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let project_root = PathBuf::from(manifest_dir)
        .parent()
        .map(|p| p.to_path_buf())
        .unwrap_or_else(|| PathBuf::from("."));

    if cfg!(target_os = "macos") {
        project_root.join("libpdfium.dylib")
    } else if cfg!(target_os = "windows") {
        project_root.join("pdfium.dll")
    } else {
        project_root.join("libpdfium.so")
    }
}

/// Log system information at startup
fn log_system_info() {
    eprintln!("═══════════════════════════════════════════════════════════");
    eprintln!("PDFIUM INITIALIZATION");
    eprintln!("═══════════════════════════════════════════════════════════");

    // Target architecture
    let arch = if cfg!(target_arch = "x86_64") {
        "x86_64"
    } else if cfg!(target_arch = "aarch64") {
        "arm64"
    } else {
        env::consts::ARCH
    };
    eprintln!("[SYSTEM] Architecture: {}", arch);

    // OS
    let os = env::consts::OS;
    eprintln!("[SYSTEM] OS: {}", os);

    // Check PDFIUM_LIB_DIR env
    if let Ok(env_dir) = env::var("PDFIUM_LIB_DIR") {
        eprintln!("[SYSTEM] PDFIUM_LIB_DIR env var: {}", env_dir);
    } else {
        eprintln!("[SYSTEM] PDFIUM_LIB_DIR env var: not set");
    }
}

/// Log information about loaded Pdfium instance
fn log_pdfium_info(_pdfium: &Pdfium) {
    // Try to get version if available via bindings
    // pdfium_render may expose version via FPDF_GetVersion or similar
    // For now, log that we loaded it
    eprintln!("[PDFIUM] Library loaded successfully");
    eprintln!("═══════════════════════════════════════════════════════════");
}
