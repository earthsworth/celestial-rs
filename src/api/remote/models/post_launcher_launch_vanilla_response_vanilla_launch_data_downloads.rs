// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};
use super::post_launcher_launch_vanilla_response_vanilla_launch_data_downloads_client::PostLauncherLaunchVanillaResponseVanillaLaunchDataDownloadsClient;
use super::post_launcher_launch_vanilla_response_vanilla_launch_data_downloads_client_mappings::PostLauncherLaunchVanillaResponseVanillaLaunchDataDownloadsClientMappings;
use super::post_launcher_launch_vanilla_response_vanilla_launch_data_downloads_server::PostLauncherLaunchVanillaResponseVanillaLaunchDataDownloadsServer;
use super::post_launcher_launch_vanilla_response_vanilla_launch_data_downloads_server_mappings::PostLauncherLaunchVanillaResponseVanillaLaunchDataDownloadsServerMappings;

/// Downloadable files for the release
#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct PostLauncherLaunchVanillaResponseVanillaLaunchDataDownloads {
    /// Downloadable file information
    pub client: PostLauncherLaunchVanillaResponseVanillaLaunchDataDownloadsClient,
    /// Downloadable file information
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_mappings: Option<PostLauncherLaunchVanillaResponseVanillaLaunchDataDownloadsClientMappings>,
    /// Downloadable file information
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server: Option<PostLauncherLaunchVanillaResponseVanillaLaunchDataDownloadsServer>,
    /// Downloadable file information
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_mappings: Option<PostLauncherLaunchVanillaResponseVanillaLaunchDataDownloadsServerMappings>,
}