use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct UsersClient {
    pub http_client: HttpClient,
}

impl UsersClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Returns information about the currently authenticated user
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
    ///     client.users.get_current_user(None).await;
    /// }
    /// ```
    pub async fn get_current_user(
        &self,
        options: Option<RequestOptions>,
    ) -> Result<User, ApiError> {
        self.http_client
            .execute_request(Method::GET, "v1/users/me", None, None, options)
            .await
    }

    /// Returns information about a specific user by their key
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
    ///     client.users.get_user(&"userKey".to_string(), None).await;
    /// }
    /// ```
    pub async fn get_user(
        &self,
        user_key: &str,
        options: Option<RequestOptions>,
    ) -> Result<User, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/users/{}", user_key),
                None,
                None,
                options,
            )
            .await
    }

    /// Updates a specific user by their key
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
    ///         .users
    ///         .update_user(
    ///             &"userKey".to_string(),
    ///             &UserUpdateBody {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update_user(
        &self,
        user_key: &str,
        request: &UserUpdateBody,
        options: Option<RequestOptions>,
    ) -> Result<User, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v1/users/{}", user_key),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Returns all the users on a single team including archived users, payer only users, agents, owners and regular members.
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
    ///         .users
    ///         .get_users_on_team(&"teamKey".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn get_users_on_team(
        &self,
        team_key: &str,
        options: Option<RequestOptions>,
    ) -> Result<UserListResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v2/teams/{}/users", team_key),
                None,
                None,
                options,
            )
            .await
    }
}
