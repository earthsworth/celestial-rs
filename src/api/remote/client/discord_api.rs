// Generated Code
// DO NOT EDIT MANUALLY

use async_trait::async_trait;
use reqwest::Method;
use super::core::RawApiClient;
use super::super::models::*;
use crate::error::ApiResult;

#[async_trait]
pub trait DiscordApi {
    /// Gets the pick of the week cosmetics. Changes every request
    async fn get_discord_potw_get_cosmetic_pool(&self) -> ApiResult<GetDiscordPotwGetCosmeticPoolResponse>;
    /// Triggers a request to the Discord bots to update the user's information
    async fn post_discord_trigger_update(&self, body: &PostDiscordTriggerUpdateRequest) -> ApiResult<PostDiscordTriggerUpdateResponse>;
    /// Updates the membership information for a user in the Database
    async fn post_discord_update_membership(&self, body: &PostDiscordUpdateMembershipRequest) -> ApiResult<PostDiscordUpdateMembershipResponse>;
    /// Links a Discord profile
    async fn post_discord_verification(&self, authorization: String, body: &PostDiscordVerificationRequest) -> ApiResult<PostDiscordVerificationResponse>;
}

#[async_trait]
impl DiscordApi for RawApiClient {
    async fn get_discord_potw_get_cosmetic_pool(&self) -> ApiResult<GetDiscordPotwGetCosmeticPoolResponse> {
        let url = self.build_url("/discord/potw/get-cosmetic-pool");
        let request = self.client.request(Method::GET, &url);
        
        self.send_request::<GetDiscordPotwGetCosmeticPoolResponse>(request).await
    }
    async fn post_discord_trigger_update(&self, body: &PostDiscordTriggerUpdateRequest) -> ApiResult<PostDiscordTriggerUpdateResponse> {
        let url = self.build_url("/discord/trigger-update");
        let mut request = self.client.request(Method::POST, &url);
        request = request.json(body);
        self.send_request::<PostDiscordTriggerUpdateResponse>(request).await
    }
    async fn post_discord_update_membership(&self, body: &PostDiscordUpdateMembershipRequest) -> ApiResult<PostDiscordUpdateMembershipResponse> {
        let url = self.build_url("/discord/update-membership");
        let mut request = self.client.request(Method::POST, &url);
        request = request.json(body);
        self.send_request::<PostDiscordUpdateMembershipResponse>(request).await
    }
    async fn post_discord_verification(&self, authorization: String, body: &PostDiscordVerificationRequest) -> ApiResult<PostDiscordVerificationResponse> {
        let url = self.build_url("/discord/verification");
        let mut request = self.client.request(Method::POST, &url);
        request = request.header("Authorization", authorization);
        request = request.json(body);
        self.send_request::<PostDiscordVerificationResponse>(request).await
    }
}
