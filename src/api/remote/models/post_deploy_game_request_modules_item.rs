// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct PostDeployGameRequestModulesItem {
    /// Other modules that must NOT be loaded with this module
    pub breaks: Vec<String>,
    /// Name of this module
    pub name: String,
    /// Other modules that must be loaded with this module
    pub requires: Vec<String>,
    /// Minecraft versions this module is compatible with
    pub versions: Vec<String>,
}