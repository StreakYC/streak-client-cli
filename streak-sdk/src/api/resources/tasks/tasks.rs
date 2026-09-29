use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct TasksClient {
    pub http_client: HttpClient,
}

impl TasksClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Lists tasks on a box, ordered by creation date.
    ///
    /// # Arguments
    ///
    /// * `box_key` - Key for the box whose tasks should be listed.
    /// * `limit` - Number of items to return. Maximum: 1000
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
    ///         .tasks
    ///         .get_tasks_for_box(
    ///             &"boxKey".to_string(),
    ///             &GetTasksForBoxQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_tasks_for_box(
        &self,
        box_key: &str,
        request: &GetTasksForBoxQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<TaskListResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v2/boxes/{}/tasks", box_key),
                None,
                QueryBuilder::new()
                    .int("limit", request.limit.clone())
                    .int("page", request.page.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Creates a task on a box.
    ///
    /// # Arguments
    ///
    /// * `box_key` - Key for the box to create the task on.
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
    ///         .tasks
    ///         .create_task(
    ///             &"boxKey".to_string(),
    ///             &TaskCreateBody {
    ///                 text: "text".to_string(),
    ///                 due_date: None,
    ///                 assigned_to: None,
    ///                 meeting_key: None,
    ///                 is_draft: None,
    ///                 team_contact_key: None,
    ///                 draft: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_task(
        &self,
        box_key: &str,
        request: &TaskCreateBody,
        options: Option<RequestOptions>,
    ) -> Result<Task, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v2/boxes/{}/tasks", box_key),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Lists tasks across all pipelines and boxes. By default, returns incomplete tasks assigned to the current user before tomorrow in the user's timezone, which includes today's and overdue tasks. Use userKeys, pipelineKey, includeCompleted, direction, limit, and sortOrder to page through other agenda slices.
    ///
    /// # Arguments
    ///
    /// * `direction` - Agenda direction. Accepted values are desc, past, asc, and future. Defaults to desc.
    /// * `include_completed` - Whether completed tasks should be included in the returned agenda.
    /// * `limit` - Maximum number of tasks to return per assigned user.
    /// * `pipeline_key` - Key for the pipeline to filter tasks to. Omit to search across all visible pipelines.
    /// * `sort_order` - Opaque 30-digit sort cursor returned as task.sortOrder. Omit to use tomorrow at local midnight as the cursor boundary.
    /// * `user_keys` - User keys whose assigned tasks should be returned. Omit to return tasks assigned to the current user.
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
    ///         .tasks
    ///         .get_assigned_tasks(
    ///             &GetAssignedTasksQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_assigned_tasks(
        &self,
        request: &GetAssignedTasksQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<TaskListResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v2/tasks",
                None,
                QueryBuilder::new()
                    .serialize("direction", request.direction.clone())
                    .bool("includeCompleted", request.include_completed.clone())
                    .int("limit", request.limit.clone())
                    .serialize("pipelineKey", request.pipeline_key.clone())
                    .serialize("sortOrder", request.sort_order.clone())
                    .serialize("userKeys", request.user_keys.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Returns a single task by key.
    ///
    /// # Arguments
    ///
    /// * `task_key` - Key for the task to return.
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
    ///     client.tasks.get_task(&"taskKey".to_string(), None).await;
    /// }
    /// ```
    pub async fn get_task(
        &self,
        task_key: &str,
        options: Option<RequestOptions>,
    ) -> Result<Task, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v2/tasks/{}", task_key),
                None,
                None,
                options,
            )
            .await
    }

    /// Updates a task by key. Omitted body fields are left unchanged.
    ///
    /// # Arguments
    ///
    /// * `task_key` - Key for the task to update.
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
    ///         .tasks
    ///         .update_task(
    ///             &"taskKey".to_string(),
    ///             &TaskUpdateBody {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update_task(
        &self,
        task_key: &str,
        request: &TaskUpdateBody,
        options: Option<RequestOptions>,
    ) -> Result<Task, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v2/tasks/{}", task_key),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Deletes a task by key.
    ///
    /// # Arguments
    ///
    /// * `task_key` - Key for the task to delete.
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
    ///     client.tasks.delete_task(&"taskKey".to_string(), None).await;
    /// }
    /// ```
    pub async fn delete_task(
        &self,
        task_key: &str,
        options: Option<RequestOptions>,
    ) -> Result<OperationResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("v2/tasks/{}", task_key),
                None,
                None,
                options,
            )
            .await
    }
}
