use std::process::Command;
use tauri::State;
use std::sync::{Arc, Mutex};
use std::path::PathBuf;

#[derive(Clone)]
struct ConversionState {
    current_file: Arc<Mutex<String>>,
}

#[tauri::command]
fn convert_pdf_to_cbz(
    input_path: String,
    output_path: String,
    dpi: u32,
    state: State<ConversionState>,
) -> Result<String, String> {
    let mut current = state.current_file.lock().unwrap();
    *current = format!("Converting: {}", input_path);
    drop(current);

    // Call the CLI binary
    let output = Command::new("pdf-to-cbz")
        .arg("pdf-to-cbz")
        .arg(&input_path)
        .arg("--output")
        .arg(&output_path)
        .arg("--dpi")
        .arg(dpi.to_string())
        .output()
        .map_err(|e| format!("Failed to run converter: {}", e))?;

    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Conversion failed: {}", error));
    }

    let mut current = state.current_file.lock().unwrap();
    *current = format!("Completed: {}", output_path);

    Ok(format!("Successfully created: {}", output_path))
}

#[tauri::command]
fn convert_cbz_to_pdf(
    input_path: String,
    output_path: String,
    state: State<ConversionState>,
) -> Result<String, String> {
    let mut current = state.current_file.lock().unwrap();
    *current = format!("Converting: {}", input_path);
    drop(current);

    // Call the CLI binary
    let output = Command::new("pdf-to-cbz")
        .arg("cbz-to-pdf")
        .arg(&input_path)
        .arg("--output")
        .arg(&output_path)
        .output()
        .map_err(|e| format!("Failed to run converter: {}", e))?;

    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Conversion failed: {}", error));
    }

    let mut current = state.current_file.lock().unwrap();
    *current = format!("Completed: {}", output_path);

    Ok(format!("Successfully created: {}", output_path))
}

#[tauri::command]
fn get_save_path(filename: String) -> Result<String, String> {
    let home = std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .unwrap_or_else(|_| ".".to_string());

    let downloads = PathBuf::from(home).join("Downloads");
    Ok(downloads.join(filename).to_string_lossy().to_string())
}

#[tauri::command]
fn open_file_dialog() -> Result<Option<String>, String> {
    // Use system file picker
    #[cfg(target_os = "macos")]
    {
        use std::process::Stdio;
        let output = Command::new("osascript")
            .arg("-e")
            .arg("choose file of type {\"com.adobe.pdf\", \"public.composite-document\", \"dyn.ah62d46rv4ge81a3dhq\"} without invisibles")
            .output()
            .map_err(|e| format!("Failed to open file dialog: {}", e))?;

        let path = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if path.is_empty() {
            Ok(None)
        } else {
            Ok(Some(path))
        }
    }

    #[cfg(not(target_os = "macos"))]
    {
        Err("File dialog not implemented for this platform".to_string())
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let state = ConversionState {
        current_file: Arc::new(Mutex::new(String::new())),
    };

    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .manage(state)
        .invoke_handler(tauri::generate_handler![
            convert_pdf_to_cbz,
            convert_cbz_to_pdf,
            get_save_path,
            open_file_dialog,
        ])
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::DragDrop(drop_event) = event {
                if let tauri::DragDropEvent::Drop { paths, .. } = drop_event {
                    let _ = window.emit("tauri://file-drop", &paths);
                }
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
