// src/tasks/metrics.rs
use tracing::info;

use messaging::subscriber::{Subscriber, Result};

use crate::models::metrics_msg::MetricsMsg;

pub async fn run<S: Subscriber>(mut sub: S) -> Result<()> {
    loop {
        let msg = sub.next_message().await?;

        if let Ok(s) = std::str::from_utf8(&msg.payload) {
            match serde_json::from_str::<MetricsMsg>(s) {
                Ok(m) => info!(
                    "[METRICS] topic={} cpu={} mem={} ts={}",
                    msg.topic, m.cpu, m.mem, m.timestamp
                ),
                Err(e) => info!("[METRICS] topic={} JSON parse failed: {}", msg.topic, e),
            }
        }
    }
}
