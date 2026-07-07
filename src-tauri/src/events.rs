use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct ProgressEvent {
    pub stage: String,
    pub percent: u32,
    pub message: String,
}
