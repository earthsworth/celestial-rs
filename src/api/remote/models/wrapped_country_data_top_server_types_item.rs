// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct WrappedCountryDataTopServerTypesItem {
    /// Server type, as defined in ServerMappings
    #[serde(rename = "type")]
    pub r#type: String,
    /// Total number of joins to servers with this type, where the player is from this country
    pub value: i64,
}