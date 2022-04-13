use std::{collections::HashMap, sync::Arc};

use axum::http::{HeaderMap, HeaderValue};
use reqwest::{Client, StatusCode};
use rusty_ulid::Ulid;
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;

use crate::error::{GlobeliseError, GlobeliseResult};

/// There should be its equivalent exposed in DAPR SDK themselves.
/// Reference struct `TopicSubscription` from dapr/proto/runtime/v1/appcallback.proto)
/// from the DAPR protobuf
#[derive(Serialize, Deserialize)]
pub struct TopicSubscription {
    #[serde(rename = "pubsubname")]
    pub pubsub_name: String,
    pub topic: String,
    pub route: String,
    pub metadata: HashMap<String, String>,
}

/// All DAPR subscriber events will come in this form.
/// Reference struct 'TopicEventRequest` from dapr/proto/runtime/v1/appcallback.proto
/// from the DAPR protobuf
/// TODO: Make some of these enums so we can type-check better.
///       It will also reduce allocations.
#[derive(Debug, Deserialize)]
pub struct TopicSubscriberEvent<T>
where
    T: PubSubData,
{
    pub data: T,
    #[serde(rename = "datacontenttype")]
    pub content_type: String,
    pub id: String,
    #[serde(rename = "pubsubname")]
    pub pubsub_name: PubSubId,
    pub source: String,
    #[serde(rename = "specversion")]
    pub spec_version: String,
    pub topic: TopicId,
    #[serde(rename = "traceid")]
    pub trace_id: String,
    #[serde(rename = "tracestate")]
    pub trace_state: String,
    #[serde(rename = "type")]
    pub event_type: String,
}

#[derive(Debug)]
pub struct PubSub(Client, String);

pub type SharedPubSub = Arc<Mutex<PubSub>>;

impl PubSub {
    pub fn new(reqwest_client: Client, dapr_address: String) -> Self {
        PubSub(reqwest_client, dapr_address)
    }

    /// TODO: Use a custom error type for this because DAPR only exposes certain error codes.
    pub async fn publish_event<T>(&self, data: T) -> GlobeliseResult<()>
    where
        T: PubSubData + Serialize,
    {
        let response = self
            .0
            .post(format!(
                "{}/v1.0/publish/{}/{}",
                self.1,
                T::as_pubsub_id().as_str(),
                T::as_topic_id().as_str()
            ))
            .headers({
                let mut headers = HeaderMap::new();
                headers.insert("Content-Type", HeaderValue::from_static("application/json"));
                headers
            })
            .json(&data)
            .send()
            .await?;
        match response.status() {
            StatusCode::OK | StatusCode::NO_CONTENT => Ok(()),
            e => Err(GlobeliseError::Internal(format!(
                "code:{} text:{}",
                e,
                response.text().await?,
            ))),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum PubSubId {
    #[serde(rename = "user-management-microservice-pubsub")]
    UserManagementMicroservice,
}

impl PubSubId {
    pub fn as_str(&self) -> &'static str {
        match self {
            PubSubId::UserManagementMicroservice => "user-management-microservice-pubsub",
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TopicId {
    #[serde(rename = "add-client-contractor-pair")]
    UpdateClientContractorPair,
    #[serde(rename = "update-user-name")]
    UpdateUserName,
}

impl TopicId {
    pub fn as_str(&self) -> &'static str {
        match self {
            TopicId::UpdateClientContractorPair => "add-client-contractor-pair",
            TopicId::UpdateUserName => "update-user-name",
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddClientContractorPair {
    pub client_ulid: Ulid,
    pub contractor_ulid: Ulid,
}

impl PubSubData for AddClientContractorPair {
    fn as_pubsub_id() -> PubSubId {
        PubSubId::UserManagementMicroservice
    }

    fn as_topic_id() -> TopicId {
        TopicId::UpdateClientContractorPair
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum UpdateUserName {
    Client(Ulid, String),
    Contractor(Ulid, String),
}

impl UpdateUserName {}

impl PubSubData for UpdateUserName {
    fn as_pubsub_id() -> PubSubId {
        PubSubId::UserManagementMicroservice
    }

    fn as_topic_id() -> TopicId {
        TopicId::UpdateUserName
    }
}

pub trait PubSubData {
    fn create_topic_subscription<S>(route: S) -> TopicSubscription
    where
        S: Into<String>,
    {
        TopicSubscription {
            pubsub_name: Self::as_pubsub_id().as_str().to_string(),
            topic: Self::as_topic_id().as_str().to_string(),
            route: route.into(),
            metadata: HashMap::default(),
        }
    }

    fn as_pubsub_id() -> PubSubId;

    fn as_topic_id() -> TopicId;
}
