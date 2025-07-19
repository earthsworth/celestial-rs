// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct GetGameMetadataResponseAlertActive {
    /// Color variant of the alert
    pub color: String,
    /// If this alert can be dismissed
    pub dismissable: bool,
    /// Icon for this alert
    pub icon: String,
    /// ID of the alert
    pub id: String,
    /// Whether the alert is clickable and should open up a link
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<String>,
    /// Name of the alert
    pub name: String,
    /// Contents of the alert
    pub text: String,
}