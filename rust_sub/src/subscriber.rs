use flume;
use zenoh::{Config, Session};

use crate::logger::Logger;

type DynError = Box<dyn std::error::Error + Send + Sync + 'static>;

pub async fn run(logger: &Logger) -> Result<(), DynError> {
    let key = "demo/hello";

    logger.info("Starting Rust Zenoh subscriber…");

    let conf = Config::from_json5(r#"
    {
      mode: "client",
      connect: { endpoints: ["tcp/zenoh:7447"] }
    }
    "#)?;

    logger.debug("Opening Zenoh session…");
    let session: Session = zenoh::open(conf).await?;

    logger.info(format!("Subscribed to key: '{}'", key));
    logger.info(format!("Log file: {}", logger.path().display()));

    let sub = session
        .declare_subscriber(key)
        .with(flume::bounded(64))
        .await?;

    logger.info("Waiting for messages…");

    loop {
        let sample = sub.recv_async().await?;

        match sample.payload().try_to_string() {
            Ok(s) => logger.debug(format!(
                "Received: key_expr={} payload='{}'",
                sample.key_expr(),
                s
            )),
            Err(e) => {
                let bytes = sample.payload().to_bytes();
                logger.warn(format!(
                    "Received non-UTF8 payload: key_expr={} bytes={:?} err={}",
                    sample.key_expr(),
                    bytes,
                    e
                ));
            }
        }
    }
}
