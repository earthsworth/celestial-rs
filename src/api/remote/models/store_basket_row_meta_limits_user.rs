// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};

/// User-specific purchase limits
#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct StoreBasketRowMetaLimitsUser {
    /// Whether user-specific purchase limits are enabled
    pub enabled: bool,
    /// Maximum purchase limit for the user
    pub limit: f64,
    /// Timestamp when the limit was set or last updated
    pub timestamp: f64,
}