// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};

/// Recommended item
#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct GetWebstoreRecommendationsResponseRecommendationItemsItem {
    /// Package ID of this item
    #[serde(rename = "packageId")]
    pub package_id: i64,
}