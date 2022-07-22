use axum::extract::{ContentLengthLimit, Extension, Json, Path};
use common_utils::{
    custom_serde::{UserRole, UserType, FORM_DATA_LENGTH_LIMIT},
    database::{onboard::pic::EntityPicDetails, CommonDatabase},
    error::{GlobeliseError, GlobeliseResult},
    token::Token,
};
use eor_admin_microservice_sdk::token::AdminAccessToken;
use user_management_microservice_sdk::token::UserAccessToken;
use uuid::Uuid;

pub async fn user_post_one_onboard_entity_pic_details(
    claims: Token<UserAccessToken>,
    ContentLengthLimit(Json(body)): ContentLengthLimit<
        Json<EntityPicDetails>,
        FORM_DATA_LENGTH_LIMIT,
    >,
    Path(role): Path<UserRole>,
    Extension(database): Extension<CommonDatabase>,
) -> GlobeliseResult<()> {
    if !matches!(claims.payload.user_type, UserType::Entity) {
        return Err(GlobeliseError::Forbidden);
    }

    let database = database.lock().await;

    database
        .insert_one_onboard_entity_pic_details(
            &claims.payload.ulid,
            &role,
            &body.first_name,
            &body.last_name,
            &body.dob,
            &body.dial_code,
            &body.phone_number,
            body.profile_picture.as_ref(),
        )
        .await?;

    Ok(())
}

pub async fn admin_post_one_onboard_entity_pic_details(
    _: Token<AdminAccessToken>,
    Path((user_ulid, user_role)): Path<(Uuid, UserRole)>,
    ContentLengthLimit(Json(body)): ContentLengthLimit<
        Json<EntityPicDetails>,
        FORM_DATA_LENGTH_LIMIT,
    >,
    Extension(database): Extension<CommonDatabase>,
) -> GlobeliseResult<()> {
    let database = database.lock().await;

    database
        .insert_one_onboard_entity_pic_details(
            &user_ulid,
            &user_role,
            &body.first_name,
            &body.last_name,
            &body.dob,
            &body.dial_code,
            &body.phone_number,
            body.profile_picture.as_ref(),
        )
        .await?;

    Ok(())
}

pub async fn user_get_one_onboard_entity_pic_details(
    claims: Token<UserAccessToken>,
    Path(user_role): Path<UserRole>,
    Extension(database): Extension<CommonDatabase>,
) -> GlobeliseResult<Json<EntityPicDetails>> {
    if !matches!(claims.payload.user_type, UserType::Entity) {
        return Err(GlobeliseError::Forbidden);
    }

    let database = database.lock().await;

    let result = database
        .select_one_onboard_entity_pic_details(claims.payload.ulid, user_role)
        .await?
        .ok_or_else(|| GlobeliseError::not_found("Cannot find PIC details for this user"))?;

    Ok(Json(result))
}

pub async fn admin_get_one_onboard_entity_pic_details(
    _: Token<AdminAccessToken>,
    Path((user_ulid, user_role)): Path<(Uuid, UserRole)>,
    Extension(database): Extension<CommonDatabase>,
) -> GlobeliseResult<Json<EntityPicDetails>> {
    let database = database.lock().await;

    let result = database
        .select_one_onboard_entity_pic_details(user_ulid, user_role)
        .await?
        .ok_or_else(|| GlobeliseError::not_found("Cannot find PIC details for this user"))?;

    Ok(Json(result))
}
