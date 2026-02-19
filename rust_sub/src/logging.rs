// src/logging.rs
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::EnvFilter;

pub fn init_logging(log_dir: &str, log_name: &str, rust_log: &str) -> Result<WorkerGuard, std::io::Error> {
    let ts = chrono::Local::now().format("%Y%m%d_%H%M%S").to_string();
    init_logging_with_ts(log_dir, log_name, rust_log, &ts)
}

pub fn init_logging_with_ts(
    log_dir: &str,
    log_name: &str,
    rust_log: &str,
    ts: &str,
) -> Result<WorkerGuard, std::io::Error> {
    std::fs::create_dir_all(log_dir)?;

    // RM-20260219_003321.log
    let filename = format!("{}/{}-{}.log", log_dir, log_name, ts);
    let file = std::fs::File::create(&filename)?;
    let (non_blocking, guard) = tracing_appender::non_blocking(file);

    // Prefer passed-in filter; fall back to env if empty
    let filter = if rust_log.trim().is_empty() {
        EnvFilter::from_default_env()
    } else {
        EnvFilter::new(rust_log)
    };

    // IMPORTANT: don't panic if already initialized
    let _ = tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_writer(non_blocking)
        .with_file(true)
        .with_line_number(true)
        .with_target(false)
        .with_ansi(false)
        .try_init();

    Ok(guard)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;
    use tempfile::tempdir;

    #[test]
    fn creates_expected_log_file_name() {
        let dir = tempdir().unwrap();
        let log_dir = dir.path().join("out");
        let log_dir_str = log_dir.to_str().unwrap();

        let ts = "20260219_003321";
        let _guard = init_logging_with_ts(log_dir_str, "RM", "info", ts).unwrap();

        let expected = log_dir.join("RM-20260219_003321.log");
        assert!(Path::new(&expected).exists());
    }

    #[test]
    fn returns_guard_even_if_subscriber_already_initialized() {
        let dir = tempdir().unwrap();
        let log_dir = dir.path().join("out");
        let log_dir_str = log_dir.to_str().unwrap();

        let _g1 = init_logging_with_ts(log_dir_str, "RM", "info", "t1").unwrap();
        let _g2 = init_logging_with_ts(log_dir_str, "RM", "info", "t2").unwrap();

        assert!(log_dir.join("RM-t1.log").exists());
        assert!(log_dir.join("RM-t2.log").exists());
    }
}
