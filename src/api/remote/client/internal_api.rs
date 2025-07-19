// Generated Code
// DO NOT EDIT MANUALLY

use async_trait::async_trait;
use reqwest::Method;
use super::core::RawApiClient;
use super::super::models::*;
use crate::error::ApiResult;

#[async_trait]
pub trait InternalApi {
    /// User profile lookup
    async fn get_user_profile(&self, uuid: String) -> ApiResult<GetUserProfileResponse>;
}

#[async_trait]
impl InternalApi for RawApiClient {
    async fn get_user_profile(&self, uuid: String) -> ApiResult<GetUserProfileResponse> {
        let url = self.build_url("/user/profile");
        let mut request = self.client.request(Method::GET, &url);
        let mut query_params = Vec::new();
        query_params.push(("uuid".to_string(), uuid.to_string()));
        request = request.query(&query_params);
        self.send_request::<GetUserProfileResponse>(request).await
    }
}
