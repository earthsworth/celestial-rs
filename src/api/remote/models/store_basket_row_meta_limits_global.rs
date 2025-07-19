// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};

/// Global purchase limits
#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct StoreBasketRowMetaLimitsGlobal {
    /// Whether global purchase limits are enabled
    pub enabled: bool,
    /// Maximum global purchase limit
    pub limit: f64,
    /// Timestamp when the global limit was set or last updated
    pub timestamp: f64,
}