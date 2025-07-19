// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};

/// Launch License
#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct LauncherLaunchLicense {
    /// File of the license
    pub file: String,
    /// Last modification time of the license, in seconds since epoch
    pub mtime: f64,
    /// Hash of the license
    pub sha1: String,
    /// Size of the license, in bytes
    pub size: f64,
    /// URL location of the license
    pub url: String,
}