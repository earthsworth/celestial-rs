// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};

/// Arguments to be passed to the game and JVM
#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct PostLauncherLaunchVanillaResponseForgeLaunchDataArguments {
    /// Game arguments
    pub game: Vec<String>,
    /// JVM arguments
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jvm: Option<Vec<String>>,
}