// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};

/// Java version information
#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct PostLauncherLaunchVanillaResponseLaunchDataJavaVersion {
    /// Java component id
    pub component: String,
    /// Major version id
    #[serde(rename = "majorVersion")]
    pub major_version: f64,
}