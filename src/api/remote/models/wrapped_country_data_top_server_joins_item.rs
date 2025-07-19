// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct WrappedCountryDataTopServerJoinsItem {
    /// ServerMappings ID for this server
    pub server_mappings_id: String,
    /// ServerMappings name for this server
    pub server_mappings_name: String,
    /// Total number of joins to this server, where the player is from this country
    pub value: i64,
}