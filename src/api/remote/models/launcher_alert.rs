// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};

/// Alert to display to the user
#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct LauncherAlert {
    /// Color variant of the alert
    pub color: String,
    /// ID of the alert
    pub id: String,
    /// Whether the alert is clickable and should open up a link
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<String>,
    /// Contents of the alert
    pub text: String,
}