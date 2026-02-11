use flume;
use zenoh::{Config, Session};

use crate::logger::Logger;

type DynError = Box<dyn std::error::Error + Send + Sync + 'static>;

pub async fn run(logger: &Logger) -> Result<(), DynError> {
    let key = "demo/hello";

    let conf = Config::from_json5(r#"
    {
      mode: "client",
      connect: { endpoints: ["tcp/zenoh:7447"] }
    }
    "#)?;

    let session: Session = zenoh::open(conf).await?;

    logger.log(format!("Rust subscriber started. Subscribing to '{key}'"));
    logger.log(format!("Logging to: {}", logger.path().display()));

    let sub = session
        .declare_subscriber(key)
        .with(flume::bounded(64))
        .await?;

    loop {
        let sample = sub.recv_async().await?;

        match sample.payload().try_to_string() {
            Ok(s) => logger.log(format!(
                "Received: key_expr={} payload='{}'",
                sample.key_expr(),
                s
            )),
            Err(e) => {
                // If it isn't valid UTF-8, fall back to raw bytes (may allocate if fragmented)
                let bytes = sample.payload().to_bytes();
                logger.log(format!(
                    "Received: key_expr={} payload_bytes={:?} (utf8_error={})",
                    sample.key_expr(),
                    bytes,
                    e
                ));
            }
        }
    }

}
