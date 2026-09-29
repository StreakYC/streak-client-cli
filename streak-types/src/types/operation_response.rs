pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct OperationResponse {
    /// Whether the operation completed successfully.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
    /// A message describing the result of the operation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl OperationResponse {
    pub fn builder() -> OperationResponseBuilder {
        <OperationResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OperationResponseBuilder {
    success: Option<bool>,
    message: Option<String>,
}

impl OperationResponseBuilder {
    pub fn success(mut self, value: bool) -> Self {
        self.success = Some(value);
        self
    }

    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`OperationResponse`].
    pub fn build(self) -> Result<OperationResponse, BuildError> {
        Ok(OperationResponse {
            success: self.success,
            message: self.message,
        })
    }
}
