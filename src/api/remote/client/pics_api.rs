// Generated Code
// DO NOT EDIT MANUALLY

use async_trait::async_trait;
use reqwest::Method;
use super::core::RawApiClient;
use super::super::models::*;
use crate::error::ApiResult;

#[async_trait]
pub trait PicsApi {
    /// List screenshots for the authenticated user
    async fn get_pics_list(&self, authorization: String, page: Option<String>, page_size: Option<String>) -> ApiResult<GetPicsListResponse>;
    /// Info about the current user
    async fn get_pics_user(&self, authorization: String) -> ApiResult<GetPicsUserResponse>;
    /// Update privacy settings for a screenshot
    async fn patch_pics_privacy(&self, authorization: String, body: &PatchPicsPrivacyRequest) -> ApiResult<PatchPicsPrivacyResponse>;
    /// Rename a screenshot
    async fn patch_pics_rename(&self, authorization: String, body: &PatchPicsRenameRequest) -> ApiResult<PatchPicsRenameResponse>;
    /// Report a screenshot for inappropriate content or policy violations
    async fn post_pics_id_report(&self, id: String, turnstile__token: String, authorization: Option<String>, body: &PostPicsIdReportRequest) -> ApiResult<PostPicsIdReportResponse>;
}

#[async_trait]
impl PicsApi for RawApiClient {
    async fn get_pics_list(&self, authorization: String, page: Option<String>, page_size: Option<String>) -> ApiResult<GetPicsListResponse> {
        let url = self.build_url("/pics/list");
        let mut request = self.client.request(Method::GET, &url);
        let mut query_params = Vec::new();
        if let Some(value) = page { query_params.push(("page".to_string(), value.to_string())); }
        if let Some(value) = page_size { query_params.push(("pageSize".to_string(), value.to_string())); }
        request = request.query(&query_params);
        request = request.header("Authorization", authorization);
        self.send_request::<GetPicsListResponse>(request).await
    }
    async fn get_pics_user(&self, authorization: String) -> ApiResult<GetPicsUserResponse> {
        let url = self.build_url("/pics/user");
        let mut request = self.client.request(Method::GET, &url);
        request = request.header("Authorization", authorization);
        self.send_request::<GetPicsUserResponse>(request).await
    }
    async fn patch_pics_privacy(&self, authorization: String, body: &PatchPicsPrivacyRequest) -> ApiResult<PatchPicsPrivacyResponse> {
        let url = self.build_url("/pics/privacy");
        let mut request = self.client.request(Method::PATCH, &url);
        request = request.header("Authorization", authorization);
        request = request.json(body);
        self.send_request::<PatchPicsPrivacyResponse>(request).await
    }
    async fn patch_pics_rename(&self, authorization: String, body: &PatchPicsRenameRequest) -> ApiResult<PatchPicsRenameResponse> {
        let url = self.build_url("/pics/rename");
        let mut request = self.client.request(Method::PATCH, &url);
        request = request.header("Authorization", authorization);
        request = request.json(body);
        self.send_request::<PatchPicsRenameResponse>(request).await
    }
    async fn post_pics_id_report(&self, id: String, turnstile__token: String, authorization: Option<String>, body: &PostPicsIdReportRequest) -> ApiResult<PostPicsIdReportResponse> {
        let url = self.build_url(&format!("/pics/{}/report", &id.to_string()));
        let mut request = self.client.request(Method::POST, &url);
        if let Some(value) = authorization { request = request.header("Authorization", value); }
        request = request.header("Turnstile-Token", turnstile__token);
        request = request.json(body);
        self.send_request::<PostPicsIdReportResponse>(request).await
    }
}
