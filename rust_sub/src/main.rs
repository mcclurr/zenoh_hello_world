use tracing::info;
use messaging::zenoh::subscriber::ZenohSubscriber;

mod models;
mod logging;
mod config;
mod tasks;

type DynError = Box<dyn std::error::Error + Send + Sync + 'static>;

#[tokio::main]
async fn main() -> Result<(), DynError> {
    let _log_guard = logging::init_logging("out")?;
    info!("Logger initialized");

    let hello_sub = ZenohSubscriber::new("demo/hello", config::zenoh_client_config()?).await?;
    let metrics_sub = ZenohSubscriber::new("demo/metrics", config::zenoh_client_config()?).await?;

    let t1 = tokio::spawn(async move { tasks::hello::run(hello_sub).await });
    let t2 = tokio::spawn(async move { tasks::metrics::run(metrics_sub).await });

    t1.await??;
    t2.await??;

    Ok(())
}
