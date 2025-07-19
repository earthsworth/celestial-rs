// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct WrappedPlayerDataTopFriendsByMessagesItem {
    /// Friend's Minecraft UUID
    pub friend_uuid: String,
    /// Player's value for this metric
    pub value: i64,
}