// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};

/// Collection
#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct GetWebstoreCollectionsResponseCollectionsItem {
    /// Image URL of this collection's store card
    #[serde(rename = "cardImage")]
    pub card_image: String,
    /// Collection description
    pub description: String,
    /// Collection end date
    #[serde(rename = "endDate")]
    pub end_date: String,
    /// Image URL of this collection's store header
    #[serde(rename = "headerImage")]
    pub header_image: String,
    /// Image URL of this collection's icon
    #[serde(rename = "iconImage")]
    pub icon_image: String,
    /// Collection ID
    pub id: i64,
    /// Category IDs linked to this collection
    #[serde(rename = "linkedCategoryIds")]
    pub linked_category_ids: Vec<i64>,
    /// Package IDs linked to this collection
    #[serde(rename = "linkedPackageIds")]
    pub linked_package_ids: Vec<i64>,
    /// Collection name
    pub name: String,
    /// Image URL of this collection's overlay icon
    #[serde(rename = "overlayIconImage")]
    pub overlay_icon_image: String,
    /// Collection release date
    #[serde(rename = "releaseDate")]
    pub release_date: String,
}