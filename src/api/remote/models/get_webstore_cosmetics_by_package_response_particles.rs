// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};

/// The particles of the package.
#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct GetWebstoreCosmeticsByPackageResponseParticles {
    /// Morphs mapping
    pub morphs: serde_json::Value,
    /// Particle schemes mapping
    pub schemes: serde_json::Value,
    /// Particle textures mapping
    pub textures: serde_json::Value,
}