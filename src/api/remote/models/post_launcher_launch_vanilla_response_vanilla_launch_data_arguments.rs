// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};

/// Arguments to be passed to the game and JVM
#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct PostLauncherLaunchVanillaResponseVanillaLaunchDataArguments {
    /// Arguments to be passed to the game
    pub game: Vec<serde_json::Value>,
    /// Arguments to be passed to the JVM
    pub jvm: Vec<serde_json::Value>,
}