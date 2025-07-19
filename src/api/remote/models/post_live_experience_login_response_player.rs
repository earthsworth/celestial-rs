// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};

/// Metadata about the player joining
#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct PostLiveExperienceLoginResponsePlayer {
    /// If this player has Lunar+
    pub has_lunar_plus: bool,
}