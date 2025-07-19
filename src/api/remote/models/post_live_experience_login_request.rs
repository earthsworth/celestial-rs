// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct PostLiveExperienceLoginRequest {
    /// Secret key for authenticating server requests
    #[serde(rename = "heartbeatKey")]
    pub heartbeat_key: String,
    /// Minecraft UUID
    #[serde(rename = "playerUuid")]
    pub player_uuid: String,
}