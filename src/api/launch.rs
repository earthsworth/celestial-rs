use async_trait::async_trait;

use crate::{environment::SystemEnvironment, error::ApiResult, hashing::Hash};

use super::ApiClient;

#[derive(Clone, PartialEq, Debug, serde::Serialize, serde::Deserialize)]
pub struct LaunchLunarRequest {
    pub environment: SystemEnvironment,
    pub launcher_version: String,
    pub canary_preference: Option<CanaryPreference>,

    pub branch: String,
    pub version: String,
    pub module: String,

    pub profile: Option<InstanceProfile>,
}

#[derive(Clone, PartialEq, Debug, serde::Serialize, serde::Deserialize)]
pub enum CanaryPreference {
    OptIn,
    OptOut,
    Normal,
}

impl CanaryPreference {
    pub fn json_name(&self) -> String {
        match self {
            Self::OptIn => "OPT_IN",
            Self::OptOut => "OPT_OUT",
            Self::Normal => "NEUTRAL",
        }
        .to_string()
    }
}

#[derive(Clone, PartialEq, Debug, serde::Serialize, serde::Deserialize)]
pub struct InstanceProfile {
    pub id: String,
    pub name: String,
    pub modrinth: Option<ModrinthProfile>,
}

#[derive(Clone, PartialEq, Debug, serde::Serialize, serde::Deserialize)]
pub struct ModrinthProfile {
    pub id: String,
    pub version_id: Option<String>,
}

#[derive(Clone, PartialEq, Debug, serde::Serialize, serde::Deserialize)]
pub struct LunarVersionManifist {
    pub main_class: String,
    pub artifacts: Vec<Artifact>,
    pub textures: TexturesManifist,
    pub canary_token: Option<String>,
    pub base_modpack: Option<ModPackManifist>,
    pub ui_manifist: Option<BrowserUiManifist>, // this doesn't exist on legacy api implementations
    pub jre_manifist: InstanceRuntimeManifist,
}

#[derive(Clone, PartialEq, Debug, serde::Serialize, serde::Deserialize)]
pub struct LunarRemoteMetadata {
    pub branch: String,
    pub version: String,
    pub module: String,
}

#[derive(Clone, PartialEq, Debug, serde::Serialize, serde::Deserialize)]
pub struct BrowserUiManifist {
    pub source_url: String,
    pub source_hash: Hash,
    pub assets: BrowserUiAssets,
}

#[derive(Clone, PartialEq, Debug, serde::Serialize, serde::Deserialize)]
pub struct BrowserUiAssets {
    pub base_url: String,
    pub index_url: String,
    pub index_hash: Hash,
}

#[derive(Clone, PartialEq, Debug, serde::Serialize, serde::Deserialize)]
pub struct InstanceRuntimeManifist {
    pub extra_vm_options: Vec<String>,
}


#[derive(Clone, PartialEq, Debug, serde::Serialize, serde::Deserialize)]
pub struct Artifact {
    pub name: String,
    pub hash: Hash,
    pub url: String,
    pub artifact_type: ArtifactType,
    pub size: Option<usize>,
    pub modify_time: Option<f64>,
}

#[derive(Clone, PartialEq, Debug, serde::Serialize, serde::Deserialize)]
pub struct ModPackManifist {
    pub version: String,
    pub hash: Hash,
    pub modrinth_pack_url: String,
    pub publish_timestamp: i64,
}

#[derive(Clone, PartialEq, Debug, serde::Serialize, serde::Deserialize)]
pub enum ArtifactType {
    Unknown, // not supported by Celestial

    Classpath,
    ExternalFile,
    NativeLibrary,

    Javaagent, // Celestial special
}

#[derive(Clone, PartialEq, Debug, serde::Serialize, serde::Deserialize)]
pub struct TexturesManifist {
    pub index_url: String,
    pub index_hash: Hash,
    pub jit_index_url: String,
    pub jit_index_hash: Hash,
    pub base_url: String,
}

#[derive(Clone, PartialEq, Debug, serde::Serialize, serde::Deserialize)]
pub struct LunarVersionsMetadata {
    pub versions: Vec<LunarVersionMetadata>,
}

#[derive(Clone, PartialEq, Debug, serde::Serialize, serde::Deserialize)]
pub struct LunarVersionMetadata {
    pub version: String, // id in the origin json
    pub available_modules: Vec<LunarModuleMetadata>,
}

#[derive(Clone, PartialEq, Debug, serde::Serialize, serde::Deserialize)]
pub struct LunarModuleMetadata {
    pub id: String, // id field in the origin json
    pub name: String,
    pub description: String,
    pub loaders: Vec<String>,
}

#[async_trait]
pub trait LaunchExt: ApiClient {
    async fn launch_lunar(&self, request: LaunchLunarRequest) -> ApiResult<LunarVersionManifist>;
    // TODO: launch vanilla
    async fn list_available_lunar_versions(
        &self,
        branch: Option<&str>,
    ) -> ApiResult<LunarVersionsMetadata>;
}
