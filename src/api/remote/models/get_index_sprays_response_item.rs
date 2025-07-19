// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};

/// A spray.
#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct GetIndexSpraysResponseItem {
    /// Whether the spray has an animated texture
    pub animated: bool,
    /// Whether the spray can cover translucent blocks
    #[serde(rename = "canCover")]
    pub can_cover: bool,
    /// Duration spray is visible (in milliseconds)
    #[serde(rename = "durationMilliseconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_milliseconds: Option<f64>,
    /// Duration spray is visible (in ticks)
    #[serde(rename = "durationTicks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_ticks: Option<f64>,
    /// Whether the spray has a glowing texture
    pub glowing: bool,
    /// Height of the spray (in blocks)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<f64>,
    /// The ID of the spray
    pub id: f64,
    /// The name of the spray
    pub name: String,
    /// X Positon relative to the block it is sprayed on (in blocks)
    #[serde(rename = "offsetX")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset_x: Option<f64>,
    /// Y Position relative to the block it is sprayed on (in blocks)
    #[serde(rename = "offsetY")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset_y: Option<f64>,
    /// The color of the particles when placing the spray (eg. FFFFFF)
    #[serde(rename = "particleColor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub particle_color: Option<String>,
    /// The resource of the spray
    pub resource: String,
    /// Width of the spray (in blocks)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<f64>,
}