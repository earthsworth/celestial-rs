// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};
use super::launcher_subversion::LauncherSubversion;
use super::launcher_version_carousel_image::LauncherVersionCarouselImage;
use super::launcher_version_images::LauncherVersionImages;

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct LauncherVersion {
    /// List of slides that will play above the description
    pub carousel: Vec<LauncherVersionCarouselImage>,
    /// Whether this is the version that should be selected on first install
    pub default: bool,
    /// Blurb about the update
    pub description: String,
    /// Identifier of the version
    pub id: String,
    /// Images that are shown with this version listing
    pub images: LauncherVersionImages,
    /// Display name of the version (from Mojang)
    pub name: String,
    /// Date that Mojang released this parent version
    #[serde(rename = "releaseDate")]
    pub release_date: String,
    /// Subversions associated with this parent version
    pub subversions: Vec<LauncherSubversion>,
}