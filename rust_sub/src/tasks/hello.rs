// src/tasks/hello.rs
use tracing::info;
use messaging::subscriber::Subscriber;
use messaging::zenoh::subscriber::ZenohSubscriber;
use crate::models::hello_msg::HelloMsg;

type DynError = Box<dyn std::error::Error + Send + Sync + 'static>;

pub async fn run(mut sub: ZenohSubscriber) -> Result<(), DynError> {
    loop {
        let (key_expr, payload) = sub.recv().await?;
        if let Ok(s) = std::str::from_utf8(&payload) {
            match serde_json::from_str::<HelloMsg>(s) {
                Ok(msg) => info!(
                    "[HELLO] key={} msg='{}' counter={} temp={} ts={}",
                    key_expr, msg.msg, msg.counter, msg.temperature, msg.timestamp
                ),
                Err(e) => info!("[HELLO] key={} JSON parse failed: {}", key_expr, e),
            }
        }
    }
}
