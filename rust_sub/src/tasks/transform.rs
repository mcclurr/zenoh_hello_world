use tracing::info;
use tokio::sync::mpsc;
use prost::Message as ProstMessage;

use messaging::subscriber::{Message, Result};
use crate::proto::demo::HelloMsg;

pub async fn run(mut rx: mpsc::Receiver<Message>) -> Result<()> {
    while let Some(mut msg) = rx.recv().await {
        let mut hello = match HelloMsg::decode(&*msg.payload) {
            Ok(h) => h,
            Err(_) => continue,
        };

        hello.msg = format!("{} goodbye", hello.msg);

        let mut out = Vec::with_capacity(hello.encoded_len());
        hello.encode(&mut out)?;

        msg.payload = out;

        info!("[XFORM] topic={} msg={}", msg.topic, hello.msg);
    }
    Ok(())
}
