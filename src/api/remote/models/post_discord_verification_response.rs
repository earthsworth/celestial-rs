// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};
use super::post_discord_verification_response_discord::PostDiscordVerificationResponseDiscord;
use super::post_discord_verification_response_user::PostDiscordVerificationResponseUser;

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct PostDiscordVerificationResponse {
    /// Info about the Discord user
    pub discord: PostDiscordVerificationResponseDiscord,
    /// If the update was successful
    pub success: bool,
    /// Info about the Minecraft player
    pub user: PostDiscordVerificationResponseUser,
}