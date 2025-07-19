// Generated Code
// DO NOT EDIT MANUALLY

use async_trait::async_trait;
use reqwest::Method;
use super::core::RawApiClient;
use super::super::models::*;
use crate::error::ApiResult;

#[async_trait]
pub trait GameApi {
    /// Loads various metadata used by the game.
    async fn get_game_metadata(&self, branch: Option<String>) -> ApiResult<GetGameMetadataResponse>;
    /// Game screenshot view
    async fn get_game_screenshot_view(&self, id: String, authorization: Option<String>) -> ApiResult<GetGameScreenshotViewResponse>;
    /// Endpoint to retrieve store packages to display in the in-game store
    async fn get_game_store(&self) -> ApiResult<GetGameStoreResponse>;
    /// Loads various jams used by the game.
    async fn get_styngr_jams(&self) -> ApiResult<Vec<GetStyngrJamsResponseItem>>;
    /// Legacy game crash report endpoint
    async fn post_game_crash_report(&self) -> ApiResult<PostGameCrashReportResponse>;
    /// Legacy game logs endpoint
    async fn post_game_logs(&self) -> ApiResult<PostGameLogsResponse>;
    /// Game screenshot upload
    async fn post_game_screenshot_upload(&self, authorization: String, body: &PostGameScreenshotUploadRequest) -> ApiResult<PostGameScreenshotUploadResponse>;
}

#[async_trait]
impl GameApi for RawApiClient {
    async fn get_game_metadata(&self, branch: Option<String>) -> ApiResult<GetGameMetadataResponse> {
        let url = self.build_url("/game/metadata");
        let mut request = self.client.request(Method::GET, &url);
        let mut query_params = Vec::new();
        if let Some(value) = branch { query_params.push(("branch".to_string(), value.to_string())); }
        request = request.query(&query_params);
        self.send_request::<GetGameMetadataResponse>(request).await
    }
    async fn get_game_screenshot_view(&self, id: String, authorization: Option<String>) -> ApiResult<GetGameScreenshotViewResponse> {
        let url = self.build_url("/game/screenshot/view");
        let mut request = self.client.request(Method::GET, &url);
        let mut query_params = Vec::new();
        query_params.push(("id".to_string(), id.to_string()));
        request = request.query(&query_params);
        if let Some(value) = authorization { request = request.header("Authorization", value); }
        self.send_request::<GetGameScreenshotViewResponse>(request).await
    }
    async fn get_game_store(&self) -> ApiResult<GetGameStoreResponse> {
        let url = self.build_url("/game/store");
        let request = self.client.request(Method::GET, &url);
        
        self.send_request::<GetGameStoreResponse>(request).await
    }
    async fn get_styngr_jams(&self) -> ApiResult<Vec<GetStyngrJamsResponseItem>> {
        let url = self.build_url("/styngr/jams");
        let request = self.client.request(Method::GET, &url);
        
        self.send_request::<Vec<GetStyngrJamsResponseItem>>(request).await
    }
    async fn post_game_crash_report(&self) -> ApiResult<PostGameCrashReportResponse> {
        let url = self.build_url("/game/crashReport");
        let request = self.client.request(Method::POST, &url);
        
        self.send_request::<PostGameCrashReportResponse>(request).await
    }
    async fn post_game_logs(&self) -> ApiResult<PostGameLogsResponse> {
        let url = self.build_url("/game/logs");
        let request = self.client.request(Method::POST, &url);
        
        self.send_request::<PostGameLogsResponse>(request).await
    }
    async fn post_game_screenshot_upload(&self, authorization: String, body: &PostGameScreenshotUploadRequest) -> ApiResult<PostGameScreenshotUploadResponse> {
        let url = self.build_url("/game/screenshot/upload");
        let mut request = self.client.request(Method::POST, &url);
        request = request.header("Authorization", authorization);
        request = request.json(body);
        self.send_request::<PostGameScreenshotUploadResponse>(request).await
    }
}
