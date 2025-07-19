// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};

/// Total number of Minecraft Realms joins that took place, where the player is from this country
#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct WrappedCountryDataTotalRealmsJoins {
    /// Absolute rank this value places the country at, over the entire world
    pub rank: i64,
    /// Country's value for this metric
    pub value: f64,
}