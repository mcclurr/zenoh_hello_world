// src/logging.rs
use chrono::Local;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::EnvFilter;

pub fn init_logging(log_dir: &str) -> Result<WorkerGuard, std::io::Error> {
    std::fs::create_dir_all(log_dir)?;

    let filename = format!(
        "{}/{}.log",
        log_dir,
        Local::now().format("%Y%m%d_%H%M%S")
    );

    let file = std::fs::File::create(filename)?;
    let (non_blocking, guard) = tracing_appender::non_blocking(file);

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_writer(non_blocking)
        .with_file(true)
        .with_line_number(true)
        .with_target(false)
        .with_ansi(false)
        .init();

    Ok(guard)
}