// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};

/// Store emote object
#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct GetGameStoreResponseStoreEmotesItem {
    /// In-game emote ID
    #[serde(rename = "emoteId")]
    pub emote_id: i64,
    /// Base price in USD
    pub price: f64,
    /// Tebex package ID
    #[serde(rename = "tebexPackageId")]
    pub tebex_package_id: i64,
}