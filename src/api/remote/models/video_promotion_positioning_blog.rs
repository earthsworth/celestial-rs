// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};

/// Positioning and properties of the blog placement for the video promotion
#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct VideoPromotionPositioningBlog {
    /// Position to insert the video promotion on the home page
    pub slot: f64,
}