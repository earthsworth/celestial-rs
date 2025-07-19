// Generated Code
// DO NOT EDIT MANUALLY

use async_trait::async_trait;
use reqwest::Method;
use super::core::RawApiClient;
use super::super::models::*;
use crate::error::ApiResult;

#[async_trait]
pub trait DeployApi {
    /// Deploys a new game release
    async fn post_deploy_game(&self, body: &PostDeployGameRequest) -> ApiResult<PostDeployGameResponse>;
    /// Deploys a new game data release
    async fn post_deploy_gamedata(&self, body: &PostDeployGamedataRequest) -> ApiResult<PostDeployGamedataResponse>;
    /// Deploys a new license release
    async fn post_deploy_licenses(&self, body: &PostDeployLicensesRequest) -> ApiResult<PostDeployLicensesResponse>;
    /// Deploys a new modpack release
    async fn post_deploy_modpacks(&self, body: &PostDeployModpacksRequest) -> ApiResult<PostDeployModpacksResponse>;
    /// Deploys a new textures release
    async fn post_deploy_textures(&self, body: &PostDeployTexturesRequest) -> ApiResult<PostDeployTexturesResponse>;
    /// Deploys a new UI release
    async fn post_deploy_ui(&self, body: &PostDeployUiRequest) -> ApiResult<PostDeployUiResponse>;
}

#[async_trait]
impl DeployApi for RawApiClient {
    async fn post_deploy_game(&self, body: &PostDeployGameRequest) -> ApiResult<PostDeployGameResponse> {
        let url = self.build_url("/deploy/game");
        let mut request = self.client.request(Method::POST, &url);
        request = request.json(body);
        self.send_request::<PostDeployGameResponse>(request).await
    }
    async fn post_deploy_gamedata(&self, body: &PostDeployGamedataRequest) -> ApiResult<PostDeployGamedataResponse> {
        let url = self.build_url("/deploy/gamedata");
        let mut request = self.client.request(Method::POST, &url);
        request = request.json(body);
        self.send_request::<PostDeployGamedataResponse>(request).await
    }
    async fn post_deploy_licenses(&self, body: &PostDeployLicensesRequest) -> ApiResult<PostDeployLicensesResponse> {
        let url = self.build_url("/deploy/licenses");
        let mut request = self.client.request(Method::POST, &url);
        request = request.json(body);
        self.send_request::<PostDeployLicensesResponse>(request).await
    }
    async fn post_deploy_modpacks(&self, body: &PostDeployModpacksRequest) -> ApiResult<PostDeployModpacksResponse> {
        let url = self.build_url("/deploy/modpacks");
        let mut request = self.client.request(Method::POST, &url);
        request = request.json(body);
        self.send_request::<PostDeployModpacksResponse>(request).await
    }
    async fn post_deploy_textures(&self, body: &PostDeployTexturesRequest) -> ApiResult<PostDeployTexturesResponse> {
        let url = self.build_url("/deploy/textures");
        let mut request = self.client.request(Method::POST, &url);
        request = request.json(body);
        self.send_request::<PostDeployTexturesResponse>(request).await
    }
    async fn post_deploy_ui(&self, body: &PostDeployUiRequest) -> ApiResult<PostDeployUiResponse> {
        let url = self.build_url("/deploy/ui");
        let mut request = self.client.request(Method::POST, &url);
        request = request.json(body);
        self.send_request::<PostDeployUiResponse>(request).await
    }
}
