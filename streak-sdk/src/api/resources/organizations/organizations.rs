use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;
use std::collections::HashMap;

pub struct OrganizationsClient {
    pub http_client: HttpClient,
}

impl OrganizationsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Returns organizations keyed by their organization keys. Missing and inaccessible organizations are omitted.
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
    ///         .organizations
    ///         .get_organizations(&vec!["string".to_string()], None)
    ///         .await;
    /// }
    /// ```
    pub async fn get_organizations(
        &self,
        request: &Vec<String>,
        options: Option<RequestOptions>,
    ) -> Result<HashMap<String, Organization>, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v2/organizations/batch/get",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Returns an organization accessible to the current user, including custom fields and relationships.
    ///
    /// # Arguments
    ///
    /// * `organization_key` - Key of the organization.
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
    ///         .organizations
    ///         .get_organization(&"organizationKey".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn get_organization(
        &self,
        organization_key: &str,
        options: Option<RequestOptions>,
    ) -> Result<Organization, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v2/organizations/{}", organization_key),
                None,
                None,
                options,
            )
            .await
    }

    /// Updates an organization. Omitted and null properties are unchanged. Supplied lists replace existing lists, subject to field validation.
    ///
    /// # Arguments
    ///
    /// * `organization_key` - Key of the organization.
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
    ///         .organizations
    ///         .update_organization(
    ///             &"organizationKey".to_string(),
    ///             &OrganizationUpdateBody {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update_organization(
        &self,
        organization_key: &str,
        request: &OrganizationUpdateBody,
        options: Option<RequestOptions>,
    ) -> Result<Organization, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v2/organizations/{}", organization_key),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Deletes an organization and schedules removal of its relationships and box links.
    ///
    /// # Arguments
    ///
    /// * `organization_key` - Key of the organization.
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
    ///         .organizations
    ///         .delete_organization(&"organizationKey".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn delete_organization(
        &self,
        organization_key: &str,
        options: Option<RequestOptions>,
    ) -> Result<OperationResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("v2/organizations/{}", organization_key),
                None,
                None,
                options,
            )
            .await
    }

    /// Returns organizations in a team, ordered by key. Pass the returned cursor to retrieve the next page.
    ///
    /// # Arguments
    ///
    /// * `team_key` - Key of the team.
    /// * `cursor` - Cursor returned by the previous response. Omit or leave blank to start from the beginning.
    /// * `limit` - Maximum number of results to return. Defaults to 100. Values above 1000 are capped.
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
    ///         .organizations
    ///         .list_organizations(
    ///             &"teamKey".to_string(),
    ///             &ListOrganizationsQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_organizations(
        &self,
        team_key: &str,
        request: &ListOrganizationsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<OrganizationPage, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v2/teams/{}/organizations", team_key),
                None,
                QueryBuilder::new()
                    .string("cursor", request.cursor.clone())
                    .int("limit", request.limit.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Creates an organization in a team. Available enrichment data may fill missing details. Provide a name or at least one domain. Set custom fields and relationships in a subsequent update.
    ///
    /// # Arguments
    ///
    /// * `team_key` - Key of the team.
    /// * `get_if_existing` - Return an existing organization matching a supplied domain instead of creating another organization. The supplied values are not merged into the existing organization.
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
    ///         .organizations
    ///         .create_organization(
    ///             &"teamKey".to_string(),
    ///             &OrganizationCreateBody {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_organization(
        &self,
        team_key: &str,
        request: &OrganizationCreateBody,
        options: Option<RequestOptions>,
    ) -> Result<Organization, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v2/teams/{}/organizations", team_key),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                QueryBuilder::new()
                    .bool("getIfExisting", request.get_if_existing.clone())
                    .build(),
                options,
            )
            .await
    }
}
