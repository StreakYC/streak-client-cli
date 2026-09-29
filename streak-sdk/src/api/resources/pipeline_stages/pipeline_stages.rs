use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;
use std::collections::HashMap;

pub struct PipelineStagesClient {
    pub http_client: HttpClient,
}

impl PipelineStagesClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Returns the pipeline's stages keyed by stage key.
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
    ///         .pipeline_stages
    ///         .list_stages(&"pipelineKey".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn list_stages(
        &self,
        pipeline_key: &str,
        options: Option<RequestOptions>,
    ) -> Result<HashMap<String, PipelineStage>, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v2/pipelines/{}/stages", pipeline_key),
                None,
                None,
                options,
            )
            .await
    }

    /// Creates a stage at the end of the pipeline's stage order.
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
    ///         .pipeline_stages
    ///         .create_stage(
    ///             &"pipelineKey".to_string(),
    ///             &PipelineStageCreate {
    ///                 name: "name".to_string(),
    ///                 color: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_stage(
        &self,
        pipeline_key: &str,
        request: &PipelineStageCreate,
        options: Option<RequestOptions>,
    ) -> Result<PipelineStage, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v2/pipelines/{}/stages", pipeline_key),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Returns a stage in a pipeline.
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
    ///         .pipeline_stages
    ///         .get_stage(&"pipelineKey".to_string(), &"stageKey".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn get_stage(
        &self,
        pipeline_key: &str,
        stage_key: &str,
        options: Option<RequestOptions>,
    ) -> Result<PipelineStage, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v2/pipelines/{}/stages/{}", pipeline_key, stage_key),
                None,
                None,
                options,
            )
            .await
    }

    /// Updates the supplied stage name or colors.
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
    ///         .pipeline_stages
    ///         .update_stage(
    ///             &"pipelineKey".to_string(),
    ///             &"stageKey".to_string(),
    ///             &PipelineStageUpdate {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update_stage(
        &self,
        pipeline_key: &str,
        stage_key: &str,
        request: &PipelineStageUpdate,
        options: Option<RequestOptions>,
    ) -> Result<PipelineStage, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v2/pipelines/{}/stages/{}", pipeline_key, stage_key),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Deletes an empty stage. The final stage in a pipeline cannot be deleted.
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
    ///         .pipeline_stages
    ///         .delete_stage(&"pipelineKey".to_string(), &"stageKey".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn delete_stage(
        &self,
        pipeline_key: &str,
        stage_key: &str,
        options: Option<RequestOptions>,
    ) -> Result<OperationResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("v2/pipelines/{}/stages/{}", pipeline_key, stage_key),
                None,
                None,
                options,
            )
            .await
    }
}
