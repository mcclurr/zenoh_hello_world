// src/tasks/metrics.rs
use tracing::info;
use prost::Message as ProstMessage;

use messaging::subscriber::{Subscriber, Result};
use crate::proto::demo::MetricsMsg;

pub async fn run<S: Subscriber>(mut sub: S) -> Result<()> {
    loop {
        let msg = sub.next_message().await?;

        match MetricsMsg::decode(&*msg.payload) {
            Ok(m) => info!(
                "[METRICS] topic={} cpu={} mem={} ts={}",
                msg.topic, m.cpu, m.mem, m.timestamp
            ),
            Err(e) => info!("[METRICS] topic={} proto decode failed: {}", msg.topic, e),
        }
    }
}
