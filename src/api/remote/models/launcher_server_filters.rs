// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};
use super::launcher_server_mode_filter::LauncherServerModeFilter;
use super::launcher_server_region_filter::LauncherServerRegionFilter;

/// Filters that can be applied to the servers tab
#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct LauncherServerFilters {
    /// List of available modes that can be filtered by
    pub modes: Vec<LauncherServerModeFilter>,
    /// List of available regions that can be filtered by
    pub regions: Vec<LauncherServerRegionFilter>,
    /// List of available versions that can be filtered by
    pub versions: Vec<String>,
}