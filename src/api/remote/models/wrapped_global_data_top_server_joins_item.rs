// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct WrappedGlobalDataTopServerJoinsItem {
    /// ServerMappings ID for this server
    pub server_mappings_id: String,
    /// ServerMappings name for this server
    pub server_mappings_name: String,
    /// Total number of joins to this server
    pub value: i64,
}