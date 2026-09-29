use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct ContactsClient {
    pub http_client: HttpClient,
}

impl ContactsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Returns accessible contacts indexed by their requested keys. Missing and inaccessible contacts are omitted.
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
    ///         .contacts
    ///         .get_contacts(&vec!["string".to_string()], None)
    ///         .await;
    /// }
    /// ```
    pub async fn get_contacts(
        &self,
        request: &Vec<String>,
        options: Option<RequestOptions>,
    ) -> Result<ContactKeyBatch, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v2/contacts/batch/get",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Returns an accessible contact by key.
    ///
    /// # Arguments
    ///
    /// * `contact_key` - Key of the contact.
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
    ///         .contacts
    ///         .get_contact(&"contactKey".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn get_contact(
        &self,
        contact_key: &str,
        options: Option<RequestOptions>,
    ) -> Result<Contact, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v2/contacts/{}", contact_key),
                None,
                None,
                options,
            )
            .await
    }

    /// Applies supplied changes. Null or omitted fields remain unchanged.
    ///
    /// # Arguments
    ///
    /// * `contact_key` - Key of the contact.
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
    ///         .contacts
    ///         .update_contact(
    ///             &"contactKey".to_string(),
    ///             &ContactUpdate {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update_contact(
        &self,
        contact_key: &str,
        request: &ContactUpdate,
        options: Option<RequestOptions>,
    ) -> Result<Contact, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v2/contacts/{}", contact_key),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Deletes a contact and schedules cleanup of its links. A missing contact returns 404.
    ///
    /// # Arguments
    ///
    /// * `contact_key` - Key of the contact.
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
    ///         .contacts
    ///         .delete_contact(&"contactKey".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn delete_contact(
        &self,
        contact_key: &str,
        options: Option<RequestOptions>,
    ) -> Result<OperationResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("v2/contacts/{}", contact_key),
                None,
                None,
                options,
            )
            .await
    }

    /// Lists contacts in stable order. Pass the returned cursor to continue; keep the same after filter across pages. A full last page may be followed by an empty page.
    ///
    /// # Arguments
    ///
    /// * `team_key` - Key of the team.
    /// * `after` - Include contacts last saved at or after this epoch-seconds timestamp.
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
    ///         .contacts
    ///         .list_contacts(
    ///             &"teamKey".to_string(),
    ///             &ListContactsQueryRequest {
    ///                 after: Some(1646870400),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_contacts(
        &self,
        team_key: &str,
        request: &ListContactsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ContactPage, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v2/teams/{}/contacts", team_key),
                None,
                QueryBuilder::new()
                    .int("after", request.after.clone())
                    .string("cursor", request.cursor.clone())
                    .int("limit", request.limit.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Creates a contact from its basic details. Links and custom fields require a subsequent update.
    ///
    /// # Arguments
    ///
    /// * `team_key` - Key of the team.
    /// * `get_if_existing` - Return an existing email match instead of creating a contact. When getIfExisting=true, an email match is returned without merging new values.
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
    ///         .contacts
    ///         .create_contact(
    ///             &"teamKey".to_string(),
    ///             &ContactCreate {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_contact(
        &self,
        team_key: &str,
        request: &ContactCreate,
        options: Option<RequestOptions>,
    ) -> Result<Contact, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v2/teams/{}/contacts", team_key),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                QueryBuilder::new()
                    .serialize("getIfExisting", request.get_if_existing.clone())
                    .build(),
                options,
            )
            .await
    }
}
