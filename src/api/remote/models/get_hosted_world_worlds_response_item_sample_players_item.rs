// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};

/// Player on the world
#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct GetHostedWorldWorldsResponseItemSamplePlayersItem {
    /// Player usernmae
    pub username: String,
    /// Player UUID
    pub uuid: String,
}