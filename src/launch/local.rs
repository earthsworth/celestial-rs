use crate::{
    api::{
        launch::{LunarRemoteMetadata, LunarVersionManifest},
        remote::get_and_verify_hash,
    },
    utils::hashing::Hash,
};

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum LocalManifest {
    Lunar {
        /// The upstream of this manifest
        remote_metdata: Option<LunarRemoteMetadata>,
        /// A flag to control the auto update feature
        locked: bool, // false => roll update, true => lock on current version
        resources: Vec<Resource>, // file_hash: file_path

        addons: Vec<Addon>,
    },
    // TODO: vanilla version
}

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Resource {
    /// The relative path to link the file to.
    /// if is_archive is true, this will be the extract dir
    pub file_relative_path: String,
    /// The hash of the resource
    pub file_hash: Hash,
    /// A remote url tracks this run
    pub remote_url: Option<String>,
    /// Is this file an archive file? Like .zip, .tar.gz and .7z
    /// Celestial will create extern resources from the archive
    pub is_archive: bool,
    /// reference to an archive file contains this resource
    pub from_archive: Option<Box<Resource>>,
}

impl LocalManifest {
    pub async fn from_lunar_manifest(
        client: &reqwest::Client,
        remote_metadata: Option<LunarRemoteMetadata>,
        manifest: LunarVersionManifest,
    ) -> anyhow::Result<Self> {
        let mut resources = Vec::new();
        // add artifacts
        let artifact_files = manifest.artifacts.iter().cloned().map(|artifact| Resource {
            file_hash: artifact.hash,
            file_relative_path: artifact.name,
            remote_url: Some(artifact.url),
            is_archive: false,
            from_archive: None,
        });
        resources.extend(artifact_files);

        // fetch textures
        let textures_index = get_and_verify_hash(
            client,
            &manifest.textures.index_url,
            &manifest.textures.index_hash,
        )
        .await?;
        let textures_index = std::str::from_utf8(&textures_index)?.lines();
        let textures = parse_index(textures_index, "textures/", &manifest.textures.base_url);
        // add textures
        resources.extend(textures);

        // NOTE: jit index is unused in the Typescript + Electron implementation, so we cannot know the behavier of
        // this stuff

        // add ui files
        if let Some(ui_manifest) = manifest.ui_manifest {
            let ui_source_resource = Resource {
                file_relative_path: "ui/".to_string(), // TODO: extract the "ui/" to a val, hard encoding is bad
                file_hash: ui_manifest.source_hash,
                remote_url: Some(ui_manifest.source_url),
                is_archive: true,
                from_archive: None,
            };
            resources.push(ui_source_resource);

            // add ui assets
            let ui_assets_manifest = ui_manifest.assets;
            let ui_assets_index = get_and_verify_hash(
                client,
                &ui_assets_manifest.index_url,
                &ui_assets_manifest.index_hash,
            )
            .await?;
            let ui_assets_index = std::str::from_utf8(&ui_assets_index)?.lines();

            let ui_assets = parse_index(ui_assets_index, "ui/assets", &ui_assets_manifest.base_url);
            // add ui_assets to resources vec
            resources.extend(ui_assets);
        }

        // PLEASE NOTICE THAT: ui and natives files should added when Celestial extract them, we cannot get
        // the file map via the api.
        Ok(Self::Lunar {
            remote_metdata: remote_metadata,
            locked: true,
            resources,
            addons: Vec::new(),
        })
    }
}

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Addon {
    pub enabled: bool,
    pub addon_name: String,
    pub addon_hash: Hash,
    pub r#type: AddonType,
}

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum AddonType {
    Javaagent,
    WeaveMod,
    FabricMod,
}

fn parse_index(
    index: std::str::Lines<'_>,
    base_dir: &str,
    base_url: &str,
) -> impl Iterator<Item = Resource> {
    index.map(|line| line.splitn(4, " ")).map(move |split| {
        let mut split = split;
        let (file_path, hash) = (split.next().unwrap(), split.next().unwrap());
        Resource {
            file_relative_path: format!("{base_dir}/{file_path}"),
            file_hash: Hash::Sha1(hash.to_string()),
            remote_url: Some(format!("{}{}", base_url, hash)),
            is_archive: false,
            from_archive: None,
        }
    })
}

#[cfg(test)]
mod tests {
    use crate::{
        api::launch::{CanaryPreference, LaunchExt},
        launch::local::LocalManifest,
    };

    #[tokio::test]
    async fn it_works() {
        let client = reqwest::Client::new();
        let api_client = crate::api::remote::client::RemoteApiClient::new(
            &client,
            "https://api.lunarclientprod.com",
        );
        let manifest = api_client
            .launch_lunar(crate::api::launch::LaunchLunarRequest {
                environment: crate::environment::SystemEnvironment::from_current_env(),
                launcher_version: "10.0.0".to_string(),
                canary_preference: Some(CanaryPreference::OptIn),
                branch: "master".to_string(),
                version: "1.8.9".to_string(),
                module: "lunar".to_string(),
                profile: None,
            })
            .await
            .unwrap();
        // create local manifest
        let local_manifest = LocalManifest::from_lunar_manifest(&client, None, manifest).await;
        dbg!(&local_manifest);
    }
}
