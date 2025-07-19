// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};
use super::post_live_experience_login_response_player::PostLiveExperienceLoginResponsePlayer;

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct PostLiveExperienceLoginResponse {
    /// Metadata about the player joining
    #[serde(skip_serializing_if = "Option::is_none")]
    pub player: Option<PostLiveExperienceLoginResponsePlayer>,
}