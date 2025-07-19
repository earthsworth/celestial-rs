// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};

/// Images that are shown with this version listing
#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct LauncherVanillaVersionImages {
    /// Image shown behind version text
    pub background: String,
    /// Image shown infront of version text
    pub foreground: String,
    /// Image shown in list view of versions page
    pub icon: String,
}