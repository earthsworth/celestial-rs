// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};
use super::launcher_server::LauncherServer;
use super::launcher_server_filters::LauncherServerFilters;

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct GetLauncherServersResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<LauncherServerFilters>,
    /// List of servers
    pub servers: Vec<LauncherServer>,
}