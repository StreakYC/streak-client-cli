use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct ApiKeysClient {
    pub http_client: HttpClient,
}

impl ApiKeysClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Lists API keys owned by the current user
    ///
    /// # Arguments
    ///
    /// * `limit` - Number of items to return.
    /// * `page` - Zero-based page number
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use streak_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = StreakClient::new(config).expect("Failed to build client");
    ///     client
    ///         .api_keys
    ///         .get_api_keys(
    ///             &GetAPIKeysQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_api_keys(
        &self,
        request: &GetApiKeysQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<Vec<ApiKey>, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1/apikeys",
                None,
                QueryBuilder::new()
                    .int("limit", request.limit.clone())
                    .int("page", request.page.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Creates an API key for the current user
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use streak_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = StreakClient::new(config).expect("Failed to build client");
    ///     client
    ///         .api_keys
    ///         .create_api_key(
    ///             &CreateAPIKeyQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_api_key(
        &self,
        request: &CreateApiKeyQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ApiKey, ApiError> {
        self.http_client
            .execute_request(
                Method::PUT,
                "v1/apikeys",
                None,
                QueryBuilder::new()
                    .serialize("teamKey", request.team_key.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Deletes an API key by key
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use streak_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = StreakClient::new(config).expect("Failed to build client");
    ///     client
    ///         .api_keys
    ///         .delete_api_key(&"apiKeyKey".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn delete_api_key(
        &self,
        api_key_key: &str,
        options: Option<RequestOptions>,
    ) -> Result<OperationResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("v1/apikeys/{}", api_key_key),
                None,
                None,
                options,
            )
            .await
    }
}
