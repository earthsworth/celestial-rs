use std::collections::HashMap;

use crate::{
    api::launch::{LunarRemoteMetadata, LunarVersionManifist},
    hashing::Hash,
};

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum LocalManifist {
    Lunar {
        remote_metdata: Option<LunarRemoteMetadata>,
        remote_manifist: Option<LunarVersionManifist>,
        locked: bool, // false => roll update, true => lock on current version
        file_link_map: HashMap<String, Hash>, // file_hash: file_path

        addons: Vec<Addon>,
    },
    // TODO: vanilla version
}

impl LocalManifist {
    pub fn parse_from_lunar_manifist(remote_metadata: Option<LunarRemoteMetadata>, manifist: LunarVersionManifist) -> Self {
        let mut file_link_map = HashMap::new();
        // add artifacts
        let artifact_files = manifist.artifacts.iter().cloned().map(|artifact| (artifact.name, artifact.hash));
        file_link_map.extend(artifact_files);

        // add textures
        // fetch textures
        
        // TODO: add ui files

        // PLEASE NOTICE THAT: ui and natives files should added when Celestial extract them, we cannot get
        // the file map via the api.
        Self::Lunar { remote_metdata: remote_metadata, remote_manifist: Some(manifist), locked: true, file_link_map, addons: Vec::new() }
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
