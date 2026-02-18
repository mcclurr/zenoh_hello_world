use chrono::Local;
use tracing::info;
use tracing_appender::non_blocking;
use tracing_subscriber::EnvFilter;

mod models;

use messaging::subscriber::Subscriber; // trait
use messaging::zenoh::subscriber::ZenohSubscriber; // concrete impl
use models::hello_msg::HelloMsg;
use models::metrics_msg::MetricsMsg;

type DynError = Box<dyn std::error::Error + Send + Sync + 'static>;

fn zenoh_client_config() -> Result<zenoh::Config, DynError> {
    Ok(zenoh::Config::from_json5(r#"
    {
      mode: "client",
      connect: { endpoints: ["tcp/zenoh:7447"] }
    }
    "#)?)
}

async fn run_hello(mut sub: ZenohSubscriber) -> Result<(), DynError> {
    loop {
        let (key_expr, payload) = sub.recv().await?;
        if let Ok(s) = std::str::from_utf8(&payload) {
            match serde_json::from_str::<HelloMsg>(s) {
                Ok(msg) => info!(
                    "[HELLO] key={} msg='{}' counter={} temp={} ts={}",
                    key_expr, msg.msg, msg.counter, msg.temperature, msg.timestamp
                ),
                Err(e) => info!("[HELLO] key={} JSON parse failed: {}", key_expr, e),
            }
        }
    }
}

async fn run_metrics(mut sub: ZenohSubscriber) -> Result<(), DynError> {
    loop {
        let (key_expr, payload) = sub.recv().await?;
        if let Ok(s) = std::str::from_utf8(&payload) {
            match serde_json::from_str::<MetricsMsg>(s) {
                Ok(m) => info!(
                    "[METRICS] key={} cpu={} mem={} ts={}",
                    key_expr, m.cpu, m.mem, m.timestamp
                ),
                Err(e) => info!("[METRICS] key={} JSON parse failed: {}", key_expr, e),
            }
        }
    }
}

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
        .with_ansi(false)
        .init();

    info!("Logger initialized");

    let conf1 = zenoh_client_config()?;
    let conf2 = zenoh_client_config()?;

    let hello_sub = ZenohSubscriber::new("demo/hello", conf1).await?;
    let metrics_sub = ZenohSubscriber::new("demo/metrics", conf2).await?;

    let t1 = tokio::spawn(async move { run_hello(hello_sub).await });
    let t2 = tokio::spawn(async move { run_metrics(metrics_sub).await });

    // If either task returns an error, bubble it up:
    t1.await??;
    t2.await??;

    Ok(())
}
