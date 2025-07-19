// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct LauncherErrorSolution {
    /// Description of the solution
    pub description: String,
    /// List of errors to look for in stderr
    pub errors: Vec<String>,
    /// Unique ID for the solution
    pub id: String,
    /// Title of the solution
    pub title: String,
}