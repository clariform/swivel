use thiserror::Error;

#[derive(Debug, Error)]
pub enum NotionError {
    #[error("NOTION_API_KEY is not set")]
    MissingApiKey,

    #[error("http request failed: {0}")]
    Request(#[from] reqwest::Error),

    #[error("json error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("notion request failed with status {status}: {body}")]
    HttpStatus {
        status: reqwest::StatusCode,
        body: String,
    },
}
