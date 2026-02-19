use tokio::sync::mpsc;
use tracing::info;

mod logging;
mod config;
mod tasks;
mod models;
mod proto;

use messaging::subscriber::Message;
use messaging::zenoh::subscriber::ZenohSubscriber;
use messaging::subscriber::Subscriber;

type DynError = Box<dyn std::error::Error + Send + Sync + 'static>;

#[tokio::main]
async fn main() -> Result<(), DynError> {
    let _log_guard = logging::init_logging("out")?;
    info!("Logger initialized");

    let (tx_hello, rx_hello) = mpsc::channel::<Message>(1024);

    let mut hello_sub = ZenohSubscriber::new(config::zenoh_client_config()?).await?;
    hello_sub.subscribe("demo/hello").await?;

    let mut metrics_sub = ZenohSubscriber::new(config::zenoh_client_config()?).await?;
    metrics_sub.subscribe("demo/metrics").await?;

    let t1 = tokio::spawn(async move { tasks::hello::run(hello_sub, tx_hello).await });
    let t2 = tokio::spawn(async move { tasks::metrics::run(metrics_sub).await });
    let t3 = tokio::spawn(async move { tasks::transform::run(rx_hello).await });

    t1.await??;
    t2.await??;
    t3.await??;

    Ok(())
}
