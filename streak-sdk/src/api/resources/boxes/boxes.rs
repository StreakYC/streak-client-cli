use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct BoxesClient {
    pub http_client: HttpClient,
}

impl BoxesClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Returns the box as one markdown document: an overview, its columns, contacts, organizations, linked boxes, tasks and pipeline, followed by its timeline. Every timestamp is ISO-8601 UTC. The document's structure is best-effort and may change; use the JSON endpoints for a stable contract.
    ///
    /// # Arguments
    ///
    /// * `max_tokens` - A token cap for the entire document, counted with OpenAI's o200k_base encoding. It covers the box sections first, then uses the remaining tokens for the timeline. A marker shows where entries were left out. If the box does not fit under the cap, a 400 error shows the minimum it needs. Omit the cap to return the full box.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// Text response
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
    ///         .boxes
    ///         .get_box_markdown(
    ///             &"boxKey".to_string(),
    ///             &GetBoxMarkdownQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_box_markdown(
        &self,
        box_key: &str,
        request: &GetBoxMarkdownQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<String, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/boxes/{}/md", box_key),
                None,
                QueryBuilder::new()
                    .serialize("maxTokens", request.max_tokens.clone())
                    .build(),
                options,
            )
            .await
    }
}
