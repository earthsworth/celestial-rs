// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};
use super::video_promotion_corner::VideoPromotionCorner;
use super::video_promotion_creative::VideoPromotionCreative;
use super::video_promotion_positioning::VideoPromotionPositioning;
use super::video_promotion_timings::VideoPromotionTimings;

/// Current video promotion information
#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct VideoPromotion {
    /// List of expected creatives
    #[serde(rename = "activeCreatives")]
    pub active_creatives: Vec<VideoPromotionCreative>,
    /// Corner functionality for the video promotion
    pub corner: VideoPromotionCorner,
    /// Thumbnail to use if the house ad index has reached the end of the list
    #[serde(rename = "fallbackThumbnail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fallback_thumbnail: Option<String>,
    /// Amount of time in milliseconds after launch click to send the pending rewards Toast. Set this to -1 to disable the toast.
    #[serde(rename = "launchToastDelay")]
    pub launch_toast_delay: f64,
    /// URL of the webpage where scripts are being injected
    #[serde(rename = "playerUrl")]
    pub player_url: String,
    pub positioning: VideoPromotionPositioning,
    pub timings: VideoPromotionTimings,
}