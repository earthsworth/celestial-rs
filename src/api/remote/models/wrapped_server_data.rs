// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};
use super::wrapped_server_data_top_countries_item::WrappedServerDataTopCountriesItem;
use super::wrapped_server_data_top_version_plays_item::WrappedServerDataTopVersionPlaysItem;

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct WrappedServerData {
    /// The ServerMappings ID this data represents
    pub server_mappings_id: String,
    /// The ServerMappings name of the requested server
    pub server_mappings_name: String,
    /// Top countries, sorted by number of joins on this server
    pub top_countries: Vec<WrappedServerDataTopCountriesItem>,
    /// Top Minecraft versions, sorted by number of joins on this server
    pub top_version_plays: Vec<WrappedServerDataTopVersionPlaysItem>,
}