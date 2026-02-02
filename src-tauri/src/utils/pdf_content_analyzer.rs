use pdfium_render::prelude::*;
use std::path::Path;

/// Analyze PDF content to determine optimal rendering resolution
/// Tests multiple resolutions to find the one closest to original file size
pub fn analyze_optimal_resolution(pdf_path: &str) -> Result<u32, String> {
    let path = Path::new(pdf_path);
    
    if !path.exists() {
        return Err("PDF file not found".to_string());
    }

    let pdf_data = std::fs::read(path)
        .map_err(|e| format!("Failed to read PDF: {}", e))?;

    let pdf_size_bytes = pdf_data.len();
    let pdf_size_mb = pdf_size_bytes as f64 / (1024.0 * 1024.0);
    
    eprintln!("[PDF_ANALYZER] Starting resolution optimization...");
    eprintln!("[PDF_ANALYZER] Original PDF: {:.1}MB ({} bytes)", pdf_size_mb, pdf_size_bytes);

    // Load PDF using pdfium-render (blocking - call from spawn_blocking)
    let pdfium = crate::utils::bind_pdfium(None)
        .map_err(|e| format!("Failed to initialize Pdfium: {}", e))?;
    let document = pdfium
        .load_pdf_from_byte_vec(pdf_data.clone(), None)
        .map_err(|e| format!("Failed to load PDF: {}", e))?;

    let page_count = document.pages().len() as u32;
    if page_count == 0 {
        return Err("PDF has no pages".to_string());
    }

    // Get first page dimensions
    let first_page = document.pages().get(0)
        .map_err(|e| format!("Failed to get first page: {}", e))?;
    
    let width_pt = first_page.width().value as f64;
    let height_pt = first_page.height().value as f64;
    
    eprintln!("[PDF_ANALYZER] Page size: {:.0}pt × {:.0}pt ({:.0}mm × {:.0}mm)", 
             width_pt, height_pt,
             width_pt * 0.3527, height_pt * 0.3527);

    // Test different DPI values to find optimal
    let test_dpis = vec![72, 100, 150, 200, 250, 300, 400, 500, 600, 800, 1000, 1200];
    let mut best_dpi = 300;
    let mut best_ratio = 999.0;

    for test_dpi in test_dpis {
        // Estimate rendered size for one page at this DPI
        let scale = test_dpi as f64 / 72.0;
        let page_width_px = (width_pt * scale).round() as u32;
        let page_height_px = (height_pt * scale).round() as u32;
        let pixels_per_page = page_width_px as u64 * page_height_px as u64;

        // Estimate PNG size: ~2-3 bytes per pixel with compression
        // Estimate JPEG size: ~0.3-0.5 bytes per pixel with quality 95
        let estimated_png_bytes = (pixels_per_page as f64 * 0.4) as u64; // conservative PNG estimate
        let estimated_total_bytes = estimated_png_bytes * page_count as u64;

        let size_ratio = estimated_total_bytes as f64 / pdf_size_bytes as f64;
        
        eprintln!("[PDF_ANALYZER] DPI {:4}: {:.0}×{:.0}px/page → ~{:.1}MB est. (ratio: {:.2}x original)",
                 test_dpi, page_width_px, page_height_px, 
                 estimated_total_bytes as f64 / 1_000_000.0,
                 size_ratio);

        // Find DPI where output size is closest to input size
        // (but not larger, to avoid excessive file sizes)
        let diff = (size_ratio - 1.0).abs();
        if diff < best_ratio && size_ratio <= 1.5 {
            best_ratio = diff;
            best_dpi = test_dpi;
        }
    }

    eprintln!("[PDF_ANALYZER] ✓ Recommended DPI: {} (estimated ratio: {:.2}x original)", 
             best_dpi, best_ratio + 1.0);

    Ok(best_dpi)
}
