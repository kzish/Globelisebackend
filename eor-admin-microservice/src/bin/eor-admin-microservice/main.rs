use std::{sync::Arc, time::Duration};

use auth::token::KEYS;
use axum::{
    error_handling::HandleErrorLayer,
    extract::Extension,
    http::{HeaderValue, Method, StatusCode},
    routing::{get, post},
    BoxError, Router,
};
use common_utils::token::PublicKeys;
use database::Database;
use tokio::sync::Mutex;
use tower::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer, Origin};

mod auth;
mod database;
mod env;
mod onboard;

use env::{FRONTEND_URL, LISTENING_ADDRESS};

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
        .route("/auth/google/signup", post(auth::google::signup))
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

async fn handle_healthz() -> String {
    if let Some(v) = option_env!("GIT_HASH") {
        v.to_string()
    } else if let Ok(v) = std::env::var("GIT_HASH") {
        v
    } else {
        env!("CARGO_PKG_VERSION").to_string()
    }
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
