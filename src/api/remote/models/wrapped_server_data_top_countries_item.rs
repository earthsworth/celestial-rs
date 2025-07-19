// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct WrappedServerDataTopCountriesItem {
    /// 2 letter country code
    pub country: String,
    /// Total number of joins from players from this country
    pub value: i64,
}