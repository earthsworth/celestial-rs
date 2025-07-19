// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};

/// Cooldowns for each promotion type
#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct VideoPromotionTimingsCooldowns {
    /// Length of cooldown for direct advertisements (Third party promos)
    pub direct: f64,
    /// Length of cooldown for house advertisements (Lunar Client promos)
    pub house: f64,
}