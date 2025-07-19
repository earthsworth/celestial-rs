// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct WrappedServerDataTopVersionPlaysItem {
    /// Total number of joins on this Minecraft version
    pub value: i64,
    /// Minecraft version (major version only)
    pub version: String,
}