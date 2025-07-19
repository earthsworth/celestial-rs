// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};
use super::post_hosted_world_request_host_connect_request_simple_voice_chat_endpoint::PostHostedWorldRequestHostConnectRequestSimpleVoiceChatEndpoint;

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct PostHostedWorldRequestHostConnectRequest {
    /// Minecraft UUID of player hosting this world
    #[serde(rename = "hostUuid")]
    pub host_uuid: String,
    /// Minecraft UUID of the player joining this world
    #[serde(rename = "joinerUuid")]
    pub joiner_uuid: String,
    /// Address of relay the world host should connect to
    #[serde(rename = "relayAddress")]
    pub relay_address: String,
    /// Port of relay the world host should connect to
    #[serde(rename = "relayPort")]
    pub relay_port: i64,
    /// Unique ID the world host should provide when connecting to the relay
    #[serde(rename = "sessionId")]
    pub session_id: String,
    /// SimpleVoiceChat endpoint for this joiner
    #[serde(rename = "simpleVoiceChatEndpoint")]
    pub simple_voice_chat_endpoint: PostHostedWorldRequestHostConnectRequestSimpleVoiceChatEndpoint,
}