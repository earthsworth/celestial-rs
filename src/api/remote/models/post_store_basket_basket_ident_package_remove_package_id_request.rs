// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct PostStoreBasketBasketIdentPackageRemovePackageIdRequest {
    /// If provided, the gift for this user will be removed from the basket
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gift_target_uuid: Option<String>,
    /// Type of the package to add
    #[serde(rename = "type")]
    pub r#type: String,
}