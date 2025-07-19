// Generated Code
// DO NOT EDIT MANUALLY

use async_trait::async_trait;
use reqwest::Method;
use super::core::RawApiClient;
use super::super::models::*;
use crate::error::ApiResult;

#[async_trait]
pub trait WebstoreApi {
    /// Lists active CMS pages on the webstore
    async fn get_webstore_cms_pages(&self) -> ApiResult<GetWebstoreCmsPagesResponse>;
    /// Lists active collections on the webstore
    async fn get_webstore_collections(&self) -> ApiResult<GetWebstoreCollectionsResponse>;
    /// Retrieve the current packages which are limited to certain countries
    async fn get_webstore_country_availability(&self) -> ApiResult<Vec<GetWebstoreCountryAvailabilityResponseItem>>;
    /// Lunar Client Store metadata lookup
    async fn get_webstore_metadata(&self) -> ApiResult<GetWebstoreMetadataResponse>;
    /// Retrieve active webstore packages for cosmetics and emotes by ID
    async fn get_webstore_package_by_cosmetic(&self, cosmetic_id: Option<String>, emote_id: Option<String>, exclude_bundles: Option<String>) -> ApiResult<Vec<serde_json::Value>>;
    /// Lists active packages on the webstore
    async fn get_webstore_packages(&self, include_rankings: Option<String>) -> ApiResult<GetWebstorePackagesResponse>;
    /// Lists active partners on the webstore
    async fn get_webstore_partners(&self, include_rankings: Option<String>) -> ApiResult<GetWebstorePartnersResponse>;
    /// Webhook to validate player info for store logins
    async fn get_webstore_player_verification(&self, uuid: String) -> ApiResult<()>;
    /// Recommend other packages a player might be interested in
    async fn get_webstore_recommendations(&self, num_recommendations: String, uuid: String) -> ApiResult<GetWebstoreRecommendationsResponse>;
    /// Generate a free product code
    async fn post_webstore_free_product_code(&self, body: &PostWebstoreFreeProductCodeRequest) -> ApiResult<PostWebstoreFreeProductCodeResponse>;
    /// Webhook to process store events
    async fn post_webstore_webhook(&self) -> ApiResult<()>;
}

#[async_trait]
impl WebstoreApi for RawApiClient {
    async fn get_webstore_cms_pages(&self) -> ApiResult<GetWebstoreCmsPagesResponse> {
        let url = self.build_url("/webstore/cms-pages");
        let request = self.client.request(Method::GET, &url);
        
        self.send_request::<GetWebstoreCmsPagesResponse>(request).await
    }
    async fn get_webstore_collections(&self) -> ApiResult<GetWebstoreCollectionsResponse> {
        let url = self.build_url("/webstore/collections");
        let request = self.client.request(Method::GET, &url);
        
        self.send_request::<GetWebstoreCollectionsResponse>(request).await
    }
    async fn get_webstore_country_availability(&self) -> ApiResult<Vec<GetWebstoreCountryAvailabilityResponseItem>> {
        let url = self.build_url("/webstore/country-availability");
        let request = self.client.request(Method::GET, &url);
        
        self.send_request::<Vec<GetWebstoreCountryAvailabilityResponseItem>>(request).await
    }
    async fn get_webstore_metadata(&self) -> ApiResult<GetWebstoreMetadataResponse> {
        let url = self.build_url("/webstore/metadata");
        let request = self.client.request(Method::GET, &url);
        
        self.send_request::<GetWebstoreMetadataResponse>(request).await
    }
    async fn get_webstore_package_by_cosmetic(&self, cosmetic_id: Option<String>, emote_id: Option<String>, exclude_bundles: Option<String>) -> ApiResult<Vec<serde_json::Value>> {
        let url = self.build_url("/webstore/package-by-cosmetic");
        let mut request = self.client.request(Method::GET, &url);
        let mut query_params = Vec::new();
        if let Some(value) = cosmetic_id { query_params.push(("cosmeticId".to_string(), value.to_string())); }
        if let Some(value) = emote_id { query_params.push(("emoteId".to_string(), value.to_string())); }
        if let Some(value) = exclude_bundles { query_params.push(("excludeBundles".to_string(), value.to_string())); }
        request = request.query(&query_params);
        self.send_request::<Vec<serde_json::Value>>(request).await
    }
    async fn get_webstore_packages(&self, include_rankings: Option<String>) -> ApiResult<GetWebstorePackagesResponse> {
        let url = self.build_url("/webstore/packages");
        let mut request = self.client.request(Method::GET, &url);
        let mut query_params = Vec::new();
        if let Some(value) = include_rankings { query_params.push(("includeRankings".to_string(), value.to_string())); }
        request = request.query(&query_params);
        self.send_request::<GetWebstorePackagesResponse>(request).await
    }
    async fn get_webstore_partners(&self, include_rankings: Option<String>) -> ApiResult<GetWebstorePartnersResponse> {
        let url = self.build_url("/webstore/partners");
        let mut request = self.client.request(Method::GET, &url);
        let mut query_params = Vec::new();
        if let Some(value) = include_rankings { query_params.push(("includeRankings".to_string(), value.to_string())); }
        request = request.query(&query_params);
        self.send_request::<GetWebstorePartnersResponse>(request).await
    }
    async fn get_webstore_player_verification(&self, uuid: String) -> ApiResult<()> {
        let url = self.build_url("/webstore/playerVerification");
        let mut request = self.client.request(Method::GET, &url);
        let mut query_params = Vec::new();
        query_params.push(("uuid".to_string(), uuid.to_string()));
        request = request.query(&query_params);
        self.send_request::<()>(request).await
    }
    async fn get_webstore_recommendations(&self, num_recommendations: String, uuid: String) -> ApiResult<GetWebstoreRecommendationsResponse> {
        let url = self.build_url("/webstore/recommendations");
        let mut request = self.client.request(Method::GET, &url);
        let mut query_params = Vec::new();
        query_params.push(("uuid".to_string(), uuid.to_string()));
        query_params.push(("numRecommendations".to_string(), num_recommendations.to_string()));
        request = request.query(&query_params);
        self.send_request::<GetWebstoreRecommendationsResponse>(request).await
    }
    async fn post_webstore_free_product_code(&self, body: &PostWebstoreFreeProductCodeRequest) -> ApiResult<PostWebstoreFreeProductCodeResponse> {
        let url = self.build_url("/webstore/free-product-code");
        let mut request = self.client.request(Method::POST, &url);
        request = request.json(body);
        self.send_request::<PostWebstoreFreeProductCodeResponse>(request).await
    }
    async fn post_webstore_webhook(&self) -> ApiResult<()> {
        let url = self.build_url("/webstore/webhook");
        let request = self.client.request(Method::POST, &url);
        
        self.send_request::<()>(request).await
    }
}
