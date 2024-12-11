use scraper::{Html, Selector};
use url::Url;
use crate::types::webmention_endpoint::WebmentionEndpoint;
use super::http_client;
use std::error::Error;

pub async fn discover_endpoints(url: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let html = http_client::get::<String>(url).await?;
    let document = Html::parse_document(&html);
    
    // Check link headers
    let mut endpoints = Vec::new();
    
    // Check HTML links
    let link_selector = Selector::parse(r#"link[rel~="webmention"]"#).unwrap();
    let a_selector = Selector::parse(r#"a[rel~="webmention"]"#).unwrap();
    
    for element in document.select(&link_selector) {
        if let Some(href) = element.value().attr("href") {
            endpoints.push(resolve_url(url, href)?);
        }
    }
    
    for element in document.select(&a_selector) {
        if let Some(href) = element.value().attr("href") {
            endpoints.push(resolve_url(url, href)?);
        }
    }
    
    Ok(endpoints)
}

fn resolve_url(base: &str, href: &str) -> Result<String, Box<dyn Error>> {
    let base_url = Url::parse(base)?;
    let full_url = base_url.join(href)?;
    Ok(full_url.to_string())
}

pub async fn send_webmention(
    source: &str,
    target: &str,
    endpoint: &str,
) -> Result<(), Box<dyn Error>> {
    let payload = serde_json::json!({
        "source": source,
        "target": target,
    });

    http_client::post::<serde_json::Value>(endpoint, &payload).await?;
    Ok(())
}

pub async fn verify_webmention(
    source: &str,
    target: &str,
) -> Result<bool, Box<dyn Error>> {
    let html = http_client::get::<String>(source).await?;
    let document = Html::parse_document(&html);
    
    // Check if the source links to the target
    let a_selector = Selector::parse("a").unwrap();
    for element in document.select(&a_selector) {
        if let Some(href) = element.value().attr("href") {
            if href == target {
                return Ok(true);
            }
        }
    }
    
    Ok(false)
}