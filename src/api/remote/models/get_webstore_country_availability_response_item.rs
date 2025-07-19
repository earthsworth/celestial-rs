// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};

/// A package and the countries it is available in
#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct GetWebstoreCountryAvailabilityResponseItem {
    /// List of country codes which the package is available in
    pub countries: Vec<String>,
    /// Package ID
    #[serde(rename = "packageId")]
    pub package_id: i64,
}