// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct GetSiteStatusResponse {
    /// Last updated time of the status
    #[serde(rename = "lastUpdated")]
    pub last_updated: String,
    /// Aggregate status of all the services across Lunar Client
    pub status: String,
}