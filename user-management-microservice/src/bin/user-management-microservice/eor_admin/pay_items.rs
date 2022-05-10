//! this module performs the same functions as branch/pay_items.rs from eor-admin

use crate::branch::pay_items::{CreatePayItem, PayItem, PayItemsIndexQuery};
use crate::database::SharedDatabase;

use axum::extract::{Extension, Json, Path, Query};

use common_utils::{error::GlobeliseResult, token::Token};
use eor_admin_microservice_sdk::token::AccessToken as AdminAccessToken;
use rusty_ulid::Ulid;

pub async fn get_pay_items(
    // Only for validation
    _: Token<AdminAccessToken>,
    Query(request): Query<PayItemsIndexQuery>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<Vec<PayItem>>> {
    let database = database.lock().await;

    let pay_items = database.get_pay_items(request).await?;

    Ok(Json(pay_items))
}

pub async fn create_pay_item(
    // Only for validation
    _: Token<AdminAccessToken>,
    Json(pay_item): Json<CreatePayItem>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    let database = database.lock().await;

    database.create_pay_item(pay_item).await?;

    Ok(())
}

pub async fn update_pay_item(
    // Only for validation
    _: Token<AdminAccessToken>,
    Json(pay_item): Json<PayItem>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    let database = database.lock().await;

    database.update_pay_item(pay_item).await?;

    Ok(())
}

pub async fn delete_pay_item(
    // Only for validation
    _: Token<AdminAccessToken>,
    Path(pay_item_ulid): Path<Ulid>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    let database = database.lock().await;

    database.delete_pay_item(pay_item_ulid).await?;

    Ok(())
}

pub async fn get_pay_item_by_id(
    // Only for validation
    _: Token<AdminAccessToken>,
    Path(pay_item_ulid): Path<Ulid>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<PayItem>> {
    let database = database.lock().await;

    let pay_item = database.get_pay_item_by_id(pay_item_ulid).await?;

    Ok(Json(pay_item))
}