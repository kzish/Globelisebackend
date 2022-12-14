use std::{sync::Arc, time::Duration};

use axum::{
    error_handling::HandleErrorLayer,
    extract::Extension,
    http::{HeaderValue, Method, StatusCode},
    routing::{get, post},
    BoxError, Json, Router,
};
use common_utils::{
    error::GlobeliseResult,
    pubsub::{PubSub, TopicSubscription},
    token::PublicKeys,
};
use database::Database;
use reqwest::Client;
use tokio::sync::Mutex;
use tower::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer, Origin};

mod common;
mod contracts;
mod database;
mod env;
mod invoice;
mod payslips;
mod tax_report;

use env::{DAPR_ADDRESS, DATABASE_URL, FRONTEND_URL, LISTENING_ADDRESS};

static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let reqwest_client = Client::builder()
        .user_agent(APP_USER_AGENT)
        .build()
        .unwrap();

    let shared_database = Arc::new(Mutex::new(Database::new().await));
    let common_database = Arc::new(Mutex::new(
        common_utils::database::Database::new(&*DATABASE_URL).await,
    ));

    let public_keys = Arc::new(Mutex::new(PublicKeys::default()));

    let shared_pubsub = Arc::new(Mutex::new(PubSub::new(
        reqwest_client.clone(),
        DAPR_ADDRESS.clone(),
    )));

    let app = Router::new()
        // ========== PUBLIC PAGES ==========
        .route(
            "/clients",
            get(contracts::user_get_many_clients_for_contractors),
        )
        .route(
            "/contractors",
            get(contracts::user_get_many_contractors_for_clients),
        )
        .route("/payslips/:role", get(payslips::user_find_many_payslips))
        .route(
            "/payslips/:role/:payslip_ulid",
            get(payslips::user_get_one_payslip_index).delete(payslips::user_delete_one_payslip),
        )
        .route(
            "/payslips/:role/:payslip_ulid/file",
            get(payslips::user_download_one_payslip_index),
        )
        .route(
            "/tax-reports/:role",
            get(tax_report::user_get_many_tax_report_index),
        )
        .route(
            "/tax-reports/:role/:tax_report_ulid",
            get(tax_report::user_get_many_tax_report_index),
        )
        .route(
            "/invoices/individual/:role",
            get(invoice::user_invoice_individual_index),
        )
        .route(
            "/invoices/group/:role",
            get(invoice::user_invoice_group_index),
        )
        //contracts-contractors
        .route(
            "/contracts/contractor",
            get(contracts::contractor_list_contracts),
        )
        .route(
            "/contracts/contractor/sign",
            post(contracts::contractor_sign_contract),
        )
        .route(
            "/contracts/contractor/revoke-sign",
            post(contracts::contractor_revoke_sign_contract),
        )
        .route(
            "/contracts/contractor/get-combine-single-contract-index",
            get(contracts::contractor_get_combine_single_contract_index),
        )
        //contracts-clients
        .route(
            "/contracts/client",
            get(contracts::client_list_contracts)
                .post(contracts::client_post_update_contract)
                .delete(contracts::client_delete_contract),
        )
        .route(
            "/contracts/client/get-combine-single-contract-index",
            get(contracts::client_get_combine_single_contract_index),
        )
        .route(
            "/contracts/client/sign",
            post(contracts::client_sign_contract),
        )
        .route(
            "/contracts/client/revoke-sign",
            post(contracts::client_revoke_sign_contract),
        )
        .route(
            "/contracts/client/invite-contractor",
            post(contracts::client_invite_contractor),
        )
        .route(
            "/contracts/eor-admin/activate-contract",
            post(contracts::admin_activate_contract_to_draft),
        )
        .route(
            "/contracts/client/activate-contract",
            post(contracts::client_activate_contract_to_draft),
        )
        .route(
            "/contracts/eor-admin/cancel-contract",
            post(contracts::admin_permanantly_cancel_contract),
        )
        .route(
            "/contracts/client/cancel-contract",
            post(contracts::client_permanantly_cancel_contract),
        )
        // ========== ADMIN PAGES ==========
        .route(
            "/eor-admin/payslips",
            get(payslips::admin_get_many_payslip_index).post(payslips::admin_post_one_payslip),
        )
        .route(
            "/eor-admin/payslips/:payslip_ulid",
            get(payslips::admin_get_one_payslip_index).delete(payslips::admin_delete_one_payslip),
        )
        .route(
            "/eor-admin/payslips/:payslip_ulid/file",
            get(payslips::admin_download_one_payslip_index),
        )
        .route(
            "/eor-admin/tax-reports",
            get(tax_report::admin_get_many_tax_report_index)
                .post(tax_report::admin_post_one_tax_report),
        )
        .route(
            "/eor-admin/tax-reports/:tax_report_ulid",
            get(tax_report::admin_get_one_tax_report_index),
        )
        .route(
            "/eor-admin/invoices/individual",
            get(invoice::eor_admin_invoice_individual_index),
        )
        .route(
            "/eor-admin/invoices/group",
            get(invoice::eor_admin_invoice_group_index),
        )
        // ========== PUBSUB PAGES ==========
        .route("/dapr/subscribe", get(dapr_subscription_list))
        // ========== DEBUG PAGES ==========
        .route("/healthz", get(handle_healthz))
        // ========== CONFIGURATIONS ==========
        .layer(
            ServiceBuilder::new()
                .layer(HandleErrorLayer::new(handle_error))
                .load_shed()
                .concurrency_limit(1024)
                .timeout(Duration::from_secs(10))
                .layer(
                    CorsLayer::new()
                        .allow_origin(Origin::predicate(|origin: &HeaderValue, _| {
                            let mut is_valid =
                                origin == HeaderValue::from_str(&*FRONTEND_URL).unwrap();

                            #[cfg(debug_assertions)]
                            {
                                is_valid |= origin.as_bytes().starts_with(b"http://localhost");
                            }

                            is_valid
                        }))
                        .allow_methods(vec![Method::GET, Method::POST])
                        .allow_credentials(true)
                        .allow_headers(Any),
                )
                .layer(Extension(shared_database))
                .layer(Extension(common_database))
                .layer(Extension(reqwest_client))
                .layer(Extension(public_keys))
                .layer(Extension(shared_pubsub)),
        );

    axum::Server::bind(
        &(*LISTENING_ADDRESS)
            .replace("localhost", "127.0.0.1")
            .parse()
            .expect("Invalid listening address"),
    )
    .serve(app.into_make_service())
    .await
    .unwrap();
}

async fn handle_healthz() -> String {
    if let Some(v) = option_env!("GIT_HASH") {
        v.to_string()
    } else if let Some(v) = option_env!("CI_COMMIT_SHA") {
        v.to_string()
    } else {
        "Healthy".to_string()
    }
}

/// Handle errors from fallible services.
async fn handle_error(error: BoxError) -> (StatusCode, &'static str) {
    if error.is::<tower::timeout::error::Elapsed>() {
        (StatusCode::REQUEST_TIMEOUT, "Request timed out")
    } else if error.is::<tower::load_shed::error::Overloaded>() {
        (
            StatusCode::SERVICE_UNAVAILABLE,
            "Service is overloaded, try again later",
        )
    } else {
        (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
    }
}

/// DAPR will invoke this endpoint to know which pubsub and topic names this app
/// will listen to.
pub async fn dapr_subscription_list() -> GlobeliseResult<Json<Vec<TopicSubscription>>> {
    Ok(Json(vec![]))
}
