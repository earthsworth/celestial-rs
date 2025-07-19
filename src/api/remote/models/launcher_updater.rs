// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};

/// Information regarding the updater
#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct LauncherUpdater {
    /// Updater channel to pull updates from
    pub channel: String,
    /// URL to check updates from
    #[serde(rename = "feedUrl")]
    pub feed_url: String,
}