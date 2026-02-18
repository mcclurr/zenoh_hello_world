use async_trait::async_trait;

type DynError = Box<dyn std::error::Error + Send + Sync + 'static>;
pub type Result<T> = std::result::Result<T, DynError>;

#[derive(Debug, Clone)]
pub struct Message {
    pub topic: String,
    pub payload: Vec<u8>,
    pub reply_to: Option<String>,
}

#[async_trait]
pub trait Subscriber: Send + Sync {
    async fn subscribe(&mut self, pattern: &str) -> Result<()>;
    async fn next_message(&mut self) -> Result<Message>;
    async fn is_connected(&self) -> bool;
}
