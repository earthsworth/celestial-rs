// Generated Code
// DO NOT EDIT MANUALLY

use async_trait::async_trait;
use reqwest::Method;
use super::core::RawApiClient;
use crate::error::ApiResult;

#[async_trait]
pub trait QuiltApi {
    /// Get the quilt loader versions compatible with the specified game version
    async fn get_quilt_v2_versions_loader_game_version(&self, arch: String, game_version: String, installation_id: String, launcher_version: String, os: String, os_release: String, hwid: Option<String>, hwid_private: Option<String>) -> ApiResult<Vec<String>>;
}

#[async_trait]
impl QuiltApi for RawApiClient {
    async fn get_quilt_v2_versions_loader_game_version(&self, arch: String, game_version: String, installation_id: String, launcher_version: String, os: String, os_release: String, hwid: Option<String>, hwid_private: Option<String>) -> ApiResult<Vec<String>> {
        let url = self.build_url(&format!("/quilt/v2/versions/loader/{}", &game_version.to_string()));
        let mut request = self.client.request(Method::GET, &url);
        let mut query_params = Vec::new();
        if let Some(value) = hwid { query_params.push(("hwid".to_string(), value.to_string())); }
        if let Some(value) = hwid_private { query_params.push(("hwid_private".to_string(), value.to_string())); }
        query_params.push(("installation_id".to_string(), installation_id.to_string()));
        query_params.push(("os".to_string(), os.to_string()));
        query_params.push(("os_release".to_string(), os_release.to_string()));
        query_params.push(("arch".to_string(), arch.to_string()));
        query_params.push(("launcher_version".to_string(), launcher_version.to_string()));
        request = request.query(&query_params);
        self.send_request::<Vec<String>>(request).await
    }
}
