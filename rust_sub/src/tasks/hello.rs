use tokio::sync::mpsc;
use tracing::info;

use messaging::subscriber::{Message, Subscriber, Result};

pub async fn run<S: Subscriber>(
    mut sub: S,
    tx: mpsc::Sender<Message>,
) -> Result<()> {
    loop {
        let msg = sub.next_message().await?;

        // your previous logging can stay
        if let Ok(s) = std::str::from_utf8(&msg.payload) {
            info!("[HELLO] topic={} payload={}", msg.topic, s);
        }

        if tx.send(msg).await.is_err() {
            info!("[HELLO] downstream dropped; exiting");
            return Ok(());
        }
    }
}
