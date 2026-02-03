use crate::utils;
use std::fs;
use std::path::PathBuf;
use tauri::Emitter;
use pdf_conversion_lib::convert_pdf_to_images_parallel;
use once_cell::sync::Lazy;
use tokio::sync::Mutex;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};

// Mutex to prevent concurrent PDFium calls (PDFium is not reentrant)
static CONVERSION_LOCK: Lazy<Mutex<()>> = Lazy::new(|| Mutex::new(()));

// Guard to prevent re-entry during IPC return
static CONVERSION_IN_PROGRESS: AtomicBool = AtomicBool::new(false);
static CONVERSION_ID: AtomicU64 = AtomicU64::new(0);

// Flag for cancellation support
static CANCEL_REQUESTED: AtomicBool = AtomicBool::new(false);

/// Validate and sanitize a file path to prevent path traversal attacks
/// Returns canonicalized path if valid, error if suspicious
fn validate_path(path: &str) -> Result<PathBuf, String> {
    let path = PathBuf::from(path);
    
    // Check for suspicious patterns
    let path_str = path.to_string_lossy();
    if path_str.contains("..") {
        return Err("Invalid path: directory traversal not allowed".to_string());
    }
    
    // Canonicalize to resolve any symlinks and get absolute path
    let canonical = path.canonicalize()
        .map_err(|e| format!("Invalid path: {}", e))?;
    
    // Verify the file exists and is accessible
    if !canonical.exists() {
        return Err("File not found".to_string());
    }
    
    Ok(canonical)
}

/// Validate an output path (parent directory must exist)
fn validate_output_path(path: &str) -> Result<PathBuf, String> {
    let path = PathBuf::from(path);
    
    // Check for suspicious patterns
    let path_str = path.to_string_lossy();
    if path_str.contains("..") {
        return Err("Invalid output path: directory traversal not allowed".to_string());
    }
    
    // Verify parent directory exists
    if let Some(parent) = path.parent() {
        if !parent.exists() {
            return Err("Output directory does not exist".to_string());
        }
    }
    
    Ok(path)
}

/// Convert internal error messages to user-friendly messages
fn user_friendly_error(internal_error: &str) -> String {
    // Map common technical errors to user-friendly messages
    if internal_error.contains("permission denied") || internal_error.contains("Permission denied") {
        return "Access denied. Please check file permissions.".to_string();
    }
    if internal_error.contains("No such file") || internal_error.contains("not found") {
        return "File not found. Please select a valid file.".to_string();
    }
    if internal_error.contains("is a directory") {
        return "A folder was selected instead of a file.".to_string();
    }
    if internal_error.contains("out of memory") || internal_error.contains("OutOfMemory") {
        return "Not enough memory. Try closing other applications or using a smaller file.".to_string();
    }
    if internal_error.contains("PDFium") || internal_error.contains("pdfium") {
        return "PDF processing error. The file may be corrupted or password-protected.".to_string();
    }
    if internal_error.contains("password") || internal_error.contains("encrypted") {
        return "This PDF is password-protected. Please provide an unprotected file.".to_string();
    }
    if internal_error.contains("corrupted") || internal_error.contains("malformed") {
        return "The file appears to be corrupted or invalid.".to_string();
    }
    if internal_error.contains("disk full") || internal_error.contains("No space") {
        return "Not enough disk space. Please free up some space and try again.".to_string();
    }
    // Default: return a simplified version without technical details
    format!("Conversion failed. Please try again with a different file.")
}

#[tauri::command]
pub async fn convert_pdf_to_cbz(
    window: tauri::Window,
    path: String,
    dpi: u32,
    quality: u32,
    lossless: bool,
) -> Result<Vec<u8>, String> {
    use std::time::Instant;
    let start_time = Instant::now();

    // Validate input path
    let validated_path = validate_path(&path)?;
    let path = validated_path.to_string_lossy().to_string();

    // Acquire lock to prevent concurrent PDFium calls
    let _lock = CONVERSION_LOCK.lock().await;

    if !PathBuf::from(&path).exists() {
        return Err("PDF file not found. Please select a valid file.".to_string());
    }

    let effective_dpi = if dpi == 0 { 200 } else { dpi };
    let effective_quality = if quality == 0 { 85 } else { quality };

    // Read PDF file
    let pdf_data = fs::read(&path)
        .map_err(|e| user_friendly_error(&e.to_string()))?;

    let _ = window.emit("conversion-progress", serde_json::json!({
        "percentage": 5,
        "message": "PDF loaded, starting conversion..."
    }));

    // Send progress: starting conversion
    let _ = window.emit("conversion-progress", serde_json::json!({
        "percentage": 25,
        "message": "Processing pages..."
    }));

    // Small delay to ensure event is processed
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    // Spawn progress ticker while conversion runs
    let window_ticker = window.clone();
    let ticker_start = start_time;
    let progress_ticker = tokio::spawn(async move {
        let mut counter = 0;
        loop {
            tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
            counter += 1;
            let spinner = match counter % 4 {
                0 => "⠋",
                1 => "⠙",
                2 => "⠹",
                _ => "⠸",
            };
            let elapsed_secs = ticker_start.elapsed().as_secs();
            let _ = window_ticker.emit("conversion-progress", serde_json::json!({
                "percentage": 50,
                "message": format!("Traitement en cours {} ({} secondes écoulées)...", spinner, elapsed_secs)
            }));
        }
    });

    let images = tokio::task::spawn_blocking(move || {
        let result = if lossless {
            convert_pdf_lossless(&pdf_data, effective_dpi)
        } else {
            // Use the optimized parallel conversion from shared library
            convert_pdf_to_images_parallel(&pdf_data, effective_dpi, effective_quality as u8, 0)
                .map_err(|e| e.to_string())
        };
        result
    })
    .await
    .map_err(|e| user_friendly_error(&e.to_string()))?
    .map_err(|e| user_friendly_error(&e))?;

    // Stop progress ticker
    progress_ticker.abort();

    if images.is_empty() {
        return Err("No pages could be extracted from this PDF. The file may be empty or corrupted.".to_string());
    }

    let page_count = images.len();
    let window_for_zip = window.clone();

    let cbz_data = utils::create_cbz_with_progress(images, move |done, total| {
        let percentage = 90 + ((done as f32 / total as f32) * 10.0) as u32;
        let _ = window_for_zip.emit("conversion-progress", serde_json::json!({
            "percentage": percentage,
            "message": format!("Creating CBZ archive {}/{}...", done, total)
        }));
    })
    .map_err(|e| user_friendly_error(&e.to_string()))?;

    let _ = window.emit("conversion-progress", serde_json::json!({
        "percentage": 100,
        "message": format!("Done! {} pages → {:.1} MB", page_count, cbz_data.len() as f64 / 1024.0 / 1024.0)
    }));

    Ok(cbz_data)
}

/// Convert PDF lossless mode (PNG at same DPI as lossy)
/// Uses the optimized pipeline from the shared library
fn convert_pdf_lossless(pdf_data: &[u8], dpi: u32) -> Result<Vec<(String, Vec<u8>)>, String> {
    use pdf_conversion_lib::extract_images_lossless_at_dpi;

    extract_images_lossless_at_dpi(pdf_data, dpi, 0)
        .map_err(|e| format!("Lossless conversion failed: {}", e))
}


#[tauri::command]
pub async fn convert_cbz_to_pdf(path: String, lossless: bool, quality: u32) -> Result<Vec<u8>, String> {
    use crate::utils::MemoryMonitor;

    // Acquire lock to prevent concurrent PDFium calls
    let _lock = CONVERSION_LOCK.lock().await;

    let cbz_path = PathBuf::from(&path);

    if !cbz_path.exists() {
        return Err("CBZ file not found".to_string());
    }

    // Check file size and warn about large files
    let file_size = fs::metadata(&cbz_path)
        .map(|m| m.len())
        .unwrap_or(0);
    let file_size_mb = file_size as f64 / (1024.0 * 1024.0);

    eprintln!("[GUI] Converting CBZ to PDF: {} ({:.1} MB) (Lossless: {}, Quality: {})",
              path, file_size_mb, lossless, quality);

    // Warn about very large files
    if file_size_mb > 500.0 {
        eprintln!("[WARNING] Large CBZ file detected ({:.1} MB). This may require significant memory.", file_size_mb);
        eprintln!("[WARNING] If the conversion fails, try with a smaller file or close other applications.");
    }

    // Start memory monitoring
    let mut mem_monitor = MemoryMonitor::new("CBZ to PDF conversion");

    let cbz_data = fs::read(&cbz_path)
        .map_err(|e| format!("Failed to read CBZ file: {}", e))?;

    mem_monitor.check("After reading CBZ file");

    let images = utils::extract_images_from_cbz(&cbz_data)
        .map_err(|e| {
            eprintln!("[ERROR] Failed to extract CBZ: {}", e);
            format!("Failed to extract CBZ: {}", e)
        })?;

    mem_monitor.check(&format!("After extracting {} images", images.len()));

    if images.is_empty() {
        return Err("No images found in CBZ file".to_string());
    }

    eprintln!("[GUI] Extracted {} images, creating PDF...", images.len());

    let pdf_data = utils::create_pdf_from_images(images, |current, total| {
        if current % 50 == 0 || current == total {
            eprintln!("[GUI] Creating PDF: {}/{} images processed", current, total);
        }
    })
    .map_err(|e| {
        eprintln!("[ERROR] Failed to create PDF: {}", e);
        format!("Failed to create PDF: {}", e)
    })?;

    mem_monitor.check("After creating PDF");

    eprintln!("[GUI] PDF created: {} bytes ({:.1} MB)", pdf_data.len(), pdf_data.len() as f64 / (1024.0 * 1024.0));

    mem_monitor.finish();

    Ok(pdf_data)
}

#[tauri::command]
pub async fn save_last_pdf(data: Vec<u8>, filename: String) -> Result<String, String> {
    let home_dir = std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .map_err(|_| "Could not determine home directory".to_string())?;

    let downloads_dir = PathBuf::from(home_dir).join("Downloads");
    fs::create_dir_all(&downloads_dir)
        .map_err(|e| format!("Failed to create downloads directory: {}", e))?;

    let file_path = downloads_dir.join(&filename);
    fs::write(&file_path, data)
        .map_err(|e| format!("Failed to save PDF: {}", e))?;

    Ok(file_path.to_string_lossy().to_string())
}

#[tauri::command]
pub async fn open_file_with_default_app(path: String) -> Result<(), String> {
    #[cfg(target_os = "macos")]
    {
        use std::process::Command;
        Command::new("open")
            .arg(&path)
            .spawn()
            .map_err(|e| format!("Failed to open file: {}", e))?;
    }

    #[cfg(target_os = "windows")]
    {
        use std::process::Command;
        Command::new("cmd")
            .args(&["/C", "start", "", &path])
            .spawn()
            .map_err(|e| format!("Failed to open file: {}", e))?;
    }

    #[cfg(target_os = "linux")]
    {
        use std::process::Command;
        Command::new("xdg-open")
            .arg(&path)
            .spawn()
            .map_err(|e| format!("Failed to open file: {}", e))?;
    }

    Ok(())
}

#[tauri::command]
pub async fn get_file_size(path: String) -> Result<u64, String> {
    let file_path = PathBuf::from(&path);
    fs::metadata(&file_path)
        .map(|m| m.len())
        .map_err(|e| format!("Failed to get file size: {}", e))
}

#[tauri::command]
pub fn cancel_conversion() -> Result<(), String> {
    CANCEL_REQUESTED.store(true, Ordering::SeqCst);
    Ok(())
}

/// Check if cancellation was requested and reset the flag
fn check_and_reset_cancel() -> bool {
    CANCEL_REQUESTED.swap(false, Ordering::SeqCst)
}

/// Convert PDF to CBZ and write directly to disk (avoids IPC bottleneck for large files)
/// Returns the file size in bytes instead of the file contents
#[tauri::command]
pub async fn convert_pdf_to_cbz_direct(
    window: tauri::Window,
    path: String,
    output_path: String,
    dpi: u32,
    quality: u32,
    lossless: bool,
) -> Result<u64, String> {
    use std::time::Instant;
    let start_time = Instant::now();

    // Clear any previous cancellation request
    CANCEL_REQUESTED.store(false, Ordering::SeqCst);

    // Validate input and output paths
    let validated_input = validate_path(&path)?;
    let validated_output = validate_output_path(&output_path)?;
    let path = validated_input.to_string_lossy().to_string();
    let output_path = validated_output.to_string_lossy().to_string();

    // Generate unique conversion ID
    let conv_id = CONVERSION_ID.fetch_add(1, Ordering::SeqCst);

    // Check and set in-progress guard
    if CONVERSION_IN_PROGRESS.compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst).is_err() {
        eprintln!("[RUST CONV#{}] ERROR: Another conversion already in progress!", conv_id);
        return Err("Another conversion is already in progress".to_string());
    }

    // Ensure we clear the guard on exit
    struct Guard;
    impl Drop for Guard {
        fn drop(&mut self) {
            CONVERSION_IN_PROGRESS.store(false, Ordering::SeqCst);
            eprintln!("[RUST] Conversion guard released");
        }
    }
    let _guard = Guard;

    eprintln!("[RUST CONV#{}] Input: {}", conv_id, path);
    eprintln!("[RUST CONV#{}] Output: {}", conv_id, output_path);
    eprintln!("[RUST CONV#{}] DPI: {}, Quality: {}, Lossless: {}", conv_id, dpi, quality, lossless);

    // Acquire mutex lock
    eprintln!("[RUST CONV#{}] Waiting for conversion lock...", conv_id);
    let _lock = CONVERSION_LOCK.lock().await;
    eprintln!("[RUST CONV#{}] Lock acquired", conv_id);

    if !PathBuf::from(&path).exists() {
        return Err("PDF file not found".to_string());
    }

    let effective_dpi = if dpi == 0 { 200 } else { dpi };
    let effective_quality = if quality == 0 { 85 } else { quality };

    // Read PDF file
    let pdf_data = fs::read(&path)
        .map_err(|e| format!("Failed to read PDF file: {}", e))?;

    eprintln!("[RUST CONV#{}] PDF loaded: {} bytes", conv_id, pdf_data.len());

    let _ = window.emit("conversion-progress", serde_json::json!({
        "percentage": 5,
        "message": "PDF chargé, démarrage conversion..."
    }));

    let _ = window.emit("conversion-progress", serde_json::json!({
        "percentage": 25,
        "message": "Traitement des pages..."
    }));

    // Progress ticker
    let window_ticker = window.clone();
    let ticker_start = start_time;
    let progress_ticker = tokio::spawn(async move {
        let mut counter = 0;
        loop {
            tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
            counter += 1;
            let spinner = match counter % 4 {
                0 => "⠋", 1 => "⠙", 2 => "⠹", _ => "⠸",
            };
            let elapsed_secs = ticker_start.elapsed().as_secs();
            let _ = window_ticker.emit("conversion-progress", serde_json::json!({
                "percentage": 50,
                "message": format!("Traitement en cours {} ({} sec)...", spinner, elapsed_secs)
            }));
        }
    });

    // Convert PDF to images
    let images = tokio::task::spawn_blocking(move || {
        if lossless {
            convert_pdf_lossless(&pdf_data, effective_dpi)
        } else {
            convert_pdf_to_images_parallel(&pdf_data, effective_dpi, effective_quality as u8, 0)
                .map_err(|e| e.to_string())
        }
    })
    .await
    .map_err(|e| format!("Task join error: {}", e))?
    .map_err(|e| format!("PDF conversion failed: {}", e))?;

    progress_ticker.abort();

    if images.is_empty() {
        return Err("No pages were extracted from PDF".to_string());
    }

    let page_count = images.len();
    eprintln!("[RUST CONV#{}] Converted {} pages", conv_id, page_count);

    // Create CBZ archive
    let window_for_zip = window.clone();
    let cbz_data = utils::create_cbz_with_progress(images, move |done, total| {
        let percentage = 90 + ((done as f32 / total as f32) * 10.0) as u32;
        let _ = window_for_zip.emit("conversion-progress", serde_json::json!({
            "percentage": percentage,
            "message": format!("Archive CBZ {}/{} fichiers...", done, total)
        }));
    })
    .map_err(|e| format!("Failed to create CBZ archive: {}", e))?;

    let cbz_size = cbz_data.len() as u64;
    eprintln!("[RUST CONV#{}] CBZ created: {} bytes ({:.1} MB)", conv_id, cbz_size, cbz_size as f64 / 1024.0 / 1024.0);

    // Write directly to disk (FAST - no IPC transfer!)
    eprintln!("[RUST CONV#{}] Writing to disk: {}", conv_id, output_path);
    fs::write(&output_path, &cbz_data)
        .map_err(|e| format!("Failed to write CBZ file: {}", e))?;

    let elapsed = start_time.elapsed();
    eprintln!("[RUST CONV#{}] ========== CONVERSION COMPLETE in {:.1}s ==========", conv_id, elapsed.as_secs_f64());

    let _ = window.emit("conversion-progress", serde_json::json!({
        "percentage": 100,
        "message": format!("Terminé! {} pages → {:.1} MB en {:.1}s", page_count, cbz_size as f64 / 1024.0 / 1024.0, elapsed.as_secs_f64())
    }));

    Ok(cbz_size)
}

