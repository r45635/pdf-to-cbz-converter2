use pdfium_render::prelude::*;
use std::path::PathBuf;
use std::env;
use tauri::{AppHandle, Emitter};
use serde_json::json;

/// Send a log message to the debug UI
fn emit_log(app: Option<&AppHandle>, message: String) {
    eprintln!("{}", message); // Still print to console for dev
    if let Some(handle) = app {
        let _ = handle.emit("pdfium-debug", json!({ "message": message }));
    }
}

/// Initialize Pdfium with explicit library path and logging
/// Searches in multiple locations to work in both development and production
pub fn bind_pdfium(app: Option<&AppHandle>) -> Result<Pdfium, PdfiumError> {
    // Determine library filename based on platform
    let lib_filename = if cfg!(target_os = "macos") {
        "libpdfium.dylib"
    } else if cfg!(target_os = "windows") {
        "pdfium.dll"
    } else {
        "libpdfium.so"
    };

    // Collect all search paths for clear debugging
    let mut search_paths: Vec<PathBuf> = Vec::new();
    let mut lib_path: Option<PathBuf> = None;

    // Try 1: PDFIUM_LIB_DIR environment variable (highest priority)
    if let Ok(env_path) = env::var("PDFIUM_LIB_DIR") {
        let p = PathBuf::from(&env_path);
        search_paths.push(p.clone());
        if p.exists() {
            lib_path = Some(p);
        }
    }

    // Try 2: Bundled resources (Tauri package)
    if lib_path.is_none() {
        if let Ok(exe_path) = env::current_exe() {
            emit_log(app, format!("[PDFIUM] Executable path: {}", exe_path.display()));
            if let Some(exe_dir) = exe_path.parent() {
                emit_log(app, format!("[PDFIUM] Executable directory: {}", exe_dir.display()));
                // Platform-specific bundle paths
                let candidates: Vec<PathBuf> = if cfg!(target_os = "macos") {
                    vec![
                        // macOS .app bundle: Contents/Resources/
                        exe_dir.join("../Resources").join(lib_filename),
                        exe_dir.join("Resources").join(lib_filename),
                        // Development: next to exe
                        exe_dir.join(lib_filename),
                    ]
                } else if cfg!(target_os = "windows") {
                    vec![
                        // Windows: same directory as exe
                        exe_dir.join(lib_filename),
                        // Tauri v2 resource patterns - try all variations with pdfium subdirectory
                        exe_dir.join("resources").join("pdfium").join(lib_filename),
                        exe_dir.join("resources").join(lib_filename),
                        exe_dir.join("_up_").join("resources").join("pdfium").join(lib_filename),
                        exe_dir.join("_up_").join("resources").join(lib_filename),
                        // Parent directories
                        exe_dir.join("..").join("resources").join("pdfium").join(lib_filename),
                        exe_dir.join("..").join("resources").join(lib_filename),
                        exe_dir.join("..").join(lib_filename),
                        // Grandparent directory patterns
                        exe_dir.parent().and_then(|p| p.parent()).map(|p| p.join("resources").join("pdfium").join(lib_filename)).unwrap_or_else(|| exe_dir.join(lib_filename)),
                        exe_dir.parent().and_then(|p| p.parent()).map(|p| p.join("resources").join(lib_filename)).unwrap_or_else(|| exe_dir.join(lib_filename)),
                        exe_dir.parent().and_then(|p| p.parent()).map(|p| p.join(lib_filename)).unwrap_or_else(|| exe_dir.join(lib_filename)),
                    ]
                } else {
                    // Linux
                    vec![
                        exe_dir.join(lib_filename),
                        exe_dir.join("resources").join(lib_filename),
                        exe_dir.join("lib").join(lib_filename),
                    ]
                };

                for candidate in candidates {
                    let canonical = if candidate.exists() {
                        candidate.canonicalize().unwrap_or(candidate.clone())
                    } else {
                        candidate.clone()
                    };
                    
                    // Log EVERY path we try, whether it exists or not
                    let status = if canonical.exists() { "EXISTS ✓" } else { "not found" };
                    emit_log(app, format!("[PDFIUM] Checking: {} - {}", canonical.display(), status));
                    
                    search_paths.push(canonical.clone());
                    if canonical.exists() {
                        emit_log(app, format!("[PDFIUM] ✓ FOUND library at: {}", canonical.display()));
                        lib_path = Some(canonical);
                        break;
                    }
                }
            }
        }
    }

    // Try 3: Development - project root
    if lib_path.is_none() {
        let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        // Check both project root and resources/pdfium
        let dev_candidates = vec![
            manifest_dir.parent().unwrap().join(lib_filename),
            manifest_dir.parent().unwrap().join("resources/pdfium").join(lib_filename),
            manifest_dir.parent().unwrap().join("lib").join(lib_filename),
        ];
        
        for candidate in dev_candidates {
            if candidate.exists() {
                search_paths.push(candidate.clone());
                lib_path = Some(candidate);
                break;
            }
        }
    }

    // Try 4: Current working directory
    if lib_path.is_none() {
        if let Ok(cwd) = env::current_dir() {
            let cwd_path = cwd.join(lib_filename);
            search_paths.push(cwd_path.clone());
            if cwd_path.exists() {
                lib_path = Some(cwd_path);
            }
        }
    }

    // If we found a library, try to load it
    if let Some(ref final_path) = lib_path {
        // Use the path directly (not pdfium_platform_library_name_at_path which expects a directory)
        match Pdfium::bind_to_library(final_path.to_string_lossy().to_string()) {
            Ok(bindings) => {
                return Ok(Pdfium::new(bindings));
            }
            Err(e) => {
                emit_log(app, format!("[PDFIUM] Failed to load from {}: {:?}", final_path.display(), e));
            }
        }
    }

    // Fallback: try system library
    match Pdfium::bind_to_system_library() {
        Ok(bindings) => Ok(Pdfium::new(bindings)),
        Err(e) => {
            emit_log(app, "[PDFIUM ERROR] Library not found. Searched in:".to_string());
            for path in &search_paths {
                emit_log(app, format!("[PDFIUM]   - {}", path.display()));
            }
            emit_log(app, format!("[PDFIUM] System library also failed: {:?}", e));
            Err(e)
        }
    }
}
