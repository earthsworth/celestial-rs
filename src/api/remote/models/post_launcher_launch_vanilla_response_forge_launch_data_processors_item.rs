// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};

/// Processors to be run before launching forge
#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct PostLauncherLaunchVanillaResponseForgeLaunchDataProcessorsItem {
    /// Arguments to pass to the processor
    pub args: Vec<String>,
    /// Classpaths to run with the processors jar file
    pub classpath: Vec<String>,
    /// Name of the jar file
    pub jar: String,
    /// Output variables
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outputs: Option<serde_json::Value>,
    /// Environments to run the processor in
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sides: Option<Vec<String>>,
}