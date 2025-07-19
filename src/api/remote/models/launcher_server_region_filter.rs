// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct LauncherServerRegionFilter {
    /// Continent code of the region
    pub code: String,
    /// Name of the region
    pub name: String,
}