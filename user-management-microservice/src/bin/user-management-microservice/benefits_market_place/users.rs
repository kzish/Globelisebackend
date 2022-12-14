use axum::{Extension, Json};
use common_utils::{error::GlobeliseResult, token::Token};
use reqwest::{header::CONTENT_TYPE, Client};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use sqlx::FromRow;
use user_management_microservice_sdk::token::UserAccessToken;

use crate::database::SharedDatabase;
#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct UserSignupRequest {
    pub username: String,
    pub password: String,
    #[serde(rename = "userProfile")]
    pub user_profile: UserProfile,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct UserProfile {
    pub firstname: String,
    pub lastname: String,
    pub email: String,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct GlobeliseUser {
    pub email: String,
}

//dummy method
pub async fn _user_registration(_request: UserSignupRequest) -> GlobeliseResult<(String, String)> {
    Ok(("200".to_string(), "".to_string()))
}
//for production
pub async fn user_registration(request: UserSignupRequest) -> GlobeliseResult<(String, String)> {
    let client = Client::builder().build().unwrap();

    let base_url = std::env::var("BENEFITS_MARKET_PLACE_BASE_URL")
        .expect("BENEFITS_MARKET_PLACE_BASE_URL not set");
    let url = format!("{}users/", base_url);
    let res = client
        .post(url)
        .header(CONTENT_TYPE, "application/json")
        .json(&request)
        .send()
        .await?;
    let status = res.status().as_str().to_string();
    let res_string = res.text().await?;
    println!("{:?}", (&status, &res_string));
    Ok((status, res_string))
}

//check if user token is valid
pub async fn check_token_is_valid(
    claims: Token<UserAccessToken>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<GlobeliseUser>> {
    let database = database.lock().await;

    let response = database.get_globelise_user(claims.payload.ulid).await?;

    Ok(Json(response))
}
