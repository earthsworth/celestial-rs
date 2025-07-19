// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};
use super::launcher_radio_premium_package::LauncherRadioPremiumPackage;
use super::launcher_radio_station::LauncherRadioStation;

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct GetLauncherMetadataRadioResponse {
    /// List of available premium packages for the user
    #[serde(rename = "premiumPackages")]
    pub premium_packages: Vec<LauncherRadioPremiumPackage>,
    /// List of available radio stations for the user
    pub stations: Vec<LauncherRadioStation>,
}