// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};

/// Info about the Discord user
#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct PostDiscordVerificationResponseDiscord {
    /// Discord user's avatar
    pub avatar: String,
    /// Discord user's unique ID
    pub id: String,
    /// Discord user's username
    pub tag: String,
}