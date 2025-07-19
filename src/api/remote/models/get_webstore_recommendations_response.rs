// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};
use super::get_webstore_recommendations_response_recommendation::GetWebstoreRecommendationsResponseRecommendation;

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct GetWebstoreRecommendationsResponse {
    /// Recommendation for this player
    pub recommendation: GetWebstoreRecommendationsResponseRecommendation,
}