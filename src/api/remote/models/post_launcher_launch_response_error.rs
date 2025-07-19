// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};

/// If the launch request was unsuccessful, information will be populated regarding it
#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct PostLauncherLaunchResponseError {
    /// Code associated with error
    pub code: String,
    /// Specific message associated with error
    pub message: String,
    /// Short version of error message
    pub short: String,
}