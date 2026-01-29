// Benchmark module for testing different CBZ compression strategies
use anyhow::Result;
use std::io::{Cursor, Write};
use std::time::Instant;
use zip::write::SimpleFileOptions;
use zip::ZipWriter;

/// Create CBZ with Stored (no compression) - optimal for JPEG
pub fn create_cbz_stored(images: &[(String, Vec<u8>)]) -> Result<Vec<u8>> {
    let buffer = Cursor::new(Vec::new());
    let mut zip = ZipWriter::new(buffer);

    let options = SimpleFileOptions::default()
        .compression_method(zip::CompressionMethod::Stored);

    for (filename, data) in images {
        zip.start_file(filename, options)?;
        zip.write_all(data)?;
    }

    let buffer = zip.finish()?;
    Ok(buffer.into_inner())
}

/// Create CBZ with Deflated compression level 6 (current default)
pub fn create_cbz_deflated(images: &[(String, Vec<u8>)]) -> Result<Vec<u8>> {
    let buffer = Cursor::new(Vec::new());
    let mut zip = ZipWriter::new(buffer);

    let options = SimpleFileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated)
        .compression_level(Some(6));

    for (filename, data) in images {
        zip.start_file(filename, options)?;
        zip.write_all(data)?;
    }

    let buffer = zip.finish()?;
    Ok(buffer.into_inner())
}

/// Run benchmark comparing compression methods
pub fn run_benchmark(images: &[(String, Vec<u8>)]) {
    println!("\n=== CBZ COMPRESSION BENCHMARK ===");
    println!("Input: {} images, {:.1} MB total",
        images.len(),
        images.iter().map(|(_, d)| d.len()).sum::<usize>() as f64 / 1024.0 / 1024.0
    );

    // Test Stored (no compression)
    let start = Instant::now();
    let stored_result = create_cbz_stored(images).unwrap();
    let stored_time = start.elapsed();
    println!("\n[STORED] No compression:");
    println!("  Time: {:.2}s", stored_time.as_secs_f64());
    println!("  Size: {:.1} MB", stored_result.len() as f64 / 1024.0 / 1024.0);

    // Test Deflated level 6
    let start = Instant::now();
    let deflated_result = create_cbz_deflated(images).unwrap();
    let deflated_time = start.elapsed();
    println!("\n[DEFLATED-6] Default compression:");
    println!("  Time: {:.2}s", deflated_time.as_secs_f64());
    println!("  Size: {:.1} MB", deflated_result.len() as f64 / 1024.0 / 1024.0);

    // Summary
    let time_saved = deflated_time.as_secs_f64() - stored_time.as_secs_f64();
    let size_diff = stored_result.len() as f64 - deflated_result.len() as f64;
    let size_diff_pct = (size_diff / stored_result.len() as f64) * 100.0;

    println!("\n=== SUMMARY ===");
    println!("Time saved with STORED: {:.2}s ({:.0}x faster)",
        time_saved,
        deflated_time.as_secs_f64() / stored_time.as_secs_f64()
    );
    println!("Size difference: {:.1} MB ({:.1}%)",
        size_diff / 1024.0 / 1024.0,
        size_diff_pct
    );
    println!("Recommendation: {} is better for JPEG images",
        if size_diff_pct < 5.0 { "STORED" } else { "DEFLATED" }
    );
}
