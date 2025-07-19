// Generated Code
// DO NOT EDIT MANUALLY

use reqwest::{Client, RequestBuilder, StatusCode};
use serde::de::DeserializeOwned;
use crate::error::{ApiError, ApiResult};

pub struct RawApiClient {
    pub client: Client,
    pub base_url: String,
}

impl RawApiClient {
    pub fn new(base_url: impl Into<String>) -> Self {
        Self {
            client: Client::new(),
            base_url: base_url.into(),
        }
    }

    pub fn with_client(base_url: impl Into<String>, client: Client) -> Self {
        Self {
            client,
            base_url: base_url.into(),
        }
    }

    pub fn build_url(&self, path: &str) -> String {
        format!("{}{}", self.base_url.trim_end_matches('/'), path)
    }

    pub async fn send_request<T: DeserializeOwned + Default + 'static>(&self, request: RequestBuilder) -> ApiResult<T> {
        let response = request.send().await?;
        let status = response.status();

        if status.is_success() {
            if status == StatusCode::NO_CONTENT || std::any::TypeId::of::<T>() == std::any::TypeId::of::<()>() {
                Ok(T::default())
            } else {
                response.json::<T>().await.map_err(ApiError::from)
            }
        } else {
            let message = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            Err(ApiError::Api { status: status.as_u16(), message })
        }
    }
}