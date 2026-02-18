use async_trait::async_trait;

type DynError = Box<dyn std::error::Error + Send + Sync + 'static>;

#[async_trait]
pub trait Publisher: Send + Sync {
    async fn publish(&self, key: &str, payload: &[u8]) -> Result<(), DynError>;
}
