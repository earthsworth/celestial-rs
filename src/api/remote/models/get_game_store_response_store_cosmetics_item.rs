// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};

/// Store cosmetic object
#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct GetGameStoreResponseStoreCosmeticsItem {
    /// In-game cosmetic ID
    #[serde(rename = "cosmeticId")]
    pub cosmetic_id: i64,
    /// Base price in USD
    pub price: f64,
    /// Tebex package ID
    #[serde(rename = "tebexPackageId")]
    pub tebex_package_id: i64,
}