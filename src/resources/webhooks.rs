use std::borrow::Cow;

use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::client::endpoint::Endpoint;
use crate::client::error::PayPalError;
use crate::client::paypal::Client;
use crate::client::EmptyResponseBody;
use crate::resources::enums::verification_status::VerificationStatus;
use crate::{AnchorType, CreateWebhookEventType, LinkDescription, Op, ShowWebhookEventType};

#[derive(Clone, Debug, Deserialize)]
pub struct Webhook {
    /// The ID of the webhook.
    pub id: String,
    /// The URL that is configured to listen on localhost for incoming POST notification messages
    /// that contain event information.
    pub url: String,

    // An array of events to which to subscribe your webhook. To subscribe to all events, including
    // events as they are added, specify the asterisk wild card. To replace the event_types array,
    // specify the asterisk wild card.
    pub event_types: Vec<ShowWebhookEventType>,

    /// An array of request-related HATEOAS links.
    pub links: Option<Vec<LinkDescription>>,
}

impl Webhook {
    /// Verifies a webhook signature.
    pub async fn verify(
        client: &Client,
        dto: VerifyWebhookSignatureDto,
    ) -> Result<VerifyWebhookSignatureResponse, PayPalError> {
        client.post(&VerifyWebhookSignature::new(dto)).await
    }

    /// Lists webhooks.
    pub async fn list(
        client: &Client,
        query: ListWebhooksQuery,
    ) -> Result<ListWebhooksResponse, PayPalError> {
        client.get(&ListWebhooks::new(query)).await
    }

    /// Shows details for a webhook.
    pub async fn show(
        client: &Client,
        id: String,
    ) -> Result<ShowWebhookDetailsResponse, PayPalError> {
        client.get(&ShowWebhookDetails::new(id)).await
    }

    /// Creates a webhook.
    pub async fn create(
        client: &Client,
        dto: CreateWebhookDto,
    ) -> Result<CreateWebhookResponse, PayPalError> {
        client.post(&CreateWebhook::new(dto)).await
    }

    /// Updates a webhook.
    pub async fn update(
        client: &Client,
        id: String,
        dto: UpdateWebhookDto,
    ) -> Result<ShowWebhookDetailsResponse, PayPalError> {
        client.patch(&UpdateWebhook::new(id, dto)).await
    }

    /// Deletes a webhook.
    pub async fn delete(client: &Client, id: String) -> Result<(), PayPalError> {
        client.delete(&DeleteWebhook::new(id)).await?;
        Ok(())
    }

    /// Simulates a webhook event.
    pub async fn simulate(
        client: &Client,
        dto: SimulateWebhookEventDto,
    ) -> Result<SimulateWebhookEventResponse, PayPalError> {
        client.post(&SimulateWebhookEvent::new(dto)).await
    }

    /// Lists available webhook events.
    pub async fn list_available(
        client: &Client,
    ) -> Result<ListAvailableWebhookEventsResponse, PayPalError> {
        client.get(&ListAvailableWebhookEvents::new()).await
    }
}

#[skip_serializing_none]
#[derive(Clone, Debug, Serialize)]
pub struct VerifyWebhookSignatureDto {
    /// The algorithm that PayPal uses to generate the signature and that you can use to verify the signature.
    /// Extract this value from the `PAYPAL-AUTH-ALGO` response header, which is received with the webhook notification.
    pub auth_algo: String,

    /// The X.509 public key certificate. Download the certificate from this URL and use it to verify the signature.
    /// Extract this value from the `PAYPAL-CERT-URL` response header, which is received with the webhook notification.
    pub cert_url: String,

    /// The ID of the HTTP transmission. Contained in the `PAYPAL-TRANSMISSION-ID` header of the notification message.
    pub transmission_id: String,

    /// The PayPal-generated asymmetric signature. Appears in the `PAYPAL-TRANSMISSION-SIG` header of the notification message.
    pub transmission_sig: String,

    /// The date and time of the HTTP transmission, in Internet date and time format.
    /// Appears in the `PAYPAL-TRANSMISSION-TIME` header of the notification message.
    pub transmission_time: String,

    /// A webhook event notification.
    /// @Note: In this case, the request body.
    pub webhook_event: serde_json::Value,

    /// The ID of the webhook as configured in your Developer Portal account.
    pub webhook_id: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct VerifyWebhookSignatureResponse {
    /// The status of the signature verification.
    pub verification_status: VerificationStatus,
}

#[derive(Debug)]
struct VerifyWebhookSignature {
    pub body: VerifyWebhookSignatureDto,
}

impl VerifyWebhookSignature {
    pub const fn new(body: VerifyWebhookSignatureDto) -> Self {
        Self { body }
    }
}

impl Endpoint for VerifyWebhookSignature {
    type QueryParams = ();
    type RequestBody = VerifyWebhookSignatureDto;
    type ResponseBody = VerifyWebhookSignatureResponse;

    fn path(&self) -> Cow<str> {
        Cow::Borrowed("v1/notifications/verify-webhook-signature")
    }

    fn request_body(&self) -> Option<Self::RequestBody> {
        Some(self.body.clone())
    }

    fn request_method(&self) -> Method {
        Method::POST
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct ListWebhooksQuery {
    anchor_type: Option<AnchorType>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ListWebhooksResponse {
    /// An array of webhooks.
    pub webhooks: Vec<Webhook>,

    /// An array of request-related HATEOAS links.
    pub links: Option<Vec<LinkDescription>>,
}

#[derive(Debug)]
struct ListWebhooks {
    query_params: ListWebhooksQuery,
}

impl ListWebhooks {
    pub const fn new(query_params: ListWebhooksQuery) -> Self {
        Self { query_params }
    }
}

impl Endpoint for ListWebhooks {
    type QueryParams = ListWebhooksQuery;
    type RequestBody = ();
    type ResponseBody = ListWebhooksResponse;

    fn path(&self) -> Cow<str> {
        Cow::Borrowed("v1/notifications/webhooks")
    }

    fn query(&self) -> Option<Self::QueryParams> {
        Some(self.query_params.clone())
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct ShowWebhookDetailsResponse {
    /// The ID of the webhook.
    pub id: Option<String>,

    /// The URL that is configured to listen on localhost for incoming POST notification messages
    /// that contain event information.
    pub url: String,

    /// An array of events to which to subscribe your webhook. To subscribe to all events, including
    /// events as they are added, specify the asterisk wild card. To replace the event_types array,
    /// specify the asterisk wild card. To list all supported events, list available events.
    pub event_types: Vec<ShowWebhookEventType>,

    /// An array of request-related HATEOAS links.
    pub links: Option<Vec<LinkDescription>>,
}

#[derive(Debug)]
struct ShowWebhookDetails {
    id: String,
}

impl ShowWebhookDetails {
    pub const fn new(id: String) -> Self {
        Self { id }
    }
}

impl Endpoint for ShowWebhookDetails {
    type QueryParams = ();
    type RequestBody = ();
    type ResponseBody = ShowWebhookDetailsResponse;

    fn path(&self) -> Cow<str> {
        Cow::Owned(format!("v1/notifications/webhooks/{}", self.id))
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct CreateWebhookDto {
    pub event_type: Vec<CreateWebhookEventType>,
}

type CreateWebhookResponse = ShowWebhookDetailsResponse;

#[derive(Debug)]
struct CreateWebhook {
    body: CreateWebhookDto,
}

impl CreateWebhook {
    pub const fn new(body: CreateWebhookDto) -> Self {
        Self { body }
    }
}

impl Endpoint for CreateWebhook {
    type QueryParams = ();
    type RequestBody = CreateWebhookDto;
    type ResponseBody = CreateWebhookResponse;

    fn path(&self) -> Cow<str> {
        Cow::Borrowed("v1/notifications/webhooks")
    }

    fn request_body(&self) -> Option<Self::RequestBody> {
        Some(self.body.clone())
    }

    fn request_method(&self) -> Method {
        Method::POST
    }
}

pub type UpdateWebhookDto = Vec<UpdateWebhookDtoItem>;

#[derive(Clone, Debug, Serialize)]
pub struct UpdateWebhookDtoItem {
    /// The operation.
    pub op: Op,
    /// The JSON Pointer to the target document location at which to complete the operation.
    pub path: String,
    /// The value to apply. The remove operation does not require a value.
    pub value: Option<String>,
    /// The JSON Pointer to the target document location from which to move the value.
    /// Required for the move operation.
    pub from: Option<String>,
}

#[derive(Debug)]
struct UpdateWebhook {
    id: String,
    body: UpdateWebhookDto,
}

impl UpdateWebhook {
    pub const fn new(id: String, body: UpdateWebhookDto) -> Self {
        Self { id, body }
    }
}

impl Endpoint for UpdateWebhook {
    type QueryParams = ();
    type RequestBody = UpdateWebhookDto;
    type ResponseBody = ShowWebhookDetailsResponse;

    fn path(&self) -> Cow<str> {
        Cow::Owned(format!("v1/notifications/webhooks/{}", self.id))
    }

    fn request_body(&self) -> Option<Self::RequestBody> {
        Some(self.body.clone())
    }

    fn request_method(&self) -> Method {
        Method::PATCH
    }
}

#[derive(Debug)]
struct DeleteWebhook {
    id: String,
}

impl DeleteWebhook {
    pub const fn new(id: String) -> Self {
        Self { id }
    }
}

impl Endpoint for DeleteWebhook {
    type QueryParams = ();
    type RequestBody = ();
    type ResponseBody = EmptyResponseBody;

    fn path(&self) -> Cow<str> {
        Cow::Owned(format!("v1/notifications/webhooks/{}", self.id))
    }

    fn request_method(&self) -> Method {
        Method::DELETE
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct SimulateWebhookEventDto {
    /// The ID of the webhook. If omitted, the URL is required.
    pub webhook_id: Option<String>,

    /// The URL for the webhook endpoint. If omitted, the webhook ID is required.
    pub url: Option<String>,

    /// The event name. Specify one of the subscribed events. For each request,
    /// provide only one event.
    pub event_type: String,

    /// The identifier for event type ex: 1.0/2.0 etc.
    pub resource_version: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SimulateWebhookEventResponse {
    /// The ID of the webhook event notification.
    pub id: Option<String>,

    /// The date and time when the webhook event notification was created, in Internet date and
    /// time format.
    pub create_time: Option<String>,

    /// The name of the resource related to the webhook notification event.
    pub resource_type: Option<String>,

    /// The event version in the webhook notification.
    pub event_version: Option<String>,

    /// The event that triggered the webhook event notification.
    pub event_type: Option<String>,

    /// A summary description for the event notification.
    pub summary: Option<String>,

    /// The resource version in the webhook notification.
    pub resource_version: Option<String>,

    /// The resource that triggered the webhook event notification.
    pub resource: Option<serde_json::Value>,

    /// An array of request-related HATEOAS links.
    pub links: Option<Vec<LinkDescription>>,
}

#[derive(Debug)]
struct SimulateWebhookEvent {
    body: SimulateWebhookEventDto,
}

impl SimulateWebhookEvent {
    pub const fn new(body: SimulateWebhookEventDto) -> Self {
        Self { body }
    }
}

impl Endpoint for SimulateWebhookEvent {
    type QueryParams = ();
    type RequestBody = SimulateWebhookEventDto;
    type ResponseBody = SimulateWebhookEventResponse;

    fn path(&self) -> Cow<str> {
        Cow::Borrowed("v1/notifications/webhooks-events")
    }

    fn request_body(&self) -> Option<Self::RequestBody> {
        Some(self.body.clone())
    }

    fn request_method(&self) -> Method {
        Method::POST
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ListAvailableWebhookEventsResponse {
    pub event_types: Vec<ShowWebhookEventType>,
}

#[derive(Debug)]
struct ListAvailableWebhookEvents;

impl ListAvailableWebhookEvents {
    pub const fn new() -> Self {
        Self
    }
}

impl Endpoint for ListAvailableWebhookEvents {
    type QueryParams = ();
    type RequestBody = ();
    type ResponseBody = ListAvailableWebhookEventsResponse;

    fn path(&self) -> Cow<str> {
        Cow::Borrowed("v1/notifications/webhooks-event-types")
    }
}
