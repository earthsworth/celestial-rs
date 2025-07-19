// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};
use super::ad::Ad;
use super::feature_flag::FeatureFlag;
use super::launcher_alert::LauncherAlert;
use super::launcher_blog_post::LauncherBlogPost;
use super::launcher_error_solution::LauncherErrorSolution;
use super::launcher_nav_item::LauncherNavItem;
use super::launcher_refresh_intervals::LauncherRefreshIntervals;
use super::launcher_servers_cta::LauncherServersCta;
use super::launcher_updater::LauncherUpdater;
use super::launcher_vanilla_version::LauncherVanillaVersion;
use super::launcher_version::LauncherVersion;
use super::metadata_shelf_server::MetadataShelfServer;
use super::online_players::OnlinePlayers;
use super::recommended_library::RecommendedLibrary;
use super::sentry_filter::SentryFilter;
use super::topbar_social::TopbarSocial;
use super::video_promotion::VideoPromotion;

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct GetLauncherMetadataResponse {
    /// Advertisements
    pub ads: Vec<Ad>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alert: Option<LauncherAlert>,
    /// Recent news to be shown on the home page
    #[serde(rename = "blogPosts")]
    pub blog_posts: Vec<LauncherBlogPost>,
    /// Whether the internal branch should be reset to master (this happens when they try get the metadata for a branch they are not authorised to get)
    #[serde(rename = "branchReset")]
    pub branch_reset: bool,
    /// Common launch error solutions
    #[serde(rename = "errorSolutions")]
    pub error_solutions: Vec<LauncherErrorSolution>,
    /// Feature Flags that should be applied to the current session
    #[serde(rename = "featureFlags")]
    pub feature_flags: Vec<FeatureFlag>,
    /// Modpacks that are immediately available for the user to install on the versions page
    pub modpacks: Vec<String>,
    /// Additional Sidebar Items
    #[serde(rename = "navItems")]
    pub nav_items: Vec<LauncherNavItem>,
    #[serde(rename = "onlinePlayers")]
    pub online_players: OnlinePlayers,
    /// Libraries and their versions that are recommended
    #[serde(rename = "recommendedLibraries")]
    pub recommended_libraries: Vec<RecommendedLibrary>,
    #[serde(rename = "refreshIntervals")]
    pub refresh_intervals: LauncherRefreshIntervals,
    /// List of exceptions that are filtered client-side before sending
    #[serde(rename = "sentryFilteredExceptions")]
    pub sentry_filtered_exceptions: Vec<SentryFilter>,
    /// Quick join servers (partnered)
    pub servers: Vec<MetadataShelfServer>,
    #[serde(rename = "serversCta")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub servers_cta: Option<LauncherServersCta>,
    /// Seasonal theme that Launcher should respect
    #[serde(skip_serializing_if = "Option::is_none")]
    pub theme: Option<String>,
    /// Socials displayed in the launcher topbar
    #[serde(rename = "topbarSocials")]
    pub topbar_socials: Vec<TopbarSocial>,
    pub updater: LauncherUpdater,
    /// Playable versions without Lunar modifications
    #[serde(rename = "vanillaVersions")]
    pub vanilla_versions: Vec<LauncherVanillaVersion>,
    /// Playable versions for the user
    pub versions: Vec<LauncherVersion>,
    #[serde(rename = "videoPromotion")]
    pub video_promotion: VideoPromotion,
}