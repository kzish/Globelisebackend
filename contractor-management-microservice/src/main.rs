use std::{sync::Arc, time::Duration};

use axum::{error_handling::HandleErrorLayer, http::StatusCode, routing::get, BoxError, Router};
use database::Database;
use state::State;
use tokio::sync::Mutex;
use tower::ServiceBuilder;
use tower_http::add_extension::AddExtensionLayer;

mod auth;
mod contracts;
mod database;
mod env;
mod error;
mod microservices;
mod state;

use env::LISTENING_ADDRESS;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let shared_state = State::new().await.expect("Could not connect to Dapr");
    let shared_state = Arc::new(Mutex::new(shared_state));

    let database = Database::new().await;
    let database = Arc::new(Mutex::new(database));

    let app = Router::new()
        // ========== PUBLIC PAGES ==========
        .route("/users/index", get(contracts::user_index))
        // ========== DEBUG PAGES ==========
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
