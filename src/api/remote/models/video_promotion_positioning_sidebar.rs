// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};

/// Positioning and properties of the sidebar placement for the video promotion
#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct VideoPromotionPositioningSidebar {
    /// Image to display in the sidebar placement for the video promotion, defaults to video icon if nothing is provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    /// Position to insert the video promotion on the sidebar
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slot: Option<f64>,
}