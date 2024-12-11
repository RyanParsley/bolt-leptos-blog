use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebmentionEndpoint {
    pub endpoint: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebmentionResponse {
    pub status: String,
    pub source: String,
    pub target: String,
    pub id: Option<String>,
}