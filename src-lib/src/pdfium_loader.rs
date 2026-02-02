use pdfium_render::prelude::*;
use std::path::PathBuf;
use std::env;

/// Initialize Pdfium with comprehensive library search
/// Works for CLI, development, and packaged applications
pub fn bind_pdfium() -> Result<Pdfium, PdfiumError> {
    log_system_info();

    // Determine library filename based on platform
    let lib_filename = if cfg!(target_os = "macos") {
        "libpdfium.dylib"
    } else if cfg!(target_os = "windows") {
        "pdfium.dll"
    } else {
        "libpdfium.so"
    };

    // Collect all search paths
    let mut search_paths: Vec<PathBuf> = Vec::new();
    let mut lib_path: Option<PathBuf> = None;

    // Priority 1: PDFIUM_LIB_DIR environment variable
    if let Ok(env_path) = env::var("PDFIUM_LIB_DIR") {
        let p = PathBuf::from(&env_path);
        search_paths.push(p.clone());
        if p.exists() {
            eprintln!("[PDFIUM] Found via PDFIUM_LIB_DIR: {}", p.display());
            lib_path = Some(p);
        }
    }

    // Priority 2: Current working directory (CLI usage)
    if lib_path.is_none() {
        if let Ok(cwd) = env::current_dir() {
            let cwd_path = cwd.join(lib_filename);
            search_paths.push(cwd_path.clone());
            if cwd_path.exists() {
                lib_path = Some(cwd_path);
            }
        }
    }

    // Priority 3: Next to the executable
    if lib_path.is_none() {
        if let Ok(exe_path) = env::current_exe() {
            eprintln!("[PDFIUM] Executable path: {}", exe_path.display());
            if let Some(exe_dir) = exe_path.parent() {
                eprintln!("[PDFIUM] Executable directory: {}", exe_dir.display());
                // Check multiple locations relative to executable
                let candidates = vec![
                    exe_dir.join(lib_filename),
                    exe_dir.join("resources").join("pdfium").join(lib_filename),
                    exe_dir.join("resources").join(lib_filename),
                    exe_dir.join("_up_").join("resources").join("pdfium").join(lib_filename),
                    exe_dir.join("_up_").join("resources").join(lib_filename),
                    exe_dir.join("..").join("resources").join("pdfium").join(lib_filename),
                    exe_dir.join("..").join("resources").join(lib_filename),
                    exe_dir.join("lib").join(lib_filename),
                    exe_dir.join("../Resources").join(lib_filename), // macOS bundle
                ];
                
                for candidate in candidates {
                    let canonical = if candidate.exists() {
                        candidate.canonicalize().unwrap_or(candidate.clone())
                    } else {
                        candidate.clone()
                    };
                    
                    let status = if canonical.exists() { "EXISTS ✓" } else { "not found" };
                    eprintln!("[PDFIUM] Checking: {} - {}", canonical.display(), status);
                    
                    search_paths.push(canonical.clone());
                    if canonical.exists() {
                        eprintln!("[PDFIUM] ✓ FOUND library at: {}", canonical.display());
                        lib_path = Some(canonical);
                        break;
                    }
                }
            }
        }
    }

    // Priority 4: Development - project root and resources/pdfium
    if lib_path.is_none() {
        let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let project_root = manifest_dir.parent().unwrap();
        
        let dev_candidates = vec![
            project_root.join(lib_filename),
            project_root.join("resources/pdfium").join(lib_filename),
            project_root.join("lib").join(lib_filename),
        ];
        
        for candidate in dev_candidates {
            if candidate.exists() {
                search_paths.push(candidate.clone());
                eprintln!("[PDFIUM] Found in dev path: {}", candidate.display());
                lib_path = Some(candidate);
                break;
            }
        }
    }

    // Try to load from found path
    if let Some(ref final_path) = lib_path {
        eprintln!("[PDFIUM] Attempting to load library from: {}", final_path.display());
        eprintln!("[PDFIUM] File exists, size: {} bytes", final_path.metadata().map(|m| m.len()).unwrap_or(0));
        
        // Use the path directly (not pdfium_platform_library_name_at_path which expects a directory)
        match Pdfium::bind_to_library(final_path.to_string_lossy().to_string()) {
            Ok(bindings) => {
                eprintln!("[PDFIUM] Successfully loaded library from: {}", final_path.display());
                log_success();
                return Ok(Pdfium::new(bindings));
            }
            Err(e) => {
                eprintln!("[PDFIUM ERROR] Failed to load from path: {:?}", e);
            }
        }
    }

    // Fallback: system library
    eprintln!("[PDFIUM] Trying system library as fallback...");
    match Pdfium::bind_to_system_library() {
        Ok(bindings) => {
            eprintln!("[PDFIUM] Successfully bound to system library");
            log_success();
            Ok(Pdfium::new(bindings))
        }
        Err(e) => {
            eprintln!("[PDFIUM ERROR] Failed to load system library: {:?}", e);
            eprintln!("[PDFIUM ERROR] Library not found. Searched in:");
            for path in &search_paths {
                eprintln!("[PDFIUM]   - {}", path.display());
            }
            Err(e)
        }
    }
}

/// Log system information at startup
fn log_system_info() {
    eprintln!("═══════════════════════════════════════════════════════════");
    eprintln!("PDFIUM INITIALIZATION");
    eprintln!("═══════════════════════════════════════════════════════════");

    let arch = if cfg!(target_arch = "x86_64") { "x86_64" } 
               else if cfg!(target_arch = "aarch64") { "arm64" } 
               else { env::consts::ARCH };
    eprintln!("[SYSTEM] Architecture: {}", arch);
    eprintln!("[SYSTEM] OS: {}", env::consts::OS);

    if let Ok(env_dir) = env::var("PDFIUM_LIB_DIR") {
        eprintln!("[SYSTEM] PDFIUM_LIB_DIR env var: {}", env_dir);
    } else {
        eprintln!("[SYSTEM] PDFIUM_LIB_DIR env var: not set");
    }
}

fn log_success() {
    eprintln!("[PDFIUM] Library loaded successfully");
    eprintln!("═══════════════════════════════════════════════════════════");
}
