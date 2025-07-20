use core::RawApiClient;
use std::str::FromStr;

use async_trait::async_trait;
use chrono::NaiveDateTime;
use launcher_api::LauncherApi;
use reqwest::Client;
use uuid::Uuid;

use crate::{
    api::{
        launch::{
            Artifact, ArtifactType, BrowserUiAssets, BrowserUiManifest, InstanceRuntimeManifest,
            LaunchExt, LaunchLunarRequest, LunarModuleMetadata, LunarVersionManifest,
            LunarVersionMetadata, LunarVersionsMetadata, ModPackManifest, TexturesManifest,
        }, ApiClient
    },
    environment::{Arch, Os},
    error::{ApiError, ApiResult}, utils::hashing::Hash,
};

use super::models::{LauncherProfile, LauncherProfileModrinth, PostLauncherLaunchRequest};

pub mod apollo_api;
pub mod core;
pub mod deploy_api;
pub mod discord_api;
pub mod fabric_api;
pub mod forge_api;
pub mod game_api;
pub mod hosted_world_api;
pub mod index_api;
pub mod internal_api;
pub mod launcher_api;
pub mod live_experience_api;
pub mod neoforge_api;
pub mod og_api;
pub mod optifine_api;
pub mod pics_api;
pub mod quilt_api;
pub mod site_api;
pub mod store_api;
pub mod streamelements_api;
pub mod styngr_api;
pub mod twitter_api;
pub mod webstore_api;
pub mod wrapped_api;

#[derive(Clone)]
pub struct RemoteApiClient {
    raw_client: RawApiClient,
}

impl RemoteApiClient {
    pub fn new(client: &Client, base_url: &str) -> Self {
        let raw_client = RawApiClient {
            client: client.to_owned(),
            base_url: base_url.to_string(),
        };
        Self { raw_client }
    }
}

impl ApiClient for RemoteApiClient {}

#[async_trait]
impl LaunchExt for RemoteApiClient {
    async fn launch_lunar(&self, request: LaunchLunarRequest) -> ApiResult<LunarVersionManifest> {
        let os = match request.environment.os {
            Os::Windows => "win32",
            Os::Linux => "linux",
            Os::Macos => "osx",
            _ => "linux", // just return a mock value, celestial.rs will process the target platform later
        }
        .to_string();

        let arch = match request.environment.arch {
            Arch::X86_64 => "x64",
            Arch::X86 => "ia32",
            Arch::Arm64 => "arm64",

            _ => "x64",
        }
        .to_string();

        let profile = request.profile.map(|profile| {
            let mr_data = profile.modrinth.map(|modrinth| LauncherProfileModrinth {
                id: modrinth.id,
                version_id: modrinth.version_id,
            });
            LauncherProfile {
                id: profile.id,
                name: profile.name,
                modrinth: mr_data,
            }
        });

        let body = PostLauncherLaunchRequest {
            hwid: Some(Uuid::nil().to_string()),
            hwid_private: Some(Uuid::nil().to_string()),
            installation_id: Uuid::new_v4().to_string(),
            os,
            os_release: String::from("0.0.0"),
            arch,
            launcher_version: Some(request.launcher_version),
            canary_preference: request.canary_preference.map(|cp| cp.json_name()),
            launch_type: Some(String::from("OFFLINE")),
            branch: request.branch,
            version: request.version,
            args: Some(Vec::new()),
            module: Some(request.module),
            profile,
        };

        let response = self.raw_client.post_launcher_launch(&body).await?;
        // parse response
        let jre_manifist = InstanceRuntimeManifest {
            extra_vm_options: response.jre.map_or(Vec::new(), |data| data.extra_arguments),
        };

        let launch_type_data = response.launch_type_data.ok_or(ApiError::Processing(
            "Field launch_type_data was None".to_string(),
        ))?;

        let artifacts = launch_type_data
            .artifacts
            .into_iter()
            .map(|artifact| {
                let artifact_type = match artifact.r#type.as_str() {
                    "CLASS_PATH" => ArtifactType::Classpath,
                    "EXTERNAL_FILE" => ArtifactType::ExternalFile,
                    "NATIVES" => ArtifactType::NativeLibrary,
                    "AGENT" | "JAVAAGENT" => ArtifactType::Javaagent,
                    _ => ArtifactType::Unknown,
                };
                Artifact {
                    name: artifact.name,
                    hash: Hash::Sha1(artifact.sha1),
                    url: artifact.url,
                    artifact_type,
                    size: Some(artifact.size as usize),
                    modify_time: Some(artifact.mtime),
                }
            })
            .collect();

        let textures = response
            .textures
            .map(|data| TexturesManifest {
                index_url: data.index_url,
                index_hash: Hash::Sha1(data.index_sha1),
                jit_index_url: data.jit_index_url,
                jit_index_hash: Hash::Sha1(data.jit_index_sha1),
                base_url: data.base_url,
            })
            .ok_or(ApiError::Processing(
                "Field textures was None on response".to_string(),
            ))?;

        let ui_manifist = response.ui.map(|ui| BrowserUiManifest {
            source_url: ui.source_url,
            source_hash: Hash::Sha1(ui.source_sha1),
            assets: BrowserUiAssets {
                base_url: ui.assets.base_url,
                index_url: ui.assets.index_url,
                index_hash: Hash::Sha1(ui.assets.index_sha1),
            },
        });

        let base_modpack = response.base_modpack.map(|modpack| {
            Ok(ModPackManifest {
                version: modpack.version,
                hash: Hash::Sha1(modpack.hash), // TODO: double check this: does this use sha1 as the hashing function?
                modrinth_pack_url: modpack.mrpack_url,
                publish_timestamp: NaiveDateTime::from_str(&modpack.published_at)
                    .map_err(|_| {
                        ApiError::Processing(
                            "Failed to parse timestamp at the base_modpack field".to_string(),
                        )
                    })?
                    .and_utc()
                    .timestamp(),
            })
        });

        // unwrap the result in a safe way
        let base_modpack = match base_modpack {
            Some(Err(err)) => {
                return Err(err);
            }
            Some(Ok(value)) => Some(value),
            None => None,
        };

        let result = LunarVersionManifest {
            main_class: launch_type_data.main_class,
            artifacts,
            textures,
            canary_token: response.canary_token,
            base_modpack,
            ui_manifest: ui_manifist,
            jre_manifest: jre_manifist,
        };
        Ok(result)
    }

    async fn list_available_lunar_versions(
        &self,
        branch: Option<&str>,
    ) -> ApiResult<LunarVersionsMetadata> {
        let response = self
            .raw_client
            .get_launcher_metadata_versions_lunar(
                Uuid::new_v4().to_string(),
                branch.map(|s| s.to_string()),
            )
            .await?;

        let versions = response
            .versions
            .into_iter()
            .flat_map(|version| version.subversions)
            .map(|version| LunarVersionMetadata {
                version: version.id,
                available_modules: version
                    .modules
                    .into_iter()
                    .map(|module| LunarModuleMetadata {
                        id: module.id,
                        name: module.name,
                        description: module.description,
                        loaders: module.loaders,
                    })
                    .collect(),
            })
            .collect();

        let result = LunarVersionsMetadata { versions };

        Ok(result)
    }
}
