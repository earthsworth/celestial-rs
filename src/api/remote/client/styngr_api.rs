// Generated Code
// DO NOT EDIT MANUALLY

use async_trait::async_trait;
use reqwest::Method;
use super::core::RawApiClient;
use super::super::models::*;
use crate::error::ApiResult;

#[async_trait]
pub trait StyngrApi {
    /// Generates Styngr JWT for the player to use with the Styngr API.
    async fn post_styngr_jwt(&self, authorization: String, x_installation_id: Option<String>, body: &PostStyngrJwtRequest) -> ApiResult<PostStyngrJwtResponse>;
}

#[async_trait]
impl StyngrApi for RawApiClient {
    async fn post_styngr_jwt(&self, authorization: String, x_installation_id: Option<String>, body: &PostStyngrJwtRequest) -> ApiResult<PostStyngrJwtResponse> {
        let url = self.build_url("/styngr/jwt");
        let mut request = self.client.request(Method::POST, &url);
        if let Some(value) = x_installation_id { request = request.header("x-installation-id", value); }
        request = request.header("Authorization", authorization);
        request = request.json(body);
        self.send_request::<PostStyngrJwtResponse>(request).await
    }
}
