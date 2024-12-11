use reqwest::Client;
use serde::de::DeserializeOwned;
use std::error::Error;

pub async fn get<T: DeserializeOwned>(url: &str) -> Result<T, Box<dyn Error>> {
    let client = Client::new();
    let response = client.get(url).send().await?;
    Ok(response.json().await?)
}

pub async fn post<T: DeserializeOwned>(url: &str, json: &impl serde::Serialize) -> Result<T, Box<dyn Error>> {
    let client = Client::new();
    let response = client.post(url)
        .header("Content-Type", "application/json")
        .json(json)
        .send()
        .await?;
    Ok(response.json().await?)
}