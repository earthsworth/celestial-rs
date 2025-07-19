// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct Ad {
    /// Text of the call to action button
    pub cta_button_text: String,
    /// URL of the call to action button
    pub cta_button_url: String,
    /// URL of the call to action icon
    pub cta_icon_url: String,
    /// Text of the call to action
    pub cta_text: String,
    /// End time of the ad
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    /// ID of the advertisement
    pub id: String,
    /// URL of the media asset
    pub media_asset_url: String,
    /// URL of the target page when the media asset is clicked
    pub media_target_url: String,
    /// Placement of the advertisement
    pub placement: String,
    /// Type of advertisement
    #[serde(rename = "type")]
    pub r#type: String,
    /// Whether to show the ad to Lunar+ users
    pub show_to_lunar_plus: bool,
    /// Start time of the ad
    pub start_time: String,
}