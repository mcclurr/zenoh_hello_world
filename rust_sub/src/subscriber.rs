use tracing::{debug, info, warn};
use zenoh::{Config, Session};
use flume;

type DynError = Box<dyn std::error::Error + Send + Sync + 'static>;

pub async fn run() -> Result<(), DynError> {
    let key = "demo/hello";

    info!("Subscriber starting");

    let conf = Config::from_json5(r#"
    {
      mode: "client",
      connect: { endpoints: ["tcp/zenoh:7447"] }
    }
    "#)?;

    let session: Session = zenoh::open(conf).await?;

    info!("Subscribed to '{}'", key);

    let sub = session
        .declare_subscriber(key)
        .with(flume::bounded(64))
        .await?;

    loop {
        let sample = sub.recv_async().await?;

        match sample.payload().try_to_string() {
            Ok(s) => debug!("Received payload: {}", s),
            Err(_) => warn!("Received non-utf8 payload"),
        }
    }
}
