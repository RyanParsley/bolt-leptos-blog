use sqlx::SqlitePool;
use chrono::{DateTime, Utc};
use crate::types::webmention::{Webmention, WebmentionAuthor, WebmentionType};
use std::error::Error;

pub struct Store {
    pool: SqlitePool,
}

impl Store {
    pub async fn new(database_url: &str) -> Result<Self, Box<dyn Error>> {
        let pool = SqlitePool::connect(database_url).await?;
        
        // Initialize tables
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS webmentions (
                id INTEGER PRIMARY KEY,
                source TEXT NOT NULL,
                target TEXT NOT NULL,
                author_name TEXT NOT NULL,
                author_url TEXT,
                author_photo TEXT,
                content TEXT,
                published DATETIME NOT NULL,
                interaction_type TEXT NOT NULL,
                verified BOOLEAN NOT NULL DEFAULT 0,
                created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
            )
            "#,
        )
        .execute(&pool)
        .await?;

        Ok(Store { pool })
    }

    pub async fn save_webmention(
        &self,
        source: &str,
        target: &str,
    ) -> Result<(), Box<dyn Error>> {
        sqlx::query(
            r#"
            INSERT INTO webmentions (source, target, author_name, published, interaction_type)
            VALUES (?, ?, ?, ?, ?)
            "#,
        )
        .bind(source)
        .bind(target)
        .bind("Unknown") // Placeholder until we fetch author info
        .bind(Utc::now())
        .bind("mention")
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn get_webmentions_for_url(&self, url: &str) -> Result<Vec<Webmention>, Box<dyn Error>> {
        let rows = sqlx::query!(
            r#"
            SELECT * FROM webmentions
            WHERE target = ? AND verified = 1
            ORDER BY published DESC
            "#,
            url
        )
        .fetch_all(&self.pool)
        .await?;

        let webmentions = rows
            .into_iter()
            .map(|row| Webmention {
                source: row.source,
                target: row.target,
                published: DateTime::from_naive_utc_and_offset(row.published, Utc),
                author: WebmentionAuthor {
                    name: row.author_name,
                    url: row.author_url,
                    photo: row.author_photo,
                },
                content: row.content,
                interaction_type: match row.interaction_type.as_str() {
                    "reply" => WebmentionType::Reply,
                    "like" => WebmentionType::Like,
                    "repost" => WebmentionType::Repost,
                    _ => WebmentionType::Mention,
                },
            })
            .collect();

        Ok(webmentions)
    }
}