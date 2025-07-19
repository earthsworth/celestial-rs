// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct RecommendedLibrary {
    /// Identifier of the library
    pub name: String,
    /// Version of the library
    pub version: String,
}