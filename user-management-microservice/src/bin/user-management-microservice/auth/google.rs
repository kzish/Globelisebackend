//! Endpoint for handling Google authentication.

use axum::extract::{Extension, Path};
use common_utils::{
    custom_serde::UserType,
    error::{GlobeliseError, GlobeliseResult},
    token::AuthBearer,
};
use google_auth::IdToken;

use crate::{
    benefits_market_place::users::{user_registration, UserProfile, UserSignupRequest},
    env::GOOGLE_CLIENT_ID,
};

use super::{SharedDatabase, SharedState};

pub async fn login(
    AuthBearer(id_token): AuthBearer,
    Extension(database): Extension<SharedDatabase>,
    Extension(shared_state): Extension<SharedState>,
) -> GlobeliseResult<String> {
    let claims = IdToken(id_token)
        .decode_and_validate(&*GOOGLE_CLIENT_ID)
        .await
        .map_err(|e| {
            GlobeliseError::unauthorized(format!("Failed to decode Google ID token because\n{}", e))
        })?;

    let database = database.lock().await;
    let mut shared_state = shared_state.lock().await;
    if let Some(user) = database
        .find_one_user(None, Some(&claims.email), None)
        .await?
    {
        let user_type = user.user_type()?;
        let refresh_token = shared_state
            .open_session(&database, user.ulid, user_type)
            .await?;

        Ok(refresh_token)
    } else {
        Err(GlobeliseError::bad_request("Please signup first"))
    }
}

pub async fn signup(
    AuthBearer(id_token): AuthBearer,
    Path(user_type): Path<UserType>,
    Extension(database): Extension<SharedDatabase>,
    Extension(shared_state): Extension<SharedState>,
) -> GlobeliseResult<String> {
    let claims = IdToken(id_token)
        .decode_and_validate(&*GOOGLE_CLIENT_ID)
        .await
        .map_err(|e| {
            GlobeliseError::unauthorized(format!("Failed to decode Google ID token because\n{}", e))
        })?;

    let database = database.lock().await;
    let mut shared_state = shared_state.lock().await;

    if let Some(user) = database
        .find_one_user(None, Some(&claims.email), None)
        .await?
    {
        let user_type = user.user_type()?;
        let refresh_token = shared_state
            .open_session(&database, user.ulid, user_type)
            .await?;

        Ok(refresh_token)
    } else {
        //register user for benefits marketplace
        let email = &(claims.email.0.clone()).to_string();
        let benefits_user = UserSignupRequest {
            username: email.to_string(),
            password: "Password@123".to_string(),
            user_profile: UserProfile {
                firstname: "Globelise".to_string(),
                lastname: "User".to_string(),
                email: email.to_string(),
            },
        };
        let res = user_registration(benefits_user).await?;
        if res.0 != "200" {
            return Err(GlobeliseError::bad_request(res.1));
        }
        let ulid = database
            .create_user(
                claims.email,
                None,
                true,
                false,
                user_type == UserType::Entity,
                user_type == UserType::Individual,
                false,
                false,
            )
            .await?;

        let refresh_token = shared_state
            .open_session(&database, ulid, user_type)
            .await?;
        Ok(refresh_token)
    }
}
