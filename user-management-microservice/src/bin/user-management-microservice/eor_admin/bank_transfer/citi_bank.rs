use axum::extract::ContentLengthLimit;
use axum::extract::{Extension, Json, Query};
use calamine::{open_workbook, Reader, Xlsx};
use chrono;
use common_utils::token::Token;
use common_utils::{
    calc_limit_and_offset,
    custom_serde::FORM_DATA_LENGTH_LIMIT,
    error::{GlobeliseError, GlobeliseResult},
};
use eor_admin_microservice_sdk::token::AdminAccessToken;
use reqwest::header::HeaderMap;
use reqwest::header::{HeaderName, HeaderValue};
use serde::{Deserialize, Serialize};
use serde_with::{base64::Base64, serde_as};
use sqlx::FromRow;
use ssh2::Session;
use std::process::Command;
use std::sync::Arc;
use tokio::sync::Mutex;
use std::{
    fs::{self},
    io::{prelude::*, Write},
    net::TcpStream,
    path::Path,
    str,
};
use umya_spreadsheet::*;
use user_management_microservice_sdk::user::UserType;
use uuid::Uuid;

use crate::database::{Database, SharedDatabase};

use super::citibank_ack_file::CitiBankACKFile;
use super::citibank_acpt_file::CitiBankACPTFile;
use super::citibank_rjct_file::CitiBankRJCTFile;
use substring::Substring;

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
    pub employee_id: Uuid,     // employee id
    pub employee_name: String, //employee name
    pub bank_name: String,     //bank name
    pub bank_account_number: String,
    pub bank_code: String,
    pub bank_branch_code: String,
    pub branch_ulid: Uuid, //client's branch ulid this contractor belongs too
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct InitCitibankTransferRequest {
    pub file_ulid: Uuid,
    pub template_name: String, //eg. template.xml
}
#[serde_as]
#[derive(Debug, Serialize, FromRow, Deserialize, Clone)]
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
    pub currency_code: String,
    pub country_code: String,
    pub employee_id: Uuid,
    pub employee_name: String,
    pub bank_name: String,
    pub bank_account_number: String,
    pub bank_code: String,
    pub swift_code: String,
    pub amount: f64,
    pub file_ulid: Uuid,
    pub transaction_status: String,
    pub transaction_status_description: Option<String>,
}

// ================ SUMMARY =========================
//encrypt(raw) -> enc;
//decrypt(enc) -> raw;
//sftp_upload(local_file, remote_file);
//sftp_download(local_file, remote_file);
//sftp_list_remote_files(sftp_root_dir) -> Vec<String>
//sftp_delete_remote_file(remote_file)
//generate_xml(template.xml, local_file, record);
//list_available_templates() -> Vec<String>
//download_citibank_transfer_initiation_template() -> FILE.xlxs
//upload_citibank_transfer_initiation_template(UploadCitiBankTransferInitiationFiles.xlxs)
//parseACK(local_file.xml) -> CitiBankACKFile
//parseACPT(local_file.xml) -> CitiBankACPTFile
//parseRJCT(local_file.xml) -> CitiBankRJCTFile

//init_citibank_transfer(file_ulid) -> pushes transction.xml file to citibank ftp folder
//refresh_citibank_transfers() -> checks transaction status and updates the records

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

/* download remote files*/
pub async fn update_transaction_status(
    _: Token<AdminAccessToken>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    let sftp_root_dir = std::env::var("CITIBANK_SFTP_ROOT_DIR").expect("failed to get root dir");
    let base_path = std::env::var("CITIBANK_BASE_PATH").expect("base_path not set");
    let remote_files = sftp_list_remote_files(sftp_root_dir).await?;
    // let guid = Uuid::new_v4().to_simple().to_string();
    let sftp_drop_dir = std::env::var("CITIBANK_SFTP_DROP_DIR").expect("sftp_drop_dir_not set");
    for file in remote_files {
        //skip this directory
        if file.replace("/", "") == sftp_drop_dir.replace("/", "") {
            continue;
        }
        let file_name = file.replace("/GRONEXTPYRSFTP/", "");
        let local_file = format!(
            "{}citibank_temp_transaction_files/{}.xml",
            &base_path, &file_name
        );
        sftp_download(local_file.to_string(), file.to_string()).await;
        //sftp_delete_remote_file(file.to_string()).await?;
    }

    _update_transaction_status(database).await?;

    Ok(())
}

/* update transaction status*/
pub async fn _update_transaction_status(db: Arc<Mutex<Database>>) -> GlobeliseResult<()> {
    let database = db.lock().await;

    let root_dir = std::env::var("CITIBANK_BASE_PATH").expect("base_path not set");
    let templates_folder = format!("{}citibank_temp_transaction_files/", &root_dir);
    let files = std::fs::read_dir(Path::new(&templates_folder))?;
    for file in files {
        let file_name = file?.file_name().to_str().expect("").to_string();
        let file_path = format!("{}{}", &templates_folder, &file_name);

        let enc_data = std::fs::read_to_string(Path::new(&file_path)).expect("failed to read file");
        let raw_data = decrypt(enc_data).await?;

        let mut record_ulid = Uuid::new_v4();
        let mut transaction_status: String = "".to_string();
        let mut transaction_status_description: String = "".to_string();

        if file_name.contains("ACK") {
            let citibank_response = parseACK(raw_data).await?;
            let file_ulid = Uuid::parse_str(
                &citibank_response
                    .CstmrPmtStsRpt
                    .OrgnlGrpInfAndSts
                    .OrgnlMsgId,
            )
            .unwrap();
            println!("================{}=================", file_ulid);
            transaction_status = "ack".to_string();
            transaction_status_description = "transaction acknowledged".to_string();
            database
                .update_status_uploaded_citibank_transfer_initiation_file(
                    file_ulid,
                    transaction_status,
                )
                .await?;
        } else if file_name.contains("ACPT") {
            let citibank_response = parseACPT(raw_data).await?;

            for record in citibank_response.CstmrPmtStsRpt.OrgnlPmtInfAndSts {
                record_ulid = Uuid::parse_str(&record.OrgnlPmtInfId).unwrap();
                transaction_status = "acpt".to_string();
                transaction_status_description = "transaction accepted".to_string();
                database.update_uploaded_citibank_transfer_initiation_file_record_by_cititbank_transaction_response(record_ulid, transaction_status, transaction_status_description).await?;
            }
        } else if file_name.contains("RJCT") {
            let citibank_response = parseRJCT(raw_data).await?;

            for record in citibank_response.CstmrPmtStsRpt.OrgnlPmtInfAndSts {
                record_ulid = Uuid::parse_str(&record.OrgnlPmtInfId).unwrap();
                transaction_status = "rcjt".to_string();
                let mut reject_reason: String = "".to_string();

                for reason in record.TxInfAndSts.StsRsnInf {
                    for aditional_info in reason.AddtlInf {
                        reject_reason.push_str(&format!("{}, ", &aditional_info));
                    }
                }

                transaction_status_description = reject_reason;

                database.update_uploaded_citibank_transfer_initiation_file_record_by_cititbank_transaction_response(record_ulid, transaction_status, transaction_status_description).await?;
            }
        }

        fs::remove_file(Path::new(&file_path)).expect("error removing file");
    }

    Ok(())
}

//parse xml string to CitiBankACKFile object
async fn parseACK(src: String) -> GlobeliseResult<CitiBankACKFile> {
    let item = serde_xml_rs::from_str(&src).unwrap();

    Ok(item)
}

//parse xml string to CitiBankACPTFile object
async fn parseACPT(src: String) -> GlobeliseResult<CitiBankACPTFile> {
    let item = serde_xml_rs::from_str(&src).unwrap();

    Ok(item)
}

//parse xml string to CitiBankRJCTFile object
async fn parseRJCT(src: String) -> GlobeliseResult<CitiBankRJCTFile> {
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
    let enc_key = std::env::var("CITIBANK_SFTP_ENCRYPTION_KEY")?;
    let sign_key = std::env::var("CITIBANK_SFTP_SIGN_KEY")?;

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
    cmd.wait()?;
    //
    let encrypted_data = fs::read_to_string(Path::new(&path_enc_data))?;
    //delete these files
    fs::remove_file(Path::new(&path_raw_data))?;
    fs::remove_file(Path::new(&path_enc_data))?;

    // Ok(raw_data)
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
    cmd.wait()?;
    //
    let decrypted_data = fs::read_to_string(Path::new(&path_raw_data))?;
    //delete these files
    fs::remove_file(Path::new(&path_raw_data))?;
    fs::remove_file(Path::new(&path_enc_data))?;

    Ok(decrypted_data)
}

async fn sftp_upload(local_file: String, remote_file: String) -> GlobeliseResult<()> {
    // Connect to the SSH server
    let sftp_username = std::env::var("CITIBANK_SFTP_USERNAME")?;
    let sftp_password = std::env::var("CITIBANK_SFTP_PASSWORD")?;
    let sftp_host = std::env::var("CITIBANK_SFTP_HOST")?;
    let sftp_port = std::env::var("CITIBANK_SFTP_PORT")?;
    let tcp = TcpStream::connect(format!("{}:{}", &sftp_host, &sftp_port))?;
    //
    let data = std::fs::read_to_string(Path::new(&local_file)).expect("failed to read data");
    //
    let mut sess = Session::new()?;
    sess.set_tcp_stream(tcp);
    sess.handshake()?;
    sess.userauth_password(&sftp_username, &sftp_password)?;
    //
    let sftp_client = sess.sftp()?;
    let mut sftp_file = sftp_client.create(Path::new(&remote_file))?;
    sftp_file.write_all(data.as_bytes())?;

    Ok(())
}

async fn sftp_download(local_file: String, remote_file: String) {
    // Connect to the SSH server
    let sftp_username =
        std::env::var("CITIBANK_SFTP_USERNAME").expect("CITIBANK_SFTP_USERNAME not set");
    let sftp_password =
        std::env::var("CITIBANK_SFTP_PASSWORD").expect("CITIBANK_SFTP_PASSWORD not set");
    let sftp_host = std::env::var("CITIBANK_SFTP_HOST").expect("CITIBANK_SFTP_HOST not set");
    let sftp_port = std::env::var("CITIBANK_SFTP_PORT").expect("CITIBANK_SFTP_PORT not set");
    let tcp = TcpStream::connect(format!("{}:{}", &sftp_host, &sftp_port))
        .expect("tcp connection failed");
    let mut sess = Session::new().expect("");
    sess.set_tcp_stream(tcp);
    sess.handshake().expect("");
    sess.userauth_password(&sftp_username, &sftp_password)
        .expect("");
    let sftp_client = sess.sftp().expect("");

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
    let sftp_username = std::env::var("CITIBANK_SFTP_USERNAME")?;
    let sftp_password = std::env::var("CITIBANK_SFTP_PASSWORD")?;
    let sftp_host = std::env::var("CITIBANK_SFTP_HOST")?;
    let sftp_port = std::env::var("CITIBANK_SFTP_PORT")?;
    let tcp = TcpStream::connect(format!("{}:{}", &sftp_host, &sftp_port))?;
    let mut sess = Session::new()?;
    sess.set_tcp_stream(tcp);
    sess.handshake()?;
    sess.userauth_password(&sftp_username, &sftp_password)?;
    let sftp_client = sess.sftp()?;
    let mut remote_files = Vec::new();
    let path_buf = sftp_client
        .readdir(Path::new(&sftp_root_dir))
        .expect("failed to read dir");
    for file in path_buf {
        let file_name = file.0.as_os_str().to_str().expect("").to_string();
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
    let files = std::fs::read_dir(Path::new(&templates_folder))?;
    for file in files {
        let file_name = file?.file_name().to_str().expect("").to_string();
        //skip this citibank_transfer_command_template_file
        if file_name == "citibank_transfer_command_template_file.xlsx" {
            continue;
        }
        //skip this item file
        if file_name.contains("item") {
            continue;
        }
        available_templates.push(file_name.to_string());
    }

    Ok(Json(available_templates))
}

async fn sftp_delete_remote_file(remote_file: String) -> GlobeliseResult<()> {
    // Connect to the SSH server
    let sftp_username = std::env::var("CITIBANK_SFTP_USERNAME")?;
    let sftp_password = std::env::var("CITIBANK_SFTP_PASSWORD")?;
    let sftp_host = std::env::var("CITIBANK_SFTP_HOST")?;
    let sftp_port = std::env::var("CITIBANK_SFTP_PORT")?;
    let tcp = TcpStream::connect(format!("{}:{}", &sftp_host, &sftp_port))?;
    let mut sess = Session::new()?;
    sess.set_tcp_stream(tcp);
    sess.handshake()?;
    sess.userauth_password(&sftp_username, &sftp_password)?;
    let sftp_client = sess.sftp()?;
    sftp_client.unlink(Path::new(&remote_file))?;

    Ok(())
}

async fn generate_xml(
    template_name: String,
    local_file: String,
    transaction_file: ListCitiBankTransferInitiationFiles,
    records: Vec<CitiBankPayRollRecord>,
) {
    let base_path = std::env::var("CITIBANK_BASE_PATH").expect("base path not set");
    //max 35 chars
    let guid = transaction_file.ulid.to_simple().to_string().to_uppercase();

    let items_string_template = fs::read_to_string(Path::new(&format!(
        "{}citibank_templates/{}",
        &base_path,
        &template_name.replace(".xml", "_item.xml")
    )))
    .unwrap();

    let mut items = "".to_string();

    let mut data = fs::read_to_string(Path::new(&format!(
        "{}citibank_templates/{}",
        &base_path, &template_name
    )))
    .unwrap();

    let mut control_sum_total = 0.0;
    let mut number_of_transactions = 0;
    let now_date_string = chrono::offset::Local::now().format("%Y-%m-%d").to_string();

    //create records
    for record in records {
        let mut item = items_string_template.clone();
        item = str::replace(
            &item,
            "{{PmtInfId}}",
            &record.ulid.to_simple().to_string().to_uppercase(),
        );
        // item = str::replace(&item, "{{PmtInfId}}", &Uuid::new_v4().to_simple().to_string().to_uppercase());
        item = str::replace(&item, "{{ReqdExctnDt}}", &now_date_string);
        item = str::replace(
            &item,
            "{{EndToEndId}}",
            &record
                .ulid
                .to_simple()
                .to_string()
                .to_uppercase()
                .substring(0, 16),
        ); //max 15 chars
        item = str::replace(&item, "{{InstdAmt}}", &record.amount.to_string());
        item = str::replace(&item, "{{bank_code}}", &record.bank_code);
        item = str::replace(&item, "{{Cdtr_Nm}}", &record.employee_name);
        item = str::replace(&item, "{{CdtrAcct_Id}}", &record.bank_account_number);
        item = str::replace(&item, "{{RmtInf_Ustrd}}", "Globelise Salary Payment");

        control_sum_total += record.amount;
        number_of_transactions += 1;

        items.push_str(&item.trim());
    }

    data = str::replace(&data, "{{MsgId}}", &guid); //unique ID
                                                    // data = str::replace(&data, "{{MsgId}}", &Uuid::new_v4().to_simple().to_string().to_uppercase()); //unique ID
    data = str::replace(
        &data,
        "{{CreDtTm}}",
        &chrono::offset::Local::now()
            .format("%Y-%m-%dT%H:%M:%S")
            .to_string(),
    );
    data = str::replace(&data, "{{CtrlSum}}", &format!("{:.2}", &control_sum_total)); //transfer amount total
    data = str::replace(&data, "{{NbOfTxs}}", &number_of_transactions.to_string()); //number of records
    data = str::replace(&data, "{{items}}", &items.trim()); //transaction items

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
        HeaderValue::from_str(&content_type).expect(""),
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
    book.new_sheet("Sheet1");

    //add headers
    book.get_sheet_by_name_mut("Sheet1")
        .unwrap()
        .get_cell_mut("A1")
        .set_value("Currency Code");
    book.get_sheet_by_name_mut("Sheet1")
        .unwrap()
        .get_cell_mut("A1")
        .set_value("Currency Code");
    book.get_sheet_by_name_mut("Sheet1")
        .unwrap()
        .get_cell_mut("B1")
        .set_value("Country Code");
    book.get_sheet_by_name_mut("Sheet1")
        .unwrap()
        .get_cell_mut("C1")
        .set_value("Employee ID");
    book.get_sheet_by_name_mut("Sheet1")
        .unwrap()
        .get_cell_mut("D1")
        .set_value("Employee Name");
    book.get_sheet_by_name_mut("Sheet1")
        .unwrap()
        .get_cell_mut("E1")
        .set_value("Bank Name");
    book.get_sheet_by_name_mut("Sheet1")
        .unwrap()
        .get_cell_mut("F1")
        .set_value("Bank Account Number");
    book.get_sheet_by_name_mut("Sheet1")
        .unwrap()
        .get_cell_mut("G1")
        .set_value("Bank Code");
    book.get_sheet_by_name_mut("Sheet1")
        .unwrap()
        .get_cell_mut("H1")
        .set_value("Swift code");
    book.get_sheet_by_name_mut("Sheet1")
        .unwrap()
        .get_cell_mut("I1")
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

    let mut row_index = 2; //start at 2 because of headers
    for contractor in contractors {
        book.get_sheet_by_name_mut("Sheet1")
            .unwrap()
            .get_cell_mut(&format!("A{}", row_index))
            .set_value(&request.currency);
        book.get_sheet_by_name_mut("Sheet1")
            .unwrap()
            .get_cell_mut(&format!("B{}", row_index))
            .set_value(&request.country);
        book.get_sheet_by_name_mut("Sheet1")
            .unwrap()
            .get_cell_mut(&format!("C{}", row_index))
            .set_value(&contractor.employee_id.to_string());
        book.get_sheet_by_name_mut("Sheet1")
            .unwrap()
            .get_cell_mut(&format!("D{}", row_index))
            .set_value(&contractor.employee_name);
        book.get_sheet_by_name_mut("Sheet1")
            .unwrap()
            .get_cell_mut(&format!("E{}", row_index))
            .set_value(&contractor.bank_name);
        book.get_sheet_by_name_mut("Sheet1")
            .unwrap()
            .get_cell_mut(&format!("F{}", row_index))
            .set_value(&contractor.bank_account_number);
        book.get_sheet_by_name_mut("Sheet1")
            .unwrap()
            .get_cell_mut(&format!("G{}", row_index))
            .set_value(&contractor.bank_code);
        book.get_sheet_by_name_mut("Sheet1")
            .unwrap()
            .get_cell_mut(&format!("H{}", row_index))
            .set_value(&contractor.bank_code); //should be swift code

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
    let mut excel: Xlsx<_> = open_workbook(&file_name)?;
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
                let employee_id_ulid =
                    Uuid::parse_str(&row[2].get_string().unwrap_or_default().to_string())?;

                let record = CitiBankPayRollRecord {
                    ulid: Uuid::new_v4(),
                    currency_code: row[0].get_string().unwrap_or_default().to_string(),
                    country_code: row[1].get_string().unwrap_or_default().to_string(),
                    employee_id: employee_id_ulid,
                    employee_name: row[3].get_string().unwrap_or_default().to_string(),
                    bank_name: row[4].get_string().unwrap_or_default().to_string(),
                    bank_account_number: row[5]
                        .get_string()
                        .unwrap_or(&row[5].get_float().unwrap_or_default().to_string())
                        .to_string()
                        .parse()?,
                    bank_code: row[6]
                        .get_string()
                        .unwrap_or(&row[6].get_float().unwrap_or_default().to_string())
                        .to_string()
                        .parse()?,
                    swift_code: row[7]
                        .get_string()
                        .unwrap_or(&row[7].get_float().unwrap_or_default().to_string())
                        .to_string(),
                    amount: row[8]
                        .get_string()
                        .unwrap_or(&row[8].get_float().unwrap_or_default().to_string())
                        .to_string()
                        .parse()?,
                    file_ulid: file_ulid,
                    transaction_status: "pending".to_string(),
                    transaction_status_description: Some("pending".to_string()),
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
            "This file is already pushed to city bank",
        ));
    }

    let transaction_file = database
        .get_uploaded_citibank_transfer_initiation_file(request.file_ulid)
        .await?;

    let records = database
        .list_uploaded_citibank_transfer_initiation_files_records(request.file_ulid)
        .await?;

    let base_path = std::env::var("CITIBANK_BASE_PATH").expect("base path not set");
    let guid = Uuid::new_v4().to_simple().to_string();
    let local_file = format!("{}citibank_temp/{}.xml", &base_path, &guid);

    generate_xml(
        request.template_name.to_string(),
        local_file.to_string(),
        transaction_file.clone(),
        records,
    )
    .await;

    let raw_data = std::fs::read_to_string(Path::new(&local_file)).expect("failed to read file");
    let enc_data = encrypt(raw_data).await?;
    std::fs::write(Path::new(&local_file), enc_data.as_bytes());
    let sftp_drop_dir = std::env::var("CITIBANK_SFTP_DROP_DIR").expect("path not set");
    let remote_file = format!("{}GRONEXT_PAYROLL_{}.xml", &sftp_drop_dir, &guid);
    sftp_upload(local_file.to_string(), remote_file).await;
    //remove local_file after upload
    fs::remove_file(Path::new(&local_file))?;
    //update status
    database
        .update_status_uploaded_citibank_transfer_initiation_file(
            request.file_ulid,
            "sent".to_string(),
        )
        .await?;

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

//all rocords for a file
pub async fn list_all_uploaded_citibank_transfer_initiation_files_records(
    _: Token<AdminAccessToken>,
    file_ulid: Uuid,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<Vec<CitiBankPayRollRecord>>> {
    let database = database.lock().await;
    let records = database
        .list_all_uploaded_citibank_transfer_initiation_files_records(file_ulid)
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

    //fetch details to prepopulate the template file before downloading
    pub async fn get_contractor_account_details_citibank_template(
        &self,
        branch_ulid: Uuid,
    ) -> GlobeliseResult<Vec<GetContractorAccountDetailsCitibankTemplateResponse>> {
        let result = sqlx::query_as(
            "SELECT * FROM 
                        contractor_bank_account_details_citibank_template
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

        println!("{}:{}", file_ulid, status);
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
                    (ulid, currency_code, country_code, employee_id, employee_name, bank_name, bank_account_number, 
                     bank_code, swift_code, amount, file_ulid, transaction_status)
                    VALUES($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13);"
            )
            .bind(&record.ulid)
            .bind(&record.currency_code)
            .bind(&record.country_code)
            .bind(&record.employee_id)
            .bind(&record.employee_name)
            .bind(&record.bank_name)
            .bind(&record.bank_account_number)
            .bind(&record.bank_code)
            .bind(&record.swift_code)
            .bind(&record.amount)
            .bind(&record.file_ulid)
            .bind(&record.transaction_status)
            .bind(&record.transaction_status_description)
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
                        currency_code = $2, 
                        country_code = $3, 
                        employee_id = $4, 
                        employee_name = $5, 
                        bank_name = $6, 
                        bank_account_number = $7, 
                        bank_code = $8, 
                        swift_code = $9, 
                        amount = $10, 
                        file_ulid =  $11, 
                        transaction_status = $12,
                        transaction_status_description = $13
                   WHERE ulid = $1",
        )
        .bind(&record.ulid)
        .bind(&record.currency_code)
        .bind(&record.country_code)
        .bind(&record.employee_id)
        .bind(&record.employee_name)
        .bind(&record.bank_name)
        .bind(&record.bank_account_number)
        .bind(&record.bank_code)
        .bind(&record.swift_code)
        .bind(&record.amount)
        .bind(&record.file_ulid)
        .bind(&record.transaction_status)
        .bind(&record.transaction_status_description)
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

    pub async fn list_all_uploaded_citibank_transfer_initiation_files_records(
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
        record_ulid: Uuid,
        transaction_status: String,
        transaction_status_description: String,
    ) -> GlobeliseResult<()> {
        sqlx::query(
            "UPDATE
                        uploaded_citibank_transfer_initiation_files_records
                    SET
                    transaction_status = $2,
                    transaction_status_description = $3
                WHERE ulid = $1",
        )
        .bind(record_ulid)
        .bind(transaction_status)
        .bind(transaction_status_description)
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
