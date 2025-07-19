// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};
use super::site_server_images::SiteServerImages;

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct SiteServer {
    /// Description of the server
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// ID of the server
    pub id: String,
    /// Images of the server
    pub images: SiteServerImages,
    /// Display name of the server
    pub name: String,
    /// Address that people will content to upon clicking
    #[serde(rename = "primaryAddress")]
    pub primary_address: String,
    /// Server's primary branding color
    #[serde(rename = "primaryColor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_color: Option<String>,
    /// Version that the server recommends you to join with
    #[serde(rename = "primaryMinecraftVersion")]
    pub primary_minecraft_version: String,
}