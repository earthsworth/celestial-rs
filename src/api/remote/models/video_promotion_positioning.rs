// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};
use super::video_promotion_positioning_blog::VideoPromotionPositioningBlog;
use super::video_promotion_positioning_sidebar::VideoPromotionPositioningSidebar;

/// Positioning of the entrypoints to the promotion in the Launcher
#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct VideoPromotionPositioning {
    /// Positioning and properties of the blog placement for the video promotion
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blog: Option<VideoPromotionPositioningBlog>,
    /// Positioning and properties of the sidebar placement for the video promotion
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sidebar: Option<VideoPromotionPositioningSidebar>,
}