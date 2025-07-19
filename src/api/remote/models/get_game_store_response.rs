// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};
use super::get_game_store_response_store_cosmetics_item::GetGameStoreResponseStoreCosmeticsItem;
use super::get_game_store_response_store_emotes_item::GetGameStoreResponseStoreEmotesItem;

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct GetGameStoreResponse {
    /// Store cosmetic packages
    #[serde(rename = "storeCosmetics")]
    pub store_cosmetics: Vec<GetGameStoreResponseStoreCosmeticsItem>,
    /// Store emote packages
    #[serde(rename = "storeEmotes")]
    pub store_emotes: Vec<GetGameStoreResponseStoreEmotesItem>,
}