// src/tasks/hello.rs
use tracing::info;
use tokio::sync::mpsc;

use messaging::subscriber::Subscriber;
use messaging::zenoh::subscriber::ZenohSubscriber;

use crate::models::hello_msg::HelloMsg;
use crate::tasks::types::PipeMsg;

type DynError = Box<dyn std::error::Error + Send + Sync + 'static>;

pub async fn run(mut sub: ZenohSubscriber, tx: mpsc::Sender<PipeMsg>) -> Result<(), DynError> {
    loop {
        let (key, payload) = sub.recv().await?;

        if let Ok(s) = std::str::from_utf8(&payload) {
            match serde_json::from_str::<HelloMsg>(s) {
                Ok(msg) => info!(
                    "[HELLO] key={} msg='{}' counter={} temp={} ts={}",
                    key, msg.msg, msg.counter, msg.temperature, msg.timestamp
                ),
                Err(e) => info!("[HELLO] key={} JSON parse failed: {}", key, e),
            }
        }

        // push into pipeline
        if tx.send(PipeMsg { key, payload }).await.is_err() {
            // receiver dropped -> exit gracefully
            info!("[HELLO] pipeline receiver dropped; exiting");
            return Ok(());
        }
    }
}
