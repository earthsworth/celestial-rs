// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};
use super::launcher_version::LauncherVersion;
use super::recommended_library::RecommendedLibrary;

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct GetLauncherMetadataVersionsLunarResponse {
    /// Whether the internal branch should be reset to master (this happens when they try get the metadata for a branch they are not authorised to get)
    #[serde(rename = "branchReset")]
    pub branch_reset: bool,
    /// Libraries and their versions that are recommended
    #[serde(rename = "recommendedLibraries")]
    pub recommended_libraries: Vec<RecommendedLibrary>,
    /// Playable versions for the user
    pub versions: Vec<LauncherVersion>,
}