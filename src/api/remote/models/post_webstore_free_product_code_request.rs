// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct PostWebstoreFreeProductCodeRequest {
    /// Discord user ID generating this code
    #[serde(rename = "discordUserId")]
    pub discord_user_id: f64,
}