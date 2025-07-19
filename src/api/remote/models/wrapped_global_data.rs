// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};
use super::wrapped_global_data_top_countries_item::WrappedGlobalDataTopCountriesItem;
use super::wrapped_global_data_top_server_joins_item::WrappedGlobalDataTopServerJoinsItem;
use super::wrapped_global_data_top_server_types_item::WrappedGlobalDataTopServerTypesItem;
use super::wrapped_global_data_top_version_plays_item::WrappedGlobalDataTopVersionPlaysItem;

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct WrappedGlobalData {
    /// Top countries, sorted by percentage of players
    pub top_countries: Vec<WrappedGlobalDataTopCountriesItem>,
    /// Top servers, sorted by number of joins
    pub top_server_joins: Vec<WrappedGlobalDataTopServerJoinsItem>,
    /// Top server types, sorted by number of joins
    pub top_server_types: Vec<WrappedGlobalDataTopServerTypesItem>,
    /// Top Minecraft versions, sorted by number of plays
    pub top_version_plays: Vec<WrappedGlobalDataTopVersionPlaysItem>,
    /// Total number of milliseconds players emoted for
    pub total_emote_time_millis: f64,
    /// Total number of emote usages that took place
    pub total_emote_uses: f64,
    /// Total number of new friendships that were formed
    pub total_friend_adds: f64,
    /// Total number of friend messages that were sent
    pub total_friend_messages: f64,
    /// Total number of Minecraft name changes that we detected
    pub total_name_changes: f64,
    /// Total number of plays that took place
    pub total_plays: f64,
    /// Total number of milliseconds of playtime that took place
    pub total_playtime_millis: f64,
    /// Total number of Minecraft Realms joins that took place
    pub total_realms_joins: f64,
    /// Total number of multiplayer server joins that took place
    pub total_server_joins: f64,
    /// Total number of singleplayer joins that took place
    pub total_singleplayer_joins: f64,
}