// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};
use super::post_launcher_launch_vanilla_response_vanilla_launch_data_logging_client::PostLauncherLaunchVanillaResponseVanillaLaunchDataLoggingClient;

/// Logging information for the release
#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct PostLauncherLaunchVanillaResponseVanillaLaunchDataLogging {
    /// Logging client configuration
    pub client: PostLauncherLaunchVanillaResponseVanillaLaunchDataLoggingClient,
}