use std::{sync::Arc, time::Duration};

use auth::token::KEYS;
use axum::{
    error_handling::HandleErrorLayer,
    http::StatusCode,
    routing::{get, post},
    BoxError, Router,
};
use common_utils::token::PublicKeys;
use database::Database;
use tokio::sync::Mutex;
use tower::ServiceBuilder;
use tower_http::add_extension::AddExtensionLayer;

mod auth;
mod database;
mod env;
mod onboard;

use env::LISTENING_ADDRESS;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let shared_state = auth::State::new().await.expect("Could not connect to Dapr");
    let shared_state = Arc::new(Mutex::new(shared_state));

    let database = Arc::new(Mutex::new(Database::new().await));

    let public_keys = Arc::new(Mutex::new(PublicKeys::default()));

    let app = Router::new()
        // ========== PUBLIC PAGES ==========
        .route("/auth/signup", post(auth::signup))
        .route("/auth/login", post(auth::login))
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
        .route(
            "/onboard/admin-details",
            post(onboard::individual::account_details),
        )
        .route("/auth/access-token", post(auth::access_token))
        .route("/auth/public-key", get(auth::public_key))
        .layer(
            ServiceBuilder::new()
                .layer(HandleErrorLayer::new(handle_error))
                .load_shed()
                .concurrency_limit(1024)
                .timeout(Duration::from_secs(10))
                .layer(AddExtensionLayer::new(database))
                .layer(AddExtensionLayer::new(shared_state))
                .layer(AddExtensionLayer::new(KEYS.decoding.clone()))
                .layer(AddExtensionLayer::new(public_keys)),
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
