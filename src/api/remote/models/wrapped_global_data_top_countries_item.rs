// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct WrappedGlobalDataTopCountriesItem {
    /// The country code this data represents
    pub country: String,
    /// Percent of players from this country, from 0.0 to 1.0
    pub value: f64,
}