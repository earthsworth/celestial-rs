// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct PostStyngrJwtResponse {
    /// JWT to use with the Styngr API
    #[serde(rename = "styngrJwt")]
    pub styngr_jwt: String,
}