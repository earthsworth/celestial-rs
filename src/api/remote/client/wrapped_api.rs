// Generated Code
// DO NOT EDIT MANUALLY

use async_trait::async_trait;
use reqwest::Method;
use super::core::RawApiClient;
use super::super::models::*;
use crate::error::ApiResult;

#[async_trait]
pub trait WrappedApi {
    /// Retrieves Wrapped country data
    async fn get_wrapped_country(&self, country: String, year: Option<i64>) -> ApiResult<WrappedCountryData>;
    /// Retrieves Wrapped global data
    async fn get_wrapped_global(&self, year: Option<i64>) -> ApiResult<WrappedGlobalData>;
    /// Retrieves Wrapped player data
    async fn get_wrapped_player(&self, authorization: String, assume_uuid: Option<String>, year: Option<i64>) -> ApiResult<WrappedPlayerData>;
    /// Retrieves Wrapped server data
    async fn get_wrapped_server(&self, server_mappings_id: String, year: Option<i64>) -> ApiResult<WrappedServerData>;
}

#[async_trait]
impl WrappedApi for RawApiClient {
    async fn get_wrapped_country(&self, country: String, year: Option<i64>) -> ApiResult<WrappedCountryData> {
        let url = self.build_url("/wrapped/country");
        let mut request = self.client.request(Method::GET, &url);
        let mut query_params = Vec::new();
        if let Some(value) = year { query_params.push(("year".to_string(), value.to_string())); }
        query_params.push(("country".to_string(), country.to_string()));
        request = request.query(&query_params);
        self.send_request::<WrappedCountryData>(request).await
    }
    async fn get_wrapped_global(&self, year: Option<i64>) -> ApiResult<WrappedGlobalData> {
        let url = self.build_url("/wrapped/global");
        let mut request = self.client.request(Method::GET, &url);
        let mut query_params = Vec::new();
        if let Some(value) = year { query_params.push(("year".to_string(), value.to_string())); }
        request = request.query(&query_params);
        self.send_request::<WrappedGlobalData>(request).await
    }
    async fn get_wrapped_player(&self, authorization: String, assume_uuid: Option<String>, year: Option<i64>) -> ApiResult<WrappedPlayerData> {
        let url = self.build_url("/wrapped/player");
        let mut request = self.client.request(Method::GET, &url);
        let mut query_params = Vec::new();
        if let Some(value) = year { query_params.push(("year".to_string(), value.to_string())); }
        if let Some(value) = assume_uuid { query_params.push(("assumeUuid".to_string(), value.to_string())); }
        request = request.query(&query_params);
        request = request.header("Authorization", authorization);
        self.send_request::<WrappedPlayerData>(request).await
    }
    async fn get_wrapped_server(&self, server_mappings_id: String, year: Option<i64>) -> ApiResult<WrappedServerData> {
        let url = self.build_url("/wrapped/server");
        let mut request = self.client.request(Method::GET, &url);
        let mut query_params = Vec::new();
        if let Some(value) = year { query_params.push(("year".to_string(), value.to_string())); }
        query_params.push(("serverMappingsId".to_string(), server_mappings_id.to_string()));
        request = request.query(&query_params);
        self.send_request::<WrappedServerData>(request).await
    }
}
