// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct GetApolloUpdatesResponseAssets {
    /// URL to download the latest bukkit artifact
    pub bukkit: String,
    /// URL to download the latest bungee artifact
    pub bungee: String,
    /// URL to download the latest folia artifact
    pub folia: String,
    /// URL to download the latest sponge artifact
    pub sponge: String,
    /// URL to download the latest velocity artifact
    pub velocity: String,
}