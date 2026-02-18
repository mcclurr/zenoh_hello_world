use tokio::sync::mpsc;
use tracing::info;
use serde_json::Value;

use messaging::subscriber::{Message, Result};

pub async fn run(mut rx: mpsc::Receiver<Message>) -> Result<()> {
    while let Some(mut msg) = rx.recv().await {
        let s = match std::str::from_utf8(&msg.payload) {
            Ok(s) => s,
            Err(_) => continue,
        };

        let mut v: Value = match serde_json::from_str(s) {
            Ok(v) => v,
            Err(_) => continue,
        };

        if let Some(old) = v.get("msg").and_then(|x| x.as_str()) {
            v["msg"] = Value::String(format!("{old} goodbye"));
        } else {
            continue;
        }

        let out = serde_json::to_string(&v)?;
        msg.payload = out.into_bytes();

        info!("[XFORM] topic={} {}", msg.topic, String::from_utf8_lossy(&msg.payload));
    }
    Ok(())
}
