// Generated Code
// DO NOT EDIT MANUALLY

use async_trait::async_trait;
use reqwest::Method;
use super::core::RawApiClient;
use super::super::models::*;
use crate::error::ApiResult;

#[async_trait]
pub trait TwitterApi {
    /// Links a Twitter profile
    async fn post_twitter_verification(&self, authorization: String, body: &PostTwitterVerificationRequest) -> ApiResult<()>;
}

#[async_trait]
impl TwitterApi for RawApiClient {
    async fn post_twitter_verification(&self, authorization: String, body: &PostTwitterVerificationRequest) -> ApiResult<()> {
        let url = self.build_url("/twitter/verification");
        let mut request = self.client.request(Method::POST, &url);
        request = request.header("Authorization", authorization);
        request = request.json(body);
        self.send_request::<()>(request).await
    }
}
