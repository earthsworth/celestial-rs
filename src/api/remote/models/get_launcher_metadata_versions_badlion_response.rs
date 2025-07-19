// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};
use super::launcher_badlion_version::LauncherBadlionVersion;

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct GetLauncherMetadataVersionsBadlionResponse {
    /// Playable Badlion versions
    pub versions: Vec<LauncherBadlionVersion>,
}