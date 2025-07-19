// Generated Code
// DO NOT EDIT MANUALLY

use async_trait::async_trait;
use reqwest::Method;
use super::core::RawApiClient;
use crate::error::ApiResult;

#[async_trait]
pub trait OgApi {
    /// Generate a Jam cover image
    async fn get_og_jam_package_id(&self, package_id: i64) -> ApiResult<()>;
}

#[async_trait]
impl OgApi for RawApiClient {
    async fn get_og_jam_package_id(&self, package_id: i64) -> ApiResult<()> {
        let url = self.build_url(&format!("/og/jam/{}", &package_id.to_string()));
        let request = self.client.request(Method::GET, &url);
        
        self.send_request::<()>(request).await
    }
}
