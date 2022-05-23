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
    pubsub::{CreateOrUpdateContracts, PubSub, PubSubData, TopicSubscription},
    token::PublicKeys,
};
use database::Database;
use reqwest::Client;
use tokio::sync::Mutex;
use tower::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer, Origin};

mod auth;
mod branch;
mod bulk_add;
mod custom_field;
mod database;
mod department;
mod employee_contractors;
mod env;
mod eor_admin;
mod onboard;
mod prefill;
mod pubsub;

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
            get(onboard::individual::get_onboard_individual_client_account_details)
                .post(onboard::individual::post_onboard_individual_client_account_details),
        )
        .route(
            "/onboard/individual-details/contractor",
            get(onboard::individual::get_onboard_individual_contractor_account_details)
                .post(onboard::individual::post_onboard_individual_contractor_account_details),
        )
        .route(
            "/onboard/entity-details/client",
            get(onboard::entity::get_onboard_entity_client_account_details)
                .post(onboard::entity::post_onboard_entity_client_account_details),
        )
        .route(
            "/onboard/entity-details/contractor",
            get(onboard::entity::get_onboard_entity_contractor_account_details)
                .post(onboard::entity::post_onboard_entity_contractor_account_details),
        )
        .route(
            "/onboard/pic-details/:role",
            get(onboard::pic::get_onboard_entity_pic_details)
                .post(onboard::pic::post_onboard_entity_pic_details),
        )
        .route(
            "/onboard/bank-details",
            get(onboard::bank::get_onboard_contractor_bank_details)
                .post(onboard::bank::post_onboard_contractor_bank_details),
        )
        .route(
            "/onboard/payment-details",
            post(onboard::payment::onboard_client_payment_details),
        )
        .route(
            "/client/branch",
            get(branch::user_get_branches)
                .post(branch::user_post_branch)
                .delete(branch::user_delete_branch),
        )
        .route(
            "/client/branch/:branch_ulid",
            get(branch::user_get_branch_by_ulid),
        )
        .route(
            "/client/branch/branch-details",
            get(branch::account::get_branch_account_details)
                .post(branch::account::post_branch_account_details),
        )
        .route(
            "/client/branch/bank-details",
            get(branch::bank::get_branch_bank_details).post(branch::bank::post_branch_bank_details),
        )
        .route(
            "/client/branch/payroll-details",
            get(branch::payroll::get_branch_payroll_details)
                .post(branch::payroll::post_branch_payroll_details),
        )
        .route(
            "/client/department",
            get(department::user_get_departments).post(department::user_post_department),
        )
        .route(
            "/client/custom-field",
            get(custom_field::user_get_custom_fields).post(custom_field::user_post_custom_field),
        )
        .route(
            "/client/branch/pay-items",
            get(branch::pay_items::get_pay_items)
                .post(branch::pay_items::create_pay_item)
                .put(branch::pay_items::update_pay_item),
        )
        .route(
            "/client/branch/pay-items/:pay_item_ulid",
            get(branch::pay_items::get_pay_item_by_id).delete(branch::pay_items::delete_pay_item),
        )
        .route(
            "/client/prefill/individual_contractor_account_details",
            get(prefill::account::individual_contractor_get_one)
                .post(prefill::account::individual_contractor_post_one),
        )
        .route(
            "/client/prefill/individual_contractor_bank_details",
            get(prefill::bank::individual_contractor_get_one)
                .post(prefill::bank::individual_contractor_post_one),
        )
        .route(
            "/client-contractors/search",
            get(employee_contractors::search_employee_contractors::get_employee_contractors),
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
            get(eor_admin::client_contractor_pair::get_many)
                .post(eor_admin::client_contractor_pair::post_one),
        )
        .route(
            "/eor-admin/users/individual/contractor_branch_pairs",
            get(eor_admin::individual_contractor_branch_pair::get_many)
                .post(eor_admin::individual_contractor_branch_pair::post_one)
                .delete(eor_admin::individual_contractor_branch_pair::delete_one),
        )
        .route(
            "/eor-admin/users/entity/contractor_branch_pairs",
            get(eor_admin::entity_contractor_branch_pair::get_many)
                .post(eor_admin::entity_contractor_branch_pair::post_one)
                .delete(eor_admin::entity_contractor_branch_pair::delete_one),
        )
        .route(
            "/eor-admin/users/add_individual_contractor",
            post(eor_admin::add_individual_contractor),
        )
        .route(
            "/eor-admin/users/add_bulk_employees",
            get(bulk_add::get_one).post(bulk_add::post_one),
        )
        .route(
            "/eor-admin/users/onboard/prefill_individual_contractor_account_details",
            get(eor_admin::prefill::account::individual_contractor_get_one)
                .post(eor_admin::prefill::account::individual_contractor_post_one),
        )
        .route(
            "/eor-admin/users/onboard/prefill_individual_contractor_bank_details",
            get(eor_admin::prefill::bank::individual_contractor_get_one)
                .post(eor_admin::prefill::bank::individual_contractor_post_one),
        )
        .route(
            "/eor-admin/entities/onboard/prefill-entity-client",
            get(eor_admin::prefill::account::entity_client_get_one)
                .post(eor_admin::prefill::account::entity_client_post_one),
        )
        .route(
            "/eor-admin/entities/onboard/prefill-entity-client/pic-details",
            get(eor_admin::prefill::pic::entity_client_get_one)
                .post(eor_admin::prefill::pic::entity_client_post_one),
        )
        .route(
            "/eor-admin/entities/onboard/prefill-entity-client/bank-details",
            get(eor_admin::prefill::bank::entity_client_get_one)
                .post(eor_admin::prefill::bank::entity_client_post_one),
        )
        .route(
            "/eor-admin/entities/onboard/prefill-entity-client/payment-details",
            get(eor_admin::prefill::payment::entity_client_get_one)
                .post(eor_admin::prefill::payment::entity_client_post_one),
        )
        .route(
            "/eor-admin/department",
            get(department::eor_admin_get_departments).post(department::eor_admin_post_department),
        )
        .route(
            "/eor-admin/custom-field",
            get(custom_field::admin_get_custom_fields).post(custom_field::admin_post_custom_field),
        )
        .route(
            "/eor-admin/custom-field/:ulid",
            get(custom_field::admin_get_custom_field_by_ulid),
        )
        .route(
            "/eor-admin/client/branch/pay-items",
            get(eor_admin::pay_items::get_pay_items)
                .post(eor_admin::pay_items::create_pay_item)
                .put(eor_admin::pay_items::update_pay_item),
        )
        .route(
            "/eor-admin/client/branch/pay-items/:pay_item_ulid",
            get(eor_admin::pay_items::get_pay_item_by_id)
                .delete(eor_admin::pay_items::delete_pay_item),
        )
        .route(
            "/eor-admin/branch",
            get(branch::admin_get_branches)
                .post(branch::admin_post_branch)
                .delete(branch::admin_delete_branch),
        )
        .route(
            "/eor-admin/branch/:branch_ulid",
            get(branch::admin_get_branch_by_ulid),
        )
        .route(
            "/eor-admin/client-contractors/search",
            get(eor_admin::search_employee_contractors::eor_admin_get_employee_contractors),
        )
        // ========== PUBSUB PAGES ==========
        .route("/dapr/subscribe", get(dapr_subscription_list))
        .route(
            "/pubsub/create_or_update_contracts",
            post(pubsub::create_or_update_contracts),
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

/// DAPR will invoke this endpoint to know which pubsub and topic names this app
/// will listen to.
pub async fn dapr_subscription_list() -> GlobeliseResult<Json<Vec<TopicSubscription>>> {
    Ok(Json(vec![
        CreateOrUpdateContracts::create_topic_subscription("pubsub/create_or_update_contracts"),
    ]))
}
