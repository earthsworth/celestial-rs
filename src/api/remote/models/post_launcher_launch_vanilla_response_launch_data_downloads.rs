// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};
use super::post_launcher_launch_vanilla_response_launch_data_downloads_client::PostLauncherLaunchVanillaResponseLaunchDataDownloadsClient;
use super::post_launcher_launch_vanilla_response_launch_data_downloads_client_mappings::PostLauncherLaunchVanillaResponseLaunchDataDownloadsClientMappings;
use super::post_launcher_launch_vanilla_response_launch_data_downloads_server::PostLauncherLaunchVanillaResponseLaunchDataDownloadsServer;
use super::post_launcher_launch_vanilla_response_launch_data_downloads_server_mappings::PostLauncherLaunchVanillaResponseLaunchDataDownloadsServerMappings;

/// Version jars and mappings
#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct PostLauncherLaunchVanillaResponseLaunchDataDownloads {
    /// Downloadable file information
    pub client: PostLauncherLaunchVanillaResponseLaunchDataDownloadsClient,
    /// Downloadable file information
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_mappings: Option<PostLauncherLaunchVanillaResponseLaunchDataDownloadsClientMappings>,
    /// Downloadable file information
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server: Option<PostLauncherLaunchVanillaResponseLaunchDataDownloadsServer>,
    /// Downloadable file information
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_mappings: Option<PostLauncherLaunchVanillaResponseLaunchDataDownloadsServerMappings>,
}