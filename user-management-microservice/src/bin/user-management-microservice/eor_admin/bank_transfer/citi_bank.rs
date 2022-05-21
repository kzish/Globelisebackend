use axum::extract::{Extension, Json, Query};
use common_utils::custom_serde::Currency;
use common_utils::{calc_limit_and_offset, error::GlobeliseResult};
use csv::{ReaderBuilder, StringRecord};
use email_address::EmailAddress;
use eor_admin_microservice_sdk::token::AdminAccessToken;
use lettre::{Message, SmtpTransport, Transport};
use serde_with::{base64::Base64, serde_as, TryFromInto};
use sqlx::{postgres::PgRow, FromRow, Row};
use std::{
    env,
    fs::{self, File},
    path::Path,
};
use umya_spreadsheet::*;
use user_management_microservice_sdk::user::UserType;

use axum::{body::StreamBody, http::header};
use serde_with::base64;
use std::net::SocketAddr;

use common_utils::{error::GlobeliseError, token::Token};
// use eor_admin_microservice_sdk::token::AdminAccessToken;
use chrono;
use chrono::Date;
use rand::Rng;
use rand070::thread_rng;
use serde::{Deserialize, Serialize};
use ssh2::Session;
use std::io::prelude::*;
use std::io::Write;
use std::net::TcpStream;
use std::process::Command;
use std::str;
//
use axum::{http::StatusCode, response::IntoResponse, routing::get, Router};

use common_utils::custom_serde::{EmailWrapper, FORM_DATA_LENGTH_LIMIT};

use crate::database::{Database, SharedDatabase};

use reqwest::header::HeaderMap;
// use tonic::codegen::http::HeaderMap;
use axum::extract::ContentLengthLimit;
use reqwest::header::{HeaderName, HeaderValue};
// use sqlx::types::Uuid;
use calamine::{open_workbook, DataType, Error, RangeDeserializerBuilder, Reader, Xlsx};
use uuid::Uuid;

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct DownloadCitibankTransferInitiationTemplateRequest {
    pub branch_ulid: Uuid,
    pub client_ulid: Uuid,
    pub department_name: String,
    pub country: String,
    pub currency: String,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct UploadCitiBankTransferInitiationFiles {
    #[serde_as(as = "Base64")]
    pub uploaded_file: Vec<u8>,
    pub title_identifier: String,
    pub client_ulid: Uuid,
    pub debug: Option<bool>,
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct PaginatedQuery {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
    pub client_ulid: Option<Uuid>,
    pub file_ulid: Option<Uuid>,
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct SearchClientsQuery {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
    pub name: Option<String>,
}

#[serde_as]
#[derive(Debug, Deserialize, Serialize, FromRow)]
#[serde(rename_all = "kebab-case")]
pub struct SearchClientsResponse {
    pub ulid: Uuid,
    pub name: String,
    pub email: String,
    pub user_role: String,
    pub user_type: UserType,
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct SearchClientsBranchesQuery {
    pub client_ulid: Uuid,
    pub department_name: Option<String>,
    pub page: Option<u32>,
    pub per_page: Option<u32>,
}

#[serde_as]
#[derive(Debug, Deserialize, Serialize, FromRow)]
#[serde(rename_all = "kebab-case")]
pub struct SearchClientsBranchesResponse {
    pub ulid: Uuid,
    pub client_ulid: Uuid,
    pub department_name: String,
    pub country: String,
    pub currency: String,
}

#[serde_as]
#[derive(Debug, Deserialize, Serialize, FromRow)]
#[serde(rename_all = "kebab-case")]
pub struct GetContractorAccountDetailsCitibankTemplateResponse {
    pub contractor_ulid: Uuid,
    pub contractor_name: String,
    pub contractor_bank_name: String,
    pub contractor_bank_code: String,
    pub contractor_bank_branch_code: String,
    pub branch_ulid: Uuid, //client's branch ulid
    pub contractor_bank_account_number: String,
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct InitCitibankTransferRequest {
    pub file_ulid: Uuid,
    pub template_name: String, //eg. template.xml
}
#[serde_as]
#[derive(Debug, Serialize, FromRow, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ListCitiBankTransferInitiationFiles {
    pub ulid: Uuid,
    pub title_identifier: String,
    pub client_ulid: Uuid,
    pub status: String,
}

#[serde_as]
#[derive(Debug, Serialize, FromRow, Deserialize, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct CitiBankPayRollRecord {
    pub ulid: Uuid,
    pub company_name: String,
    pub currency_code: String,
    pub country_code: String,
    pub employee_id: String,
    pub employee_name: String,
    pub bank_name_creditor: String,
    pub bank_account_number_creditor: String,
    pub bic_swift_code_creditor: String,
    pub bank_account_number_debitor: String,
    pub amount: f64,
    pub file_ulid: Uuid,
    pub transaction_status: String,
}

//======== xml structure of citibank transaction response
#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct CitiBankTransactionResponse {
    pub CstmrPmtStsRpt: CstmrPmtStsRpt,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct CstmrPmtStsRpt {
    pub OrgnlGrpInfAndSts: OrgnlGrpInfAndSts,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct OrgnlGrpInfAndSts {
    pub OrgnlMsgId: Uuid,
    pub OrgnlMsgNmId: String,
    pub OrgnlCreDtTm: String,
    pub OrgnlNbOfTxs: String,
    pub OrgnlCtrlSum: String,
    pub GrpSts: String,
    pub StsRsnInf: StsRsnInf,
}
#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct StsRsnInf {
    pub AddtlInf: String,
}

// ================ SUMMARY =========================
//encrypt(raw) -> enc;
//decrypt(enc) -> raw;
//sftp_upload(local_file, remote_file);
//sftp_download(local_file, remote_file);
//sftp_list_remote_files(sftp_root_dir) -> Vec<String>
//sftp_delete_remote_file(remote_file)
//generate_xml(template.xml, local_file, record);
//list_available_templates() -> ['citi_bank_sg.xml']
//download_citibank_transfer_initiation_template() -> FILE.xlxs
//upload_citibank_transfer_initiation_template(UploadCitiBankTransferInitiationFiles)

//init_citibank_transfer(file_ulid) -> pushes transction.xml file to citibank ftp folder
//refresh_citibank_transfers() -> checks transaction status and updates the records
//xml_to_citibank_transaction_response(raw_xml_string: String) -> CitiBankTransactionResponse

pub async fn search_clients(
    _: Token<AdminAccessToken>,
    Query(request): Query<SearchClientsQuery>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<Vec<SearchClientsResponse>>> {
    let database = database.lock().await;

    let result = database.search_clients(request).await?;

    Ok(Json(result))
}

pub async fn search_clients_branches(
    _: Token<AdminAccessToken>,
    Query(request): Query<SearchClientsBranchesQuery>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<Vec<SearchClientsBranchesResponse>>> {
    let database = database.lock().await;

    let result = database.search_clients_branches(request).await?;

    Ok(Json(result))
}

pub async fn update_transaction_status(
    _: Token<AdminAccessToken>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    let sftp_root_dir = std::env::var("CITIBANK_SFTP_ROOT_DIR").expect("failed to get root dir");
    let base_path = std::env::var("CITIBANK_BASE_PATH").expect("base_path not set");
    let database = database.lock().await;
    let remote_files = sftp_list_remote_files(sftp_root_dir).await?;
    let guid = Uuid::new_v4().to_simple().to_string();
    let sftp_drop_dir = std::env::var("CITIBANK_SFTP_DROP_DIR").expect("sftp_drop_dir_not set");
    for file in remote_files {
        //skip this directory
        if file.replace("/", "") == sftp_drop_dir.replace("/", "") {
            continue;
        }
        let local_file = format!("{}citibank_temp/{}.xml", &base_path, &guid);
        sftp_download(local_file.to_string(), file.to_string()).await;
        let enc_data =
            std::fs::read_to_string(Path::new(&local_file)).expect("failed to read file");
        let raw_data = enc_data; //decrypt(enc_data).await?;
        sftp_delete_remote_file(file.to_string()).await;
        fs::remove_file(Path::new(&local_file)).expect("error removing file");
        let citibank_response = xml_to_citibank_transaction_response(raw_data).await?;
        database.update_uploaded_citibank_transfer_initiation_file_record_by_cititbank_transaction_response(citibank_response).await?;
    }

    Ok(())
}

async fn xml_to_citibank_transaction_response(
    src: String,
) -> GlobeliseResult<CitiBankTransactionResponse> {
    // let src = std::fs::read_to_string(Path::new(&local_file)).expect("failed to read xml file");
    let item = serde_xml_rs::from_str(&src).unwrap();

    Ok(item)
}

async fn encrypt(raw_data: String) -> GlobeliseResult<String> {
    // encrypt using openpgp cli
    //source: https://www.openpgp.org/
    //source: https://www.gpg4win.org/
    //source: https://www.gnupg.org/gph/en/manual/x56.html

    //generate guid for unique file name
    let guid = Uuid::new_v4().to_simple().to_string();
    //
    let base_path = std::env::var("CITIBANK_BASE_PATH").expect("base path not set");
    //
    let path_raw_data = format!("{}citibank_temp/{}.txt", &base_path, &guid);
    let path_enc_data = format!("{}citibank_temp/{}.txt.asc", &base_path, &guid);
    //write plain text file
    fs::write(Path::new(&path_raw_data), raw_data.as_bytes())?;
    //
    let enc_key = std::env::var("CITIBANK_SFTP_ENCRYPTION_KEY").unwrap();
    let sign_key = std::env::var("CITIBANK_SFTP_SIGN_KEY").unwrap();

    //gpg cli encrypt raw data
    let mut cmd = Command::new("gpg")
        .arg("-e")
        .arg("-a")
        .arg("-s")
        .arg("--default-key")
        .arg(sign_key)
        .arg(format!("-r {}", enc_key))
        .arg("--openpgp")
        .arg(Path::new(&path_raw_data))
        .spawn()
        .expect("failed to encrypt file");
    cmd.wait();
    //
    let encrypted_data = fs::read_to_string(Path::new(&path_enc_data))?;
    //delete these files
    fs::remove_file(Path::new(&path_raw_data))?;
    fs::remove_file(Path::new(&path_enc_data))?;

    Ok(encrypted_data)
}

async fn decrypt(enc_data: String) -> GlobeliseResult<String> {
    // decrypt using openpgp cli
    //source: https://www.openpgp.org/
    //source: https://www.gpg4win.org/
    //source: https://www.gnupg.org/gph/en/manual/x56.html

    //generate guid for unique file name
    let guid_raw = Uuid::new_v4().to_simple().to_string();
    let guid_enc = Uuid::new_v4().to_simple().to_string();
    //
    let base_path = std::env::var("CITIBANK_BASE_PATH").expect("base path not set");
    //
    let path_raw_data = format!("{}citibank_temp/{}.txt", &base_path, &guid_raw);
    let path_enc_data = format!("{}citibank_temp/{}.txt", &base_path, &guid_enc);
    //write enc text file
    fs::write(Path::new(&path_enc_data), enc_data.as_bytes())?;

    //gpg cli decrypt enc data
    let mut cmd = Command::new("gpg")
        .arg("-d")
        .arg("-o")
        .arg(Path::new(&path_raw_data))
        .arg(Path::new(&path_enc_data))
        .spawn()
        .expect("failed to decrypt file");
    cmd.wait();
    //
    let decrypted_data = fs::read_to_string(Path::new(&path_raw_data))?;
    //delete these files
    fs::remove_file(Path::new(&path_raw_data))?;
    fs::remove_file(Path::new(&path_enc_data))?;

    Ok(decrypted_data)
}

async fn sftp_upload(local_file: String, remote_file: String) {
    // Connect to the SSH server
    let sftp_username = std::env::var("CITIBANK_SFTP_USERNAME").unwrap();
    let sftp_password = std::env::var("CITIBANK_SFTP_PASSWORD").unwrap();
    let sftp_host = std::env::var("CITIBANK_SFTP_HOST").unwrap();
    let sftp_port = std::env::var("CITIBANK_SFTP_PORT").unwrap();
    let tcp = TcpStream::connect(format!("{}:{}", &sftp_host, &sftp_port)).unwrap();
    //
    let data = std::fs::read_to_string(Path::new(&local_file)).expect("failed to read data");
    //
    let mut sess = Session::new().unwrap();
    sess.set_tcp_stream(tcp);
    sess.handshake().unwrap();
    sess.userauth_password(&sftp_username, &sftp_password)
        .unwrap();
    //
    let sftp_client = sess.sftp().unwrap();
    let mut sftp_file = sftp_client.create(Path::new(&remote_file)).unwrap();
    sftp_file.write_all(data.as_bytes());
}

async fn sftp_download(local_file: String, remote_file: String) {
    // Connect to the SSH server
    let sftp_username = std::env::var("CITIBANK_SFTP_USERNAME").unwrap();
    let sftp_password = std::env::var("CITIBANK_SFTP_PASSWORD").unwrap();
    let sftp_host = std::env::var("CITIBANK_SFTP_HOST").unwrap();
    let sftp_port = std::env::var("CITIBANK_SFTP_PORT").unwrap();
    let tcp = TcpStream::connect(format!("{}:{}", &sftp_host, &sftp_port)).unwrap();
    let mut sess = Session::new().unwrap();
    sess.set_tcp_stream(tcp);
    sess.handshake().unwrap();
    sess.userauth_password(&sftp_username, &sftp_password)
        .unwrap();
    let sftp_client = sess.sftp().unwrap();

    let mut sftp_file = sftp_client
        .open(Path::new(&remote_file))
        .expect("failed to open file");
    let mut content_data = "".to_string();
    sftp_file
        .read_to_string(&mut content_data)
        .expect("error reading file");
    std::fs::write(local_file, content_data).expect("failed to write to file");
}

async fn sftp_list_remote_files(sftp_root_dir: String) -> GlobeliseResult<Vec<String>> {
    // Connect to the SSH server
    let sftp_username = std::env::var("CITIBANK_SFTP_USERNAME").unwrap();
    let sftp_password = std::env::var("CITIBANK_SFTP_PASSWORD").unwrap();
    let sftp_host = std::env::var("CITIBANK_SFTP_HOST").unwrap();
    let sftp_port = std::env::var("CITIBANK_SFTP_PORT").unwrap();
    let tcp = TcpStream::connect(format!("{}:{}", &sftp_host, &sftp_port)).unwrap();
    let mut sess = Session::new().unwrap();
    sess.set_tcp_stream(tcp);
    sess.handshake().unwrap();
    sess.userauth_password(&sftp_username, &sftp_password)
        .unwrap();
    let sftp_client = sess.sftp().unwrap();
    let mut remote_files = Vec::new();
    let path_buf = sftp_client
        .readdir(Path::new(&sftp_root_dir))
        .expect("failed to read dir");
    for file in path_buf {
        let file_name = file.0.as_os_str().to_str().unwrap().to_string();
        remote_files.push(file_name.to_string());
    }

    Ok(remote_files)
}

pub async fn list_available_templates(
    _: Token<AdminAccessToken>,
) -> GlobeliseResult<Json<Vec<String>>> {
    let mut available_templates = Vec::new();
    let root_dir = std::env::var("CITIBANK_BASE_PATH").expect("failed to get root dir");
    let templates_folder = format!("{}citibank_templates/", &root_dir);
    let files = std::fs::read_dir(Path::new(&templates_folder)).unwrap();
    for file in files {
        let file_name = file.unwrap().file_name().to_str().unwrap().to_string();
        //skip this citibank_transfer_command_template_file
        if file_name == "citibank_transfer_command_template_file.xlsx" {
            continue;
        }
        available_templates.push(file_name.to_string());
    }

    Ok(Json(available_templates))
}

async fn sftp_delete_remote_file(remote_file: String) {
    // Connect to the SSH server
    let sftp_username = std::env::var("CITIBANK_SFTP_USERNAME").unwrap();
    let sftp_password = std::env::var("CITIBANK_SFTP_PASSWORD").unwrap();
    let sftp_host = std::env::var("CITIBANK_SFTP_HOST").unwrap();
    let sftp_port = std::env::var("CITIBANK_SFTP_PORT").unwrap();
    let tcp = TcpStream::connect(format!("{}:{}", &sftp_host, &sftp_port)).unwrap();
    let mut sess = Session::new().unwrap();
    sess.set_tcp_stream(tcp);
    sess.handshake().unwrap();
    sess.userauth_password(&sftp_username, &sftp_password)
        .unwrap();
    let sftp_client = sess.sftp().unwrap();
    sftp_client.unlink(Path::new(&remote_file));
}

async fn generate_xml(template_name: String, local_file: String, record: CitiBankPayRollRecord) {
    //
    let base_path = std::env::var("CITIBANK_BASE_PATH").expect("base path not set");
    //max 35 chars
    let guid = Uuid::parse_str(&record.ulid.to_string())
        .unwrap()
        .to_simple()
        .to_string();
    //
    let transfer_amount: f64 = record.amount;
    //
    let mut data = fs::read_to_string(Path::new(&format!(
        "{}citibank_templates/{}",
        &base_path, &template_name
    )))
    .unwrap();
    data = str::replace(&data, "{{MsgId}}", &guid); //unique ID
                                                    //
    data = str::replace(
        &data,
        "{{CreDtTm}}",
        &chrono::offset::Local::now()
            .format("%Y-%m-%dT%H:%M:%S")
            .to_string(),
    );
    //
    data = str::replace(&data, "{{CtgyPurp_Cd}}", "SALA"); //salary
    data = str::replace(&data, "{{CtrlSum}}", &transfer_amount.to_string()); //transfer amount
    data = str::replace(&data, "{{InitgPty_Nm}}", &record.company_name.to_string()); //initiating party
    data = str::replace(&data, "{{PmtInfId}}", &guid); //unique ID
    data = str::replace(&data, "{{debit_bank_bic}}", &record.bic_swift_code_creditor);
    data = str::replace(
        &data,
        "{{creditor_bank_bic}}",
        &record.bic_swift_code_creditor,
    );
    data = str::replace(&data, "{{CtgyPurp_Cd}}", "SALA");
    data = str::replace(
        &data,
        "{{ReqdExctnDt}}",
        &chrono::offset::Local::now().format("%Y-%m-%d").to_string(),
    ); //date clreaing agent required to process payment YYYY-MM-DD
    data = str::replace(&data, "{{Dbtr_Nm}}", &"globelise".to_string()); //ordering party name / client name
    data = str::replace(
        &data,
        "{{DbtrAcct_Id_Othr_Id}}",
        &record.bank_account_number_debitor.to_string(),
    ); //debit account number without any leading zero.
    data = str::replace(&data, "{{FinInstnId_BIC}}", "CITISGSG"); //bank identification number
    data = str::replace(&data, "{{EndToEndId}}", &guid); //guid
    data = str::replace(&data, "{{Ccy}}", &record.currency_code.to_string()); //Currency
    data = str::replace(&data, "{{InstdAmt}}", &transfer_amount.to_string());
    data = str::replace(&data, "{{Ctry}}", &record.country_code.to_string()); //SG - singapor
    data = str::replace(&data, "{{BrnchId}}", "7214"); //
    data = str::replace(&data, "{{Cdtr_Nm}}", &record.employee_name.to_string()); //beneficiary name
    data = str::replace(
        &data,
        "{{Cdtr_PstlAdr_Ctry}}",
        &record.country_code.to_string(),
    ); //beneficiary country
    data = str::replace(
        &data,
        "{{CdtrAcct_Id}}",
        &record.bank_account_number_creditor.to_string(),
    ); //creditor account number
    data = str::replace(&data, "{{RmtInf_Ustrd}}", "globelise salary payment"); //payment reference

    std::fs::write(local_file, data.as_bytes()).expect("failed to write to file");
}

/**
 *  download_citibank_transfer_initiation_template.xlxs
 */
pub async fn download_citibank_transfer_initiation_template(
    _: Token<AdminAccessToken>,
    Json(request): Json<DownloadCitibankTransferInitiationTemplateRequest>,
    Extension(database): Extension<SharedDatabase>,
) -> (HeaderMap, Vec<u8>) {
    let database = database.lock().await;

    let content_type = "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet";
    let mut headers = HeaderMap::new();
    headers.insert(
        HeaderName::from_static("content-type"),
        HeaderValue::from_str(&content_type).unwrap(),
    );
    let base_path = std::env::var("CITIBANK_BASE_PATH").expect("base path not set");

    let template_file_name = format!(
        "{}citibank_templates/citibank_transfer_command_template_file.xlsx",
        &base_path
    );

    let contractors = database
        .get_contractor_account_details_citibank_template(request.branch_ulid)
        .await
        .unwrap();

    //generate pre populated xlsx file
    let mut book = new_file();
    let _ = book.new_sheet("Sheet1");

    //add headers

    book.get_sheet_by_name_mut("Sheet1")
        .unwrap()
        .get_cell_mut("A1")
        .set_value("Company Name");
    book.get_sheet_by_name_mut("Sheet1")
        .unwrap()
        .get_cell_mut("B1")
        .set_value("Currency Code");
    book.get_sheet_by_name_mut("Sheet1")
        .unwrap()
        .get_cell_mut("C1")
        .set_value("Country Code");
    book.get_sheet_by_name_mut("Sheet1")
        .unwrap()
        .get_cell_mut("D1")
        .set_value("Employee ID");
    book.get_sheet_by_name_mut("Sheet1")
        .unwrap()
        .get_cell_mut("E1")
        .set_value("Employee Name");
    book.get_sheet_by_name_mut("Sheet1")
        .unwrap()
        .get_cell_mut("F1")
        .set_value("Bank Name");
    book.get_sheet_by_name_mut("Sheet1")
        .unwrap()
        .get_cell_mut("G1")
        .set_value("Creditor Bank Account Number");
    book.get_sheet_by_name_mut("Sheet1")
        .unwrap()
        .get_cell_mut("H1")
        .set_value("Bank Account Number Debitor");
    book.get_sheet_by_name_mut("Sheet1")
        .unwrap()
        .get_cell_mut("I1")
        .set_value("Contractor Bank Code");
    book.get_sheet_by_name_mut("Sheet1")
        .unwrap()
        .get_cell_mut("J1")
        .set_value("Amount");

    //set color
    book.get_sheet_by_name_mut("Sheet1")
        .unwrap()
        .get_style_mut("A1")
        .set_background_color(Color::COLOR_YELLOW);
    book.get_sheet_by_name_mut("Sheet1")
        .unwrap()
        .get_style_mut("B1")
        .set_background_color(Color::COLOR_YELLOW);
    book.get_sheet_by_name_mut("Sheet1")
        .unwrap()
        .get_style_mut("C1")
        .set_background_color(Color::COLOR_YELLOW);
    book.get_sheet_by_name_mut("Sheet1")
        .unwrap()
        .get_style_mut("D1")
        .set_background_color(Color::COLOR_YELLOW);
    book.get_sheet_by_name_mut("Sheet1")
        .unwrap()
        .get_style_mut("E1")
        .set_background_color(Color::COLOR_YELLOW);
    book.get_sheet_by_name_mut("Sheet1")
        .unwrap()
        .get_style_mut("F1")
        .set_background_color(Color::COLOR_YELLOW);
    book.get_sheet_by_name_mut("Sheet1")
        .unwrap()
        .get_style_mut("G1")
        .set_background_color(Color::COLOR_YELLOW);
    book.get_sheet_by_name_mut("Sheet1")
        .unwrap()
        .get_style_mut("H1")
        .set_background_color(Color::COLOR_YELLOW);
    book.get_sheet_by_name_mut("Sheet1")
        .unwrap()
        .get_style_mut("I1")
        .set_background_color(Color::COLOR_YELLOW);
    book.get_sheet_by_name_mut("Sheet1")
        .unwrap()
        .get_style_mut("J1")
        .set_background_color(Color::COLOR_YELLOW);

    let mut row_index = 2; //start at 2
    for contractor in contractors {
        book.get_sheet_by_name_mut("Sheet1")
            .unwrap()
            .get_cell_mut(&format!("A{}", row_index))
            .set_value(&contractor.contractor_name);
        book.get_sheet_by_name_mut("Sheet1")
            .unwrap()
            .get_cell_mut(&format!("B{}", row_index))
            .set_value(&request.currency);
        book.get_sheet_by_name_mut("Sheet1")
            .unwrap()
            .get_cell_mut(&format!("C{}", row_index))
            .set_value(&request.country);
        book.get_sheet_by_name_mut("Sheet1")
            .unwrap()
            .get_cell_mut(&format!("D{}", row_index))
            .set_value(&contractor.contractor_ulid.to_string());
        book.get_sheet_by_name_mut("Sheet1")
            .unwrap()
            .get_cell_mut(&format!("E{}", row_index))
            .set_value(&contractor.contractor_name);
        book.get_sheet_by_name_mut("Sheet1")
            .unwrap()
            .get_cell_mut(&format!("F{}", row_index))
            .set_value(&contractor.contractor_bank_name);
        book.get_sheet_by_name_mut("Sheet1")
            .unwrap()
            .get_cell_mut(&format!("G{}", row_index))
            .set_value(&contractor.contractor_bank_account_number);
        book.get_sheet_by_name_mut("Sheet1")
            .unwrap()
            .get_cell_mut(&format!("H{}", row_index))
            .set_value("bank account number debitor");
        book.get_sheet_by_name_mut("Sheet1")
            .unwrap()
            .get_cell_mut(&format!("I{}", row_index))
            .set_value(&contractor.contractor_bank_code);
        book.get_sheet_by_name_mut("Sheet1")
            .unwrap()
            .get_cell_mut(&format!("J{}", row_index))
            .set_value("");

        row_index += 1;
    }

    // writer
    let path = std::path::Path::new(&template_file_name);
    let _ = writer::xlsx::write(&book, path);

    (headers, std::fs::read(&template_file_name).unwrap())
}

/**
 *  upload_citibank_transfer_initiation_template
 */
pub async fn upload_citibank_transfer_initiation_template(
    _: Token<AdminAccessToken>,
    ContentLengthLimit(Json(request)): ContentLengthLimit<
        Json<UploadCitiBankTransferInitiationFiles>,
        FORM_DATA_LENGTH_LIMIT,
    >,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    let guid = Uuid::new_v4().to_simple().to_string();
    let base_path = std::env::var("CITIBANK_BASE_PATH").expect("base path not set");
    let file_name = format!("{}citibank_temp/{}.xlsx", &base_path, &guid);
    std::fs::write(Path::new(&file_name), &request.uploaded_file).expect("failed to write file");

    // read file
    let mut excel: Xlsx<_> = open_workbook(&file_name).unwrap();
    if let Some(Ok(r)) = excel.worksheet_range("Sheet1") {
        let mut index = 0;

        let database = database.lock().await;
        let file_ulid = Uuid::new_v4();
        let record_file = ListCitiBankTransferInitiationFiles {
            ulid: file_ulid,
            title_identifier: request.title_identifier,
            client_ulid: request.client_ulid,
            status: "pending".to_string(),
        };

        //db entry
        database
            .create_uploaded_citibank_transfer_initiation_file(record_file)
            .await?;

        //read records and save into db
        for row in r.rows() {
            //skip first row with titles
            if index != 0 {
                // let employee_id_ulid_str  =  Uuid::parse_str(&row[3].get_string().unwrap().to_string()).unwrap();

                let record = CitiBankPayRollRecord {
                    ulid: Uuid::new_v4(),
                    company_name: row[0].get_string().unwrap_or_default().to_string(),
                    currency_code: row[1].get_string().unwrap_or_default().to_string(),
                    country_code: row[2].get_string().unwrap_or_default().to_string(),
                    employee_id: row[3].get_string().unwrap_or_default().to_string(), //employee_id_ulid_str,
                    employee_name: row[4].get_string().unwrap_or_default().to_string(),
                    bank_name_creditor: row[5].get_string().unwrap_or_default().to_string(),
                    bank_account_number_creditor: row[6]
                        .get_string()
                        .unwrap_or(&row[6].get_float().unwrap_or_default().to_string())
                        .to_string()
                        .parse()
                        .unwrap(),
                    bank_account_number_debitor: row[7]
                        .get_string()
                        .unwrap_or(&row[7].get_float().unwrap_or_default().to_string())
                        .to_string()
                        .parse()
                        .unwrap(),
                    bic_swift_code_creditor: row[8]
                        .get_string()
                        .unwrap_or(&row[8].get_float().unwrap_or_default().to_string())
                        .to_string(),
                    amount: row[9]
                        .get_string()
                        .unwrap_or(&row[9].get_float().unwrap_or_default().to_string())
                        .to_string()
                        .parse()
                        .unwrap(),
                    file_ulid: file_ulid,
                    transaction_status: "pending".to_string(),
                };

                database
                    .create_uploaded_citibank_transfer_initiation_file_record(record)
                    .await?;
            }
            index += 1;
        }
    }

    //delete uploaded file
    fs::remove_file(Path::new(&file_name))?;

    Ok(())
}

/**
 * push the file to citibank
 */
pub async fn init_citibank_transfer(
    _: Token<AdminAccessToken>,
    Json(request): Json<InitCitibankTransferRequest>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    let database = database.lock().await;

    let uploaded_file = database
        .get_uploaded_citibank_transfer_initiation_file(request.file_ulid)
        .await?;

    if uploaded_file.status != "pending" {
        return Err(GlobeliseError::bad_request(
            "This file already pushed to city bank",
        ));
    }

    let pending_records = database
        .list_pending_uploaded_citibank_transfer_initiation_files_records(request.file_ulid)
        .await?;

    for record in pending_records {
        let base_path = std::env::var("CITIBANK_BASE_PATH").expect("base path not set");
        let guid = Uuid::new_v4().to_simple().to_string();
        let local_file = format!("{}citibank_temp/{}.xml", &base_path, &guid);
        //
        generate_xml(
            request.template_name.to_string(),
            local_file.to_string(),
            record.clone(),
        )
        .await;
        //
        let raw_data =
            std::fs::read_to_string(Path::new(&local_file)).expect("failed to read file");
        let enc_data = encrypt(raw_data).await?;
        std::fs::write(Path::new(&local_file), enc_data.as_bytes());
        let sftp_drop_dir = std::env::var("CITIBANK_SFTP_DROP_DIR").expect("path not set");
        let remote_file = format!("{}GRONEXT_PAYROLL_{}.xml", &sftp_drop_dir, &guid);
        sftp_upload(local_file.to_string(), remote_file).await;
        //remove local_file after upload
        fs::remove_file(Path::new(&local_file))?;
        //TODO change status to sent
        database.update_status_uploaded_citibank_transfer_initiation_file_record(
            record.ulid,
            "sent".to_string(),
        );
    }

    //update status
    database.update_status_uploaded_citibank_transfer_initiation_file(
        request.file_ulid,
        "sent".to_string(),
    );

    Ok(())
}

//list all files for a client
pub async fn list_all_uploaded_citibank_transfer_initiation_files_for_client(
    _: Token<AdminAccessToken>,
    Query(request): Query<PaginatedQuery>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<Vec<ListCitiBankTransferInitiationFiles>>> {
    let database = database.lock().await;

    let files = database
        .list_all_uploaded_citibank_transfer_initiation_files_for_client(request)
        .await?;

    Ok(Json(files))
}

//all rocords for a file
pub async fn list_uploaded_citibank_transfer_initiation_files_records(
    _: Token<AdminAccessToken>,
    Query(request): Query<PaginatedQuery>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<Vec<CitiBankPayRollRecord>>> {
    let database = database.lock().await;
    let ulid = request.file_ulid.unwrap_or(Uuid::new_v4());
    let records = database
        .list_uploaded_citibank_transfer_initiation_files_records(ulid)
        .await?;

    Ok(Json(records))
}

//update single uploaded file record
pub async fn update_uploaded_citibank_transfer_initiation_file_record(
    _: Token<AdminAccessToken>,
    Json(record): Json<CitiBankPayRollRecord>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    let database = database.lock().await;
    database
        .update_uploaded_citibank_transfer_initiation_file_record(record)
        .await?;

    Ok(())
}

//delete single uploaded file record
pub async fn delete_uploaded_citibank_transfer_initiation_file_record(
    _: Token<AdminAccessToken>,
    axum::extract::Path(ulid): axum::extract::Path<Uuid>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    let database = database.lock().await;
    database
        .delete_uploaded_citibank_transfer_initiation_file_record(ulid)
        .await?;

    Ok(())
}

impl Database {
    // ============ files

    pub async fn search_clients(
        &self,
        request: SearchClientsQuery,
    ) -> GlobeliseResult<Vec<SearchClientsResponse>> {
        let (limit, offset) = calc_limit_and_offset(request.per_page, request.page);

        let result = sqlx::query_as(
            "SELECT * FROM 
                        onboarded_user_index
                    WHERE name LIKE $1",
        )
        .bind(format!("%{}%", request.name.unwrap_or_default()))
        .fetch_all(&self.0)
        .await?;

        Ok(result)
    }

    pub async fn search_clients_branches(
        &self,
        request: SearchClientsBranchesQuery,
    ) -> GlobeliseResult<Vec<SearchClientsBranchesResponse>> {
        let (limit, offset) = calc_limit_and_offset(request.per_page, request.page);

        let result = sqlx::query_as(
            "SELECT * FROM 
                        search_client_branches
                    WHERE 
                        client_ulid = $1
                    AND 
                        department_name LIKE $2",
        )
        .bind(request.client_ulid)
        .bind(format!("%{}%", request.department_name.unwrap_or_default()))
        .fetch_all(&self.0)
        .await?;

        Ok(result)
    }

    //fech details to prepopulate the template file before downloading
    pub async fn get_contractor_account_details_citibank_template(
        &self,
        branch_ulid: Uuid,
    ) -> GlobeliseResult<Vec<GetContractorAccountDetailsCitibankTemplateResponse>> {
        let result = sqlx::query_as(
            "SELECT * FROM 
                        contractor_account_details_citibank_template
                    WHERE 
                        branch_ulid = $1
                    ",
        )
        .bind(branch_ulid)
        .fetch_all(&self.0)
        .await?;

        Ok(result)
    }

    //fetch by file ulid
    pub async fn get_uploaded_citibank_transfer_initiation_file(
        &self,
        file_ulid: Uuid,
    ) -> GlobeliseResult<ListCitiBankTransferInitiationFiles> {
        let result = sqlx::query_as(
            "SELECT * FROM 
                            uploaded_citibank_transfer_initiation_files
                    WHERE ulid = $1",
        )
        .bind(&file_ulid)
        .fetch_one(&self.0)
        .await?;

        Ok(result)
    }

    pub async fn create_uploaded_citibank_transfer_initiation_file(
        &self,
        request: ListCitiBankTransferInitiationFiles,
    ) -> GlobeliseResult<()> {
        //remove old data with same title, cascades delete records
        sqlx::query(
            "DELETE FROM
                            uploaded_citibank_transfer_initiation_files
                    WHERE title_identifier = $1",
        )
        .bind(&request.title_identifier)
        .execute(&self.0)
        .await?;

        //creates new or updates files and records
        sqlx::query(
            "INSERT INTO
                            uploaded_citibank_transfer_initiation_files
                    (ulid, title_identifier, status, client_ulid)
                    VALUES($1, $2, $3, $4);",
        )
        .bind(&request.ulid)
        .bind(&request.title_identifier)
        .bind(&request.status)
        .bind(&request.client_ulid)
        .execute(&self.0)
        .await?;

        Ok(())
    }

    pub async fn delete_uploaded_citibank_transfer_initiation_file(
        &self,
        ulid: Uuid,
    ) -> GlobeliseResult<()> {
        sqlx::query(
            "DELETE FROM
                            uploaded_citibank_transfer_initiation_files
                    WHERE ulid = $1",
        )
        .bind(ulid)
        .execute(&self.0)
        .await?;

        Ok(())
    }

    pub async fn update_status_uploaded_citibank_transfer_initiation_file(
        &self,
        file_ulid: Uuid,
        status: String,
    ) -> GlobeliseResult<()> {
        sqlx::query(
            "UPDATE 
                                uploaded_citibank_transfer_initiation_files
                        SET 
                        status = $2
                        WHERE ulid = $1",
        )
        .bind(&file_ulid)
        .bind(&status)
        .execute(&self.0)
        .await?;

        Ok(())
    }

    //list by file_ulid
    pub async fn list_uploaded_citibank_transfer_initiation_files(
        &self,
        file_ulid: Uuid,
    ) -> GlobeliseResult<Vec<CitiBankPayRollRecord>> {
        let result = sqlx::query_as(
            "SELECT * FROM
                        uploaded_citibank_transfer_initiation_files
                    WHERE file_ulid = $1",
        )
        .bind(file_ulid)
        .fetch_all(&self.0)
        .await?;

        Ok(result)
    }

    //all files for a client
    pub async fn list_all_uploaded_citibank_transfer_initiation_files_for_client(
        &self,
        request: PaginatedQuery,
    ) -> GlobeliseResult<Vec<ListCitiBankTransferInitiationFiles>> {
        let (limit, offset) = calc_limit_and_offset(request.per_page, request.page);

        let result = sqlx::query_as(
            "SELECT * FROM
                        uploaded_citibank_transfer_initiation_files
                    WHERE client_ulid = $3
                    LIMIT $1 OFFSET $2",
        )
        .bind(limit)
        .bind(offset)
        .bind(&request.client_ulid)
        .fetch_all(&self.0)
        .await?;

        Ok(result)
    }

    // ===================== records

    pub async fn create_uploaded_citibank_transfer_initiation_file_record(
        &self,
        record: CitiBankPayRollRecord,
    ) -> GlobeliseResult<()> {
        sqlx::query(
                "INSERT INTO
                            uploaded_citibank_transfer_initiation_files_records
                    (ulid, company_name, currency_code, country_code, employee_id, employee_name, bank_name_creditor, bank_account_number_creditor, 
                     bic_swift_code_creditor, bank_account_number_debitor, amount, file_ulid, transaction_status)
                    VALUES($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13);"
            )
            .bind(&record.ulid)
            .bind(&record.company_name)
            .bind(&record.currency_code)
            .bind(&record.country_code)
            .bind(&record.employee_id)
            .bind(&record.employee_name)
            .bind(&record.bank_name_creditor)
            .bind(&record.bank_account_number_creditor)
            .bind(&record.bic_swift_code_creditor)
            .bind(&record.bank_account_number_debitor)
            .bind(&record.amount)
            .bind(&record.file_ulid)
            .bind(&record.transaction_status)
            .execute(&self.0)
            .await?;

        Ok(())
    }

    pub async fn update_uploaded_citibank_transfer_initiation_file_record(
        &self,
        record: CitiBankPayRollRecord,
    ) -> GlobeliseResult<()> {
        sqlx::query(
            "UPDATE 
                        uploaded_citibank_transfer_initiation_files_records
                    SET 
                        company_name = $2, 
                        currency_code = $3, 
                        country_code = $4, 
                        employee_id = $5, 
                        employee_name = $6, 
                        bank_name_creditor = $7, 
                        bank_account_number_creditor = $8, 
                        bic_swift_code_creditor = $9, 
                        bank_account_number_debitor = $10, 
                        amount = $11, 
                        file_ulid =  $12, 
                        transaction_status = $13
                   WHERE ulid = $1",
        )
        .bind(&record.ulid)
        .bind(&record.company_name)
        .bind(&record.currency_code)
        .bind(&record.country_code)
        .bind(&record.employee_id)
        .bind(&record.employee_name)
        .bind(&record.bank_name_creditor)
        .bind(&record.bank_account_number_creditor)
        .bind(&record.bic_swift_code_creditor)
        .bind(&record.bank_account_number_debitor)
        .bind(&record.amount)
        .bind(&record.file_ulid)
        .bind(&record.transaction_status)
        .execute(&self.0)
        .await?;

        Ok(())
    }

    pub async fn delete_uploaded_citibank_transfer_initiation_file_record(
        &self,
        ulid: Uuid,
    ) -> GlobeliseResult<()> {
        sqlx::query(
            "DELETE FROM
                            uploaded_citibank_transfer_initiation_files_records
                    WHERE ulid = $1",
        )
        .bind(ulid)
        .execute(&self.0)
        .await?;

        Ok(())
    }

    pub async fn list_uploaded_citibank_transfer_initiation_files_records(
        &self,
        file_ulid: Uuid,
    ) -> GlobeliseResult<Vec<CitiBankPayRollRecord>> {
        let result = sqlx::query_as(
            "SELECT * FROM
                        uploaded_citibank_transfer_initiation_files_records
                    WHERE file_ulid = $1",
        )
        .bind(file_ulid)
        .fetch_all(&self.0)
        .await?;

        Ok(result)
    }

    pub async fn list_pending_uploaded_citibank_transfer_initiation_files_records(
        &self,
        file_ulid: Uuid,
    ) -> GlobeliseResult<Vec<CitiBankPayRollRecord>> {
        let result = sqlx::query_as(
            "SELECT * FROM
                        uploaded_citibank_transfer_initiation_files_records
                    WHERE file_ulid = $1
                    AND transaction_status = 'pending'",
        )
        .bind(file_ulid)
        .fetch_all(&self.0)
        .await?;

        Ok(result)
    }

    pub async fn get_record_uploaded_citibank_transfer_initiation_files_record(
        &self,
        record_ulid: Uuid,
    ) -> GlobeliseResult<CitiBankPayRollRecord> {
        let result = sqlx::query_as(
            "SELECT * FROM
                        uploaded_citibank_transfer_initiation_files_records
                    WHERE ulid = $1",
        )
        .bind(record_ulid)
        .fetch_one(&self.0)
        .await?;

        Ok(result)
    }

    pub async fn update_uploaded_citibank_transfer_initiation_file_record_by_cititbank_transaction_response(
        &self,
        record: CitiBankTransactionResponse,
    ) -> GlobeliseResult<()> {
        sqlx::query(
            "UPDATE
                        uploaded_citibank_transfer_initiation_files_records
                    SET
                    transaction_status = $2,
                    trasaction_status_description = $3
                WHERE ulid = $1",
        )
        .bind(&record.CstmrPmtStsRpt.OrgnlGrpInfAndSts.OrgnlMsgId)
        .bind(&record.CstmrPmtStsRpt.OrgnlGrpInfAndSts.GrpSts)
        .bind(&record.CstmrPmtStsRpt.OrgnlGrpInfAndSts.StsRsnInf.AddtlInf)
        .execute(&self.0)
        .await?;

        Ok(())
    }

    pub async fn update_status_uploaded_citibank_transfer_initiation_file_record(
        &self,
        record_ulid: Uuid,
        status: String,
    ) -> GlobeliseResult<()> {
        sqlx::query(
            "UPDATE 
                                uploaded_citibank_transfer_initiation_files_records
                        SET 
                        transaction_status = $2
                        WHERE ulid = $1",
        )
        .bind(&record_ulid)
        .bind(&status)
        .execute(&self.0)
        .await?;

        Ok(())
    }

    pub async fn delete_status_uploaded_citibank_transfer_initiation_file_record(
        &self,
        record_ulid: Uuid,
        status: String,
    ) -> GlobeliseResult<()> {
        sqlx::query(
            "DELETE FROM 
                            uploaded_citibank_transfer_initiation_files_records
                WHERE ulid = $1",
        )
        .bind(&record_ulid)
        .execute(&self.0)
        .await?;

        Ok(())
    }
}
