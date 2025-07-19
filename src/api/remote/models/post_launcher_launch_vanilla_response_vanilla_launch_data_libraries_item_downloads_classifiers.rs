// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};
use super::post_launcher_launch_vanilla_response_vanilla_launch_data_libraries_item_downloads_classifiers_natives_linux::PostLauncherLaunchVanillaResponseVanillaLaunchDataLibrariesItemDownloadsClassifiersNativesLinux;
use super::post_launcher_launch_vanilla_response_vanilla_launch_data_libraries_item_downloads_classifiers_natives_macos::PostLauncherLaunchVanillaResponseVanillaLaunchDataLibrariesItemDownloadsClassifiersNativesMacos;
use super::post_launcher_launch_vanilla_response_vanilla_launch_data_libraries_item_downloads_classifiers_natives_osx::PostLauncherLaunchVanillaResponseVanillaLaunchDataLibrariesItemDownloadsClassifiersNativesOsx;
use super::post_launcher_launch_vanilla_response_vanilla_launch_data_libraries_item_downloads_classifiers_natives_windows::PostLauncherLaunchVanillaResponseVanillaLaunchDataLibrariesItemDownloadsClassifiersNativesWindows;
use super::post_launcher_launch_vanilla_response_vanilla_launch_data_libraries_item_downloads_classifiers_natives_windows32::PostLauncherLaunchVanillaResponseVanillaLaunchDataLibrariesItemDownloadsClassifiersNativesWindows32;
use super::post_launcher_launch_vanilla_response_vanilla_launch_data_libraries_item_downloads_classifiers_natives_windows64::PostLauncherLaunchVanillaResponseVanillaLaunchDataLibrariesItemDownloadsClassifiersNativesWindows64;

/// Artifact classifiers for the library. Only appears in some libraries
#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct PostLauncherLaunchVanillaResponseVanillaLaunchDataLibrariesItemDownloadsClassifiers {
    /// Downloadable file information
    #[serde(rename = "natives-linux")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub natives_linux: Option<PostLauncherLaunchVanillaResponseVanillaLaunchDataLibrariesItemDownloadsClassifiersNativesLinux>,
    /// Downloadable file information
    #[serde(rename = "natives-macos")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub natives_macos: Option<PostLauncherLaunchVanillaResponseVanillaLaunchDataLibrariesItemDownloadsClassifiersNativesMacos>,
    /// Downloadable file information
    #[serde(rename = "natives-osx")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub natives_osx: Option<PostLauncherLaunchVanillaResponseVanillaLaunchDataLibrariesItemDownloadsClassifiersNativesOsx>,
    /// Downloadable file information
    #[serde(rename = "natives-windows")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub natives_windows: Option<PostLauncherLaunchVanillaResponseVanillaLaunchDataLibrariesItemDownloadsClassifiersNativesWindows>,
    /// Downloadable file information
    #[serde(rename = "natives-windows-32")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub natives_windows_32: Option<PostLauncherLaunchVanillaResponseVanillaLaunchDataLibrariesItemDownloadsClassifiersNativesWindows32>,
    /// Downloadable file information
    #[serde(rename = "natives-windows-64")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub natives_windows_64: Option<PostLauncherLaunchVanillaResponseVanillaLaunchDataLibrariesItemDownloadsClassifiersNativesWindows64>,
}