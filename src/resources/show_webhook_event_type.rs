use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ShowWebhookEventType {
    /// The unique event name.
    /// Note: To subscribe to all events, including events as they are added, specify an * as
    /// the value to represent a wildcard.
    pub name: String,

    /// A human-readable description of the event.
    pub description: Option<String>,

    /// The status of a webhook event.
    pub status: Option<String>,

    /// Identifier for the event type example: 1.0/2.0 etc.
    pub resource_versions: Option<Vec<String>>,
}
