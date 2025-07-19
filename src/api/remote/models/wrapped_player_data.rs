// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};
use super::wrapped_player_data_cosmetics_purchased_item::WrappedPlayerDataCosmeticsPurchasedItem;
use super::wrapped_player_data_emotes_purchased_item::WrappedPlayerDataEmotesPurchasedItem;
use super::wrapped_player_data_top_cosmetics_item::WrappedPlayerDataTopCosmeticsItem;
use super::wrapped_player_data_top_daily_streak::WrappedPlayerDataTopDailyStreak;
use super::wrapped_player_data_top_emotes_by_use_item::WrappedPlayerDataTopEmotesByUseItem;
use super::wrapped_player_data_top_friends_by_messages_item::WrappedPlayerDataTopFriendsByMessagesItem;
use super::wrapped_player_data_top_server_joins_item::WrappedPlayerDataTopServerJoinsItem;
use super::wrapped_player_data_top_server_types_item::WrappedPlayerDataTopServerTypesItem;
use super::wrapped_player_data_top_version_plays_item::WrappedPlayerDataTopVersionPlaysItem;
use super::wrapped_player_data_total_cosmetic_changes::WrappedPlayerDataTotalCosmeticChanges;
use super::wrapped_player_data_total_emote_time_millis::WrappedPlayerDataTotalEmoteTimeMillis;
use super::wrapped_player_data_total_emote_uses::WrappedPlayerDataTotalEmoteUses;
use super::wrapped_player_data_total_friend_messages::WrappedPlayerDataTotalFriendMessages;
use super::wrapped_player_data_total_name_changes::WrappedPlayerDataTotalNameChanges;
use super::wrapped_player_data_total_plays::WrappedPlayerDataTotalPlays;
use super::wrapped_player_data_total_playtime_millis::WrappedPlayerDataTotalPlaytimeMillis;
use super::wrapped_player_data_total_realms_joins::WrappedPlayerDataTotalRealmsJoins;
use super::wrapped_player_data_total_server_joins::WrappedPlayerDataTotalServerJoins;
use super::wrapped_player_data_total_singleplayer_joins::WrappedPlayerDataTotalSingleplayerJoins;

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct WrappedPlayerData {
    /// UUIDs of players added as a friend in the year (including removed friends)
    pub added_friends: Vec<String>,
    /// Whether the player has all cosmetics
    #[serde(rename = "allCosmetics")]
    pub all_cosmetics: bool,
    /// Whether the player has all emotes
    #[serde(rename = "allEmotes")]
    pub all_emotes: bool,
    /// Cosmetics purchased by the player
    #[serde(rename = "cosmeticsPurchased")]
    pub cosmetics_purchased: Vec<WrappedPlayerDataCosmeticsPurchasedItem>,
    /// Detected 'home' country of the player, used for country ranks/percentiles
    pub country: String,
    /// Emotes purchased by the player
    #[serde(rename = "emotesPurchased")]
    pub emotes_purchased: Vec<WrappedPlayerDataEmotesPurchasedItem>,
    /// Friends with Lunar+
    #[serde(rename = "friendsWithLunarPlus")]
    pub friends_with_lunar_plus: Vec<String>,
    /// Whether the player has Lunar+
    #[serde(rename = "lunarPlus")]
    pub lunar_plus: bool,
    /// Minecraft username of the player
    #[serde(rename = "playerName")]
    pub player_name: String,
    /// Minecraft UUID of the player
    pub player_uuid: String,
    /// Rank of the player
    pub rank: String,
    /// Top cosmetics, sorted by number of uses
    pub top_cosmetics: Vec<WrappedPlayerDataTopCosmeticsItem>,
    /// Top cosmetics, sorted by number of uses
    #[serde(rename = "topCosmetics")]
    pub top_cosmetics_1: Vec<WrappedPlayerDataTopCosmeticsItem>,
    /// Longest daily login streak (consecutive plays)
    pub top_daily_streak: WrappedPlayerDataTopDailyStreak,
    /// Top emotes, sorted by number of uses
    pub top_emotes_by_use: Vec<WrappedPlayerDataTopEmotesByUseItem>,
    /// Top emotes, sorted by number of uses
    #[serde(rename = "topEmotesByUse")]
    pub top_emotes_by_use_1: Vec<WrappedPlayerDataTopEmotesByUseItem>,
    /// Top friends, sorted by number of messages sent to them
    pub top_friends_by_messages: Vec<WrappedPlayerDataTopFriendsByMessagesItem>,
    /// Top friends, sorted by number of messages sent to them
    #[serde(rename = "topFriendsByMessages")]
    pub top_friends_by_messages_1: Vec<WrappedPlayerDataTopFriendsByMessagesItem>,
    /// Top multiplayer servers, sorted by number of joins
    pub top_server_joins: Vec<WrappedPlayerDataTopServerJoinsItem>,
    /// Top multiplayer servers, sorted by number of joins
    #[serde(rename = "topServerJoins")]
    pub top_server_joins_1: Vec<WrappedPlayerDataTopServerJoinsItem>,
    /// Top server types, sorted by number of joins
    pub top_server_types: Vec<WrappedPlayerDataTopServerTypesItem>,
    /// Top Minecraft versions, sorted by number of plays
    pub top_version_plays: Vec<WrappedPlayerDataTopVersionPlaysItem>,
    /// Number of cosmetic changes
    pub total_cosmetic_changes: WrappedPlayerDataTotalCosmeticChanges,
    /// Duration spent emoting (in milliseconds)
    pub total_emote_time_millis: WrappedPlayerDataTotalEmoteTimeMillis,
    /// Number of emote usages that took place
    pub total_emote_uses: WrappedPlayerDataTotalEmoteUses,
    /// Number of friend messages that were sent
    pub total_friend_messages: WrappedPlayerDataTotalFriendMessages,
    /// Number of Minecraft name changes that we detected
    pub total_name_changes: WrappedPlayerDataTotalNameChanges,
    /// Number of plays (websocket connections)
    pub total_plays: WrappedPlayerDataTotalPlays,
    /// Playtime (in milliseconds)
    pub total_playtime_millis: WrappedPlayerDataTotalPlaytimeMillis,
    /// Number of Minecraft Realms joins
    pub total_realms_joins: WrappedPlayerDataTotalRealmsJoins,
    /// Number of multiplayer server joins
    pub total_server_joins: WrappedPlayerDataTotalServerJoins,
    /// Number of singleplayer joins
    pub total_singleplayer_joins: WrappedPlayerDataTotalSingleplayerJoins,
}