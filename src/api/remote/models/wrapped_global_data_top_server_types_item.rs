// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct WrappedGlobalDataTopServerTypesItem {
    /// Server type, as defined in ServerMappings
    #[serde(rename = "type")]
    pub r#type: String,
    /// Total number of joins to servers with this type
    pub value: i64,
}