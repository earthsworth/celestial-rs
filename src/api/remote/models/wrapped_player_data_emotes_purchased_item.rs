// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};

/// Emote
#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct WrappedPlayerDataEmotesPurchasedItem {
    /// ID of the emote
    pub emote_id: i64,
    /// Image URL of the emote
    pub emote_image_url: String,
    /// Name of the emote
    pub emote_name: String,
}