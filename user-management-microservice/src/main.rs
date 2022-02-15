//! User management microservice.
//!
//! Requirements:
//!     - Dapr for state storage
//!     - RSA key pair files `private.pem` and `public.pem`
//!     - GOOGLE_CLIENT_ID environment variable

use std::{net::SocketAddr, sync::Arc, time::Duration};

use axum::{
    error_handling::HandleErrorLayer,
    http::StatusCode,
    routing::{get, post},
    BoxError, Router,
};
use dotenv;
use tokio::sync::Mutex;
use tower::ServiceBuilder;
use tower_http::add_extension::AddExtensionLayer;

mod auth;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let shared_state = auth::State::new().await.expect("Could not connect to Dapr");
    let shared_state = Arc::new(Mutex::new(shared_state));

    let app = Router::new()
        .route("/signup/:role", post(auth::create_account))
        .route("/login/:role", post(auth::login))
        .route("/google/loginpage", get(auth::google::login_page))
        .route("/google/login/:role", post(auth::google::login))
        .route("/google/authorize", post(auth::google::get_refresh_token))
        .route("/auth/refresh", post(auth::renew_access_token))
        .route("/auth/keys", get(auth::public_key))
        .layer(
            ServiceBuilder::new()
                .layer(HandleErrorLayer::new(handle_error))
                .load_shed()
                .concurrency_limit(1024)
                .timeout(Duration::from_secs(100))
                .layer(AddExtensionLayer::new(shared_state)),
        );

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
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
