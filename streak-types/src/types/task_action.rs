pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Suggested action inferred from task text.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TaskAction {
    /// Suggested action type.
    pub r#type: ActionType,
    /// Suggested action value, such as an email address or phone number.
    #[serde(default)]
    pub value: String,
    /// Contact this action is associated with.
    #[serde(default)]
    pub contact: Contact,
}

impl TaskAction {
    pub fn builder() -> TaskActionBuilder {
        <TaskActionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TaskActionBuilder {
    r#type: Option<ActionType>,
    value: Option<String>,
    contact: Option<Contact>,
}

impl TaskActionBuilder {
    pub fn r#type(mut self, value: ActionType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    pub fn contact(mut self, value: Contact) -> Self {
        self.contact = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TaskAction`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](TaskActionBuilder::r#type)
    /// - [`value`](TaskActionBuilder::value)
    /// - [`contact`](TaskActionBuilder::contact)
    pub fn build(self) -> Result<TaskAction, BuildError> {
        Ok(TaskAction {
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
            value: self.value.ok_or_else(|| BuildError::missing_field("value"))?,
            contact: self.contact.ok_or_else(|| BuildError::missing_field("contact"))?,
        })
    }
}
