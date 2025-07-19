// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct WrappedPlayerDataTopCosmeticsItem {
    /// ID of the cosmetic
    pub cosmetic_id: i64,
    /// Player's value for this metric
    pub value: i64,
}