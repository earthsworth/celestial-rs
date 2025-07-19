// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};
use super::launch_type_artifact::LaunchTypeArtifact;

/// Information about the specific launch configuration
#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct PostLauncherLaunchResponseLaunchTypeData {
    /// Artifacts associated with launch
    pub artifacts: Vec<LaunchTypeArtifact>,
    /// Main class of the game to launch
    #[serde(rename = "mainClass")]
    pub main_class: String,
}