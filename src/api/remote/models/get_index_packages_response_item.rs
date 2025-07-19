// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};
use super::get_index_packages_response_item_items_item::GetIndexPackagesResponseItemItemsItem;

/// A package.
#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct GetIndexPackagesResponseItem {
    /// The ID of the package
    pub id: f64,
    /// The image of the package
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    /// The items in the package
    pub items: Vec<GetIndexPackagesResponseItemItemsItem>,
}