// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};

/// Partner
#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct GetWebstorePartnersResponsePartnersItem {
    /// Image URL of this partner's store card
    #[serde(rename = "cardImage")]
    pub card_image: String,
    /// Image URL of this partner's store header
    #[serde(rename = "headerImage")]
    pub header_image: String,
    /// Image URL of this partner's icon
    #[serde(rename = "iconImage")]
    pub icon_image: String,
    /// Instagram username of this partner
    #[serde(rename = "instagramUsername")]
    pub instagram_username: String,
    /// Category IDs linked to this partner
    #[serde(rename = "linkedCategoryIds")]
    pub linked_category_ids: Vec<i64>,
    /// Partner name
    pub name: String,
    /// Image URL of this partner's overlay icon
    #[serde(rename = "overlayIconImage")]
    pub overlay_icon_image: String,
    /// Date and time this partner was partnered
    #[serde(rename = "partneredAt")]
    pub partnered_at: String,
    /// Partner type
    #[serde(rename = "type")]
    pub r#type: String,
    /// Ranking of this partner (if ranked in the last 14 days)
    pub ranking: f64,
    /// TikTok username of this partner
    #[serde(rename = "tiktokUsername")]
    pub tiktok_username: String,
    /// Twitch username of this partner
    #[serde(rename = "twitchUsername")]
    pub twitch_username: String,
    /// X username of this partner
    #[serde(rename = "xUsername")]
    pub x_username: String,
    /// YouTube username of this partner
    #[serde(rename = "youtubeUsername")]
    pub youtube_username: String,
}