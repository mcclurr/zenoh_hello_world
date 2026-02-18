// src/tasks/transform.rs
use tracing::info;
use tokio::sync::mpsc;
use serde_json::Value;

use crate::tasks::types::PipeMsg;

type DynError = Box<dyn std::error::Error + Send + Sync + 'static>;

pub async fn run(mut rx: mpsc::Receiver<PipeMsg>) -> Result<(), DynError> {
    while let Some(mut msg) = rx.recv().await {
        let s = match std::str::from_utf8(&msg.payload) {
            Ok(s) => s,
            Err(_) => {
                info!("[XFORM] non-utf8 payload key={} len={}", msg.key, msg.payload.len());
                continue;
            }
        };

        let mut v: Value = match serde_json::from_str(s) {
            Ok(v) => v,
            Err(e) => {
                info!("[XFORM] JSON parse failed key={} err={}", msg.key, e);
                continue;
            }
        };

        // deterministic transform: append " goodbye" to "msg"
        if let Some(old) = v.get("msg").and_then(|x| x.as_str()) {
            v["msg"] = Value::String(format!("{old} goodbye"));
        } else {
            info!("[XFORM] key={} missing 'msg' field (skipping)", msg.key);
            continue;
        }

        let out = serde_json::to_string(&v)?;
        info!("[XFORM] key={} {}", msg.key, out);

        // if you later want, you can forward to another channel here
        msg.payload = out.into_bytes();
    }

    info!("[XFORM] input channel closed; exiting");
    Ok(())
}
