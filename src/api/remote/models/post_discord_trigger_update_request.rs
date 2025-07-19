// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct PostDiscordTriggerUpdateRequest {
    /// The Discord ID of the user we should update
    #[serde(rename = "discordId")]
    pub discord_id: String,
}