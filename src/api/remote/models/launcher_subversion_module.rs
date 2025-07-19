// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct LauncherSubversionModule {
    /// Author of the module
    pub credits: String,
    /// Whether this is the default module within the parent subversion
    pub default: bool,
    /// Description of the module
    pub description: String,
    /// Identifier of the module
    pub id: String,
    /// Icon displayed with the module
    pub image: String,
    /// Icon displayed on the launch button next to the module name
    #[serde(rename = "launchButtonImage")]
    pub launch_button_image: String,
    /// Name displayed on the launch button
    #[serde(rename = "launchButtonName")]
    pub launch_button_name: String,
    /// Loaders that are compatible with this module
    pub loaders: Vec<String>,
    /// Display name of the module
    pub name: String,
    /// Order of the module within subversion
    pub sort: f64,
}