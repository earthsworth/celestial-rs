// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct GetGameMetadataResponseStarServersItem {
    /// Regex to match server IPs in the multiplayer list
    pub pattern: String,
    /// Custom texture resource to render
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
}