use axum::{Extension, Json};
use common_utils::{
    error::GlobeliseResult,
    pubsub::{AddClientContractorPair, TopicSubscriberEvent, UpdateUserName},
};

use crate::database::SharedDatabase;

pub async fn update_client_contractor_pair(
    Json(value): Json<TopicSubscriberEvent<AddClientContractorPair>>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    let database = database.lock().await;

    database
        .update_client_contractor_pair(value.data.client_ulid, value.data.contractor_ulid)
        .await?;

    Ok(())
}

pub async fn update_user_name(
    Json(value): Json<TopicSubscriberEvent<UpdateUserName>>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    let database = database.lock().await;

    match value.data {
        UpdateUserName::Client(ulid, name) => database.update_client_name(ulid, name).await?,
        UpdateUserName::Contractor(ulid, name) => {
            database.update_contractor_name(ulid, name).await?
        }
    }

    Ok(())
}
