// Generated Code
// DO NOT EDIT MANUALLY

use async_trait::async_trait;
use reqwest::Method;
use super::core::RawApiClient;
use super::super::models::*;
use crate::error::ApiResult;

#[async_trait]
pub trait LauncherApi {
    /// Loads various metadata used by the Lunar Client launcher.
    async fn get_launcher_metadata(&self, arch: String, launcher_version: String, os: String, blog_amount: Option<i64>, branch: Option<String>, branch_change: Option<String>, branch_changed: Option<String>, canary_preference: Option<String>, hwid: Option<String>, hwid_private: Option<String>, installation_id: Option<String>, language: Option<String>, launcher_update_stream: Option<String>, os_release: Option<String>, private: Option<bool>) -> ApiResult<GetLauncherMetadataResponse>;
    /// Loads layout metadata used by the Lunar Client launcher.
    async fn get_launcher_metadata_layout(&self) -> ApiResult<GetLauncherMetadataLayoutResponse>;
    /// Loads miscellaneous metadata used by the Lunar Client launcher.
    async fn get_launcher_metadata_misc(&self, installation_id: String, language: Option<String>, launcher_update_stream: Option<String>) -> ApiResult<GetLauncherMetadataMiscResponse>;
    /// Loads information about the radio in the launcher
    async fn get_launcher_metadata_radio(&self) -> ApiResult<GetLauncherMetadataRadioResponse>;
    /// Loads information about Badlion versions playable via Lunar Client
    async fn get_launcher_metadata_versions_badlion(&self) -> ApiResult<GetLauncherMetadataVersionsBadlionResponse>;
    /// Loads Lunar versions available in the Lunar Client launcher.
    async fn get_launcher_metadata_versions_lunar(&self, installation_id: String, branch: Option<String>) -> ApiResult<GetLauncherMetadataVersionsLunarResponse>;
    /// Loads information about vanilla versions playable via Lunar Client
    async fn get_launcher_metadata_versions_vanilla(&self, launcher_version: String) -> ApiResult<GetLauncherMetadataVersionsVanillaResponse>;
    /// Get specific server from discovery within Launcher
    async fn get_launcher_server_identifier_or_address(&self, arch: String, identifier_or_address: String, installation_id: String, launcher_version: String, os: String, os_release: String, hwid: Option<String>, hwid_private: Option<String>, language: Option<String>) -> ApiResult<LauncherServer>;
    /// Server discovery within Launcher
    async fn get_launcher_servers(&self, arch: String, installation_id: String, launcher_version: String, os: String, os_release: String, community: Option<String>, favorites: Option<Vec<String>>, game_type: Option<String>, hwid: Option<String>, hwid_private: Option<String>, language: Option<String>, minecraft_version: Option<String>, region: Option<String>, search: Option<String>, show_filters: Option<String>, sort: Option<String>) -> ApiResult<GetLauncherServersResponse>;
    /// Basic connection status endpoint
    async fn get_launcher_status(&self, arch: String, installation_id: String, launcher_version: String, os: String, os_release: String, hwid: Option<String>, hwid_private: Option<String>) -> ApiResult<GetLauncherStatusResponse>;
    /// Launch into the game
    async fn post_launcher_launch(&self, body: &PostLauncherLaunchRequest) -> ApiResult<PostLauncherLaunchResponse>;
    /// Launch into a vanilla version.
    async fn post_launcher_launch_vanilla(&self, body: &PostLauncherLaunchVanillaRequest) -> ApiResult<PostLauncherLaunchVanillaResponse>;
    /// Completes an in-progress log upload
    async fn post_launcher_log_upload_complete(&self, body: &PostLauncherLogUploadCompleteRequest) -> ApiResult<PostLauncherLogUploadCompleteResponse>;
    /// Starts a new log upload
    async fn post_launcher_log_upload_start(&self, body: &PostLauncherLogUploadStartRequest) -> ApiResult<PostLauncherLogUploadStartResponse>;
    /// Check if an image is allowed based on the Content Moderation (including NSFW) filter
    async fn post_launcher_moderation_nsfw(&self) -> ApiResult<PostLauncherModerationNsfwResponse>;
    /// Check if a text is allowed based on the profanity filter
    async fn post_launcher_moderation_profanity(&self, body: &PostLauncherModerationProfanityRequest) -> ApiResult<PostLauncherModerationProfanityResponse>;
}

#[async_trait]
impl LauncherApi for RawApiClient {
    async fn get_launcher_metadata(&self, arch: String, launcher_version: String, os: String, blog_amount: Option<i64>, branch: Option<String>, branch_change: Option<String>, branch_changed: Option<String>, canary_preference: Option<String>, hwid: Option<String>, hwid_private: Option<String>, installation_id: Option<String>, language: Option<String>, launcher_update_stream: Option<String>, os_release: Option<String>, private: Option<bool>) -> ApiResult<GetLauncherMetadataResponse> {
        let url = self.build_url("/launcher/metadata");
        let mut request = self.client.request(Method::GET, &url);
        let mut query_params = Vec::new();
        if let Some(value) = hwid { query_params.push(("hwid".to_string(), value.to_string())); }
        if let Some(value) = hwid_private { query_params.push(("hwid_private".to_string(), value.to_string())); }
        if let Some(value) = installation_id { query_params.push(("installation_id".to_string(), value.to_string())); }
        query_params.push(("os".to_string(), os.to_string()));
        if let Some(value) = os_release { query_params.push(("os_release".to_string(), value.to_string())); }
        query_params.push(("arch".to_string(), arch.to_string()));
        query_params.push(("launcher_version".to_string(), launcher_version.to_string()));
        if let Some(value) = canary_preference { query_params.push(("canary_preference".to_string(), value.to_string())); }
        if let Some(value) = branch { query_params.push(("branch".to_string(), value.to_string())); }
        if let Some(value) = language { query_params.push(("language".to_string(), value.to_string())); }
        if let Some(value) = launcher_update_stream { query_params.push(("launcher_update_stream".to_string(), value.to_string())); }
        if let Some(value) = blog_amount { query_params.push(("blog_amount".to_string(), value.to_string())); }
        if let Some(value) = private { query_params.push(("private".to_string(), value.to_string())); }
        if let Some(value) = branch_change { query_params.push(("branch_change".to_string(), value.to_string())); }
        if let Some(value) = branch_changed { query_params.push(("branch_changed".to_string(), value.to_string())); }
        request = request.query(&query_params);
        self.send_request::<GetLauncherMetadataResponse>(request).await
    }
    async fn get_launcher_metadata_layout(&self) -> ApiResult<GetLauncherMetadataLayoutResponse> {
        let url = self.build_url("/launcher/metadata/layout");
        let request = self.client.request(Method::GET, &url);
        
        self.send_request::<GetLauncherMetadataLayoutResponse>(request).await
    }
    async fn get_launcher_metadata_misc(&self, installation_id: String, language: Option<String>, launcher_update_stream: Option<String>) -> ApiResult<GetLauncherMetadataMiscResponse> {
        let url = self.build_url("/launcher/metadata/misc");
        let mut request = self.client.request(Method::GET, &url);
        let mut query_params = Vec::new();
        query_params.push(("installation_id".to_string(), installation_id.to_string()));
        if let Some(value) = language { query_params.push(("language".to_string(), value.to_string())); }
        if let Some(value) = launcher_update_stream { query_params.push(("launcher_update_stream".to_string(), value.to_string())); }
        request = request.query(&query_params);
        self.send_request::<GetLauncherMetadataMiscResponse>(request).await
    }
    async fn get_launcher_metadata_radio(&self) -> ApiResult<GetLauncherMetadataRadioResponse> {
        let url = self.build_url("/launcher/metadata/radio");
        let request = self.client.request(Method::GET, &url);
        
        self.send_request::<GetLauncherMetadataRadioResponse>(request).await
    }
    async fn get_launcher_metadata_versions_badlion(&self) -> ApiResult<GetLauncherMetadataVersionsBadlionResponse> {
        let url = self.build_url("/launcher/metadata/versions/badlion");
        let request = self.client.request(Method::GET, &url);
        
        self.send_request::<GetLauncherMetadataVersionsBadlionResponse>(request).await
    }
    async fn get_launcher_metadata_versions_lunar(&self, installation_id: String, branch: Option<String>) -> ApiResult<GetLauncherMetadataVersionsLunarResponse> {
        let url = self.build_url("/launcher/metadata/versions/lunar");
        let mut request = self.client.request(Method::GET, &url);
        let mut query_params = Vec::new();
        query_params.push(("installation_id".to_string(), installation_id.to_string()));
        if let Some(value) = branch { query_params.push(("branch".to_string(), value.to_string())); }
        request = request.query(&query_params);
        self.send_request::<GetLauncherMetadataVersionsLunarResponse>(request).await
    }
    async fn get_launcher_metadata_versions_vanilla(&self, launcher_version: String) -> ApiResult<GetLauncherMetadataVersionsVanillaResponse> {
        let url = self.build_url("/launcher/metadata/versions/vanilla");
        let mut request = self.client.request(Method::GET, &url);
        let mut query_params = Vec::new();
        query_params.push(("launcher_version".to_string(), launcher_version.to_string()));
        request = request.query(&query_params);
        self.send_request::<GetLauncherMetadataVersionsVanillaResponse>(request).await
    }
    async fn get_launcher_server_identifier_or_address(&self, arch: String, identifier_or_address: String, installation_id: String, launcher_version: String, os: String, os_release: String, hwid: Option<String>, hwid_private: Option<String>, language: Option<String>) -> ApiResult<LauncherServer> {
        let url = self.build_url(&format!("/launcher/server/{}", &identifier_or_address.to_string()));
        let mut request = self.client.request(Method::GET, &url);
        let mut query_params = Vec::new();
        if let Some(value) = hwid { query_params.push(("hwid".to_string(), value.to_string())); }
        if let Some(value) = hwid_private { query_params.push(("hwid_private".to_string(), value.to_string())); }
        query_params.push(("installation_id".to_string(), installation_id.to_string()));
        query_params.push(("os".to_string(), os.to_string()));
        query_params.push(("os_release".to_string(), os_release.to_string()));
        query_params.push(("arch".to_string(), arch.to_string()));
        query_params.push(("launcher_version".to_string(), launcher_version.to_string()));
        if let Some(value) = language { query_params.push(("language".to_string(), value.to_string())); }
        request = request.query(&query_params);
        self.send_request::<LauncherServer>(request).await
    }
    async fn get_launcher_servers(&self, arch: String, installation_id: String, launcher_version: String, os: String, os_release: String, community: Option<String>, favorites: Option<Vec<String>>, game_type: Option<String>, hwid: Option<String>, hwid_private: Option<String>, language: Option<String>, minecraft_version: Option<String>, region: Option<String>, search: Option<String>, show_filters: Option<String>, sort: Option<String>) -> ApiResult<GetLauncherServersResponse> {
        let url = self.build_url("/launcher/servers");
        let mut request = self.client.request(Method::GET, &url);
        let mut query_params = Vec::new();
        if let Some(value) = hwid { query_params.push(("hwid".to_string(), value.to_string())); }
        if let Some(value) = hwid_private { query_params.push(("hwid_private".to_string(), value.to_string())); }
        query_params.push(("installation_id".to_string(), installation_id.to_string()));
        query_params.push(("os".to_string(), os.to_string()));
        query_params.push(("os_release".to_string(), os_release.to_string()));
        query_params.push(("arch".to_string(), arch.to_string()));
        query_params.push(("launcher_version".to_string(), launcher_version.to_string()));
        if let Some(value) = community { query_params.push(("community".to_string(), value.to_string())); }
        if let Some(value) = search { query_params.push(("search".to_string(), value.to_string())); }
        if let Some(value) = minecraft_version { query_params.push(("minecraftVersion".to_string(), value.to_string())); }
        if let Some(value) = region { query_params.push(("region".to_string(), value.to_string())); }
        if let Some(value) = game_type { query_params.push(("gameType".to_string(), value.to_string())); }
        if let Some(value) = sort { query_params.push(("sort".to_string(), value.to_string())); }
        if let Some(value) = favorites { query_params.push(("favorites".to_string(), value.join(","))); }
        if let Some(value) = show_filters { query_params.push(("showFilters".to_string(), value.to_string())); }
        if let Some(value) = language { query_params.push(("language".to_string(), value.to_string())); }
        request = request.query(&query_params);
        self.send_request::<GetLauncherServersResponse>(request).await
    }
    async fn get_launcher_status(&self, arch: String, installation_id: String, launcher_version: String, os: String, os_release: String, hwid: Option<String>, hwid_private: Option<String>) -> ApiResult<GetLauncherStatusResponse> {
        let url = self.build_url("/launcher/status");
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
        self.send_request::<GetLauncherStatusResponse>(request).await
    }
    async fn post_launcher_launch(&self, body: &PostLauncherLaunchRequest) -> ApiResult<PostLauncherLaunchResponse> {
        let url = self.build_url("/launcher/launch");
        let mut request = self.client.request(Method::POST, &url);
        request = request.json(body);
        self.send_request::<PostLauncherLaunchResponse>(request).await
    }
    async fn post_launcher_launch_vanilla(&self, body: &PostLauncherLaunchVanillaRequest) -> ApiResult<PostLauncherLaunchVanillaResponse> {
        let url = self.build_url("/launcher/launch/vanilla");
        let mut request = self.client.request(Method::POST, &url);
        request = request.json(body);
        self.send_request::<PostLauncherLaunchVanillaResponse>(request).await
    }
    async fn post_launcher_log_upload_complete(&self, body: &PostLauncherLogUploadCompleteRequest) -> ApiResult<PostLauncherLogUploadCompleteResponse> {
        let url = self.build_url("/launcher/log-upload/complete");
        let mut request = self.client.request(Method::POST, &url);
        request = request.json(body);
        self.send_request::<PostLauncherLogUploadCompleteResponse>(request).await
    }
    async fn post_launcher_log_upload_start(&self, body: &PostLauncherLogUploadStartRequest) -> ApiResult<PostLauncherLogUploadStartResponse> {
        let url = self.build_url("/launcher/log-upload/start");
        let mut request = self.client.request(Method::POST, &url);
        request = request.json(body);
        self.send_request::<PostLauncherLogUploadStartResponse>(request).await
    }
    async fn post_launcher_moderation_nsfw(&self) -> ApiResult<PostLauncherModerationNsfwResponse> {
        let url = self.build_url("/launcher/moderation/nsfw");
        let request = self.client.request(Method::POST, &url);
        
        self.send_request::<PostLauncherModerationNsfwResponse>(request).await
    }
    async fn post_launcher_moderation_profanity(&self, body: &PostLauncherModerationProfanityRequest) -> ApiResult<PostLauncherModerationProfanityResponse> {
        let url = self.build_url("/launcher/moderation/profanity");
        let mut request = self.client.request(Method::POST, &url);
        request = request.json(body);
        self.send_request::<PostLauncherModerationProfanityResponse>(request).await
    }
}
