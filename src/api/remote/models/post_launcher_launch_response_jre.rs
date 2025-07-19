// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};
use super::post_launcher_launch_response_jre_download::PostLauncherLaunchResponseJreDownload;

/// JRE information
#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct PostLauncherLaunchResponseJre {
    /// Download information of the JRE
    pub download: PostLauncherLaunchResponseJreDownload,
    /// List of paths to exe in zip
    #[serde(rename = "executablePathInArchive")]
    pub executable_path_in_archive: Vec<String>,
    /// Extra arguments that need to be passed to launch sequence
    #[serde(rename = "extraArguments")]
    pub extra_arguments: Vec<String>,
    /// Checksum of JRE folder
    #[serde(rename = "folderChecksum")]
    pub folder_checksum: String,
    /// URL to additional javaw override
    #[serde(rename = "javawDownload")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub javaw_download: Option<String>,
    /// Checksum of additional javaw override
    #[serde(rename = "javawExeChecksum")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub javaw_exe_checksum: Option<String>,
}