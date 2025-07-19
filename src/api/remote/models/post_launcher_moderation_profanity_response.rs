// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct PostLauncherModerationProfanityResponse {
    /// Whether the text is allowed or not based on the profanity filter
    pub allowed: bool,
}