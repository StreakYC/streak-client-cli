use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct MeetingsClient {
    pub http_client: HttpClient,
}

impl MeetingsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Lists meetings and call logs on a box
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
    ///         .meetings
    ///         .get_meetings(
    ///             &"boxKey".to_string(),
    ///             &GetMeetingsQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_meetings(
        &self,
        box_key: &str,
        request: &GetMeetingsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<MeetingListResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v2/boxes/{}/meetings", box_key),
                None,
                QueryBuilder::new()
                    .int("limit", request.limit.clone())
                    .int("page", request.page.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Creates a meeting or call log on a box
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
    ///         .meetings
    ///         .create_meeting(
    ///             &"boxKey".to_string(),
    ///             &MeetingCreateBody {
    ///                 meeting_type: MeetingType::CallLog,
    ///                 start_timestamp: 1646870400000,
    ///                 duration: None,
    ///                 notes: None,
    ///                 task_keys: None,
    ///                 is_draft: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_meeting(
        &self,
        box_key: &str,
        request: &MeetingCreateBody,
        options: Option<RequestOptions>,
    ) -> Result<Meeting, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v2/boxes/{}/meetings", box_key),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Returns a meeting or call log by key
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
    ///         .meetings
    ///         .get_meeting(&"meetingKey".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn get_meeting(
        &self,
        meeting_key: &str,
        options: Option<RequestOptions>,
    ) -> Result<Meeting, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v2/meetings/{}", meeting_key),
                None,
                None,
                options,
            )
            .await
    }

    /// Updates a meeting or call log by key
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
    ///         .meetings
    ///         .update_meeting(
    ///             &"meetingKey".to_string(),
    ///             &MeetingUpdateBody {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update_meeting(
        &self,
        meeting_key: &str,
        request: &MeetingUpdateBody,
        options: Option<RequestOptions>,
    ) -> Result<Meeting, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v2/meetings/{}", meeting_key),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Deletes a meeting or call log by key
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
    ///         .meetings
    ///         .delete_meeting(&"meetingKey".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn delete_meeting(
        &self,
        meeting_key: &str,
        options: Option<RequestOptions>,
    ) -> Result<OperationResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("v2/meetings/{}", meeting_key),
                None,
                None,
                options,
            )
            .await
    }
}
