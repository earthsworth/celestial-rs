// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct PostLiveExperienceHeartbeatRequest {
    /// Max number of players that can join this server when including staff/friend joins
    #[serde(rename = "hardMaxPlayers")]
    pub hard_max_players: i64,
    /// Secret key for authenticating server requests
    #[serde(rename = "heartbeatKey")]
    pub heartbeat_key: String,
    /// Server ID
    pub id: String,
    /// Minecraft address that players connect with
    #[serde(rename = "joinAddress")]
    pub join_address: String,
    /// Minecraft port that players connect with
    #[serde(rename = "joinPort")]
    pub join_port: i64,
    /// Minecraft UUIDs that are currently online
    #[serde(rename = "onlinePlayers")]
    pub online_players: Vec<String>,
    /// Max number of players that can join this server under normal circumstances
    #[serde(rename = "softMaxPlayers")]
    pub soft_max_players: i64,
    /// State of the server
    pub state: String,
}