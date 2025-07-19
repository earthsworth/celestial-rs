// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};

/// Number of views before cooldown is applied for each promotion type
#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct VideoPromotionTimingsViewsBeforeCooldown {
    /// Amount of direct ad views before cooldown is applied
    pub direct: f64,
    /// Amount of house ad views before cooldown is applied
    pub house: f64,
}