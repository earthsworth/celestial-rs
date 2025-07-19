// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct TopbarSocial {
    /// Icon to display for the social. Can either be a URL or a Fontawesome icon class
    pub icon: String,
    /// Identifier for social item
    pub id: String,
    /// Name of the social (ex. Twitter)
    pub name: String,
    /// URL that gets opened on click
    pub url: String,
}