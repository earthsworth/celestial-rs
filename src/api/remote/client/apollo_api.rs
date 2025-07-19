// Generated Code
// DO NOT EDIT MANUALLY

use async_trait::async_trait;
use reqwest::Method;
use super::core::RawApiClient;
use super::super::models::*;
use crate::error::ApiResult;

#[async_trait]
pub trait ApolloApi {
    /// Reports the latest version of Apollo.
    async fn get_apollo_updates(&self) -> ApiResult<GetApolloUpdatesResponse>;
}

#[async_trait]
impl ApolloApi for RawApiClient {
    async fn get_apollo_updates(&self) -> ApiResult<GetApolloUpdatesResponse> {
        let url = self.build_url("/apollo/updates");
        let request = self.client.request(Method::GET, &url);
        
        self.send_request::<GetApolloUpdatesResponse>(request).await
    }
}
