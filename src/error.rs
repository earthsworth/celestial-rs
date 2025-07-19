#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("Serialization error: {0}")]
    Serde(#[from] serde_json::Error),
    #[error("API error: {status} - {message}")]
    Api { status: u16, message: String },
    #[error("Error in processing: {0}")]
    Processing(String),
}

pub type ApiResult<T> = Result<T, ApiError>;
