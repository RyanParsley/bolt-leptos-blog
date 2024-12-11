use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Webmention {
    pub source: String,
    pub target: String,
    pub published: DateTime<Utc>,
    pub author: WebmentionAuthor,
    pub content: Option<String>,
    pub interaction_type: WebmentionType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebmentionAuthor {
    pub name: String,
    pub url: Option<String>,
    pub photo: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WebmentionType {
    Reply,
    Like,
    Repost,
    Mention,
}