use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc, NaiveDate};
use super::webmention::Webmention;
use super::syndication::SyndicationLink;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlogPost {
    pub title: String,
    pub author: String,
    pub date: NaiveDate,
    pub description: String,
    pub content: String,
    pub slug: String,
    pub author_url: Option<String>,
    pub syndication: Vec<SyndicationLink>,
    pub in_reply_to: Option<String>,
    pub like_of: Option<String>,
    pub repost_of: Option<String>,
    pub webmentions: Vec<Webmention>,
}