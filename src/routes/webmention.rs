use axum::{
    routing::{post},
    Router,
    Json,
    extract::State,
};
use serde::{Deserialize, Serialize};
use crate::utils::webmention::verify_webmention;
use crate::state::AppState;

#[derive(Debug, Deserialize)]
pub struct WebmentionRequest {
    source: String,
    target: String,
}

#[derive(Debug, Serialize)]
pub struct WebmentionResponse {
    status: String,
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/webmention", post(receive_webmention))
}

async fn receive_webmention(
    State(state): State<AppState>,
    Json(payload): Json<WebmentionRequest>,
) -> Json<WebmentionResponse> {
    // 1. Verify the webmention
    match verify_webmention(&payload.source, &payload.target).await {
        Ok(true) => {
            // 2. Store the verified webmention
            if let Err(e) = state.store.save_webmention(&payload.source, &payload.target).await {
                tracing::error!("Failed to save webmention: {}", e);
                return Json(WebmentionResponse {
                    status: "error".to_string(),
                });
            }
            
            Json(WebmentionResponse {
                status: "accepted".to_string(),
            })
        },
        Ok(false) => Json(WebmentionResponse {
            status: "invalid".to_string(),
        }),
        Err(e) => {
            tracing::error!("Failed to verify webmention: {}", e);
            Json(WebmentionResponse {
                status: "error".to_string(),
            })
        }
    }
}