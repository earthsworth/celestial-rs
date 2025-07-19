// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct GetStoreBasketBasketIdentInfoResponse {
    pub basket: serde_json::Value,
    /// Whether the request was successful
    pub success: bool,
}