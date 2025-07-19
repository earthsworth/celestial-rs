// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct LauncherRadioStation {
    /// Description of the radio station
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// ID of the radio station
    pub id: String,
    /// Image of the radio station
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    /// Name of the radio station
    pub name: String,
}