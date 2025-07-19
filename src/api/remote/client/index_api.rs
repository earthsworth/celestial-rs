// Generated Code
// DO NOT EDIT MANUALLY

use async_trait::async_trait;
use reqwest::Method;
use super::core::RawApiClient;
use super::super::models::*;
use crate::error::ApiResult;

#[async_trait]
pub trait IndexApi {
    /// Index of all cosmetics.
    async fn get_index_cosmetics(&self, branch: Option<String>) -> ApiResult<Vec<GetIndexCosmeticsResponseItem>>;
    /// Index of all emotes.
    async fn get_index_emotes(&self, branch: Option<String>) -> ApiResult<Vec<GetIndexEmotesResponseItem>>;
    /// Index of all jams.
    async fn get_index_jams(&self) -> ApiResult<Vec<GetIndexJamsResponseItem>>;
    /// Index of all webstore packages.
    async fn get_index_packages(&self) -> ApiResult<Vec<GetIndexPackagesResponseItem>>;
    /// Index of all particles.
    async fn get_index_particles(&self) -> ApiResult<GetIndexParticlesResponse>;
    /// Index of all sprays.
    async fn get_index_sprays(&self, branch: Option<String>) -> ApiResult<Vec<GetIndexSpraysResponseItem>>;
}

#[async_trait]
impl IndexApi for RawApiClient {
    async fn get_index_cosmetics(&self, branch: Option<String>) -> ApiResult<Vec<GetIndexCosmeticsResponseItem>> {
        let url = self.build_url("/index/cosmetics");
        let mut request = self.client.request(Method::GET, &url);
        let mut query_params = Vec::new();
        if let Some(value) = branch { query_params.push(("branch".to_string(), value.to_string())); }
        request = request.query(&query_params);
        self.send_request::<Vec<GetIndexCosmeticsResponseItem>>(request).await
    }
    async fn get_index_emotes(&self, branch: Option<String>) -> ApiResult<Vec<GetIndexEmotesResponseItem>> {
        let url = self.build_url("/index/emotes");
        let mut request = self.client.request(Method::GET, &url);
        let mut query_params = Vec::new();
        if let Some(value) = branch { query_params.push(("branch".to_string(), value.to_string())); }
        request = request.query(&query_params);
        self.send_request::<Vec<GetIndexEmotesResponseItem>>(request).await
    }
    async fn get_index_jams(&self) -> ApiResult<Vec<GetIndexJamsResponseItem>> {
        let url = self.build_url("/index/jams");
        let request = self.client.request(Method::GET, &url);
        
        self.send_request::<Vec<GetIndexJamsResponseItem>>(request).await
    }
    async fn get_index_packages(&self) -> ApiResult<Vec<GetIndexPackagesResponseItem>> {
        let url = self.build_url("/index/packages");
        let request = self.client.request(Method::GET, &url);
        
        self.send_request::<Vec<GetIndexPackagesResponseItem>>(request).await
    }
    async fn get_index_particles(&self) -> ApiResult<GetIndexParticlesResponse> {
        let url = self.build_url("/index/particles");
        let request = self.client.request(Method::GET, &url);
        
        self.send_request::<GetIndexParticlesResponse>(request).await
    }
    async fn get_index_sprays(&self, branch: Option<String>) -> ApiResult<Vec<GetIndexSpraysResponseItem>> {
        let url = self.build_url("/index/sprays");
        let mut request = self.client.request(Method::GET, &url);
        let mut query_params = Vec::new();
        if let Some(value) = branch { query_params.push(("branch".to_string(), value.to_string())); }
        request = request.query(&query_params);
        self.send_request::<Vec<GetIndexSpraysResponseItem>>(request).await
    }
}
