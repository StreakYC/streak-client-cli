//! Service clients and API endpoints
//!
//! This module contains client implementations for:
//!
//! - **API Keys**
//! - **Boxes**
//! - **Pipelines**
//! - **Users**
//! - **Comments**
//! - **Meetings**
//! - **Tasks**
//! - **Contacts**
//! - **Organizations**
//! - **Pipeline Stages**
//! - **Teams**

use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub mod api_keys;
pub mod boxes;
pub mod comments;
pub mod contacts;
pub mod meetings;
pub mod organizations;
pub mod pipeline_stages;
pub mod pipelines;
pub mod tasks;
pub mod teams;
pub mod users;
pub struct ApiClient {
    pub config: ClientConfig,
    pub http_client: HttpClient,
    pub api_keys: ApiKeysClient,
    pub boxes: BoxesClient,
    pub pipelines: PipelinesClient,
    pub users: UsersClient,
    pub comments: CommentsClient,
    pub meetings: MeetingsClient,
    pub tasks: TasksClient,
    pub contacts: ContactsClient,
    pub organizations: OrganizationsClient,
    pub pipeline_stages: PipelineStagesClient,
    pub teams: TeamsClient,
}

impl ApiClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            config: config.clone(),
            http_client: HttpClient::new(config.clone())?,
            api_keys: ApiKeysClient::new(config.clone())?,
            boxes: BoxesClient::new(config.clone())?,
            pipelines: PipelinesClient::new(config.clone())?,
            users: UsersClient::new(config.clone())?,
            comments: CommentsClient::new(config.clone())?,
            meetings: MeetingsClient::new(config.clone())?,
            tasks: TasksClient::new(config.clone())?,
            contacts: ContactsClient::new(config.clone())?,
            organizations: OrganizationsClient::new(config.clone())?,
            pipeline_stages: PipelineStagesClient::new(config.clone())?,
            teams: TeamsClient::new(config.clone())?,
        })
    }

    /// Searches visible boxes, contacts, and organizations by query, or visible boxes by exact name. Exactly one of query or name must be provided.
    ///
    /// # Arguments
    ///
    /// * `name` - Exact box name to search for.
    /// * `page` - Zero-based page number.
    /// * `pipeline_key` - Pipeline keys to constrain the search to.
    /// * `query` - Full-text query to search across boxes, contacts, and organizations.
    /// * `stage_key` - Stage keys to constrain the search to.
    /// * `team_key` - Team keys to constrain the search to.
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
    ///         .search(
    ///             &SearchQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn search(
        &self,
        request: &SearchQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<SearchResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1/search",
                None,
                QueryBuilder::new()
                    .serialize("name", request.name.clone())
                    .int("page", request.page.clone())
                    .serialize("pipelineKey", request.pipeline_key.clone())
                    .structured_query("query", request.query.clone())
                    .serialize("stageKey", request.stage_key.clone())
                    .serialize("teamKey", request.team_key.clone())
                    .build(),
                options,
            )
            .await
    }
}

pub use api_keys::ApiKeysClient;
pub use boxes::BoxesClient;
pub use comments::CommentsClient;
pub use contacts::ContactsClient;
pub use meetings::MeetingsClient;
pub use organizations::OrganizationsClient;
pub use pipeline_stages::PipelineStagesClient;
pub use pipelines::PipelinesClient;
pub use tasks::TasksClient;
pub use teams::TeamsClient;
pub use users::UsersClient;
