use std::{collections::HashMap, sync::Arc};

use axum::http::{HeaderMap, HeaderValue};
use reqwest::{Client, StatusCode};
use rusty_ulid::Ulid;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, TryFromInto};
use tokio::sync::Mutex;

use crate::{
    custom_serde::{Currency, DateWrapper},
    error::{GlobeliseError, GlobeliseResult},
};

pub const GLOBELISE_PUBSUB_TOPIC_ID: &str = "globelise-pubsub";

/// There should be its equivalent exposed in DAPR SDK themselves.
/// Reference struct `TopicSubscription` from dapr/proto/runtime/v1/appcallback.proto)
/// from the DAPR protobuf
#[derive(Serialize, Deserialize)]
pub struct TopicSubscription {
    #[serde(rename = "pubsubname")]
    pub pubsub_name: &'static str,
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
    pub pubsub_name: String,
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
                GLOBELISE_PUBSUB_TOPIC_ID,
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
            _ => Err(GlobeliseError::internal(response.text().await?)),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TopicId {
    #[serde(rename = "add-client-contractor-pair")]
    UpdateClientContractorPair,
    #[serde(rename = "update-user-name")]
    UpdateUserName,
    #[serde(rename = "create-or-update-contracts")]
    CreateOrUpdateContracts,
}

impl TopicId {
    pub fn as_str(&self) -> &'static str {
        match self {
            TopicId::UpdateClientContractorPair => "add-client-contractor-pair",
            TopicId::UpdateUserName => "update-user-name",
            TopicId::CreateOrUpdateContracts => "create-or-update-contracts",
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddClientContractorPair {
    pub client_ulid: Ulid,
    pub contractor_ulid: Ulid,
}

impl PubSubData for AddClientContractorPair {
    fn as_topic_id() -> TopicId {
        TopicId::UpdateClientContractorPair
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum UpdateUserName {
    Client(Ulid, String),
    Contractor(Ulid, String),
}

impl PubSubData for UpdateUserName {
    fn as_topic_id() -> TopicId {
        TopicId::UpdateUserName
    }
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateOrUpdateContracts {
    pub ulid: Ulid,
    pub client_ulid: Ulid,
    pub contractor_ulid: Ulid,
    pub contract_name: String,
    pub contract_type: String,
    pub contract_status: String,
    pub contract_amount: sqlx::types::Decimal,
    pub currency: Currency,
    pub job_title: String,
    pub seniority: String,
    #[serde_as(as = "TryFromInto<DateWrapper>")]
    pub begin_at: sqlx::types::time::Date,
    #[serde_as(as = "TryFromInto<DateWrapper>")]
    pub end_at: sqlx::types::time::Date,
    pub branch_ulid: Ulid,
}

impl PubSubData for CreateOrUpdateContracts {
    fn as_topic_id() -> TopicId {
        TopicId::CreateOrUpdateContracts
    }
}

pub trait PubSubData {
    fn create_topic_subscription<S>(route: S) -> TopicSubscription
    where
        S: Into<String>,
    {
        TopicSubscription {
            pubsub_name: GLOBELISE_PUBSUB_TOPIC_ID,
            topic: Self::as_topic_id().as_str().to_string(),
            route: route.into(),
            metadata: HashMap::default(),
        }
    }

    fn as_topic_id() -> TopicId;
}
