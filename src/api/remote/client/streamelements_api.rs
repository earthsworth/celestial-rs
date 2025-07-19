// Generated Code
// DO NOT EDIT MANUALLY

use async_trait::async_trait;
use reqwest::Method;
use super::core::RawApiClient;
use super::super::models::*;
use crate::error::ApiResult;

#[async_trait]
pub trait StreamelementsApi {
    /// Callback endpoint for StreamElements OAuth flow that initializes the partner's StreamElements setup
    async fn get_streamelements_callback(&self, code: String, state: String) -> ApiResult<GetStreamelementsCallbackResponse>;
    /// Authorizes a partner with StreamElements by validating their one-time code and redirecting to StreamElements OAuth flow
    async fn get_streamelements_link(&self, one_time_code: String) -> ApiResult<()>;
}

#[async_trait]
impl StreamelementsApi for RawApiClient {
    async fn get_streamelements_callback(&self, code: String, state: String) -> ApiResult<GetStreamelementsCallbackResponse> {
        let url = self.build_url("/streamelements/callback");
        let mut request = self.client.request(Method::GET, &url);
        let mut query_params = Vec::new();
        query_params.push(("code".to_string(), code.to_string()));
        query_params.push(("state".to_string(), state.to_string()));
        request = request.query(&query_params);
        self.send_request::<GetStreamelementsCallbackResponse>(request).await
    }
    async fn get_streamelements_link(&self, one_time_code: String) -> ApiResult<()> {
        let url = self.build_url("/streamelements/link");
        let mut request = self.client.request(Method::GET, &url);
        let mut query_params = Vec::new();
        query_params.push(("one_time_code".to_string(), one_time_code.to_string()));
        request = request.query(&query_params);
        self.send_request::<()>(request).await
    }
}
