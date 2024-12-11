use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyndicationLink {
    pub url: String,
    pub platform: String,
}