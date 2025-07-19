// Generated Code
// DO NOT EDIT MANUALLY

use async_trait::async_trait;
use reqwest::Method;
use super::core::RawApiClient;
use super::super::models::*;
use crate::error::ApiResult;

#[async_trait]
pub trait LiveExperienceApi {
    /// Heartbeat for a live experience server
    async fn post_live_experience_heartbeat(&self, body: &PostLiveExperienceHeartbeatRequest) -> ApiResult<PostLiveExperienceHeartbeatResponse>;
    /// Player join for a live experience server
    async fn post_live_experience_login(&self, body: &PostLiveExperienceLoginRequest) -> ApiResult<PostLiveExperienceLoginResponse>;
    /// Record objective completion for a player on a live experience
    async fn post_live_experience_objective_completed(&self, body: &PostLiveExperienceObjectiveCompletedRequest) -> ApiResult<PostLiveExperienceObjectiveCompletedResponse>;
}

#[async_trait]
impl LiveExperienceApi for RawApiClient {
    async fn post_live_experience_heartbeat(&self, body: &PostLiveExperienceHeartbeatRequest) -> ApiResult<PostLiveExperienceHeartbeatResponse> {
        let url = self.build_url("/live-experience/heartbeat");
        let mut request = self.client.request(Method::POST, &url);
        request = request.json(body);
        self.send_request::<PostLiveExperienceHeartbeatResponse>(request).await
    }
    async fn post_live_experience_login(&self, body: &PostLiveExperienceLoginRequest) -> ApiResult<PostLiveExperienceLoginResponse> {
        let url = self.build_url("/live-experience/login");
        let mut request = self.client.request(Method::POST, &url);
        request = request.json(body);
        self.send_request::<PostLiveExperienceLoginResponse>(request).await
    }
    async fn post_live_experience_objective_completed(&self, body: &PostLiveExperienceObjectiveCompletedRequest) -> ApiResult<PostLiveExperienceObjectiveCompletedResponse> {
        let url = self.build_url("/live-experience/objective-completed");
        let mut request = self.client.request(Method::POST, &url);
        request = request.json(body);
        self.send_request::<PostLiveExperienceObjectiveCompletedResponse>(request).await
    }
}
