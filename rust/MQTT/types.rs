use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct LoadTestResponse {
    pub message: String,
}

#[derive(Serialize, Deserialize)]
pub struct LoadTestRequest {
    pub payload: String,
}
