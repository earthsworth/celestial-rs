// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct GetGameMetadataResponsePinnedServersItem {
    /// Unix timestamp to automatically remove this pin at
    #[serde(rename = "expirationDate")]
    pub expiration_date: i64,
    /// IP of the pinned server in the multiplayer list
    pub ip: String,
    /// Name of the pinned server in the multiplayer list
    pub name: String,
    /// If this pinned server can be deleted by the user
    pub removable: bool,
    /// Minecraft versions that this server is pinned on
    pub versions: Vec<String>,
}