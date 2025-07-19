// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};
use super::user_profile_rank::UserProfileRank;

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct UserProfile {
    /// The active cosmetics of the user
    pub active_cosmetics: Vec<f64>,
    /// The equipped emotes of the user
    pub equipped_emotes: Vec<f64>,
    /// Whether the user has flipped their shoulder pet
    pub flip_shoulder_pet: bool,
    /// The color of the user's Lunar+
    pub lunar_plus_color: String,
    pub rank: UserProfileRank,
    /// The username of the user
    pub username: String,
    /// The UUID of the user
    pub uuid: String,
}