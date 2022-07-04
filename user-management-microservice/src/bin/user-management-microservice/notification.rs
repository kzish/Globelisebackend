use axum::{
    extract::{ContentLengthLimit, Query},
    Extension, Json,
};
use common_utils::{
    custom_serde::{UserRole, UserType, FORM_DATA_LENGTH_LIMIT},
    database::{notification::NotificationIndex, CommonDatabase},
    error::GlobeliseResult,
    token::Token,
};
use eor_admin_microservice_sdk::token::AdminAccessToken;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use user_management_microservice_sdk::token::UserAccessToken;
use uuid::Uuid;

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct GetNotificationRequest {
    user_ulid: Option<Uuid>,
    query: Option<String>,
    read: Option<bool>,
    per_page: Option<u32>,
    page: Option<u32>,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct PutNotificationRequest {
    ulid: Uuid,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct PostNotificationRequest {
    message: String,
    audience: NotificationAudience,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
struct UserGroup {
    user_type: Option<UserType>,
    user_role: Option<UserRole>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
enum NotificationAudience {
    UserGroup(UserGroup),
    SpecificUsers(Vec<Uuid>),
}

pub async fn user_get_many(
    claims: Token<UserAccessToken>,
    Query(query): Query<GetNotificationRequest>,
    Extension(database): Extension<CommonDatabase>,
) -> GlobeliseResult<Json<Vec<NotificationIndex>>> {
    let database = database.lock().await;

    let result = database
        .select_many_user_notifications(
            Some(claims.payload.ulid),
            query.read,
            query.query,
            query.per_page,
            query.page,
        )
        .await?;

    Ok(Json(result))
}

pub async fn admin_get_many(
    _: Token<AdminAccessToken>,
    Query(query): Query<GetNotificationRequest>,
    Extension(database): Extension<CommonDatabase>,
) -> GlobeliseResult<Json<Vec<NotificationIndex>>> {
    let database = database.lock().await;

    let result = database
        .select_many_user_notifications(
            query.user_ulid,
            query.read,
            query.query,
            query.per_page,
            query.page,
        )
        .await?;

    Ok(Json(result))
}

pub async fn admin_get_many_for_user(
    _: Token<AdminAccessToken>,
    Query(query): Query<GetNotificationRequest>,
    Extension(database): Extension<CommonDatabase>,
) -> GlobeliseResult<Json<Vec<NotificationIndex>>> {
    let database = database.lock().await;

    let result = database
        .select_many_user_notifications(
            query.user_ulid,
            query.read,
            query.query,
            query.per_page,
            query.page,
        )
        .await?;

    Ok(Json(result))
}

pub async fn user_put_one(
    claims: Token<UserAccessToken>,
    ContentLengthLimit(Json(body)): ContentLengthLimit<
        Json<PutNotificationRequest>,
        FORM_DATA_LENGTH_LIMIT,
    >,
    Extension(database): Extension<CommonDatabase>,
) -> GlobeliseResult<()> {
    let database = database.lock().await;

    database
        .update_one_notification_as_read(claims.payload.ulid, body.ulid)
        .await?;

    Ok(())
}

pub async fn admin_post_one_for_user(
    _: Token<AdminAccessToken>,
    ContentLengthLimit(Json(body)): ContentLengthLimit<
        Json<PostNotificationRequest>,
        FORM_DATA_LENGTH_LIMIT,
    >,
    Extension(database): Extension<CommonDatabase>,
) -> GlobeliseResult<()> {
    let database = database.lock().await;

    let notification_ulid = database.create_one_user_notification(body.message).await?;

    match body.audience {
        NotificationAudience::UserGroup(UserGroup {
            user_role,
            user_type,
        }) => {
            database
                .create_one_user_see_notification_for_group_of_users(
                    notification_ulid,
                    user_type,
                    user_role,
                )
                .await?
        }
        NotificationAudience::SpecificUsers(users) => {
            database
                .create_one_user_see_notification_for_specific_users(&users, notification_ulid)
                .await?
        }
    }

    Ok(())
}
