use async_trait::async_trait;

type DynError = Box<dyn std::error::Error + Send + Sync + 'static>;

#[async_trait]
pub trait Subscriber: Send + Sync {
    async fn recv(&mut self) -> Result<(String, Vec<u8>), DynError>;
}
