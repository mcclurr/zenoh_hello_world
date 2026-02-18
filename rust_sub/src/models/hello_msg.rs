use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct HelloMsg {
    pub msg: String,
    pub counter: i64,
    pub temperature: f64,
    pub timestamp: String,
}
