// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};

/// Links to override in-game
#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct GetGameMetadataResponseLinks {
    /// Custom URL for Store button
    pub store: String,
    /// Custom URL for Trailer button
    pub trailer: String,
    /// Custom URL for Wrapped button
    pub wrapped: String,
    /// Custom URL for Youtooz button
    pub youtooz: String,
}