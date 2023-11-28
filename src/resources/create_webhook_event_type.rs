use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateWebhookEventType {
    /// The unique event name.
    pub name: String,
}
