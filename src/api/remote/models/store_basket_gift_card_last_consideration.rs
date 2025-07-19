// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};

/// Last consideration details
#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct StoreBasketGiftCardLastConsideration {
    /// Consideration amount
    pub consideration: f64,
    /// Date of the last consideration
    pub date: String,
}