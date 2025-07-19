// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};
use super::video_promotion_timings_cooldowns::VideoPromotionTimingsCooldowns;
use super::video_promotion_timings_views_before_cooldown::VideoPromotionTimingsViewsBeforeCooldown;

/// Values that are applied to each promotion type
#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct VideoPromotionTimings {
    /// Cooldowns for each promotion type
    pub cooldowns: VideoPromotionTimingsCooldowns,
    /// Number of views before cooldown is applied for each promotion type
    #[serde(rename = "viewsBeforeCooldown")]
    pub views_before_cooldown: VideoPromotionTimingsViewsBeforeCooldown,
}