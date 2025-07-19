// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct DeleteStoreBasketBasketIdentOmniboxRemoveOmniMethodRequest {
    /// The code to remove from the basket
    pub code: String,
}