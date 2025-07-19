// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};

/// UTM parameters for tracking purposes
#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct PostStoreBasketCreateRequestUtmParams {
    /// UTM campaign
    pub utm_campaign: String,
    /// UTM content
    pub utm_content: String,
    /// UTM medium
    pub utm_medium: String,
    /// UTM source
    pub utm_source: String,
    /// UTM term
    pub utm_term: String,
}