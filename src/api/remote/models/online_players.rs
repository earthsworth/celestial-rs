// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};

/// Number of concurrent players across Lunar Client platforms
#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct OnlinePlayers {
    /// Number of player with the game open
    pub game: f64,
    /// Number of player with the launcher open
    pub launcher: f64,
}