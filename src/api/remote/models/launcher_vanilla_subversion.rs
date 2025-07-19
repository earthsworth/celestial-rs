// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};
use super::launcher_subversion_module::LauncherSubversionModule;

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct LauncherVanillaSubversion {
    /// Whether this is the default subversion within the parent
    pub default: bool,
    /// Identifier of the subversion
    pub id: String,
    /// Modules associated with this subversion
    pub modules: Vec<LauncherSubversionModule>,
}