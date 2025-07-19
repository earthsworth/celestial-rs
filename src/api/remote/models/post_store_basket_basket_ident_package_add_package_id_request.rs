// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct PostStoreBasketBasketIdentPackageAddPackageIdRequest {
    /// How the package was added, we use this for analytics
    #[serde(rename = "addedVia")]
    pub added_via: String,
    /// Quantity of the package to add
    pub quantity: i64,
    /// Type of the package to add
    #[serde(rename = "type")]
    pub r#type: String,
}