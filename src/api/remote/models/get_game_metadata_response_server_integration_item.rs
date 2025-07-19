// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct GetGameMetadataResponseServerIntegrationItem {
    /// Server brand packet to match
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brand: Option<String>,
    #[serde(rename = "clientSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_settings: Option<serde_json::Value>,
    /// Server IPs to match
    pub ip: Vec<String>,
    #[serde(rename = "modSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mod_settings: Option<serde_json::Value>,
}