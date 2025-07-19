// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};
use super::launcher_server_images::LauncherServerImages;
use super::server_mappings_compliance::ServerMappingsCompliance;
use super::server_mappings_socials::ServerMappingsSocials;

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct LauncherServer {
    /// Domains that represent the server (used for discovery)
    pub addresses: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance: Option<ServerMappingsCompliance>,
    /// Whether the server supports crossplay with Bedrock players
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crossplay: Option<bool>,
    /// Description of the server
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// List of Game Types that the server provides to play
    #[serde(rename = "gameTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_types: Option<Vec<String>>,
    /// ID of the server
    pub id: String,
    /// Images of the server
    pub images: LauncherServerImages,
    /// List of languages that the server supports
    #[serde(skip_serializing_if = "Option::is_none")]
    pub languages: Option<Vec<String>>,
    /// URL to the server's merchandise
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merch: Option<String>,
    /// List of versions that the server lists as playable
    #[serde(rename = "minecraftVersions")]
    pub minecraft_versions: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modpack: Option<serde_json::Value>,
    /// Display name of the server
    pub name: String,
    /// Whether the server has an active Lunar Client partnership (front page)
    pub partnered: bool,
    /// YouTube ID of a server's Trailer/Introduction video
    #[serde(rename = "presentationVideo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presentation_video: Option<String>,
    /// Address that people will content to upon clicking
    #[serde(rename = "primaryAddress")]
    pub primary_address: String,
    /// Server's primary branding color
    #[serde(rename = "primaryColor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_color: Option<String>,
    /// Primary language that server is based on
    #[serde(rename = "primaryLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_language: Option<String>,
    /// Version that the server recommends you to join with
    #[serde(rename = "primaryMinecraftVersion")]
    pub primary_minecraft_version: String,
    /// Primary region that the server operates in
    #[serde(rename = "primaryRegion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_region: Option<String>,
    /// List of regions that the server operates in (physical hardware)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regions: Option<Vec<String>>,
    /// Server's secondary branding color
    #[serde(rename = "secondaryColor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub socials: Option<ServerMappingsSocials>,
    /// URL to the server's store
    #[serde(skip_serializing_if = "Option::is_none")]
    pub store: Option<String>,
    /// URL to the server's website
    #[serde(skip_serializing_if = "Option::is_none")]
    pub website: Option<String>,
    /// URL to the server's wiki
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wiki: Option<String>,
}