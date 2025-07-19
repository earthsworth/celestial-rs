// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};

/// Package
#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct GetWebstorePackagesResponsePackagesItem {
    /// Package ID
    pub id: f64,
    /// Image URL of this package's icon
    pub image: String,
    /// Ranking of this package (if ranked in the last 14 days)
    pub ranking: f64,
}