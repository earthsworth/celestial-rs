// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};
use super::get_webstore_recommendations_response_recommendation_items_item::GetWebstoreRecommendationsResponseRecommendationItemsItem;

/// Recommendation for this player
#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct GetWebstoreRecommendationsResponseRecommendation {
    /// ID to identify this recommendation in analytics
    pub id: String,
    /// Recommended items
    pub items: Vec<GetWebstoreRecommendationsResponseRecommendationItemsItem>,
}