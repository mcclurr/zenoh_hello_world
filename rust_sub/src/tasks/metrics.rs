// src/tasks/metrics.rs
use tracing::info;
use messaging::subscriber::Subscriber;
use messaging::zenoh::subscriber::ZenohSubscriber;
use crate::models::metrics_msg::MetricsMsg;

type DynError = Box<dyn std::error::Error + Send + Sync + 'static>;

pub async fn run(mut sub: ZenohSubscriber) -> Result<(), DynError> {
    loop {
        let (key_expr, payload) = sub.recv().await?;
        if let Ok(s) = std::str::from_utf8(&payload) {
            match serde_json::from_str::<MetricsMsg>(s) {
                Ok(m) => info!(
                    "[METRICS] key={} cpu={} mem={} ts={}",
                    key_expr, m.cpu, m.mem, m.timestamp
                ),
                Err(e) => info!("[METRICS] key={} JSON parse failed: {}", key_expr, e),
            }
        }
    }
}
