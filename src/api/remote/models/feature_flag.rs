// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct FeatureFlag {
    /// Identifier of the feature
    pub identifier: String,
    /// If this feature is active or not
    pub value: bool,
}