// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};
use super::feature_flag::FeatureFlag;
use super::get_game_metadata_response_alert::GetGameMetadataResponseAlert;
use super::get_game_metadata_response_blog_posts_item::GetGameMetadataResponseBlogPostsItem;
use super::get_game_metadata_response_lang_override::GetGameMetadataResponseLangOverride;
use super::get_game_metadata_response_links::GetGameMetadataResponseLinks;
use super::get_game_metadata_response_pinned_servers_item::GetGameMetadataResponsePinnedServersItem;
use super::get_game_metadata_response_server_integration_item::GetGameMetadataResponseServerIntegrationItem;
use super::get_game_metadata_response_star_servers_item::GetGameMetadataResponseStarServersItem;
use super::get_game_metadata_response_store::GetGameMetadataResponseStore;
use super::sentry_filter::SentryFilter;

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct GetGameMetadataResponse {
    pub alert: GetGameMetadataResponseAlert,
    /// Blog posts that should be show on the home screen
    #[serde(rename = "blogPosts")]
    pub blog_posts: Vec<GetGameMetadataResponseBlogPostsItem>,
    #[serde(rename = "clientSettings")]
    pub client_settings: serde_json::Value,
    /// Active feature flags
    #[serde(rename = "featureFlag")]
    pub feature_flag: Vec<FeatureFlag>,
    /// Dummy data for language overrides
    #[serde(rename = "langOverride")]
    pub lang_override: GetGameMetadataResponseLangOverride,
    /// Links to override in-game
    pub links: GetGameMetadataResponseLinks,
    #[serde(rename = "modSettings")]
    pub mod_settings: serde_json::Value,
    /// Pinned servers on the multiplayer list
    #[serde(rename = "pinnedServers")]
    pub pinned_servers: Vec<GetGameMetadataResponsePinnedServersItem>,
    /// Exceptions to filter before sending to Sentry
    #[serde(rename = "sentryFilteredExceptions")]
    pub sentry_filtered_exceptions: Vec<SentryFilter>,
    /// Server-specific mod/settings overrides
    #[serde(rename = "serverIntegration")]
    pub server_integration: Vec<GetGameMetadataResponseServerIntegrationItem>,
    /// Servers with a star on the multiplayer list
    #[serde(rename = "starServers")]
    pub star_servers: Vec<GetGameMetadataResponseStarServersItem>,
    /// Dummy data for store
    pub store: GetGameMetadataResponseStore,
}