// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};
use super::video_promotion_creative_cta::VideoPromotionCreativeCta;
use super::video_promotion_creative_identifiers::VideoPromotionCreativeIdentifiers;
use super::video_promotion_creative_vendor::VideoPromotionCreativeVendor;
use super::video_promotion_reward::VideoPromotionReward;

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct VideoPromotionCreative {
    /// Call to action information associated with the creative
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cta: Option<VideoPromotionCreativeCta>,
    /// Moonsworth ID of the creative
    pub id: String,
    pub identifiers: VideoPromotionCreativeIdentifiers,
    /// Moonsworth name of the creative
    pub name: String,
    /// Type of creative
    #[serde(rename = "type")]
    pub r#type: String,
    /// Rewards attached with the creative
    pub rewards: Vec<VideoPromotionReward>,
    /// Order of creative within the pool (used for sequence tracking)
    pub sort: f64,
    /// Thumbnail to show in he blog area
    pub thumbnail: String,
    /// Vendor information associated with the creative
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor: Option<VideoPromotionCreativeVendor>,
}