// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};
use super::post_launcher_launch_vanilla_response_launch_data_logging_client::PostLauncherLaunchVanillaResponseLaunchDataLoggingClient;

/// Logging information for the release
#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct PostLauncherLaunchVanillaResponseLaunchDataLogging {
    /// Logging client configuration
    pub client: PostLauncherLaunchVanillaResponseLaunchDataLoggingClient,
}