use chrono::Local;
use tracing::info;
use tracing_appender::non_blocking;
use tracing_subscriber::EnvFilter;

mod subscriber;

type DynError = Box<dyn std::error::Error + Send + Sync + 'static>;

#[tokio::main]
async fn main() -> Result<(), DynError> {
    std::fs::create_dir_all("out")?;

    let filename = format!("{}.log", Local::now().format("%Y%m%d_%H%M%S"));
    let file = std::fs::File::create(format!("out/{}", filename))?;

    let (non_blocking_file, _guard) = non_blocking(file);

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_writer(non_blocking_file)
        .with_file(true)
        .with_line_number(true)
        .with_target(false)
        .with_ansi(false)   // ← THIS is the fix
        .init();

    info!("Logger initialized");
    subscriber::run().await
}
