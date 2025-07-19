// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};

/// An item in a package.
#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct GetIndexPackagesResponseItemItemsItem {
    /// Whether the item is display only (if it grants the item when purchased)
    #[serde(rename = "displayOnly")]
    pub display_only: bool,
    /// The ID of the item
    pub id: f64,
    /// The type of the item
    #[serde(rename = "type")]
    pub r#type: String,
}