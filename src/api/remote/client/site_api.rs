// Generated Code
// DO NOT EDIT MANUALLY

use async_trait::async_trait;
use reqwest::Method;
use super::core::RawApiClient;
use super::super::models::*;
use crate::error::ApiResult;

#[async_trait]
pub trait SiteApi {
    /// Website download redirect
    async fn get_site_download(&self, os: String, arch: Option<String>, bl_opt_in: Option<String>, dclid: Option<String>, fbclid: Option<String>, gad_source: Option<String>, gclid: Option<String>, gclsrc: Option<String>, igshid: Option<String>, include_extra: Option<bool>, li_fat_id: Option<String>, msclkid: Option<String>, rdt_cid: Option<String>, ttclid: Option<String>, twclid: Option<String>, utm_campaign: Option<String>, utm_content: Option<String>, utm_medium: Option<String>, utm_source: Option<String>, utm_term: Option<String>, wbraid: Option<String>) -> ApiResult<()>;
    /// Website metadata lookup
    async fn get_site_metadata(&self) -> ApiResult<GetSiteMetadataResponse>;
    /// Package lookup
    async fn get_site_packages(&self, package: String, include_global_extras: Option<String>, include_particles: Option<String>) -> ApiResult<GetSitePackagesResponse>;
    /// Get specific server for the website
    async fn get_site_server_identifier_or_address(&self, identifier_or_address: String) -> ApiResult<()>;
    /// Lunar Client status
    async fn get_site_status(&self) -> ApiResult<GetSiteStatusResponse>;
    /// Package lookup
    async fn get_webstore_cosmetics_by_package(&self, package_id: String) -> ApiResult<GetWebstoreCosmeticsByPackageResponse>;
    /// Records uninstall feedback
    async fn post_site_uninstall(&self, body: &PostSiteUninstallRequest) -> ApiResult<PostSiteUninstallResponse>;
}

#[async_trait]
impl SiteApi for RawApiClient {
    async fn get_site_download(&self, os: String, arch: Option<String>, bl_opt_in: Option<String>, dclid: Option<String>, fbclid: Option<String>, gad_source: Option<String>, gclid: Option<String>, gclsrc: Option<String>, igshid: Option<String>, include_extra: Option<bool>, li_fat_id: Option<String>, msclkid: Option<String>, rdt_cid: Option<String>, ttclid: Option<String>, twclid: Option<String>, utm_campaign: Option<String>, utm_content: Option<String>, utm_medium: Option<String>, utm_source: Option<String>, utm_term: Option<String>, wbraid: Option<String>) -> ApiResult<()> {
        let url = self.build_url("/site/download");
        let mut request = self.client.request(Method::GET, &url);
        let mut query_params = Vec::new();
        query_params.push(("os".to_string(), os.to_string()));
        if let Some(value) = arch { query_params.push(("arch".to_string(), value.to_string())); }
        if let Some(value) = utm_source { query_params.push(("utm_source".to_string(), value.to_string())); }
        if let Some(value) = utm_medium { query_params.push(("utm_medium".to_string(), value.to_string())); }
        if let Some(value) = utm_campaign { query_params.push(("utm_campaign".to_string(), value.to_string())); }
        if let Some(value) = include_extra { query_params.push(("include_extra".to_string(), value.to_string())); }
        if let Some(value) = utm_term { query_params.push(("utm_term".to_string(), value.to_string())); }
        if let Some(value) = utm_content { query_params.push(("utm_content".to_string(), value.to_string())); }
        if let Some(value) = gclid { query_params.push(("gclid".to_string(), value.to_string())); }
        if let Some(value) = gad_source { query_params.push(("gad_source".to_string(), value.to_string())); }
        if let Some(value) = gclsrc { query_params.push(("gclsrc".to_string(), value.to_string())); }
        if let Some(value) = dclid { query_params.push(("dclid".to_string(), value.to_string())); }
        if let Some(value) = wbraid { query_params.push(("wbraid".to_string(), value.to_string())); }
        if let Some(value) = twclid { query_params.push(("twclid".to_string(), value.to_string())); }
        if let Some(value) = li_fat_id { query_params.push(("li_fat_id".to_string(), value.to_string())); }
        if let Some(value) = igshid { query_params.push(("igshid".to_string(), value.to_string())); }
        if let Some(value) = msclkid { query_params.push(("msclkid".to_string(), value.to_string())); }
        if let Some(value) = fbclid { query_params.push(("fbclid".to_string(), value.to_string())); }
        if let Some(value) = ttclid { query_params.push(("ttclid".to_string(), value.to_string())); }
        if let Some(value) = rdt_cid { query_params.push(("rdt_cid".to_string(), value.to_string())); }
        if let Some(value) = bl_opt_in { query_params.push(("bl_opt_in".to_string(), value.to_string())); }
        request = request.query(&query_params);
        self.send_request::<()>(request).await
    }
    async fn get_site_metadata(&self) -> ApiResult<GetSiteMetadataResponse> {
        let url = self.build_url("/site/metadata");
        let request = self.client.request(Method::GET, &url);
        
        self.send_request::<GetSiteMetadataResponse>(request).await
    }
    async fn get_site_packages(&self, package: String, include_global_extras: Option<String>, include_particles: Option<String>) -> ApiResult<GetSitePackagesResponse> {
        let url = self.build_url("/site/packages");
        let mut request = self.client.request(Method::GET, &url);
        let mut query_params = Vec::new();
        query_params.push(("package".to_string(), package.to_string()));
        if let Some(value) = include_global_extras { query_params.push(("includeGlobalExtras".to_string(), value.to_string())); }
        if let Some(value) = include_particles { query_params.push(("includeParticles".to_string(), value.to_string())); }
        request = request.query(&query_params);
        self.send_request::<GetSitePackagesResponse>(request).await
    }
    async fn get_site_server_identifier_or_address(&self, identifier_or_address: String) -> ApiResult<()> {
        let url = self.build_url(&format!("/site/server/{}", &identifier_or_address.to_string()));
        let request = self.client.request(Method::GET, &url);
        
        self.send_request::<()>(request).await
    }
    async fn get_site_status(&self) -> ApiResult<GetSiteStatusResponse> {
        let url = self.build_url("/site/status");
        let request = self.client.request(Method::GET, &url);
        
        self.send_request::<GetSiteStatusResponse>(request).await
    }
    async fn get_webstore_cosmetics_by_package(&self, package_id: String) -> ApiResult<GetWebstoreCosmeticsByPackageResponse> {
        let url = self.build_url("/webstore/cosmetics-by-package");
        let mut request = self.client.request(Method::GET, &url);
        let mut query_params = Vec::new();
        query_params.push(("packageId".to_string(), package_id.to_string()));
        request = request.query(&query_params);
        self.send_request::<GetWebstoreCosmeticsByPackageResponse>(request).await
    }
    async fn post_site_uninstall(&self, body: &PostSiteUninstallRequest) -> ApiResult<PostSiteUninstallResponse> {
        let url = self.build_url("/site/uninstall");
        let mut request = self.client.request(Method::POST, &url);
        request = request.json(body);
        self.send_request::<PostSiteUninstallResponse>(request).await
    }
}
