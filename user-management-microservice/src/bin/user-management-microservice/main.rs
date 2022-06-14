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

mod auth;
mod branch;
mod bulk_add;
mod contractor_account_settings;
mod custom_field;
mod database;
mod department;
mod employee_contractors;
mod env;
mod eor_admin;
mod notification;
mod onboard;
mod prefill;

use crate::auth::token::KEYS;
use env::{DAPR_ADDRESS, FRONTEND_URL, LISTENING_ADDRESS};

static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let shared_state = auth::State::new().await.expect("Could not connect to Dapr");
    let shared_state = Arc::new(Mutex::new(shared_state));

    let database = Arc::new(Mutex::new(Database::new().await));

    let public_keys = Arc::new(Mutex::new(PublicKeys::default()));

    let shared_reqwest_client = Client::builder()
        .user_agent(APP_USER_AGENT)
        .build()
        .unwrap();

    let shared_pubsub = Arc::new(Mutex::new(PubSub::new(
        shared_reqwest_client.clone(),
        DAPR_ADDRESS.clone(),
    )));

    let app = Router::new()
        // ========== PUBLIC PAGES ==========
        .route("/auth/signup/:user_type", post(auth::signup))
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
            get(onboard::payment::get_onboard_client_payment_details)
                .post(onboard::payment::post_onboard_client_payment_details),
        )
        .route(
            "/client/branch",
            get(branch::user::get_many_branches)
                .post(branch::user::post_one_branch)
                .delete(branch::user::delete_one_branch),
        )
        .route(
            "/client/branch/:branch_ulid",
            get(branch::user::get_one_branch_by_ulid),
        )
        .route(
            "/client/branch/:branch_ulid/branch-details",
            get(branch::account::get_branch_account_details)
                .post(branch::account::post_branch_account_details),
        )
        .route(
            "/client/branch/:branch_ulid/bank-details",
            get(branch::bank::get_branch_bank_details).post(branch::bank::post_branch_bank_details),
        )
        .route(
            "/client/branch/:branch_ulid/payroll-details",
            get(branch::payroll::user_get_branch_payroll_details)
                .post(branch::payroll::user_post_branch_payroll_details),
        )
        .route(
            "/client/branch/individual-contractors",
            get(branch::user::get_many_individual_contractors)
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
        .route(
            "/users/notifications",
            get(notification::get_many).
            put(notification::put_one),
        )
        // ========== contractor account settings (contractors routes) ==========
        .route(
            "/account-settings/contractors/bank-details/entity",
            get(contractor_account_settings::contractor::bank_details::get_bank_details_entity)
            .post(contractor_account_settings::contractor::bank_details::post_bank_details_entity),
        )
        .route(
            "/account-settings/contractors/bank-details/individual",
            get(contractor_account_settings::contractor::bank_details::get_bank_details_individual)
            .post(contractor_account_settings::contractor::bank_details::post_bank_details_individual),
        )
        .route(
            "/account-settings/contractors/personal-information/entity",
            get(contractor_account_settings::contractor::personal_information::get_profile_settings_entity)
            .post(contractor_account_settings::contractor::personal_information::post_profile_settings_entity),
        )
        .route(
            "/account-settings/contractors/personal-information/individual",
            get(contractor_account_settings::contractor::personal_information::get_profile_settings_individual)
            .post(contractor_account_settings::contractor::personal_information::post_profile_settings_individual),
        )
        // ========== contractor account settings (pic routes) ==========
        .route(
            "/admin-pic/account-settings/contractors/bank-details/entity/:contractor_ulid",
            get(contractor_account_settings::client_pic::bank_details::get_bank_details_entity)
            .post(contractor_account_settings::client_pic::bank_details::post_bank_details_entity),
        )
        .route(
            "/admin-pic/account-settings/contractors/bank-details/individual/:contractor_ulid",
            get(contractor_account_settings::client_pic::bank_details::get_bank_details_individual)
            .post(contractor_account_settings::client_pic::bank_details::post_bank_details_individual),
        )
        .route(
            "/admin-pic/account-settings/contractors/personal-information/entity/:contractor_ulid",
            get(contractor_account_settings::client_pic::personal_information::get_profile_settings_entity)
            .post(contractor_account_settings::client_pic::personal_information::post_profile_settings_entity),
        )
        .route(
            "/admin-pic/account-settings/contractors/personal-information/individual/:contractor_ulid",
            get(contractor_account_settings::client_pic::personal_information::get_profile_settings_individual)
            .post(contractor_account_settings::client_pic::personal_information::post_profile_settings_individual),
        )
         .route(
            "/admin-pic/account-settings/contractors/payroll-information/entity/:contractor_ulid",
            get(contractor_account_settings::client_pic::payroll_information::get_payroll_information_entity)
            .post(contractor_account_settings::client_pic::payroll_information::post_payroll_information_entity),
        )
        .route(
            "/admin-pic/admin-pic/account-settings/contractors/payroll-information/individual/:contractor_ulid",
            get(contractor_account_settings::client_pic::payroll_information::get_payroll_information_individual)
            .post(contractor_account_settings::client_pic::payroll_information::post_payroll_information_individual),
        )
        .route(
            "/admin-pic/account-settings/contractors/employment-information/entity/:contractor_ulid",
            get(contractor_account_settings::client_pic::employment_information::get_employment_information_entity)
            .post(contractor_account_settings::client_pic::employment_information::post_employment_information_entity),
        )
        .route(
            "/admin-pic/account-settings/contractors/employment-information/individual/:contractor_ulid",
            get(contractor_account_settings::client_pic::employment_information::get_employment_information_individual)
            .post(contractor_account_settings::client_pic::employment_information::post_employment_information_individual),
        )

    // ========== contractor account settings (eor admin routes) ==========
        .route(
            "/eor-admin/account-settings/contractors/bank-details/entity/:contractor_ulid",
            get(contractor_account_settings::eor_admin::bank_details::get_bank_details_entity)
            .post(contractor_account_settings::eor_admin::bank_details::post_bank_details_entity),
        )
        .route(
            "/eor-admin/account-settings/contractors/bank-details/individual/:contractor_ulid",
            get(contractor_account_settings::eor_admin::bank_details::get_bank_details_individual)
            .post(contractor_account_settings::eor_admin::bank_details::post_bank_details_individual),
        )
        .route(
            "/eor-admin/account-settings/contractors/personal-information/entity/:contractor_ulid",
            get(contractor_account_settings::eor_admin::personal_information::get_profile_settings_entity)
            .post(contractor_account_settings::eor_admin::personal_information::post_profile_settings_entity),
        )
        .route(
            "/eor-admin/account-settings/contractors/personal-information/individual/:contractor_ulid",
            get(contractor_account_settings::eor_admin::personal_information::get_profile_settings_individual)
            .post(contractor_account_settings::eor_admin::personal_information::post_profile_settings_individual),
        )
         .route(
            "/eor-admin/account-settings/contractors/payroll-information/entity/:contractor_ulid",
            get(contractor_account_settings::eor_admin::payroll_information::get_payroll_information_entity)
            .post(contractor_account_settings::eor_admin::payroll_information::post_payroll_information_entity),
        )
        .route(
            "/eor-admin/account-settings/contractors/payroll-information/individual/:contractor_ulid",
            get(contractor_account_settings::eor_admin::payroll_information::get_payroll_information_individual)
            .post(contractor_account_settings::eor_admin::payroll_information::post_payroll_information_individual),
        )
        .route(
            "/eor-admin/account-settings/contractors/employment-information/entity/:contractor_ulid",
            get(contractor_account_settings::eor_admin::employment_information::get_employment_information_entity)
            .post(contractor_account_settings::eor_admin::employment_information::post_employment_information_entity),
        )
        .route(
            "/eor-admin/account-settings/contractors/employment-information/individual/:contractor_ulid",
            get(contractor_account_settings::eor_admin::employment_information::get_employment_information_individual)
            .post(contractor_account_settings::eor_admin::employment_information::post_employment_information_individual),
        )
        .route(
            "/eor-admin/account-settings/contractors/employment-information",//list all for this client
            get(contractor_account_settings::eor_admin::employment_information::get_employment_information_all)
        )
        .route(
            "/eor-admin/account-settings/contractors/payroll-information",//list all for this client
            get(contractor_account_settings::eor_admin::payroll_information::get_payroll_information_all)
        )
        // ========== ADMIN APIS ==========
        .route(
            "/eor-admin/citibank/download-citibank-transfer-initiation-template",
            post(eor_admin::bank_transfer::citi_bank::download_citibank_transfer_initiation_template),
        )
        .route(
            "/eor-admin/citibank/upload-citibank-transfer-initiation-template",
            post(eor_admin::bank_transfer::citi_bank::upload_citibank_transfer_initiation_template),
        )
        .route(
            "/eor-admin/citibank/init-citibank-transfer",
            post(eor_admin::bank_transfer::citi_bank::init_citibank_transfer),
        )
        .route(
            "/eor-admin/citibank/list-all-uploaded-citibank-transfer-initiation-files-for-client",
            get(eor_admin::bank_transfer::citi_bank::list_all_uploaded_citibank_transfer_initiation_files_for_client),
        )
        .route(
            "/eor-admin/citibank/list-uploaded-citibank-transfer-initiation-files-records",
            get(eor_admin::bank_transfer::citi_bank::list_uploaded_citibank_transfer_initiation_files_records),
        )
        .route(
            "/eor-admin/citibank/list-available-templates",
            get(eor_admin::bank_transfer::citi_bank::list_available_templates),
        )
        .route(
            "/eor-admin/citibank/update-uploaded-citibank-transfer-initiation-file-record",
            post(eor_admin::bank_transfer::citi_bank::update_uploaded_citibank_transfer_initiation_file_record),
        )
        .route(
            "/eor-admin/citibank/delete-uploaded-citibank-transfer-initiation-file/:ulid",
            post(eor_admin::bank_transfer::citi_bank::delete_uploaded_citibank_transfer_initiation_file),
        )
        .route(
            "/eor-admin/citibank/delete-uploaded-citibank-transfer-initiation-file-record/:ulid",
            post(eor_admin::bank_transfer::citi_bank::delete_uploaded_citibank_transfer_initiation_file_record),
        ).route(
            "/eor-admin/citibank/search-clients",
            get(eor_admin::bank_transfer::citi_bank::search_clients),
        ).route(
            "/eor-admin/citibank/search-clients-branches",
            get(eor_admin::bank_transfer::citi_bank::search_clients_branches),
        ).route(
            "/eor-admin/citibank/update-transaction-status",
            get(eor_admin::bank_transfer::citi_bank::update_transaction_status),
        )
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
            get(branch::eor_admin::get_many_branches)
                .post(branch::eor_admin::post_one_branch)
                .delete(branch::eor_admin::delete_one_branch),
        )
        .route(
            "/eor-admin/branch/individual-contractors",
            get(branch::eor_admin::get_many_individual_contractors),
        )
        .route(
            "/eor-admin/branch/:branch_ulid",
            get(branch::eor_admin::get_one_branch_by_ulid),
        )
        .route(
            "/eor-admin/branch/:branch_ulid/payroll-details",
            get(branch::payroll::admin_get_branch_payroll_details)
                .post(branch::payroll::admin_post_branch_payroll_details),
        )
        .route(
            "/eor-admin/client-contractors/search",
            get(eor_admin::search_employee_contractors::eor_admin_get_employee_contractors),
        )
        .route(
            "/eor-admin/sap/upload_payroll_to_s4hana",
            post(eor_admin::sap::s4_hana::post_one),
        )
        .route(
            "/eor-admin/sap/mulesoft/payroll_journal/entries",
            get(eor_admin::sap::s4_hana::get_many_entries),
        )
        .route(
            "/eor-admin/sap/mulesoft/payroll_journal/rows",
            get(eor_admin::sap::s4_hana::get_many_rows),
        )
        .route(
            "/eor-admin/sap/journal_template.xlsx",
            get(eor_admin::sap::s4_hana::download),
        )
        // ========== PUBSUB PAGES ==========
        .route("/dapr/subscribe", get(dapr_subscription_list))
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
                .layer(Extension(database))
                .layer(Extension(shared_state))
                .layer(Extension(KEYS.decoding.clone()))
                .layer(Extension(public_keys))
                .layer(Extension(shared_pubsub))
                .layer(Extension(shared_reqwest_client)),
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
    Ok(Json(vec![]))
}
