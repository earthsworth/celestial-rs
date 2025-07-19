// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};
use super::get_discord_potw_get_cosmetic_pool_response_cosmetics_item::GetDiscordPotwGetCosmeticPoolResponseCosmeticsItem;

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct GetDiscordPotwGetCosmeticPoolResponse {
    /// List of cosmetics
    pub cosmetics: Vec<GetDiscordPotwGetCosmeticPoolResponseCosmeticsItem>,
    /// Success status
    pub success: bool,
}