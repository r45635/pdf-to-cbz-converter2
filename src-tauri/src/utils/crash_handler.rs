use std::panic;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Instant;

static PANIC_HANDLER_INSTALLED: AtomicBool = AtomicBool::new(false);

/// Install a custom panic handler that logs panic information
/// This helps debug crashes that happen silently
pub fn install_panic_handler() {
    // Only install once
    if PANIC_HANDLER_INSTALLED.compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst).is_err() {
        return;
    }

    panic::set_hook(Box::new(|panic_info| {
        let payload = panic_info.payload();
        let message = if let Some(s) = payload.downcast_ref::<&str>() {
            s.to_string()
        } else if let Some(s) = payload.downcast_ref::<String>() {
            s.clone()
        } else {
            "Unknown panic payload".to_string()
        };

        let location = if let Some(location) = panic_info.location() {
            format!("{}:{}:{}", location.file(), location.line(), location.column())
        } else {
            "Unknown location".to_string()
        };

        eprintln!("╔════════════════════════════════════════════════════════════╗");
        eprintln!("║                   APPLICATION PANIC                        ║");
        eprintln!("╠════════════════════════════════════════════════════════════╣");
        eprintln!("║ Message: {:<50} ║", truncate(&message, 50));
        eprintln!("║ Location: {:<49} ║", truncate(&location, 49));
        eprintln!("║ Time: {:<53} ║", truncate(&chrono::Local::now().to_rfc3339(), 53));
        eprintln!("╠════════════════════════════════════════════════════════════╣");
        eprintln!("║ This crash information has been logged.                   ║");
        eprintln!("║ Please report this issue with the above details.          ║");
        eprintln!("╚════════════════════════════════════════════════════════════╝");

        // Try to write to a crash log file
        if let Err(e) = write_crash_log(&message, &location) {
            eprintln!("[CRASH] Failed to write crash log: {}", e);
        }
    }));

    eprintln!("[CRASH_HANDLER] Panic handler installed successfully");
}

/// Write crash information to a log file
fn write_crash_log(message: &str, location: &str) -> std::io::Result<()> {
    use std::io::Write;

    let log_dir = std::env::temp_dir().join("pdf-to-cbz-converter");
    std::fs::create_dir_all(&log_dir)?;

    let log_file = log_dir.join("crash.log");
    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_file)?;

    writeln!(file, "==========================================")?;
    writeln!(file, "Crash at: {}", chrono::Local::now().to_rfc3339())?;
    writeln!(file, "Message: {}", message)?;
    writeln!(file, "Location: {}", location)?;
    writeln!(file, "==========================================")?;

    Ok(())
}

fn truncate(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}...", &s[..max_len.saturating_sub(3)])
    }
}

/// Get current process memory usage in MB
pub fn get_memory_usage_mb() -> f64 {
    #[cfg(target_os = "windows")]
    {
        use std::mem;
        use winapi::um::processthreadsapi::GetCurrentProcess;
        use winapi::um::psapi::{GetProcessMemoryInfo, PROCESS_MEMORY_COUNTERS};

        unsafe {
            let process = GetCurrentProcess();
            let mut pmc: PROCESS_MEMORY_COUNTERS = mem::zeroed();
            pmc.cb = mem::size_of::<PROCESS_MEMORY_COUNTERS>() as u32;

            if GetProcessMemoryInfo(process, &mut pmc, pmc.cb) != 0 {
                return pmc.WorkingSetSize as f64 / (1024.0 * 1024.0);
            }
        }
    }

    #[cfg(target_os = "macos")]
    {
        // Note: macOS memory monitoring requires mach kernel APIs
        // For simplicity and to avoid potential compilation issues,
        // we're skipping memory monitoring on macOS for now
        // The panic handler and crash logging will still work
    }

    #[cfg(target_os = "linux")]
    {
        if let Ok(status) = std::fs::read_to_string("/proc/self/status") {
            for line in status.lines() {
                if line.starts_with("VmRSS:") {
                    if let Some(kb_str) = line.split_whitespace().nth(1) {
                        if let Ok(kb) = kb_str.parse::<f64>() {
                            return kb / 1024.0; // Convert KB to MB
                        }
                    }
                }
            }
        }
    }

    0.0 // Fallback if we can't determine memory usage
}

/// Memory monitor that tracks usage during operations
pub struct MemoryMonitor {
    start_memory: f64,
    start_time: Instant,
    operation_name: String,
    last_report: Instant,
}

impl MemoryMonitor {
    pub fn new(operation_name: &str) -> Self {
        let start_memory = get_memory_usage_mb();
        let now = Instant::now();

        eprintln!("[MEM] Starting operation '{}' - Initial memory: {:.1} MB", operation_name, start_memory);

        Self {
            start_memory,
            start_time: now,
            operation_name: operation_name.to_string(),
            last_report: now,
        }
    }

    /// Check and report memory usage if it's been a while or memory increased significantly
    pub fn check(&mut self, context: &str) {
        let current_memory = get_memory_usage_mb();
        let elapsed = self.start_time.elapsed();
        let since_last_report = self.last_report.elapsed();

        let delta = current_memory - self.start_memory;

        // Report every 5 seconds or if memory increased by 100MB
        if since_last_report.as_secs() >= 5 || delta > 100.0 {
            eprintln!(
                "[MEM] {} @ {}: {:.1} MB (Δ{:+.1} MB) - {:.1}s elapsed",
                self.operation_name,
                context,
                current_memory,
                delta,
                elapsed.as_secs_f64()
            );
            self.last_report = Instant::now();
        }

        // Warn if memory usage is very high
        if current_memory > 2048.0 {
            eprintln!(
                "[MEM WARNING] High memory usage: {:.1} MB - Consider closing other applications",
                current_memory
            );
        }
    }

    /// Final report on completion
    pub fn finish(self) {
        let final_memory = get_memory_usage_mb();
        let elapsed = self.start_time.elapsed();
        let delta = final_memory - self.start_memory;

        eprintln!(
            "[MEM] Completed '{}': {:.1} MB (Δ{:+.1} MB) in {:.1}s",
            self.operation_name,
            final_memory,
            delta,
            elapsed.as_secs_f64()
        );
    }
}
