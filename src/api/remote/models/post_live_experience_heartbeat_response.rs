// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct PostLiveExperienceHeartbeatResponse {
    /// If the heartbeat was successful
    pub success: bool,
    /// If we should trigger the server to start draining
    pub trigger_drain: bool,
}