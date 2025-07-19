// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};
use super::launcher_vanilla_version::LauncherVanillaVersion;

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct GetLauncherMetadataVersionsVanillaResponse {
    /// Playable versions without Lunar modifications
    pub versions: Vec<LauncherVanillaVersion>,
}