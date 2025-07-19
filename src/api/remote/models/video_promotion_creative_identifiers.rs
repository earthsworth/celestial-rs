// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};

/// List of identifiers that will tried to be matched
#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct VideoPromotionCreativeIdentifiers {
    /// Advertising ID that is being played
    #[serde(rename = "adId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ad_id: Option<String>,
    /// Creative ID that is being served with the advertisement
    #[serde(rename = "creativeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creative_id: Option<String>,
    /// Title attributed to the creative
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}