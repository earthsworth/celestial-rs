// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};

/// SimpleVoiceChat endpoint for this joiner
#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct PostHostedWorldRequestHostConnectRequestSimpleVoiceChatEndpoint {
    /// Network address
    pub address: String,
    /// Network port
    pub port: i64,
}