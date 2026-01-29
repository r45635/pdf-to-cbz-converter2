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

    eprintln!("[GUI {:?}] ========== CONVERSION START (acquiring lock) ==========", start_time.elapsed());

    // Acquire lock to prevent concurrent PDFium calls
    eprintln!("[GUI {:?}] Waiting for conversion lock...", start_time.elapsed());
    let _lock = CONVERSION_LOCK.lock().await;
    eprintln!("[GUI {:?}] ✓ Lock acquired, proceeding with conversion", start_time.elapsed());

    eprintln!("[GUI {:?}] Input path: {}", start_time.elapsed(), path);
    eprintln!("[GUI {:?}] DPI: {}, Quality: {}, Lossless: {}", start_time.elapsed(), dpi, quality, lossless);

    if !PathBuf::from(&path).exists() {
        eprintln!("[GUI ERROR {:?}] PDF file not found: {}", start_time.elapsed(), path);
        return Err("PDF file not found".to_string());
    }
    eprintln!("[GUI {:?}] File exists, checking size...", start_time.elapsed());

    let effective_dpi = if dpi == 0 { 200 } else { dpi };
    let effective_quality = if quality == 0 { 85 } else { quality };

    eprintln!("[GUI {:?}] Effective DPI: {}, Effective Quality: {}", start_time.elapsed(), effective_dpi, effective_quality);

    // Read PDF file
    eprintln!("[GUI {:?}] Reading PDF file...", start_time.elapsed());
    let pdf_data = fs::read(&path)
        .map_err(|e| {
            let err_msg = format!("Failed to read PDF file: {}", e);
            eprintln!("[GUI ERROR {:?}] {}", start_time.elapsed(), err_msg);
            err_msg
        })?;

    eprintln!("[GUI {:?}] PDF file read successfully: {} bytes", start_time.elapsed(), pdf_data.len());

    let _ = window.emit("conversion-progress", serde_json::json!({
        "percentage": 5,
        "message": "PDF chargé, démarrage conversion..."
    }));

    // Use optimized shared library function with progress tracking
    eprintln!("[GUI {:?}] Starting conversion task with optimized pipeline...", start_time.elapsed());

    // Send progress: starting conversion
    eprintln!("[GUI {:?}] ========== EMITTING 25% PROGRESS ==========", start_time.elapsed());
    let emit_result = window.emit("conversion-progress", serde_json::json!({
        "percentage": 25,
        "message": "Traitement des pages (cette opération prend 2-3 minutes)..."
    }));
    eprintln!("[GUI {:?}] 25% event emit result: {:?}", start_time.elapsed(), emit_result);

    // Add small delay to ensure event is processed
    eprintln!("[GUI {:?}] Waiting 100ms before spawn_blocking...", start_time.elapsed());
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    eprintln!("[GUI {:?}] Sleep completed, about to spawn_blocking", start_time.elapsed());

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
        let block_start = Instant::now();
        eprintln!("[GUI BLOCKING START] Task started after {:?}", block_start.elapsed());
        eprintln!("[GUI BLOCKING] Starting PDF to images conversion...");

        let result = if lossless {
            eprintln!("[GUI {:?}] Using lossless mode (PNG at {} DPI)", block_start.elapsed(), effective_dpi);
            convert_pdf_lossless(&pdf_data, effective_dpi)
        } else {
            eprintln!("[GUI {:?}] Using optimized parallel mode with DPI={}, Quality={}", block_start.elapsed(), effective_dpi, effective_quality);
            // Use the optimized parallel conversion from shared library
            convert_pdf_to_images_parallel(&pdf_data, effective_dpi, effective_quality as u8, 0)
                .map_err(|e| e.to_string())
        };

        match &result {
            Ok(imgs) => eprintln!("[GUI {:?}] Conversion succeeded: {} images (total time: {:.2}s)", block_start.elapsed(), imgs.len(), block_start.elapsed().as_secs_f64()),
            Err(e) => eprintln!("[GUI ERROR {:?}] Conversion failed: {} (total time: {:.2}s)", block_start.elapsed(), e, block_start.elapsed().as_secs_f64()),
        }

        result
    })
    .await
    .map_err(|e| {
        let err_msg = format!("Task join error: {}", e);
        eprintln!("[GUI ERROR {:?}] {}", start_time.elapsed(), err_msg);
        err_msg
    })?
    .map_err(|e| {
        let err_msg = format!("PDF conversion failed: {}", e);
        eprintln!("[GUI ERROR {:?}] {}", start_time.elapsed(), err_msg);
        err_msg
    })?;

    // Stop progress ticker
    progress_ticker.abort();

    if images.is_empty() {
        eprintln!("[GUI ERROR {:?}] No pages extracted", start_time.elapsed());
        return Err("No pages were extracted from PDF".to_string());
    }

    let page_count = images.len();
    eprintln!("[GUI {:?}] Converted {} pages, creating CBZ archive...", start_time.elapsed(), page_count);

    let zip_start = std::time::Instant::now();
    let window_for_zip = window.clone();

    let cbz_data = utils::create_cbz_with_progress(images, move |done, total| {
        // Map zip progress from 90% to 100% (10% range)
        let percentage = 90 + ((done as f32 / total as f32) * 10.0) as u32;
        let _ = window_for_zip.emit("conversion-progress", serde_json::json!({
            "percentage": percentage,
            "message": format!("Archive CBZ {}/{} fichiers...", done, total)
        }));
    })
        .map_err(|e| {
            let err_msg = format!("Failed to create CBZ archive: {}", e);
            eprintln!("[GUI ERROR {:?}] {}", start_time.elapsed(), err_msg);
            err_msg
        })?;

    let zip_elapsed = zip_start.elapsed().as_secs_f64();
    eprintln!("[GUI {:?}] CBZ archive created in {:.2}s", start_time.elapsed(), zip_elapsed);

    eprintln!("[GUI {:?}] ========== CONVERSION SUCCESS ==========", start_time.elapsed());
    eprintln!("[GUI {:?}] CBZ created: {} bytes ({:.2} MB)", start_time.elapsed(), cbz_data.len(), cbz_data.len() as f64 / (1024.0 * 1024.0));
    eprintln!("[GUI {:?}] TOTAL TIME: {:?}", start_time.elapsed(), start_time.elapsed());

    let _ = window.emit("conversion-progress", serde_json::json!({
        "percentage": 100,
        "message": format!("Terminé! {} pages → {} MB", page_count, cbz_data.len() / 1024 / 1024)
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
    // Acquire lock to prevent concurrent PDFium calls
    let _lock = CONVERSION_LOCK.lock().await;

    let cbz_path = PathBuf::from(&path);

    if !cbz_path.exists() {
        return Err("CBZ file not found".to_string());
    }

    eprintln!("[GUI] Converting CBZ to PDF: {} (Lossless: {}, Quality: {})", path, lossless, quality);

    let cbz_data = fs::read(&cbz_path)
        .map_err(|e| format!("Failed to read CBZ file: {}", e))?;

    let images = utils::extract_images_from_cbz(&cbz_data)
        .map_err(|e| format!("Failed to extract CBZ: {}", e))?;

    if images.is_empty() {
        return Err("No images found in CBZ file".to_string());
    }

    eprintln!("[GUI] Extracted {} images, creating PDF...", images.len());

    let pdf_data = utils::create_pdf_from_images(images, |_current, _total| {})
        .map_err(|e| format!("Failed to create PDF: {}", e))?;

    eprintln!("[GUI] PDF created: {} bytes", pdf_data.len());

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
    Ok(())
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

    // Generate unique conversion ID
    let conv_id = CONVERSION_ID.fetch_add(1, Ordering::SeqCst);
    eprintln!("[RUST CONV#{}] ========== CONVERSION START ==========", conv_id);

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

