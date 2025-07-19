// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};

/// Cosmetic
#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct WrappedPlayerDataCosmeticsPurchasedItem {
    /// ID of the cosmetic
    pub cosmetic_id: i64,
    /// Image URL of the cosmetic
    pub cosmetic_image_url: String,
    /// Name of the cosmetic
    pub cosmetic_name: String,
}