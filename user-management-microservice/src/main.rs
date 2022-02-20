//! User management microservice.
//!
//! Requirements:
//!     - Dapr for state storage
//!     - RSA key pair files `private.pem` and `public.pem`
//!     - GOOGLE_CLIENT_ID environment variable

use std::{sync::Arc, time::Duration};

use axum::{
    error_handling::HandleErrorLayer,
    http::StatusCode,
    routing::{get, post},
    BoxError, Router,
};
use tokio::sync::Mutex;
use tower::ServiceBuilder;
use tower_http::add_extension::AddExtensionLayer;

mod auth;
mod env;

use env::LISTENING_ADDRESS;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let shared_state = auth::State::new().await.expect("Could not connect to Dapr");
    let shared_state = Arc::new(Mutex::new(shared_state));

    let database = auth::Database::new().await;
    let database = Arc::new(Mutex::new(database));

    let app = Router::new()
        // ========== PUBLIC PAGES ==========
        .route("/signup/:role", post(auth::create_account))
        .route("/login/:role", post(auth::login))
        .route("/lostpassword/:role", post(auth::password::lost_password))
        .route(
            "/changepassword/:role",
            post(auth::password::change_password),
        )
        .route("/google/login/:role", post(auth::google::login))
        .route("/google/authorize", post(auth::google::get_refresh_token))
        .route("/auth/refresh", post(auth::renew_access_token))
        .route("/auth/keys", get(auth::public_key))
        .route(
            "/onboard/individual_details",
            post(auth::onboarding::individual_details),
        )
        .route(
            "/onboard/entity_details",
            post(auth::onboarding::entity_details),
        )
        .route("/onboard/pic_details", post(auth::onboarding::pic_details))
        .route(
            "/onboard/bank_details",
            post(auth::onboarding::bank_details),
        )
        // ========== DEBUG PAGES ==========
        .route("/google/loginpage", get(auth::google::login_page))
        .route(
            "/changepasswordpage/:role",
            get(auth::password::change_password_page),
        )
        .route(
            "/lostpasswordpage/:role",
            get(auth::password::lost_password_page),
        )
        .layer(
            ServiceBuilder::new()
                .layer(HandleErrorLayer::new(handle_error))
                .load_shed()
                .concurrency_limit(1024)
                .timeout(Duration::from_secs(10))
                .layer(AddExtensionLayer::new(database))
                .layer(AddExtensionLayer::new(shared_state)),
        );

    axum::Server::bind(
        &(*LISTENING_ADDRESS)
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
