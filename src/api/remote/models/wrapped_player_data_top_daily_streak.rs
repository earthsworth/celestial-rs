// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};

/// Longest daily login streak (consecutive plays)
#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct WrappedPlayerDataTopDailyStreak {
    /// Percentile this value places the player at, in the player's country
    pub country_percentile: f64,
    /// Absolute rank this value places the player at, in the player's country
    pub country_rank: i64,
    /// Percentile this value places the player at, over the entire world
    pub global_percentile: f64,
    /// Absolute rank this value places the player at, over the entire world
    pub global_rank: i64,
    /// Date the top streak ended
    pub streak_end: String,
    /// Date the top streak started
    pub streak_start: String,
    /// Player's value for this metric
    pub value: i64,
}