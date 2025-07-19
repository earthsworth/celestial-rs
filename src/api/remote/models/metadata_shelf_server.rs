// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};
use super::server_mappings_modpack::ServerMappingsModpack;

/// Server metadata for server shelf functionality
#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct MetadataShelfServer {
    /// Icon of the server
    pub icon: String,
    /// Id of the server
    pub id: String,
    /// IP that should be joined on click
    #[serde(rename = "joinIp")]
    pub join_ip: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modpack: Option<ServerMappingsModpack>,
    /// Display name of the server
    pub name: String,
    /// Server's primary branding color
    #[serde(rename = "primaryColor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_color: Option<String>,
    /// Recommended version of the server
    #[serde(rename = "recommendedVersion")]
    pub recommended_version: String,
    /// Supported Versions of the server
    #[serde(rename = "supportedVersions")]
    pub supported_versions: Vec<String>,
}