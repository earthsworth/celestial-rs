// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};
use super::feature_flag::FeatureFlag;
use super::get_launcher_metadata_misc_response_language::GetLauncherMetadataMiscResponseLanguage;
use super::get_launcher_metadata_misc_response_terms_item::GetLauncherMetadataMiscResponseTermsItem;
use super::launcher_error_solution::LauncherErrorSolution;
use super::launcher_refresh_intervals::LauncherRefreshIntervals;
use super::launcher_updater::LauncherUpdater;
use super::online_players::OnlinePlayers;

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct GetLauncherMetadataMiscResponse {
    /// Common launch error solutions
    #[serde(rename = "errorSolutions")]
    pub error_solutions: Vec<LauncherErrorSolution>,
    /// Feature Flags that should be applied to the current session
    #[serde(rename = "featureFlags")]
    pub feature_flags: Vec<FeatureFlag>,
    /// Language information
    pub language: GetLauncherMetadataMiscResponseLanguage,
    /// Modpacks that are immediately available for the user to install on the versions page
    pub modpacks: Vec<String>,
    #[serde(rename = "onlinePlayers")]
    pub online_players: OnlinePlayers,
    #[serde(rename = "refreshIntervals")]
    pub refresh_intervals: LauncherRefreshIntervals,
    /// List of domains that are allowed to be used for SSO authentication
    #[serde(rename = "ssoDomains")]
    pub sso_domains: Vec<String>,
    /// Terms information
    pub terms: Vec<GetLauncherMetadataMiscResponseTermsItem>,
    pub updater: LauncherUpdater,
}