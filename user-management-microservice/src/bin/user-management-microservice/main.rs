use std::{sync::Arc, time::Duration};

use axum::{
    error_handling::HandleErrorLayer,
    extract::Extension,
    http::{HeaderValue, Method, StatusCode},
    routing::{get, post},
    BoxError, Router,
};
use common_utils::{pubsub::PubSub, token::PublicKeys};
use database::Database;
use reqwest::Client;
use tokio::sync::Mutex;
use tower::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer, Origin};

mod auth;
mod database;
mod env;
mod eor_admin;
mod onboard;

use crate::auth::token::KEYS;
use env::{FRONTEND_URL, LISTENING_ADDRESS};

static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let dapr_address: String = std::env::var("USER_MANAGEMENT_MICROSERVICE_DOMAIN_URL")
        .unwrap()
        .parse()
        .unwrap();

    let shared_state = auth::State::new().await.expect("Could not connect to Dapr");
    let shared_state = Arc::new(Mutex::new(shared_state));

    let database = Arc::new(Mutex::new(Database::new().await));

    let public_keys = Arc::new(Mutex::new(PublicKeys::default()));

    let shared_reqwest_client = Client::builder()
        .user_agent(APP_USER_AGENT)
        .build()
        .unwrap();

    let shared_pubsub = Arc::new(Mutex::new(PubSub::new(shared_reqwest_client, dapr_address)));

    let app = Router::new()
        // ========== PUBLIC PAGES ==========
        .route("/auth/signup/:user_type", post(auth::signup))
        .route("/auth/login", post(auth::login))
        .route("/auth/google/signup/:user_type", post(auth::google::signup))
        .route("/auth/google/login", post(auth::google::login))
        .route(
            "/auth/password/reset/email",
            post(auth::password::reset::send_email),
        )
        .route(
            "/auth/password/reset/initiate",
            get(auth::password::reset::initiate),
        )
        .route(
            "/auth/password/reset/execute",
            post(auth::password::reset::execute),
        )
        .route("/auth/access-token", post(auth::access_token))
        .route("/auth/public-key", get(auth::public_key))
        .route(
            "/onboard/individual-details/client",
            post(onboard::individual::client_account_details),
        )
        .route(
            "/onboard/individual-details/contractor",
            post(onboard::individual::contractor_account_details),
        )
        .route(
            "/onboard/entity-details/client",
            post(onboard::entity::client_account_details),
        )
        .route(
            "/onboard/entity-details/contractor",
            post(onboard::entity::contractor_account_details),
        )
        .route(
            "/onboard/pic-details/:role",
            post(onboard::entity::pic_details),
        )
        .route("/onboard/bank-details", post(onboard::bank::bank_details))
        .route(
            "/onboard/payment-details",
            post(onboard::bank::payment_details),
        )
        .route(
            "/onboard/fully_onboarded/:role",
            get(onboard::fully_onboarded),
        )
        // ========== ADMIN APIS ==========
        .route(
            "/eor-admin/onboarded-users",
            get(eor_admin::eor_admin_onboarded_user_index),
        )
        .route("/eor-admin/users", get(eor_admin::eor_admin_user_index))
        .route(
            "/eor-admin/users/create_client_contractor_pairs",
            post(eor_admin::eor_admin_create_client_contractor_pairs),
        )
        .route(
            "/eor-admin/users/add_individual_contractor",
            post(eor_admin::add_individual_contractor),
        )
        .route(
            "/eor-admin/users/add_bulk_employees",
            post(eor_admin::eor_admin_add_employees_in_bulk),
        )
        .route(
            "/eor-admin/users/onboard/prefill_individual_contractor_account_details",
            post(onboard::prefill::prefill_individual_contractor_account_details),
        )
        .route(
            "/eor-admin/users/onboard/prefill_individual_contractor_bank_details",
            post(onboard::prefill::prefill_individual_contractor_bank_details),
        )
        // ========== DEBUG PAGES ==========
        .route("/debug/google/login", get(auth::google::login_page))
        .route("/healthz", get(handle_healthz))
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
                                is_valid |= origin.as_bytes().starts_with(b"http://localhost");
                            }

                            is_valid
                        }))
                        .allow_methods(vec![Method::GET, Method::POST])
                        .allow_credentials(true)
                        .allow_headers(Any),
                )
                .layer(Extension(database))
                .layer(Extension(shared_state))
                .layer(Extension(KEYS.decoding.clone()))
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

async fn handle_healthz() -> &'static str {
    dbg!(option_env!("GIT_HASH").unwrap_or(env!("CARGO_PKG_VERSION")))
}

/// Handles errors from fallible services.
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
