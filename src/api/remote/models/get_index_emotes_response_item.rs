// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};

/// An emote.
#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct GetIndexEmotesResponseItem {
    /// The author of the emote.
    pub author: String,
    /// The duration of the emote (in milliseconds).
    #[serde(rename = "durationMillis")]
    pub duration_millis: f64,
    /// The duration of the emote (in ticks).
    #[serde(rename = "durationTicks")]
    pub duration_ticks: f64,
    /// The ID of the emote.
    pub id: f64,
    /// Whether the emote loops.
    pub looping: bool,
    /// The morph of the emote.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub morph: Option<String>,
    /// The name of the emote.
    pub name: String,
    /// The particle effect of the emote.
    #[serde(rename = "particleEffect")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub particle_effect: Option<String>,
    /// The date the emote was released.
    #[serde(rename = "releasedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub released_at: Option<String>,
    /// Whether the cloak should render while emoting.
    #[serde(rename = "renderCloak")]
    pub render_cloak: bool,
    /// Whether the emote should stop when moving.
    #[serde(rename = "stopOnMove")]
    pub stop_on_move: bool,
}