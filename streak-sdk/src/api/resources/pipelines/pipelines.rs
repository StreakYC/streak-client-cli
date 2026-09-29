use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct PipelinesClient {
    pub http_client: HttpClient,
}

impl PipelinesClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Returns the fields in a pipeline in their stored order.
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
    ///         .pipelines
    ///         .list_fields(&"pipelineKey".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn list_fields(
        &self,
        pipeline_key: &str,
        options: Option<RequestOptions>,
    ) -> Result<Vec<PipelineField>, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/pipelines/{}/fields", pipeline_key),
                None,
                None,
                options,
            )
            .await
    }

    /// Returns a field by its key within a pipeline.
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
    ///         .pipelines
    ///         .get_field(&"pipelineKey".to_string(), &"fieldKey".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn get_field(
        &self,
        pipeline_key: &str,
        field_key: &str,
        options: Option<RequestOptions>,
    ) -> Result<PipelineField, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/pipelines/{}/fields/{}", pipeline_key, field_key),
                None,
                None,
                options,
            )
            .await
    }

    /// Updates a field's name, default value, options, or AI settings.
    ///
    /// # Arguments
    ///
    /// * `add_only` - Append dropdown or tag options while retaining existing options. Defaults to false.
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
    ///         .pipelines
    ///         .update_field(
    ///             &"pipelineKey".to_string(),
    ///             &"fieldKey".to_string(),
    ///             &PipelineFieldUpdate {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update_field(
        &self,
        pipeline_key: &str,
        field_key: &str,
        request: &PipelineFieldUpdate,
        options: Option<RequestOptions>,
    ) -> Result<PipelineField, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v1/pipelines/{}/fields/{}", pipeline_key, field_key),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                QueryBuilder::new()
                    .bool("addOnly", request.add_only.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Deletes a custom field and clears the value from all the pipeline's boxes.
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
    ///         .pipelines
    ///         .delete_field(&"pipelineKey".to_string(), &"fieldKey".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn delete_field(
        &self,
        pipeline_key: &str,
        field_key: &str,
        options: Option<RequestOptions>,
    ) -> Result<OperationResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("v1/pipelines/{}/fields/{}", pipeline_key, field_key),
                None,
                None,
                options,
            )
            .await
    }

    /// Lists pipelines accessible to the current user.
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
    ///         .pipelines
    ///         .list_pipelines(
    ///             &ListPipelinesQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_pipelines(
        &self,
        request: &ListPipelinesQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<PipelineListResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v2/pipelines",
                None,
                QueryBuilder::new()
                    .serialize("sortBy", request.sort_by.clone())
                    .int("limit", request.limit.clone())
                    .int("page", request.page.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Creates a basic pipeline.
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
    ///         .pipelines
    ///         .create_pipeline(
    ///             &PipelineCreate {
    ///                 name: "name".to_string(),
    ///                 team_key: "teamKey".to_string(),
    ///                 stages: vec!["stages".to_string()],
    ///                 icon: None,
    ///                 description: None,
    ///                 template_type: None,
    ///                 fields: None,
    ///                 default_permission_set_name: None,
    ///                 sharing_restricted_to_team: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_pipeline(
        &self,
        request: &PipelineCreate,
        options: Option<RequestOptions>,
    ) -> Result<Pipeline, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v2/pipelines",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Returns a pipeline by key.
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
    ///         .pipelines
    ///         .get_pipeline(&"pipelineKey".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn get_pipeline(
        &self,
        pipeline_key: &str,
        options: Option<RequestOptions>,
    ) -> Result<Pipeline, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v2/pipelines/{}", pipeline_key),
                None,
                None,
                options,
            )
            .await
    }

    /// Updates the supplied Pipeline fields. Omitted fields remain unchanged.
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
    ///         .pipelines
    ///         .update_pipeline(
    ///             &"pipelineKey".to_string(),
    ///             &PipelineUpdate {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update_pipeline(
        &self,
        pipeline_key: &str,
        request: &PipelineUpdate,
        options: Option<RequestOptions>,
    ) -> Result<Pipeline, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v2/pipelines/{}", pipeline_key),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Deletes an empty pipeline.
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
    ///         .pipelines
    ///         .delete_pipeline(&"pipelineKey".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn delete_pipeline(
        &self,
        pipeline_key: &str,
        options: Option<RequestOptions>,
    ) -> Result<OperationResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("v2/pipelines/{}", pipeline_key),
                None,
                None,
                options,
            )
            .await
    }

    /// Creates a custom field in a pipeline.
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
    ///         .pipelines
    ///         .create_field(
    ///             &"pipelineKey".to_string(),
    ///             &PipelineFieldCreate {
    ///                 name: "name".to_string(),
    ///                 r#type: FieldType::TextInput,
    ///                 default_value: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_field(
        &self,
        pipeline_key: &str,
        request: &PipelineFieldCreate,
        options: Option<RequestOptions>,
    ) -> Result<PipelineField, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v2/pipelines/{}/fields", pipeline_key),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
