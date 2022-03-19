use std::{sync::Arc, time::Duration};

use axum::{
    error_handling::HandleErrorLayer,
    http::{HeaderValue, Method, StatusCode},
    routing::{get, post},
    BoxError, Router,
};
use database::Database;
use tokio::sync::Mutex;
use tower::ServiceBuilder;
use tower_http::{
    add_extension::AddExtensionLayer,
    cors::{Any, CorsLayer, Origin},
};

mod auth;
mod database;
mod env;
mod eor_admin;
mod onboard;

use env::{FRONTEND_URL, LISTENING_ADDRESS};

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let shared_state = auth::State::new().await.expect("Could not connect to Dapr");
    let shared_state = Arc::new(Mutex::new(shared_state));

    let database = Arc::new(Mutex::new(Database::new().await));

    let app = Router::new()
        // ========== PUBLIC PAGES ==========
        .route("/auth/signup/:user_type", post(auth::signup))
        .route("/auth/login/:user_type", post(auth::login))
        .route("/auth/google/login/:user_type", post(auth::google::login))
        .route(
            "/auth/password/reset/email/:user_type",
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
            "/onboard/individual-details/:role",
            post(onboard::individual::account_details),
        )
        .route(
            "/onboard/entity-details/:role",
            post(onboard::entity::account_details),
        )
        .route(
            "/onboard/pic-details/:role",
            post(onboard::entity::pic_details),
        )
        .route("/onboard/bank-details", post(onboard::bank::bank_details))
        // ========== ADMIN APIS ==========
        .route("/eor-admin/users/index", get(eor_admin::user_index))
        .route(
            "/eor-admin/users/add_individual_contractor",
            post(eor_admin::add_individual_contractor),
        )
        .route(
            "/eor-admin/users/onboard/prefill_individual_contractor",
            post(onboard::individual::prefill_individual_contractor_account_details),
        )
        // ========== DEBUG PAGES ==========
        .route("/debug/google/login", get(auth::google::login_page))
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
                .layer(AddExtensionLayer::new(database))
                .layer(AddExtensionLayer::new(shared_state)),
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
