// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};
use super::post_launcher_launch_vanilla_response_launch_data_logging_client_file::PostLauncherLaunchVanillaResponseLaunchDataLoggingClientFile;

/// Logging client configuration
#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct PostLauncherLaunchVanillaResponseLaunchDataLoggingClient {
    /// Argument for the logging client
    pub argument: String,
    /// Downloadable file information
    pub file: PostLauncherLaunchVanillaResponseLaunchDataLoggingClientFile,
    /// Type associated with the logging client
    #[serde(rename = "type")]
    pub r#type: String,
}