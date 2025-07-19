// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};
use super::post_launcher_launch_vanilla_response_optifine_launch_data_libraries_item_downloads_classifiers_natives_linux::PostLauncherLaunchVanillaResponseOptifineLaunchDataLibrariesItemDownloadsClassifiersNativesLinux;
use super::post_launcher_launch_vanilla_response_optifine_launch_data_libraries_item_downloads_classifiers_natives_macos::PostLauncherLaunchVanillaResponseOptifineLaunchDataLibrariesItemDownloadsClassifiersNativesMacos;
use super::post_launcher_launch_vanilla_response_optifine_launch_data_libraries_item_downloads_classifiers_natives_osx::PostLauncherLaunchVanillaResponseOptifineLaunchDataLibrariesItemDownloadsClassifiersNativesOsx;
use super::post_launcher_launch_vanilla_response_optifine_launch_data_libraries_item_downloads_classifiers_natives_windows::PostLauncherLaunchVanillaResponseOptifineLaunchDataLibrariesItemDownloadsClassifiersNativesWindows;
use super::post_launcher_launch_vanilla_response_optifine_launch_data_libraries_item_downloads_classifiers_natives_windows32::PostLauncherLaunchVanillaResponseOptifineLaunchDataLibrariesItemDownloadsClassifiersNativesWindows32;
use super::post_launcher_launch_vanilla_response_optifine_launch_data_libraries_item_downloads_classifiers_natives_windows64::PostLauncherLaunchVanillaResponseOptifineLaunchDataLibrariesItemDownloadsClassifiersNativesWindows64;

/// Artifact classifiers for the library. Only appears in some libraries
#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct PostLauncherLaunchVanillaResponseOptifineLaunchDataLibrariesItemDownloadsClassifiers {
    /// Downloadable file information
    #[serde(rename = "natives-linux")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub natives_linux: Option<PostLauncherLaunchVanillaResponseOptifineLaunchDataLibrariesItemDownloadsClassifiersNativesLinux>,
    /// Downloadable file information
    #[serde(rename = "natives-macos")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub natives_macos: Option<PostLauncherLaunchVanillaResponseOptifineLaunchDataLibrariesItemDownloadsClassifiersNativesMacos>,
    /// Downloadable file information
    #[serde(rename = "natives-osx")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub natives_osx: Option<PostLauncherLaunchVanillaResponseOptifineLaunchDataLibrariesItemDownloadsClassifiersNativesOsx>,
    /// Downloadable file information
    #[serde(rename = "natives-windows")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub natives_windows: Option<PostLauncherLaunchVanillaResponseOptifineLaunchDataLibrariesItemDownloadsClassifiersNativesWindows>,
    /// Downloadable file information
    #[serde(rename = "natives-windows-32")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub natives_windows_32: Option<PostLauncherLaunchVanillaResponseOptifineLaunchDataLibrariesItemDownloadsClassifiersNativesWindows32>,
    /// Downloadable file information
    #[serde(rename = "natives-windows-64")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub natives_windows_64: Option<PostLauncherLaunchVanillaResponseOptifineLaunchDataLibrariesItemDownloadsClassifiersNativesWindows64>,
}