// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};

/// Extraction information for the library
#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct PostLauncherLaunchVanillaResponseVanillaLaunchDataLibrariesItemExtract {
    /// Items to exclude from the extraction
    pub exclude: Vec<String>,
}