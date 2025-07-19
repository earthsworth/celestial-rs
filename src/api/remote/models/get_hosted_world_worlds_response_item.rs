// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};
use super::get_hosted_world_worlds_response_item_sample_players_item::GetHostedWorldWorldsResponseItemSamplePlayersItem;

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct GetHostedWorldWorldsResponseItem {
    /// Minecraft username of the world host
    #[serde(rename = "hostUsername")]
    pub host_username: String,
    /// Minecraft UUID of the world host
    #[serde(rename = "hostUuid")]
    pub host_uuid: String,
    /// Max number of players allowed online
    #[serde(rename = "maxPlayers")]
    pub max_players: i64,
    /// Number of players currently online
    #[serde(rename = "onlinePlayers")]
    pub online_players: i64,
    /// Sample players to display on server list ping
    #[serde(rename = "samplePlayers")]
    pub sample_players: Vec<GetHostedWorldWorldsResponseItemSamplePlayersItem>,
    /// Time this world started to be hosted
    #[serde(rename = "startedAt")]
    pub started_at: String,
}