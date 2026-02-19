use tracing::info;
use prost::Message as ProstMessage;

use messaging::subscriber::{Message, Subscriber, Result};
use crate::proto::demo::HelloMsg;

pub async fn run<S: Subscriber>(
    mut sub: S,
    tx: tokio::sync::mpsc::Sender<Message>,
) -> Result<()> {
    loop {
        let msg = sub.next_message().await?;

        match HelloMsg::decode(&*msg.payload) {
            Ok(h) => {
                info!(
                    "[HELLO] topic={} msg={} counter={} temp={} ts={}",
                    msg.topic, h.msg, h.counter, h.temperature, h.timestamp
                );
            }
            Err(e) => {
                info!("[HELLO] failed to decode proto: {}", e);
            }
        }

        if tx.send(msg).await.is_err() {
            info!("[HELLO] downstream dropped; exiting");
            return Ok(());
        }
    }
}
