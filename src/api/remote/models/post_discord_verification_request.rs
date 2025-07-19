// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct PostDiscordVerificationRequest {
    /// The Discord token of the user we should link
    pub discord_token: String,
}