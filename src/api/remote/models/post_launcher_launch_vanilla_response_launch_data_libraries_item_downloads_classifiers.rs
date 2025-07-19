// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};
use super::post_launcher_launch_vanilla_response_launch_data_libraries_item_downloads_classifiers_natives_linux::PostLauncherLaunchVanillaResponseLaunchDataLibrariesItemDownloadsClassifiersNativesLinux;
use super::post_launcher_launch_vanilla_response_launch_data_libraries_item_downloads_classifiers_natives_macos::PostLauncherLaunchVanillaResponseLaunchDataLibrariesItemDownloadsClassifiersNativesMacos;
use super::post_launcher_launch_vanilla_response_launch_data_libraries_item_downloads_classifiers_natives_osx::PostLauncherLaunchVanillaResponseLaunchDataLibrariesItemDownloadsClassifiersNativesOsx;
use super::post_launcher_launch_vanilla_response_launch_data_libraries_item_downloads_classifiers_natives_windows::PostLauncherLaunchVanillaResponseLaunchDataLibrariesItemDownloadsClassifiersNativesWindows;
use super::post_launcher_launch_vanilla_response_launch_data_libraries_item_downloads_classifiers_natives_windows32::PostLauncherLaunchVanillaResponseLaunchDataLibrariesItemDownloadsClassifiersNativesWindows32;
use super::post_launcher_launch_vanilla_response_launch_data_libraries_item_downloads_classifiers_natives_windows64::PostLauncherLaunchVanillaResponseLaunchDataLibrariesItemDownloadsClassifiersNativesWindows64;

/// Artifact classifiers for the library. Only appears in some libraries
#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct PostLauncherLaunchVanillaResponseLaunchDataLibrariesItemDownloadsClassifiers {
    /// Downloadable file information
    #[serde(rename = "natives-linux")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub natives_linux: Option<PostLauncherLaunchVanillaResponseLaunchDataLibrariesItemDownloadsClassifiersNativesLinux>,
    /// Downloadable file information
    #[serde(rename = "natives-macos")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub natives_macos: Option<PostLauncherLaunchVanillaResponseLaunchDataLibrariesItemDownloadsClassifiersNativesMacos>,
    /// Downloadable file information
    #[serde(rename = "natives-osx")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub natives_osx: Option<PostLauncherLaunchVanillaResponseLaunchDataLibrariesItemDownloadsClassifiersNativesOsx>,
    /// Downloadable file information
    #[serde(rename = "natives-windows")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub natives_windows: Option<PostLauncherLaunchVanillaResponseLaunchDataLibrariesItemDownloadsClassifiersNativesWindows>,
    /// Downloadable file information
    #[serde(rename = "natives-windows-32")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub natives_windows_32: Option<PostLauncherLaunchVanillaResponseLaunchDataLibrariesItemDownloadsClassifiersNativesWindows32>,
    /// Downloadable file information
    #[serde(rename = "natives-windows-64")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub natives_windows_64: Option<PostLauncherLaunchVanillaResponseLaunchDataLibrariesItemDownloadsClassifiersNativesWindows64>,
}