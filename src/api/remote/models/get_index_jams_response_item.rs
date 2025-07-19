// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};

/// A jam.
#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct GetIndexJamsResponseItem {
    /// The album that the jam is from
    pub album: String,
    /// The artists of the jam
    pub artist: String,
    /// Whether the jam is considered 'copyright safe'
    #[serde(rename = "copyrightSafe")]
    pub copyright_safe: bool,
    /// The duration of the jam in milliseconds
    #[serde(rename = "durationMillis")]
    pub duration_millis: f64,
    /// The genres of the jam
    pub genres: Vec<String>,
    /// The ID of the jam
    pub id: f64,
    /// The name of the jam
    pub name: String,
    /// The song that the jam is from
    pub song: String,
    /// The STYNGR ID of the jam
    #[serde(rename = "styngrId")]
    pub styngr_id: String,
}