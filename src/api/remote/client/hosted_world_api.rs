// Generated Code
// DO NOT EDIT MANUALLY

use async_trait::async_trait;
use reqwest::Method;
use super::core::RawApiClient;
use super::super::models::*;
use crate::error::ApiResult;

#[async_trait]
pub trait HostedWorldApi {
    /// Loads privacy settings about who can join a Hosted World
    async fn get_hosted_world_world_privacy_settings(&self, host_uuid: String) -> ApiResult<GetHostedWorldWorldPrivacySettingsResponse>;
    /// Lists active Hosted Worlds
    async fn get_hosted_world_worlds(&self) -> ApiResult<Vec<GetHostedWorldWorldsResponseItem>>;
    /// Requests a world host connect to a relay to accept a connection
    async fn post_hosted_world_request_host_connect(&self, body: &PostHostedWorldRequestHostConnectRequest) -> ApiResult<PostHostedWorldRequestHostConnectResponse>;
}

#[async_trait]
impl HostedWorldApi for RawApiClient {
    async fn get_hosted_world_world_privacy_settings(&self, host_uuid: String) -> ApiResult<GetHostedWorldWorldPrivacySettingsResponse> {
        let url = self.build_url("/hosted-world/world-privacy-settings");
        let mut request = self.client.request(Method::GET, &url);
        let mut query_params = Vec::new();
        query_params.push(("hostUuid".to_string(), host_uuid.to_string()));
        request = request.query(&query_params);
        self.send_request::<GetHostedWorldWorldPrivacySettingsResponse>(request).await
    }
    async fn get_hosted_world_worlds(&self) -> ApiResult<Vec<GetHostedWorldWorldsResponseItem>> {
        let url = self.build_url("/hosted-world/worlds");
        let request = self.client.request(Method::GET, &url);
        
        self.send_request::<Vec<GetHostedWorldWorldsResponseItem>>(request).await
    }
    async fn post_hosted_world_request_host_connect(&self, body: &PostHostedWorldRequestHostConnectRequest) -> ApiResult<PostHostedWorldRequestHostConnectResponse> {
        let url = self.build_url("/hosted-world/request-host-connect");
        let mut request = self.client.request(Method::POST, &url);
        request = request.json(body);
        self.send_request::<PostHostedWorldRequestHostConnectResponse>(request).await
    }
}
