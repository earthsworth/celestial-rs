// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};
use super::ad::Ad;
use super::launcher_alert::LauncherAlert;
use super::launcher_blog_post::LauncherBlogPost;
use super::launcher_nav_item::LauncherNavItem;
use super::launcher_servers_cta::LauncherServersCta;
use super::metadata_shelf_server::MetadataShelfServer;
use super::topbar_social::TopbarSocial;
use super::video_promotion::VideoPromotion;

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct GetLauncherMetadataLayoutResponse {
    /// Advertisements
    pub ads: Vec<Ad>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alert: Option<LauncherAlert>,
    /// Recent news to be shown on the home page
    #[serde(rename = "blogPosts")]
    pub blog_posts: Vec<LauncherBlogPost>,
    /// Additional Sidebar Items
    #[serde(rename = "navItems")]
    pub nav_items: Vec<LauncherNavItem>,
    #[serde(rename = "serversCta")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub servers_cta: Option<LauncherServersCta>,
    /// Quick join servers (partnered)
    #[serde(rename = "shelfServers")]
    pub shelf_servers: Vec<MetadataShelfServer>,
    /// Socials displayed in the launcher topbar
    #[serde(rename = "topbarSocials")]
    pub topbar_socials: Vec<TopbarSocial>,
    #[serde(rename = "videoPromotion")]
    pub video_promotion: VideoPromotion,
}