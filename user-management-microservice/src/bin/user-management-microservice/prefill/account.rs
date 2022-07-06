use axum::extract::{ContentLengthLimit, Extension, Json};
use common_utils::{
    custom_serde::{
        Country, EmailWrapper, ImageData, OffsetDateWrapper, UserType, FORM_DATA_LENGTH_LIMIT,
    },
    database::{
        onboard::{
            entity::EntityClientAccountDetails, individual::IndividualContractorAccountDetails,
        },
        CommonDatabase,
    },
    error::{GlobeliseError, GlobeliseResult},
    token::Token,
};
use eor_admin_microservice_sdk::token::AdminAccessToken;
use serde::{Deserialize, Serialize};
use serde_with::{base64::Base64, serde_as, TryFromInto};
use sqlx::FromRow;
use user_management_microservice_sdk::token::UserAccessToken;
use uuid::Uuid;

#[serde_as]
#[derive(Debug, FromRow, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct PrefillIndividualContractorAccountDetails {
    pub email: EmailWrapper,
    pub client_ulid: Uuid,
    pub first_name: String,
    pub last_name: String,
    #[serde_as(as = "TryFromInto<OffsetDateWrapper>")]
    pub dob: sqlx::types::time::OffsetDateTime,
    pub dial_code: String,
    pub phone_number: String,
    pub country: Country,
    pub city: String,
    pub address: String,
    pub postal_code: String,
    #[serde(default)]
    pub tax_id: Option<String>,
    pub time_zone: String,
    pub gender: String,
    pub marital_status: String,
    pub nationality: Option<String>,
    pub email_address: Option<EmailWrapper>,
    pub national_id: Option<String>,
    pub passport_number: Option<String>,
    pub passport_expiry_date: Option<String>,
    pub work_permit: Option<String>,
    pub added_related_pay_item_id: Option<Uuid>,
    pub total_dependants: Option<i64>,
}

pub async fn user_post_one_individual_contractor(
    token: Token<UserAccessToken>,
    ContentLengthLimit(Json(body)): ContentLengthLimit<
        Json<PrefillIndividualContractorAccountDetails>,
        FORM_DATA_LENGTH_LIMIT,
    >,
    Extension(database): Extension<CommonDatabase>,
) -> GlobeliseResult<()> {
    if !matches!(token.payload.user_type, UserType::Entity) {
        return Err(GlobeliseError::Forbidden);
    }

    let database = database.lock().await;

    if database
        .find_one_user(None, Some(&body.email), None)
        .await?
        .is_some()
    {
        return Err(GlobeliseError::bad_request(
            "There's already a user with this email",
        ));
    }

    let ulid = database
        .insert_one_user(&body.email, None, false, false, false, true, false, true)
        .await?;

    if database
        .select_one_onboard_individual_client_account_details(ulid)
        .await?
        .is_none()
    {
        database
            .insert_one_onboard_individual_contractor_account_details(
                ulid,
                &IndividualContractorAccountDetails {
                    first_name: body.first_name,
                    last_name: body.last_name,
                    dob: body.dob,
                    dial_code: body.dial_code,
                    phone_number: body.phone_number,
                    country: body.country,
                    city: body.city,
                    address: body.address,
                    postal_code: body.postal_code,
                    tax_id: body.tax_id,
                    time_zone: body.time_zone,
                    profile_picture: None,
                    cv: None,
                    gender: body.gender,
                    marital_status: body.marital_status,
                    nationality: body.nationality,
                    email_address: None,
                    national_id: body.national_id,
                    passport_number: body.passport_number,
                    passport_expiry_date: body.passport_expiry_date,
                    work_permit: body.work_permit,
                    added_related_pay_item_id: None,
                    total_dependants: body.total_dependants,
                },
            )
            .await?;
    }

    Ok(())
}

#[serde_as]
#[derive(Debug, FromRow, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct PrefillEntityClientAccountDetails {
    pub email: EmailWrapper,
    pub company_name: String,
    pub country: Country,
    pub entity_type: String,
    #[serde(default)]
    pub registration_number: Option<String>,
    #[serde(default)]
    pub tax_id: Option<String>,
    pub company_address: String,
    pub city: String,
    pub postal_code: String,
    pub time_zone: String,
    #[serde_as(as = "Option<Base64>")]
    #[serde(default)]
    pub logo: Option<ImageData>,
}

pub async fn admin_post_one_entity_client(
    // Only needed for validation
    _: Token<AdminAccessToken>,
    ContentLengthLimit(Json(body)): ContentLengthLimit<
        Json<PrefillEntityClientAccountDetails>,
        FORM_DATA_LENGTH_LIMIT,
    >,
    Extension(database): Extension<CommonDatabase>,
) -> GlobeliseResult<()> {
    let database = database.lock().await;

    if database
        .find_one_user(None, Some(&body.email), None)
        .await?
        .is_some()
    {
        return Err(GlobeliseError::bad_request(
            "User with this email already exists",
        ));
    }

    let ulid = database
        .insert_one_user(&body.email, None, false, false, true, false, true, false)
        .await?;

    database
        .insert_one_onboard_entity_client_account_details(
            ulid,
            &EntityClientAccountDetails {
                company_name: body.company_name,
                country: body.country,
                entity_type: body.entity_type,
                registration_number: body.registration_number,
                tax_id: body.tax_id,
                company_address: body.company_address,
                city: body.city,
                postal_code: body.postal_code,
                time_zone: body.time_zone,
                logo: body.logo,
            },
        )
        .await?;

    Ok(())
}

#[serde_as]
#[derive(Debug, FromRow, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct PrefilledIndividualContractorAccountDetails {
    pub email: EmailWrapper,
    pub first_name: String,
    pub last_name: String,
    #[serde_as(as = "TryFromInto<OffsetDateWrapper>")]
    pub dob: sqlx::types::time::OffsetDateTime,
    pub dial_code: String,
    pub phone_number: String,
    pub country: Country,
    pub city: String,
    pub address: String,
    pub postal_code: String,
    #[serde(default)]
    pub tax_id: Option<String>,
    pub time_zone: String,
    #[serde_as(as = "Option<Base64>")]
    #[serde(default)]
    pub profile_picture: Option<ImageData>,
    #[serde_as(as = "Option<Base64>")]
    #[serde(default)]
    pub cv: Option<Vec<u8>>,
    pub gender: String,
    pub marital_status: String,
    pub nationality: Option<String>,
    pub email_address: Option<EmailWrapper>,
    pub national_id: Option<String>,
    pub passport_number: Option<String>,
    pub passport_expiry_date: Option<String>,
    pub work_permit: Option<String>,
    pub added_related_pay_item_id: Option<Uuid>,
    pub total_dependants: Option<i64>,
}

pub async fn admin_post_one_individual_contractor(
    // Only needed for validation
    _: Token<AdminAccessToken>,
    ContentLengthLimit(Json(body)): ContentLengthLimit<
        Json<PrefilledIndividualContractorAccountDetails>,
        FORM_DATA_LENGTH_LIMIT,
    >,
    Extension(database): Extension<CommonDatabase>,
) -> GlobeliseResult<()> {
    let database = database.lock().await;

    let ulid = database
        .find_one_user(None, Some(&body.email), None)
        .await?
        .ok_or_else(|| GlobeliseError::not_found("Cannot find a user with this email"))?
        .ulid;

    if database
        .select_one_onboard_individual_contractor_account_details(ulid)
        .await?
        .is_none()
    {
        database
            .insert_one_onboard_individual_contractor_account_details(
                ulid,
                &IndividualContractorAccountDetails {
                    first_name: body.first_name,
                    last_name: body.last_name,
                    dob: body.dob,
                    dial_code: body.dial_code,
                    phone_number: body.phone_number,
                    country: body.country,
                    city: body.city,
                    address: body.address,
                    postal_code: body.postal_code,
                    tax_id: body.tax_id,
                    time_zone: body.time_zone,
                    profile_picture: body.profile_picture,
                    cv: body.cv,
                    gender: body.gender,
                    marital_status: body.marital_status,
                    nationality: body.nationality,
                    email_address: body.email_address,
                    national_id: body.national_id,
                    passport_number: body.passport_number,
                    passport_expiry_date: body.passport_expiry_date,
                    work_permit: body.work_permit,
                    added_related_pay_item_id: body.added_related_pay_item_id,
                    total_dependants: body.total_dependants,
                },
            )
            .await?;
    }

    Ok(())
}
