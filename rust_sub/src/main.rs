use chrono::Local;
use tracing::info;
use tracing_appender::non_blocking;
use tracing_subscriber::EnvFilter;

mod messaging;
mod subscriber;
mod models;

use messaging::subscriber::Subscriber; // trait
use messaging::zenoh::subscriber::ZenohSubscriber; // concrete impl
use models::hello_msg::HelloMsg;

type DynError = Box<dyn std::error::Error + Send + Sync + 'static>;

fn zenoh_client_config() -> Result<zenoh::Config, DynError> {
    Ok(zenoh::Config::from_json5(r#"
    {
      mode: "client",
      connect: { endpoints: ["tcp/zenoh:7447"] }
    }
    "#)?)
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
    let key = "demo/hello";
    let conf = zenoh_client_config()?;

    // This is the “real subscriber” (concrete implementation)
    let mut sub = ZenohSubscriber::new(key, conf).await?;

    info!("Listening on {}", key);

    loop {
        let (key_expr, payload) = sub.recv().await?;

        match std::str::from_utf8(&payload) {
            Ok(s) => match serde_json::from_str::<HelloMsg>(s) {
                Ok(msg) => info!(
                    "Received: key={} msg='{}' counter={} temp={} ts={}",
                    key_expr, msg.msg, msg.counter, msg.temperature, msg.timestamp
                ),
                Err(e) => info!("Received key={} but JSON parse failed: {}", key_expr, e),
            },
            Err(_) => info!("Received key={} non-utf8 payload len={}", key_expr, payload.len()),
        }
    }
}
