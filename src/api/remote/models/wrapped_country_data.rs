// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};
use super::wrapped_country_data_top_server_joins_item::WrappedCountryDataTopServerJoinsItem;
use super::wrapped_country_data_top_server_types_item::WrappedCountryDataTopServerTypesItem;
use super::wrapped_country_data_total_cosmetic_changes::WrappedCountryDataTotalCosmeticChanges;
use super::wrapped_country_data_total_emote_time_millis::WrappedCountryDataTotalEmoteTimeMillis;
use super::wrapped_country_data_total_emote_uses::WrappedCountryDataTotalEmoteUses;
use super::wrapped_country_data_total_friend_adds::WrappedCountryDataTotalFriendAdds;
use super::wrapped_country_data_total_friend_messages::WrappedCountryDataTotalFriendMessages;
use super::wrapped_country_data_total_plays::WrappedCountryDataTotalPlays;
use super::wrapped_country_data_total_playtime_millis::WrappedCountryDataTotalPlaytimeMillis;
use super::wrapped_country_data_total_realms_joins::WrappedCountryDataTotalRealmsJoins;
use super::wrapped_country_data_total_server_joins::WrappedCountryDataTotalServerJoins;
use super::wrapped_country_data_total_singleplayer_joins::WrappedCountryDataTotalSingleplayerJoins;

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct WrappedCountryData {
    /// The country code this data represents
    pub country: String,
    /// Top servers, sorted by number of joins, where the player is from this country
    pub top_server_joins: Vec<WrappedCountryDataTopServerJoinsItem>,
    /// Top server types, sorted by number of joins, where the player is from this country
    pub top_server_types: Vec<WrappedCountryDataTopServerTypesItem>,
    /// Total number of cosmetic outfit changes that took place, where the player is from this country
    pub total_cosmetic_changes: WrappedCountryDataTotalCosmeticChanges,
    /// Total number of milliseconds players emoted for, where the player is from this country
    pub total_emote_time_millis: WrappedCountryDataTotalEmoteTimeMillis,
    /// Total number of emote usages that took place, where the player is from this country
    pub total_emote_uses: WrappedCountryDataTotalEmoteUses,
    /// Total number of new friendships that were formed, where at least one player is from this country
    pub total_friend_adds: WrappedCountryDataTotalFriendAdds,
    /// Total number of friend messages that were sent, where the sending player is from this country
    pub total_friend_messages: WrappedCountryDataTotalFriendMessages,
    /// Total number of plays that took place, where the player is from this country
    pub total_plays: WrappedCountryDataTotalPlays,
    /// Total number of milliseconds of playtime that took place, where the player is from this country
    pub total_playtime_millis: WrappedCountryDataTotalPlaytimeMillis,
    /// Total number of Minecraft Realms joins that took place, where the player is from this country
    pub total_realms_joins: WrappedCountryDataTotalRealmsJoins,
    /// Total number of multiplayer server joins that took place, where the player is from this country
    pub total_server_joins: WrappedCountryDataTotalServerJoins,
    /// Total number of singleplayer joins that took place, where the player is from this country
    pub total_singleplayer_joins: WrappedCountryDataTotalSingleplayerJoins,
}