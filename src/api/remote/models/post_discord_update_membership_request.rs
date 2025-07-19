// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct PostDiscordUpdateMembershipRequest {
    /// The Discord ID of the user we should update
    #[serde(rename = "discordId")]
    pub discord_id: String,
    /// The date the user joined the Discord, if they just joined
    #[serde(rename = "joinedAt")]
    pub joined_at: String,
    /// The date the user left the Discord, if they just left
    #[serde(rename = "leftAt")]
    pub left_at: String,
}