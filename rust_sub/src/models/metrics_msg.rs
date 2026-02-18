// rust_sub/src/models/metrics_msg.rs
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct MetricsMsg {
    pub cpu: f64,
    pub mem: f64,
    pub timestamp: String,
}
