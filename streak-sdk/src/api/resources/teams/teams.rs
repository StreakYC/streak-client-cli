use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct TeamsClient {
    pub http_client: HttpClient,
}

impl TeamsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Creates a team with an initial member roster.
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
    ///         .teams
    ///         .create_team(
    ///             &TeamCreateBody {
    ///                 name: "name".to_string(),
    ///                 members: vec![TeamMemberCreate {
    ///                     ..Default::default()
    ///                 }],
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_team(
        &self,
        request: &TeamCreateBody,
        options: Option<RequestOptions>,
    ) -> Result<Team, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v2/teams",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Returns a team by key.
    ///
    /// # Arguments
    ///
    /// * `team_key` - Key for the team to return.
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
    ///     client.teams.get_team(&"teamKey".to_string(), None).await;
    /// }
    /// ```
    pub async fn get_team(
        &self,
        team_key: &str,
        options: Option<RequestOptions>,
    ) -> Result<Team, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v2/teams/{}", team_key),
                None,
                None,
                options,
            )
            .await
    }

    /// Updates a team by key. Omitted body fields are left unchanged.
    ///
    /// # Arguments
    ///
    /// * `team_key` - Key for the team to update.
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
    ///         .teams
    ///         .update_team(
    ///             &"teamKey".to_string(),
    ///             &TeamUpdateBody {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update_team(
        &self,
        team_key: &str,
        request: &TeamUpdateBody,
        options: Option<RequestOptions>,
    ) -> Result<Team, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v2/teams/{}", team_key),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Lists the teams the current user belongs to, newest first.
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
    ///         .teams
    ///         .get_current_user_teams(
    ///             &GetCurrentUserTeamsQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_current_user_teams(
        &self,
        request: &GetCurrentUserTeamsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<TeamListResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v2/users/me/teams",
                None,
                QueryBuilder::new()
                    .int("limit", request.limit.clone())
                    .int("page", request.page.clone())
                    .build(),
                options,
            )
            .await
    }
}
