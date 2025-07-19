// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};

/// A cosmetic
#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct GetIndexCosmeticsResponseItem {
    /// Whether the cosmetic is animated
    pub animated: bool,
    /// The category of the cosmetic
    pub category: String,
    /// The colors of the cosmetic
    pub colors: Vec<String>,
    /// Whether cosmetic is a geckolib cosmetic
    #[serde(rename = "geckolibCosmetic")]
    pub geckolib_cosmetic: bool,
    /// Whether to hide on profile external
    #[serde(rename = "hideOnProfileExternal")]
    pub hide_on_profile_external: bool,
    /// The ID of the cosmetic
    pub id: f64,
    /// The index type of the cosmetic
    #[serde(rename = "indexType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_type: Option<String>,
    /// The morph of the cosmetic, a morph is a group of particles
    #[serde(skip_serializing_if = "Option::is_none")]
    pub morph: Option<String>,
    /// The morph duration of the cosmetic (in ticks)
    #[serde(rename = "morphDuration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub morph_duration: Option<f64>,
    /// The name of the cosmetic
    pub name: String,
    /// The date the cosmetic was released.
    #[serde(rename = "releasedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub released_at: Option<String>,
    /// The resource of the cosmetic
    pub resource: String,
    /// Whether the cosmetic is special
    pub special: bool,
    /// The tags of the cosmetic
    pub tags: Vec<String>,
}