use flume::Receiver;
use zenoh::{Config, Session};
use async_trait::async_trait;

use crate::messaging::subscriber::Subscriber;

type DynError = Box<dyn std::error::Error + Send + Sync + 'static>;

pub struct ZenohSubscriber {
    _session: Session,               // keep session alive
    sub: zenoh::pubsub::Subscriber<flume::Receiver<zenoh::sample::Sample>>,
}

impl ZenohSubscriber {
    pub async fn new(key: &str, config: Config) -> Result<Self, DynError> {
        let session = zenoh::open(config).await?;

        let sub = session
            .declare_subscriber(key)
            .with(flume::bounded(64))
            .await?;

        Ok(Self {
            _session: session,
            sub,
        })
    }
}

#[async_trait]
impl Subscriber for ZenohSubscriber {
    async fn recv(&mut self) -> Result<(String, Vec<u8>), DynError> {
        let sample = self.sub.recv_async().await?;

        // key expr
        let key = sample.key_expr().to_string();

        // payload -> bytes (handles fragmented)
        let bytes: Vec<u8> = sample.payload().to_bytes().to_vec();

        Ok((key, bytes))
    }
}
