// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct PostStoreBasketBasketIdentPackageGiftPackageIdRequest {
    /// How the package was added, we use this for analytics
    #[serde(rename = "addedVia")]
    pub added_via: String,
    /// If true, the gift will be anonymous
    pub anonymous: bool,
    /// Message for the gift recipient
    pub message: String,
    /// Quantity of the package to add
    pub quantity: i64,
    /// Type of the package to add
    #[serde(rename = "type")]
    pub r#type: String,
    /// Username of the user to gift the package to
    #[serde(rename = "targetUsername")]
    pub target_username: String,
}