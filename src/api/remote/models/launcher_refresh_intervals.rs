// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};

/// Refresh intervals across the Launcher
#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct LauncherRefreshIntervals {
    /// How often the AssetServer heartbeat should be sent
    pub heartbeat: f64,
    /// How often the metadata should refresh
    pub metadata: f64,
    /// How often the server counts fetch is sent
    pub servers: f64,
    /// How often the auto updater should check for updates
    pub updates: f64,
}