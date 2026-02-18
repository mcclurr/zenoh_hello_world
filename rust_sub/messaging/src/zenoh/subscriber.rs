use async_trait::async_trait;
use flume::Receiver;
use zenoh::{Config, Session};

use crate::subscriber::{Message, Result, Subscriber};

pub struct ZenohSubscriber {
    session: Session,
    sub: Option<zenoh::pubsub::Subscriber<Receiver<zenoh::sample::Sample>>>,
}

impl ZenohSubscriber {
    pub async fn new(config: Config) -> Result<Self> {
        let session = zenoh::open(config).await?;
        Ok(Self { session, sub: None })
    }
}

#[async_trait]
impl Subscriber for ZenohSubscriber {
    async fn subscribe(&mut self, pattern: &str) -> Result<()> {
        let sub = self
            .session
            .declare_subscriber(pattern)
            .with(flume::bounded(1024))
            .await?;

        self.sub = Some(sub);
        Ok(())
    }

    async fn next_message(&mut self) -> Result<Message> {
        let sub = self
            .sub
            .as_mut()
            .ok_or_else(|| "ZenohSubscriber: subscribe() not called".to_string())?;

        let sample = sub.recv_async().await?;

        Ok(Message {
            topic: sample.key_expr().to_string(),
            payload: sample.payload().to_bytes().to_vec(),
            reply_to: None,
        })
    }

    async fn is_connected(&self) -> bool {
        true
    }
}
