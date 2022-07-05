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
mod benefits_market_place;
mod branch;
mod bulk_add;
mod client_account_settings;
mod client_contractor_pair;
mod constant;
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
mod user;

use crate::auth::{state::State, token::KEYS};
use env::{DAPR_ADDRESS, DATABASE_URL, FRONTEND_URL, LISTENING_ADDRESS};

static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let shared_state = State::new().await.expect("Could not connect to Dapr");
    let shared_state = Arc::new(Mutex::new(shared_state));

    let shared_database = Arc::new(Mutex::new(Database::new().await));
    let common_database = Arc::new(Mutex::new(
        common_utils::database::Database::new(&*DATABASE_URL).await,
    ));

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
            "/:user_role/users",
            get(user::user_get_many_users),
        )
        .route(
            "/onboard/individual-details/client",
            get(onboard::individual::user_get_one_client_account_details)
                .post(onboard::individual::user_post_one_client_account_details),
        )
        .route(
            "/onboard/individual-details/contractor",
            get(onboard::individual::user_get_one_contractor_account_details)
                .post(onboard::individual::user_post_one_contractor_account_details),
        )
        .route(
            "/onboard/entity-details/client",
            get(onboard::entity::user_get_one_client_account_details)
                .post(onboard::entity::user_post_one_client_account_details),
        )
        .route(
            "/onboard/entity-details/contractor",
            get(onboard::entity::user_get_one_contractor_account_details)
                .post(onboard::entity::user_post_one_contractor_account_details),
        )
        .route(
            "/onboard/pic-details/:user_role",
            get(onboard::pic::user_get_one_onboard_entity_pic_details)
                .post(onboard::pic::user_post_one_onboard_entity_pic_details),
        )
        .route(
            "/onboard/bank-details",
            get(onboard::bank::user_get_one_bank_details)
                .post(onboard::bank::user_post_one_bank_details),
        )
        .route(
            "/onboard/payment-details",
            get(onboard::payment::user_get_one_payment_details)
                .post(onboard::payment::user_post_one_payment_details),
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
            get(prefill::account::user_get_one_individual_contractor)
                .post(prefill::account::user_post_one_individual_contractor),
        )
        .route(
            "/client/prefill/individual_contractor_bank_details",
            get(prefill::bank::user_get_one_individual_contractor)
                .post(prefill::bank::user_post_one_individual_contractor),
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
            get(notification::user_get_many).
            put(notification::user_put_one),
        )
        .route(
            "/admin-pic/teams/create-team",
            post(branch::teams::create_team),
        ).
        route(
            "/admin-pic/teams/delete-team/:team_ulid",
            post(branch::teams::delete_team),
        )
        .route(
            "/admin-pic/teams/update-team",
            post(branch::teams::update_team),
        )
        .route(
            "/admin-pic/teams/list-teams",
            get(branch::teams::list_teams),//filter by branch ulid
        )
        .route(
            "/admin-pic/teams/list-teams-by-client-ulid",
            get(branch::teams::list_teams_by_client_ulid),//filter by client ulid
        )
        .route(
            "/admin-pic/teams/add-contrator-to-team",
            post(branch::teams::add_contrator_to_team),
        )
         .route(
            "/admin-pic/teams/delete-contrator-from-team",
            post(branch::teams::delete_contrator_from_team),
        )
        .route(
            "/admin-pic/teams/list-team-contractors",
            get(branch::teams::list_team_contractors),
        )
        .route(
            "/admin-pic/cost-center/create-cost-center",
            post(branch::cost_center::create_cost_center),
        )
        .route(
            "/admin-pic/cost-center/update-cost-center",
            post(branch::cost_center::update_cost_center),
        )
        .route(
            "/admin-pic/cost-center/delete-cost-center/:cost_center_ulid",
            post(branch::cost_center::delete_cost_center),
        ).route(
            "/admin-pic/cost-center/list-cost-centers",
            get(branch::cost_center::list_cost_centers),//filter by branch ulid
        )
        .route(
            "/admin-pic/cost-center/list-cost-centers-by-client-ulid",
            get(branch::cost_center::list_cost_centers_by_client_ulid),//filter by client-ulid
        )
        .route(
            "/admin-pic/cost-center/list-cost-center-contractors",
            get(branch::cost_center::list_cost_center_contractors),
        )
        .route(
            "/admin-pic/cost-center/add-contractor-to-cost-center",
            post(branch::cost_center::add_contractor_to_cost_center),
        )
        .route(
            "/admin-pic/cost-center/delete-contractor-from-cost-center",
            post(branch::cost_center::delete_contractor_from_cost_center),
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
        .route(
            "/client-account-settings/change-password",
                post(client_account_settings::user_change_password)
        )
        .route(
            "/eor-admin-account-settings/change-password",
                post(client_account_settings::admin_change_password)
        )
        .route(
            "/client-account-settings/client/entity/entity-client-pic-details",
            get(client_account_settings::client::entity::get_entity_client_pic_details)
                .post(client_account_settings::client::entity::update_entity_client_pic_details)
                .delete(client_account_settings::client::entity::delete_entity_client_pic_details)
        )
        .route(
            "/client-account-settings/client/entity/entity-client-account-details",
            get(client_account_settings::client::entity::get_entity_client_account_details)
                .post(client_account_settings::client::entity::update_entity_client_account_details)
                .delete(client_account_settings::client::entity::delete_entity_client_account_details)
        )
        .route(
            "/client-account-settings/client/entity/entity-client-branch-account-details",
            get(client_account_settings::client::entity::get_entity_client_branch_account_details)
                .post(client_account_settings::client::entity::update_entity_client_branch_account_details)
                .delete(client_account_settings::client::entity::delete_entity_client_branch_account_details)
        ).route(
            "/client-account-settings/client/entity/entity-client-branch-bank-account-details",
            get(client_account_settings::client::entity::get_entity_client_branch_bank_details)
                .post(client_account_settings::client::entity::update_entity_client_branch_bank_details)
                .delete(client_account_settings::client::entity::delete_entity_client_branch_bank_details)
        )
        .route(
            "/client-account-settings/client/entity/entity-client-branch-payroll-details",
            get(client_account_settings::client::entity::get_entity_client_branch_payroll_details)
                .post(client_account_settings::client::entity::update_entity_client_branch_payroll_details)
                .delete(client_account_settings::client::entity::delete_entity_client_branch_payroll_details)
        )
        .route(
            "/client-account-settings/client/entity/entity-client-payment-details",
            get(client_account_settings::client::entity::get_entity_client_payment_details)
                .post(client_account_settings::client::entity::update_entity_client_payment_details)
                .delete(client_account_settings::client::entity::delete_entity_client_payment_details)
        )
         .route(
            "/client-account-settings/client/individual/individual-client-account-details",
            get(client_account_settings::client::individual::get_individual_client_account_details)
                .post(client_account_settings::client::individual::update_individual_client_account_details)
                .delete(client_account_settings::client::individual::delete_individual_client_account_details)
        )
        .route(
            "/client-account-settings/client/individual/individual-client-payment-details",
            get(client_account_settings::client::individual::get_individual_client_payment_details)
                .post(client_account_settings::client::individual::update_individual_client_payment_details)
                .delete(client_account_settings::client::individual::delete_individual_client_payment_details)
        )
        .route(
            "/eor-admin/client-account-settings/client/entity/entity-client-pic-details",
            get(client_account_settings::eor_admin::entity::get_entity_client_pic_details)
                .post(client_account_settings::eor_admin::entity::update_entity_client_pic_details)
                .delete(client_account_settings::eor_admin::entity::delete_entity_client_pic_details)
        )
        .route(
            "/eor-admin/client-account-settings/client/entity/entity-client-account-details",
             get(client_account_settings::eor_admin::entity::get_entity_client_account_details)
                .post(client_account_settings::eor_admin::entity::update_entity_client_account_details)
                .delete(client_account_settings::eor_admin::entity::delete_entity_client_account_details)
        )
        .route(
            "/eor-admin/client-account-settings/client/entity/entity-client-branch-account-details",
            get(client_account_settings::eor_admin::entity::get_entity_client_branch_account_details)
                .post(client_account_settings::eor_admin::entity::update_entity_client_branch_account_details)
                .delete(client_account_settings::eor_admin::entity::delete_entity_client_branch_account_details)
        )
        .route(
            "/eor-admin/client-account-settings/client/entity/entity-client-branch-bank-account-details",
            get(client_account_settings::eor_admin::entity::get_entity_client_branch_bank_details)
                .post(client_account_settings::eor_admin::entity::update_entity_client_branch_bank_details)
                .delete(client_account_settings::eor_admin::entity::delete_entity_client_branch_bank_details)
        )
        .route(
            "/eor-admin/client-account-settings/client/entity/entity-client-branch-payroll-details",
            get(client_account_settings::eor_admin::entity::get_entity_client_branch_payroll_details)
                .post(client_account_settings::eor_admin::entity::update_entity_client_branch_payroll_details)
                .delete(client_account_settings::eor_admin::entity::delete_entity_client_branch_payroll_details)
        )
        .route(
            "/eor-admin/client-account-settings/client/entity/entity-client-payment-details",
            get(client_account_settings::eor_admin::entity::get_entity_client_payment_details)
                .post(client_account_settings::eor_admin::entity::update_entity_client_payment_details)
                .delete(client_account_settings::eor_admin::entity::delete_entity_client_payment_details)
        )
         .route(
            "/eor-admin/client-account-settings/client/individual/individual-client-account-details",
            get(client_account_settings::eor_admin::individual::get_individual_client_account_details)
                .post(client_account_settings::eor_admin::individual::update_individual_client_account_details)
                .delete(client_account_settings::eor_admin::individual::delete_individual_client_account_details)
        )
        .route(
            "/eor-admin/client-account-settings/client/individual/individual-client-payment-details",
            get(client_account_settings::eor_admin::individual::get_individual_client_payment_details)
                .post(client_account_settings::eor_admin::individual::update_individual_client_payment_details)
                .delete(client_account_settings::eor_admin::individual::delete_individual_client_payment_details)
        )
        .route(
            "/contractor-account-settings/contractor/entity/entity-contractor-bank-details",
                get(contractor_account_settings::contractor::entity::get_entity_contractor_bank_details)
                .post(contractor_account_settings::contractor::entity::update_entity_contractor_bank_details)
                .delete(contractor_account_settings::contractor::entity::delete_entity_contractor_bank_details)
        )
        .route(
            "/contractor-account-settings/contractor/entity/entity-contractor-account-settings",
                get(contractor_account_settings::contractor::entity::get_entity_contractor_account_details)
                .post(contractor_account_settings::contractor::entity::update_entity_contractor_account_details)
                .delete(contractor_account_settings::contractor::entity::delete_entity_contractor_account_details)
        )
        .route(
            "/contractor-account-settings/contractor/entity/entity-contractor-employment-information",
                get(contractor_account_settings::contractor::entity::get_entity_contractor_employment_information)
                .post(contractor_account_settings::contractor::entity::update_entity_contractor_employment_information)
                .delete(contractor_account_settings::contractor::entity::delete_entity_contractor_employment_information)
        )
        .route(
            "/contractor-account-settings/contractor/entity/entity-contractor-payroll-information",
                get(contractor_account_settings::contractor::entity::get_entity_contractor_payroll_information)
                .post(contractor_account_settings::contractor::entity::update_entity_contractor_payroll_information)
                .delete(contractor_account_settings::contractor::entity::delete_entity_contractor_payroll_information)
        )
        .route(
            "/contractor-account-settings/contractor/entity/entity-contractor-pic-details",
                get(contractor_account_settings::contractor::entity::get_entity_contractor_pic_details)
                .post(contractor_account_settings::contractor::entity::update_entity_contractor_pic_details)
                .delete(contractor_account_settings::contractor::entity::delete_entity_contractor_pic_details)
        )
        .route(
            "/contractor-account-settings/contractor/individual/individual-contractor-account-settings",
                get(contractor_account_settings::contractor::individual::get_individual_contractor_account_details)
                .post(contractor_account_settings::contractor::individual::update_individual_contractor_account_details)
                .delete(contractor_account_settings::contractor::individual::delete_individual_contractor_account_details)
        )
        .route(
            "/contractor-account-settings/contractor/individual/individual-contractor-bank-details",
                get(contractor_account_settings::contractor::individual::get_individual_contractor_bank_details)
                .post(contractor_account_settings::contractor::individual::update_individual_contractor_bank_details)
                .delete(contractor_account_settings::contractor::individual::delete_individual_contractor_bank_details)
        )
         .route(
            "/contractor-account-settings/contractor/individual/individual-contractor-employment-information",
                get(contractor_account_settings::contractor::individual::get_individual_contractor_employment_information)
                .post(contractor_account_settings::contractor::individual::update_individual_contractor_employment_information)
                .delete(contractor_account_settings::contractor::individual::delete_individual_contractor_employment_information)
        )
        .route(
            "/contractor-account-settings/contractor/individual/individual-contractor-payroll-information",
                get(contractor_account_settings::contractor::individual::get_individual_contractor_payroll_information)
                .post(contractor_account_settings::contractor::individual::update_individual_contractor_payroll_information)
                .delete(contractor_account_settings::contractor::individual::delete_individual_contractor_payroll_information)
        )
       .route(
            "/eor-admin/contractor-account-settings/contractor/entity/entity-contractor-account-settings",
                get(contractor_account_settings::eor_admin::entity::get_entity_contractor_account_details)
                .post(contractor_account_settings::eor_admin::entity::update_entity_contractor_account_details)
                .delete(contractor_account_settings::eor_admin::entity::delete_entity_contractor_account_details)
        )
        .route(
            "/eor-admin/contractor-account-settings/contractor/entity/entity-contractor-employment-information",
                get(contractor_account_settings::eor_admin::entity::get_entity_contractor_employment_information)
                .post(contractor_account_settings::eor_admin::entity::update_entity_contractor_employment_information)
                .delete(contractor_account_settings::eor_admin::entity::delete_entity_contractor_employment_information)
        )
        .route(
            "/eor-admin/contractor-account-settings/contractor/entity/entity-contractor-payroll-information",
                get(contractor_account_settings::eor_admin::entity::get_entity_contractor_payroll_information)
                .post(contractor_account_settings::eor_admin::entity::update_entity_contractor_payroll_information)
                .delete(contractor_account_settings::eor_admin::entity::delete_entity_contractor_payroll_information)
        )
        .route(
            "/eor-admin/contractor-account-settings/contractor/entity/entity-contractor-pic-details",
                get(contractor_account_settings::eor_admin::entity::get_entity_contractor_pic_details)
                .post(contractor_account_settings::eor_admin::entity::update_entity_contractor_pic_details)
                .delete(contractor_account_settings::eor_admin::entity::delete_entity_contractor_pic_details)
        ) .route(
            "/eor-admin/contractor-account-settings/contractor/entity/entity-contractor-bank-details",
                get(contractor_account_settings::eor_admin::entity::get_entity_contractor_bank_details)
                .post(contractor_account_settings::eor_admin::entity::update_entity_contractor_bank_details)
                .delete(contractor_account_settings::eor_admin::entity::delete_entity_contractor_bank_details)
        )
        .route(
            "/eor-admin/contractor-account-settings/contractor/individual/individual-contractor-account-settings",
                get(contractor_account_settings::eor_admin::individual::get_individual_contractor_account_details)
                .post(contractor_account_settings::eor_admin::individual::update_individual_contractor_account_details)
                .delete(contractor_account_settings::eor_admin::individual::delete_individual_contractor_account_details)
        )
        .route(
            "/eor-admin/contractor-account-settings/contractor/individual/individual-contractor-bank-details",
                get(contractor_account_settings::eor_admin::individual::get_individual_contractor_bank_details)
                .post(contractor_account_settings::eor_admin::individual::update_individual_contractor_bank_details)
                .delete(contractor_account_settings::eor_admin::individual::delete_individual_contractor_bank_details)
        )
         .route(
            "/eor-admin/contractor-account-settings/contractor/individual/individual-contractor-employment-information",
                get(contractor_account_settings::eor_admin::individual::get_individual_contractor_employment_information)
                .post(contractor_account_settings::eor_admin::individual::update_individual_contractor_employment_information)
                .delete(contractor_account_settings::eor_admin::individual::delete_individual_contractor_employment_information)
        )
        .route(
            "/eor-admin/contractor-account-settings/contractor/individual/individual-contractor-payroll-information",
                get(contractor_account_settings::eor_admin::individual::get_individual_contractor_payroll_information)
                .post(contractor_account_settings::eor_admin::individual::update_individual_contractor_payroll_information)
                .delete(contractor_account_settings::eor_admin::individual::delete_individual_contractor_payroll_information)
        )
        // ========== ADMIN APIS ==========
        .route(
            "/eor-admin/onboard/:user_ulid/individual-details/client",
            get(onboard::individual::admin_get_one_client_account_details)
                .post(onboard::individual::admin_post_one_client_account_details),
        )
        .route(
            "/eor-admin/onboard/:user_ulid/individual-details/contractor",
            get(onboard::individual::admin_get_one_contractor_account_details)
                .post(onboard::individual::admin_post_one_contractor_account_details),
        )
        .route(
            "/eor-admin/onboard/:user_ulid/entity-details/client",
            get(onboard::entity::admin_get_one_client_account_details)
                .post(onboard::entity::admin_post_one_client_account_details),
        )
        .route(
            "/eor-admin/onboard/:user_ulid/entity-details/contractor",
            get(onboard::entity::admin_get_one_contractor_account_details)
                .post(onboard::entity::admin_post_one_contractor_account_details),
        )
        .route(
            "/eor-admin/onboard/:user_ulid/pic-details/:user_role",
            get(onboard::pic::admin_get_one_onboard_entity_pic_details)
                .post(onboard::pic::admin_post_one_onboard_entity_pic_details),
        )
        .route(
            "/eor-admin/onboard/:user_ulid/bank-details/:user_type",
            get(onboard::bank::admin_get_one_bank_details)
                .post(onboard::bank::admin_post_one_bank_details),
        )
        .route(
            "/eor-admin/onboard/:user_ulid/payment-details/:user_type",
            get(onboard::payment::admin_get_one_payment_details)
                .post(onboard::payment::admin_post_one_payment_details),
        )
        .route(
            "/eor-admin/teams/create-team",
            post(eor_admin::teams::create_team),
        ).
        route(
            "/eor-admin/teams/delete-team/:team_ulid",
            post(eor_admin::teams::delete_team),
        )
        .route(
            "/eor-admin/teams/update-team",
            post(eor_admin::teams::update_team),
        )
        .route(
            "/eor-admin/teams/list-teams",
            get(eor_admin::teams::list_teams),//filter by branch ulid
        )
        .route(
            "/eor-admin/teams/list-teams-by-client-ulid",
            get(eor_admin::teams::list_teams_by_client_ulid),//filter by client ulid
        )
        .route(
            "/eor-admin/teams/add-contrator-to-team",
            post(eor_admin::teams::add_contrator_to_team),
        )
         .route(
            "/eor-admin/teams/delete-contrator-from-team",
            post(eor_admin::teams::delete_contrator_from_team),
        )
        .route(
            "/eor-admin/teams/list-team-contractors",
            get(eor_admin::teams::list_team_contractors),
        )
        .route(
            "/eor-admin/cost-center/create-cost-center",
            post(eor_admin::cost_center::create_cost_center),
        )
        .route(
            "/eor-admin/cost-center/update-cost-center",
            post(eor_admin::cost_center::update_cost_center),
        )
        .route(
            "/eor-admin/cost-center/delete-cost-center/:cost_center_ulid",
            post(eor_admin::cost_center::delete_cost_center),
        ).route(
            "/eor-admin/cost-center/list-cost-centers",
            get(eor_admin::cost_center::list_cost_centers),//filter by branch ulid
        )
        .route(
            "/eor-admin/cost-center/list-cost-centers-by-client-ulid",
            get(eor_admin::cost_center::list_cost_centers_by_client_ulid),//filter by client ulid
        )
        .route(
            "/eor-admin/cost-center/list-cost-center-contractors",
            get(eor_admin::cost_center::list_cost_center_contractors),
        )
        .route(
            "/eor-admin/cost-center/add-contractor-to-cost-center",
            post(eor_admin::cost_center::add_contractor_to_cost_center),
        )
        .route(
            "/eor-admin/cost-center/delete-contractor-from-cost-center",
            post(eor_admin::cost_center::delete_contractor_from_cost_center),
        )
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
            "/eor-admin/:user_role/users",
            get(user::admin_get_many_users),
        )
        .route(
            "/eor-admin/onboarded-users",
            get(eor_admin::admin_get_many_onboarded_user_index),
        )
        .route("/eor-admin/users", get(eor_admin::admin_get_many_user_index))
        .route(
            "/eor-admin/client-contractor-pair",
            get(client_contractor_pair::admin_get_many_client_contractor_pair_index)
                .post(client_contractor_pair::admin_post_one_client_contractor_pair),
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
            "/eor-admin/sap/mulesoft/payroll_journal/entry/:entry_ulid/download",
            get(eor_admin::sap::s4_hana::download_one_entry),
        )
        .route(
            "/eor-admin/sap/mulesoft/payroll_journal/rows",
            get(eor_admin::sap::s4_hana::get_many_rows),
        )
        .route(
            "/eor-admin/sap/journal_template.xlsx",
            get(eor_admin::sap::s4_hana::download),
        )
        .route(
            "/eor-admin/notifications",
            get(notification::admin_get_many_for_user).
            post(notification::admin_post_one_for_user),
        )
        .route(
            "/eor-admin/admin-notifications",
            get(notification::admin_get_many)
        )
        // ========== CONSTANT PAGES ========
        .route("/constant/country_code",get(constant::country_code::get_many).post(constant::country_code::post_one).delete(constant::country_code::delete_one))
        .route("/constant/currency_code",get(constant::currency_code::get_many).post(constant::currency_code::post_one).delete(constant::currency_code::delete_one))
        .route("/constant/entity_type",get(constant::entity_type::get_many).post(constant::entity_type::post_one).delete(constant::entity_type::delete_one))
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
                .layer(Extension(shared_database))
                .layer(Extension(common_database))
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
