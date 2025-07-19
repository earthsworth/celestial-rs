// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};

/// Number of singleplayer joins
#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct WrappedPlayerDataTotalSingleplayerJoins {
    /// Percentile this value places the player at, in the player's country
    pub country_percentile: f64,
    /// Absolute rank this value places the player at, in the player's country
    pub country_rank: i64,
    /// Percentile this value places the player at, over the entire world
    pub global_percentile: f64,
    /// Absolute rank this value places the player at, over the entire world
    pub global_rank: i64,
    /// Player's value for this metric
    pub value: i64,
}