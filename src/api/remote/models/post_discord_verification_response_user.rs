// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};

/// Info about the Minecraft player
#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct PostDiscordVerificationResponseUser {
    /// Minecraft username of the linked player
    pub username: String,
    /// Minecraft UUID of the linked player
    pub uuid: String,
}