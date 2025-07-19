// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct GetHostedWorldWorldPrivacySettingsResponse {
    /// Join sources that are allowed to connect to the Hosted World
    #[serde(rename = "allowedJoinSources")]
    pub allowed_join_sources: Vec<String>,
    /// Minecraft players who are friends with the host
    #[serde(rename = "friendUuids")]
    pub friend_uuids: Vec<String>,
    /// Minecraft players who are specifically whitelisted on this Hosted World
    #[serde(rename = "whitelistedUuids")]
    pub whitelisted_uuids: Vec<String>,
}