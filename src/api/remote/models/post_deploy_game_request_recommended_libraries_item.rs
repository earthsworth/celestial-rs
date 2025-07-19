// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct PostDeployGameRequestRecommendedLibrariesItem {
    /// ID of this library
    pub name: String,
    /// Recommended version of this library
    pub version: String,
}