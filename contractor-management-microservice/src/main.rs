use std::{sync::Arc, time::Duration};

use axum::{
    error_handling::HandleErrorLayer,
    extract::Extension,
    http::{HeaderValue, Method, StatusCode},
    routing::get,
    BoxError, Router,
};
use common_utils::token::PublicKeys;
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

use env::{FRONTEND_URL, LISTENING_ADDRESS};

static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let reqwest_client = Client::builder()
        .user_agent(APP_USER_AGENT)
        .build()
        .unwrap();

    let database = Arc::new(Mutex::new(Database::new().await));

    let public_keys = Arc::new(Mutex::new(PublicKeys::default()));

    let app = Router::new()
        // ========== PUBLIC PAGES ==========
        .route("/clients", get(contracts::clients_index))
        .route("/contractors", get(contracts::contractors_index))
        .route("/contracts/:role", get(contracts::contracts_index))
        .route("/payslips/:role", get(payslips::user_payslips_index))
        .route("/tax-reports/:role", get(tax_report::user_tax_report_index))
        .route(
            "/invoices/individual/:role",
            get(invoice::user_invoice_individual_index),
        )
        .route(
            "/invoices/group/:role",
            get(invoice::user_invoice_group_index),
        )
        // ========== ADMIN PAGES ==========
        .route("/eor-admin/users", get(contracts::user_index))
        .route(
            "/eor-admin/payslips",
            get(payslips::eor_admin_payslips_index).post(payslips::eor_admin_create_payslip),
        )
        .route(
            "/eor-admin/tax-reports",
            get(tax_report::eor_admin_tax_report_index)
                .post(tax_report::eor_admin_create_tax_report),
        )
        .route(
            "/eor-admin/contracts",
            get(contracts::eor_admin_contracts_index),
        )
        .route(
            "/eor-admin/invoices/individual",
            get(invoice::eor_admin_invoice_individual_index),
        )
        .route(
            "/eor-admin/invoices/group",
            get(invoice::eor_admin_invoice_group_index),
        )
        // ========== DEBUG PAGES ==========
        .layer(
            ServiceBuilder::new()
                .layer(HandleErrorLayer::new(handle_error))
                .load_shed()
                .concurrency_limit(1024)
                .timeout(Duration::from_secs(10))
                .layer(
                    CorsLayer::new()
                        .allow_origin(Origin::predicate(|origin: &HeaderValue, _| {
                            let mut is_valid = origin == *FRONTEND_URL;

                            #[cfg(debug_assertions)]
                            {
                                is_valid |= origin.as_bytes().starts_with(b"http://localhost:");
                            }

                            is_valid
                        }))
                        .allow_methods(vec![Method::GET, Method::POST])
                        .allow_credentials(true)
                        .allow_headers(Any),
                )
                .layer(Extension(database))
                .layer(Extension(reqwest_client))
                .layer(Extension(public_keys)),
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
