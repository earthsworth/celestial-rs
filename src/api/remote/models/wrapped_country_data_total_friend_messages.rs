// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};

/// Total number of friend messages that were sent, where the sending player is from this country
#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct WrappedCountryDataTotalFriendMessages {
    /// Absolute rank this value places the country at, over the entire world
    pub rank: i64,
    /// Country's value for this metric
    pub value: f64,
}