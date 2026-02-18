#[derive(Debug)]
pub struct PipeMsg {
    pub key: String,
    pub payload: Vec<u8>,
}