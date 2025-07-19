// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};

/// Corner functionality for the video promotion
#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct VideoPromotionCorner {
    /// Action to take when the countdown is finished on pre-video thumbnail
    #[serde(rename = "countdownAction")]
    pub countdown_action: String,
    /// Amount of time in milliseconds to countdown
    #[serde(rename = "countdownAmount")]
    pub countdown_amount: f64,
    /// Whether or not the corner should be shown
    pub enabled: bool,
    /// Amount of time in milliseconds to wait for the player to start playing before it automatically closes
    #[serde(rename = "playerTimeout")]
    pub player_timeout: f64,
    /// URL of the webpage where scripts are being injected
    #[serde(rename = "playerUrl")]
    pub player_url: String,
    /// Amount of time in milliseconds to wait before showing the corner thumbnail on launch click
    #[serde(rename = "showDelay")]
    pub show_delay: f64,
    /// Thumbnail to show in the corner before video
    pub thumbnail: String,
}