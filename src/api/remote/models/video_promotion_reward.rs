// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct VideoPromotionReward {
    /// Identifier of cosmetic / emote
    pub id: f64,
    /// Name of the reward
    pub name: String,
    /// Type of reward
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(rename = "viewerData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub viewer_data: Option<serde_json::Value>,
}