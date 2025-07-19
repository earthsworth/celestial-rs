// Generated Code
// DO NOT EDIT MANUALLY

use async_trait::async_trait;
use reqwest::Method;
use super::core::RawApiClient;
use super::super::models::*;
use crate::error::ApiResult;

#[async_trait]
pub trait StoreApi {
    /// Removes a coupon, gift card or creator code from the basket
    async fn delete_store_basket_basket_ident_omnibox_remove_omni_method(&self, basket_ident: String, omni_method: String, body: &DeleteStoreBasketBasketIdentOmniboxRemoveOmniMethodRequest) -> ApiResult<DeleteStoreBasketBasketIdentOmniboxRemoveOmniMethodResponse>;
    /// Get the current basket info
    async fn get_store_basket_basket_ident_info(&self, basket_ident: String) -> ApiResult<GetStoreBasketBasketIdentInfoResponse>;
    /// Adds a coupon, gift card or creator code from the basket
    async fn post_store_basket_basket_ident_omnibox_add(&self, basket_ident: String, body: &PostStoreBasketBasketIdentOmniboxAddRequest) -> ApiResult<PostStoreBasketBasketIdentOmniboxAddResponse>;
    /// Add a package to the current basket
    async fn post_store_basket_basket_ident_package_add_package_id(&self, basket_ident: String, package_id: i64, body: &PostStoreBasketBasketIdentPackageAddPackageIdRequest) -> ApiResult<PostStoreBasketBasketIdentPackageAddPackageIdResponse>;
    /// Add a package to the current basket
    async fn post_store_basket_basket_ident_package_gift_package_id(&self, basket_ident: String, package_id: i64, body: &PostStoreBasketBasketIdentPackageGiftPackageIdRequest) -> ApiResult<PostStoreBasketBasketIdentPackageGiftPackageIdResponse>;
    /// Remove a package from the current basket
    async fn post_store_basket_basket_ident_package_remove_package_id(&self, basket_ident: String, package_id: i64, body: &PostStoreBasketBasketIdentPackageRemovePackageIdRequest) -> ApiResult<PostStoreBasketBasketIdentPackageRemovePackageIdResponse>;
    /// Create a new basket
    async fn post_store_basket_create(&self, authorization: Option<String>, body: &PostStoreBasketCreateRequest) -> ApiResult<PostStoreBasketCreateResponse>;
}

#[async_trait]
impl StoreApi for RawApiClient {
    async fn delete_store_basket_basket_ident_omnibox_remove_omni_method(&self, basket_ident: String, omni_method: String, body: &DeleteStoreBasketBasketIdentOmniboxRemoveOmniMethodRequest) -> ApiResult<DeleteStoreBasketBasketIdentOmniboxRemoveOmniMethodResponse> {
        let url = self.build_url(&format!("/store/basket/{}/omnibox/remove/{}", &basket_ident.to_string(), &omni_method.to_string()));
        let mut request = self.client.request(Method::DELETE, &url);
        request = request.json(body);
        self.send_request::<DeleteStoreBasketBasketIdentOmniboxRemoveOmniMethodResponse>(request).await
    }
    async fn get_store_basket_basket_ident_info(&self, basket_ident: String) -> ApiResult<GetStoreBasketBasketIdentInfoResponse> {
        let url = self.build_url(&format!("/store/basket/{}/info", &basket_ident.to_string()));
        let request = self.client.request(Method::GET, &url);
        
        self.send_request::<GetStoreBasketBasketIdentInfoResponse>(request).await
    }
    async fn post_store_basket_basket_ident_omnibox_add(&self, basket_ident: String, body: &PostStoreBasketBasketIdentOmniboxAddRequest) -> ApiResult<PostStoreBasketBasketIdentOmniboxAddResponse> {
        let url = self.build_url(&format!("/store/basket/{}/omnibox/add", &basket_ident.to_string()));
        let mut request = self.client.request(Method::POST, &url);
        request = request.json(body);
        self.send_request::<PostStoreBasketBasketIdentOmniboxAddResponse>(request).await
    }
    async fn post_store_basket_basket_ident_package_add_package_id(&self, basket_ident: String, package_id: i64, body: &PostStoreBasketBasketIdentPackageAddPackageIdRequest) -> ApiResult<PostStoreBasketBasketIdentPackageAddPackageIdResponse> {
        let url = self.build_url(&format!("/store/basket/{}/package/add/{}", &basket_ident.to_string(), &package_id.to_string()));
        let mut request = self.client.request(Method::POST, &url);
        request = request.json(body);
        self.send_request::<PostStoreBasketBasketIdentPackageAddPackageIdResponse>(request).await
    }
    async fn post_store_basket_basket_ident_package_gift_package_id(&self, basket_ident: String, package_id: i64, body: &PostStoreBasketBasketIdentPackageGiftPackageIdRequest) -> ApiResult<PostStoreBasketBasketIdentPackageGiftPackageIdResponse> {
        let url = self.build_url(&format!("/store/basket/{}/package/gift/{}", &basket_ident.to_string(), &package_id.to_string()));
        let mut request = self.client.request(Method::POST, &url);
        request = request.json(body);
        self.send_request::<PostStoreBasketBasketIdentPackageGiftPackageIdResponse>(request).await
    }
    async fn post_store_basket_basket_ident_package_remove_package_id(&self, basket_ident: String, package_id: i64, body: &PostStoreBasketBasketIdentPackageRemovePackageIdRequest) -> ApiResult<PostStoreBasketBasketIdentPackageRemovePackageIdResponse> {
        let url = self.build_url(&format!("/store/basket/{}/package/remove/{}", &basket_ident.to_string(), &package_id.to_string()));
        let mut request = self.client.request(Method::POST, &url);
        request = request.json(body);
        self.send_request::<PostStoreBasketBasketIdentPackageRemovePackageIdResponse>(request).await
    }
    async fn post_store_basket_create(&self, authorization: Option<String>, body: &PostStoreBasketCreateRequest) -> ApiResult<PostStoreBasketCreateResponse> {
        let url = self.build_url("/store/basket/create");
        let mut request = self.client.request(Method::POST, &url);
        if let Some(value) = authorization { request = request.header("Authorization", value); }
        request = request.json(body);
        self.send_request::<PostStoreBasketCreateResponse>(request).await
    }
}
