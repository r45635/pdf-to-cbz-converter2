mod commands;
mod models;
mod utils;

use commands::*;
use tauri::Emitter;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Initialize panic handler for crash debugging
    utils::install_panic_handler();

    // Initialize tracing
    tracing_subscriber::fmt::init();

    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .setup(|app| {
            // Test PDFium loading on startup and log to debug UI
            let app_handle = app.handle().clone();
            std::thread::spawn(move || {
                let _ = crate::utils::bind_pdfium(Some(&app_handle));
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            analyze_pdf,
            analyze_cbz,
            generate_preview,
            generate_cbz_preview,
            convert_pdf_to_cbz,
            convert_pdf_to_cbz_direct,
            convert_cbz_to_pdf,
            save_last_pdf,
            open_file_with_default_app,
            get_file_size,
            cancel_conversion,
        ])
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::DragDrop(drop_event) = event {
                eprintln!("[Tauri] Window event DragDrop detected");
                // Try to emit file paths when drop happens
                if let tauri::DragDropEvent::Drop { paths, .. } = drop_event {
                    eprintln!("[Tauri] Drop detected with {} paths", paths.len());
                    for path in paths {
                        eprintln!("[Tauri] Path: {}", path.display());
                    }
                    // Emit the paths as a Tauri event that frontend can listen to
                    let _ = window.emit("tauri://file-drop", &paths);
                }
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
