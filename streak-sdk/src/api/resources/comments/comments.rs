use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct CommentsClient {
    pub http_client: HttpClient,
}

impl CommentsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Lists comments on a box
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
    ///         .comments
    ///         .get_comments(&"boxKey".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn get_comments(
        &self,
        box_key: &str,
        options: Option<RequestOptions>,
    ) -> Result<CommentListResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v2/boxes/{}/comments", box_key),
                None,
                None,
                options,
            )
            .await
    }

    /// Creates a comment on a box
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
    ///         .comments
    ///         .create_comment(
    ///             &"boxKey".to_string(),
    ///             &CommentCreateBody {
    ///                 message: "message".to_string(),
    ///                 mentions: None,
    ///                 parent: None,
    ///                 search_for_at_mentions: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_comment(
        &self,
        box_key: &str,
        request: &CommentCreateBody,
        options: Option<RequestOptions>,
    ) -> Result<Comment, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v2/boxes/{}/comments", box_key),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Returns a comment by key
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
    ///         .comments
    ///         .get_comment(&"commentKey".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn get_comment(
        &self,
        comment_key: &str,
        options: Option<RequestOptions>,
    ) -> Result<Comment, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v2/comments/{}", comment_key),
                None,
                None,
                options,
            )
            .await
    }

    /// Updates a comment by key
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
    ///         .comments
    ///         .update_comment(
    ///             &"commentKey".to_string(),
    ///             &CommentUpdateBody {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update_comment(
        &self,
        comment_key: &str,
        request: &CommentUpdateBody,
        options: Option<RequestOptions>,
    ) -> Result<Comment, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v2/comments/{}", comment_key),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Deletes a comment by key
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
    ///         .comments
    ///         .delete_comment(&"commentKey".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn delete_comment(
        &self,
        comment_key: &str,
        options: Option<RequestOptions>,
    ) -> Result<OperationResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("v2/comments/{}", comment_key),
                None,
                None,
                options,
            )
            .await
    }

    /// Adds an emoji reaction to a comment
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
    ///         .comments
    ///         .react_to_comment(
    ///             &"commentKey".to_string(),
    ///             &CommentReactionBody {
    ///                 emoji: "emoji".to_string(),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn react_to_comment(
        &self,
        comment_key: &str,
        request: &CommentReactionBody,
        options: Option<RequestOptions>,
    ) -> Result<Comment, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v2/comments/{}/react", comment_key),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Removes an emoji reaction from a comment
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
    ///         .comments
    ///         .unreact_to_comment(
    ///             &"commentKey".to_string(),
    ///             &CommentReactionBody {
    ///                 emoji: "emoji".to_string(),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn unreact_to_comment(
        &self,
        comment_key: &str,
        request: &CommentReactionBody,
        options: Option<RequestOptions>,
    ) -> Result<Comment, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v2/comments/{}/unreact", comment_key),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
