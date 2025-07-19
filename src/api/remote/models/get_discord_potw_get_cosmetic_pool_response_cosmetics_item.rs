// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};

/// Pick of the week cosmetic object
#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct GetDiscordPotwGetCosmeticPoolResponseCosmeticsItem {
    /// In-game cosmetic ID
    #[serde(rename = "cosmeticId")]
    pub cosmetic_id: i64,
    /// In-game cosmetic name
    pub name: String,
}