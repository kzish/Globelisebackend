use axum::{
    extract::{Extension, Query},
    Json,
};
use common_utils::{
    custom_serde::DateWrapper,
    error::GlobeliseResult,
    token::{Token, TokenString},
};
use eor_admin_microservice_sdk::AccessToken as AdminAccessToken;
use reqwest::Client;
use rusty_ulid::Ulid;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, FromInto};
use sqlx::{postgres::PgRow, FromRow, Row};
use user_management_microservice_sdk::{AccessToken as UserAccessToken, GetUserInfoRequest, Role};

use crate::{
    database::{ulid_from_sql_uuid, SharedDatabase},
    env::USER_MANAGEMENT_MICROSERVICE_DOMAIN_URL,
};

/// Lists all the users plus some information about them.
pub async fn user_index(
    TokenString(access_token): TokenString,
    Query(request): Query<GetUserInfoRequest>,
    Extension(shared_client): Extension<Client>,
    Extension(shared_database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<Vec<UserIndex>>> {
    let response = user_management_microservice_sdk::get_users_info(
        &shared_client,
        &*USER_MANAGEMENT_MICROSERVICE_DOMAIN_URL,
        access_token,
        request,
    )
    .await?;

    let mut result = Vec::with_capacity(response.len());

    let database = shared_database.lock().await;

    for v in response {
        let count = database
            .count_number_of_contracts(v.ulid, v.user_role)
            .await?;
        result.push(UserIndex {
            ulid: v.ulid,
            name: v.name,
            role: v.user_role,
            contract_count: count,
            created_at: v.created_at,
            email: v.email,
        })
    }
    Ok(Json(result))
}

pub async fn contractors_index(
    access_token: Token<UserAccessToken>,
    Query(query): Query<PaginationQuery>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<Vec<ContractorsIndex>>> {
    let ulid = access_token.payload.ulid.parse::<Ulid>().unwrap();
    let database = database.lock().await;
    Ok(Json(database.contractors_index(ulid, query).await?))
}

pub async fn contracts_index_for_client(
    access_token: Token<UserAccessToken>,
    Query(query): Query<PaginationQuery>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<Vec<ContractsIndexForClient>>> {
    let ulid = access_token.payload.ulid.parse::<Ulid>().unwrap();
    let database = database.lock().await;
    Ok(Json(
        database.contracts_index_for_client(ulid, query).await?,
    ))
}

pub async fn contracts_index_for_contractor(
    access_token: Token<UserAccessToken>,
    Query(query): Query<PaginationQuery>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<Vec<ContractsIndexForContractor>>> {
    let ulid = access_token.payload.ulid.parse::<Ulid>().unwrap();
    let database = database.lock().await;
    Ok(Json(
        database.contracts_index_for_contractor(ulid, query).await?,
    ))
}

pub async fn eor_admin_contract_index(
    _: Token<AdminAccessToken>,
    Query(query): Query<PaginationQuery>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<Vec<ContractsIndexForClient>>> {
    let database = database.lock().await;
    Ok(Json(database.eor_admin_contract_index(query).await?))
}

#[derive(Debug, Serialize)]
pub struct UserIndex {
    pub ulid: Ulid,
    pub name: String,
    pub role: Role,
    pub contract_count: i64,
    pub created_at: String,
    pub email: String,
}

#[derive(Debug, Deserialize)]
pub struct PaginationQuery {
    #[serde(default = "PaginationQuery::default_page")]
    pub page: i64,
    #[serde(default = "PaginationQuery::default_per_page")]
    pub per_page: i64,
    pub search_text: Option<String>,
}

impl PaginationQuery {
    fn default_page() -> i64 {
        1
    }

    fn default_per_page() -> i64 {
        25
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct ContractorsIndex {
    contractor_ulid: Ulid,
    #[serde(flatten)]
    other_fields: ContractorsIndexSqlHelper,
}

impl<'r> FromRow<'r, PgRow> for ContractorsIndex {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            contractor_ulid: ulid_from_sql_uuid(row.try_get("contractor_ulid")?),
            other_fields: ContractorsIndexSqlHelper::from_row(row)?,
        })
    }
}

#[derive(Debug, FromRow, Serialize)]
#[serde(rename_all = "kebab-case")]
struct ContractorsIndexSqlHelper {
    contractor_name: String,
    contract_name: String,
    contract_status: String,
    job_title: String,
    seniority: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct ContractsIndexForClient {
    contractor_ulid: Ulid,
    contractor_name: String,
    ulid: Ulid,
    #[serde(flatten)]
    common_info: ContractsIndexCommonInfoSqlHelper,
}

impl<'r> FromRow<'r, PgRow> for ContractsIndexForClient {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            contractor_ulid: ulid_from_sql_uuid(row.try_get("contractor_ulid")?),
            contractor_name: row.try_get("contractor_name")?,
            ulid: ulid_from_sql_uuid(row.try_get("ulid")?),
            common_info: ContractsIndexCommonInfoSqlHelper::from_row(row)?,
        })
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct ContractsIndexForContractor {
    client_ulid: Ulid,
    client_name: String,
    ulid: Ulid,
    #[serde(flatten)]
    common_info: ContractsIndexCommonInfoSqlHelper,
}

impl<'r> FromRow<'r, PgRow> for ContractsIndexForContractor {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            client_ulid: ulid_from_sql_uuid(row.try_get("client_ulid")?),
            client_name: row.try_get("client_name")?,
            ulid: ulid_from_sql_uuid(row.try_get("ulid")?),
            common_info: ContractsIndexCommonInfoSqlHelper::from_row(row)?,
        })
    }
}

#[serde_as]
#[derive(Debug, FromRow, Serialize)]
#[serde(rename_all = "kebab-case")]
struct ContractsIndexCommonInfoSqlHelper {
    contract_name: String,
    contract_type: String,
    job_title: String,
    contract_status: String,
    contract_amount: sqlx::types::Decimal,
    currency: Currency,
    #[serde_as(as = "FromInto<DateWrapper>")]
    begin_at: sqlx::types::time::Date,
    #[serde_as(as = "FromInto<DateWrapper>")]
    end_at: sqlx::types::time::Date,
}

#[derive(Debug, sqlx::Type, Deserialize, Serialize)]
#[sqlx(type_name = "currency")]
#[allow(clippy::upper_case_acronyms)]
enum Currency {
    AED,
    AFN,
    ALL,
    AMD,
    AOA,
    ARS,
    AWG,
    AZN,
    BAM,
    BBD,
    BDT,
    BGN,
    BHD,
    BIF,
    BMD,
    BND,
    BOB,
    BOV,
    BRL,
    BSD,
    BTN,
    BWP,
    BYN,
    BZD,
    CAD,
    CDF,
    CHE,
    CHW,
    CLF,
    CLP,
    CNY,
    COP,
    COU,
    CRC,
    CUC,
    CUP,
    CVE,
    CZK,
    DJF,
    DOP,
    DZD,
    EGP,
    ERN,
    ETB,
    FJD,
    FKP,
    GEL,
    GHS,
    GIP,
    GMD,
    GNF,
    GTQ,
    GYD,
    HKD,
    HNL,
    HRK,
    HTG,
    HUF,
    IDR,
    ILS,
    IQD,
    IRR,
    ISK,
    JMD,
    JOD,
    JPY,
    KES,
    KGS,
    KHR,
    KMF,
    KPW,
    KRW,
    KWD,
    KYD,
    KZT,
    LAK,
    LBP,
    LKR,
    LRD,
    LSL,
    LYD,
    MDL,
    MGA,
    MKD,
    MMK,
    MNT,
    MOP,
    MRU,
    MUR,
    MVR,
    MWK,
    MXN,
    MXV,
    MYR,
    MZN,
    NAD,
    NGN,
    NIO,
    NPR,
    OMR,
    PAB,
    PEN,
    PGK,
    PHP,
    PKR,
    PLN,
    PYG,
    QAR,
    RON,
    RSD,
    RUB,
    RWF,
    SAR,
    SBD,
    SCR,
    SDG,
    SEK,
    SGD,
    SHP,
    SLL,
    SOS,
    SRD,
    SSP,
    STN,
    SVC,
    SYP,
    SZL,
    THB,
    TJS,
    TMT,
    TND,
    TOP,
    TRY,
    TTD,
    TWD,
    TZS,
    UAH,
    UGX,
    USN,
    UYI,
    UYU,
    UYW,
    UZS,
    VED,
    VES,
    VND,
    VUV,
    WST,
    XAG,
    XAU,
    XBA,
    XBB,
    XBC,
    XBD,
    XDR,
    XPD,
    XPT,
    XSU,
    XTS,
    XUA,
    XXX,
    YER,
    ZMW,
    ZWL,
}
