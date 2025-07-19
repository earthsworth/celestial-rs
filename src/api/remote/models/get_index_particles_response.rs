// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};

/// Particles response
#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct GetIndexParticlesResponse {
    /// Morphs mapping
    pub morphs: serde_json::Value,
    /// Particle schemes mapping
    pub schemes: serde_json::Value,
    /// Particle textures mapping
    pub textures: serde_json::Value,
}