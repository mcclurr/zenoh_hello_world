use async_trait::async_trait;
use zenoh::{Config, Session};

use crate::messaging::publisher::Publisher;

type DynError = Box<dyn std::error::Error + Send + Sync + 'static>;

pub struct ZenohPublisher {
    session: Session,
}

impl ZenohPublisher {
    pub async fn new(config: Config) -> Result<Self, DynError> {
        let session = zenoh::open(config).await?;
        Ok(Self { session })
    }
}

#[async_trait]
impl Publisher for ZenohPublisher {
    async fn publish(&self, key: &str, payload: &[u8]) -> Result<(), DynError> {
        let pub_ = self.session.declare_publisher(key).await?;
        pub_.put(payload).await?;
        Ok(())
    }
}
