// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};

/// Arguments to be passed to the game and JVM
#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct PostLauncherLaunchVanillaResponseOptifineLaunchDataArguments {
    /// Game arguments
    pub game: Vec<String>,
    /// JVM arguments
    pub jvm: Vec<String>,
}