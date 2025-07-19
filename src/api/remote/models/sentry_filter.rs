// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct SentryFilter {
    /// Identifier of the filter
    pub identifier: String,
    /// Regex to match events with
    pub regex: String,
}