mod logger;
mod subscriber;

use logger::Logger;

type DynError = Box<dyn std::error::Error + Send + Sync + 'static>;

#[tokio::main]
async fn main() -> Result<(), DynError> {
    let logger = Logger::new("out")?;

    logger.info("Rust subscriber container starting up");

    subscriber::run(&logger).await
}
