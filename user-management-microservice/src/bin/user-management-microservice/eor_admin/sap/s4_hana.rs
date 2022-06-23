use std::io::Cursor;

use axum::{
    extract::{ContentLengthLimit, Extension, Json, Path, Query},
    headers::HeaderName,
    http::{header::CONTENT_TYPE, HeaderMap, HeaderValue},
    response::IntoResponse,
};
use calamine::Reader;
use common_utils::{
    custom_serde::{Country, EmailWrapper, FORM_DATA_LENGTH_LIMIT},
    error::{GlobeliseError, GlobeliseResult},
    token::Token,
};
use eor_admin_microservice_sdk::token::AdminAccessToken;
use itertools::Itertools;
use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};
use serde_with::{base64::Base64, serde_as};
use sqlx::{FromRow, Row};
use strum::IntoStaticStr;
use uuid::Uuid;

use crate::{
    database::{Database, SharedDatabase},
    env::{MULESOFT_API_URL, MULESOFT_CLIENT_ID, MULESOFT_CLIENT_SECRET},
};

#[derive(Debug, Eq, PartialEq, Copy, Clone, Serialize, Deserialize)]
pub enum GlAccount {
    #[serde(rename = "430101001")]
    Salary,
    #[serde(rename = "120202003")]
    BankOutgoing,
}

impl GlAccount {
    pub fn as_str(&self) -> &'static str {
        match self {
            GlAccount::Salary => "430101001",
            GlAccount::BankOutgoing => "120202003",
        }
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, IntoStaticStr, Serialize, Deserialize)]
pub enum DebitCreditCode {
    #[serde(rename = "40")]
    /// Code 40. If amount is positive (payment to center)
    S,
    #[serde(rename = "50")]
    /// Code 50. If amount is negative (payment to bank)
    H,
}

impl DebitCreditCode {
    pub fn as_str(&self) -> &'static str {
        match self {
            DebitCreditCode::S => "S",
            DebitCreditCode::H => "H",
        }
    }

    pub fn as_code(&self) -> &'static str {
        match self {
            DebitCreditCode::S => "40",
            DebitCreditCode::H => "50",
        }
    }
}

#[serde_as]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct PostPayrollJournalS4Hana {
    #[serde_as(as = "Base64")]
    pub file: Vec<u8>,
    pub client_ulid: Uuid,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct GetPrefillIndividualContractorDetailsForBulkUpload {
    pub email: EmailWrapper,
}

pub async fn download(_: Token<AdminAccessToken>) -> impl IntoResponse {
    let bytes = include_bytes!("journal_template.xlsx").to_vec();
    (
        [(
            CONTENT_TYPE,
            HeaderValue::from_static(
                "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
            ),
        )],
        bytes,
    )
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct SapMulesoftPayrollJournalRowQuery {
    pub entry_ulid: Option<Uuid>,
}

#[serde_as]
#[derive(Debug, FromRow, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct SapMulesoftPayrollJournalRow {
    pub ulid: Uuid,
    pub entry_ulid: Uuid,
    pub posting_date: String,
    pub doc_type: String,
    pub company_code: String,
    pub currency_code: String,
    pub reference: String,
    pub debit_credit_code: String,
    pub document_header_text: String,
    pub gl_account: String,
    pub amount: sqlx::types::Decimal,
    pub cost_center: Option<String>,
    pub uploaded: bool,
}

pub async fn get_many_rows(
    // Only for validation
    _: Token<AdminAccessToken>,
    Query(query): Query<SapMulesoftPayrollJournalRowQuery>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<Vec<SapMulesoftPayrollJournalRow>>> {
    let database = database.lock().await;
    let result = database
        .select_many_sap_mulesoft_payroll_journal_rows(query.entry_ulid)
        .await?;
    Ok(Json(result))
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct SapMulesoftPayrollJournalEntryQuery {
    pub client_ulid: Option<Uuid>,
    pub country_code: Option<Country>,
}

#[serde_as]
#[derive(Debug, FromRow, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct SapMulesoftPayrollJournalEntry {
    pub client_ulid: Uuid,
    pub country_code: Country,
}

pub async fn get_many_entries(
    // Only for validation
    _: Token<AdminAccessToken>,
    Query(query): Query<SapMulesoftPayrollJournalEntryQuery>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<Vec<SapMulesoftPayrollJournalEntry>>> {
    let database = database.lock().await;
    let result = database
        .select_many_sap_mulesoft_payroll_journal_entries(query.client_ulid, query.country_code)
        .await?;
    Ok(Json(result))
}

pub async fn download_one_entry(
    // Only for validation
    _: Token<AdminAccessToken>,
    Path(entry_ulid): Path<Uuid>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<(HeaderMap, Vec<u8>)> {
    let database = database.lock().await;
    let result = database
        .download_one_sap_mulesoft_payroll_journal_entry(entry_ulid)
        .await?
        .ok_or_else(|| {
            GlobeliseError::bad_request("Cannot find payroll journal entry with that UUID")
        })?;
    let mut headers = HeaderMap::new();
    headers.insert(
        HeaderName::from_static("content-type"),
        HeaderValue::from_static(
            "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
        ),
    );
    Ok((headers, result))
}

pub async fn post_one(
    // Only for validation
    _: Token<AdminAccessToken>,
    ContentLengthLimit(Json(body)): ContentLengthLimit<
        Json<PostPayrollJournalS4Hana>,
        FORM_DATA_LENGTH_LIMIT,
    >,
    Extension(reqwest_client): Extension<Client>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<String> {
    let database = database.lock().await;

    let mut workbook =
        calamine::open_workbook_auto_from_rs(Cursor::new(&body.file)).map_err(|_| {
            GlobeliseError::bad_request(
                "Unsupported file format. Please provide XLSX, XLSB, XLS or ODS files",
            )
        })?;
    if let Some((_, data)) = workbook.worksheets().get(0) {
        let des = data.deserialize()?;

        #[derive(Debug, Serialize)]
        struct A {
            id: String,
            name: String,
        }

        #[derive(Debug, Serialize)]
        struct B {
            #[serde(rename = "referenceDocumentItem")]
            reference_document_item: String,
            #[serde(rename = "glAccount")]
            gl_account: GlAccount,
            #[serde(rename = "currencyCode")]
            currency_code: String,
            #[serde(rename = "debitCreditCode")]
            debit_credit_code: String,
            #[serde(rename = "costCenter")]
            cost_center: String,
            #[serde(rename = "amount")]
            amount: String,
            #[serde(rename = "documentItemText")]
            document_item_text: Option<String>,
        }

        #[derive(Debug, Serialize)]
        struct C {
            #[serde(rename = "integration")]
            integration: A,
            #[serde(rename = "country_iso")]
            country_iso: String,
            #[serde(rename = "companyCode")]
            company_code: String,
            #[serde(rename = "postingDate")]
            posting_date: String,
            #[serde(rename = "details")]
            details: Vec<B>,
        }

        let raw_payroll_journals = des
            .into_iter()
            .collect::<Result<
                Vec<(
                    // Posting date
                    String,
                    // Doc type
                    String,
                    // Company code
                    String,
                    // Currency
                    String,
                    // Reference
                    String,
                    // Posting key. AKA the debit/credit code
                    DebitCreditCode,
                    // Document header text
                    String,
                    // GL Account
                    GlAccount,
                    // Amount
                    String,
                    // Cost center
                    Option<String>,
                    // Document Item Text
                    Option<String>,
                )>,
                _,
            >>()
            .map_err(|_| {
                GlobeliseError::bad_request(
                    "Invalid Excel file. Please follow the format in the template",
                )
            })?
            .into_iter()
            .map(
                |(
                    posting_date,
                    doc_type,
                    company_code,
                    currency_code,
                    reference,
                    debit_credit_code,
                    document_header_text,
                    gl_account,
                    amount,
                    cost_center,
                    document_item_text,
                )| {
                    Ok(InsertPayrollJournalRowData {
                        posting_date,
                        doc_type,
                        company_code,
                        currency_code,
                        reference,
                        debit_credit_code,
                        document_header_text,
                        gl_account,
                        amount: lexical::parse(amount)?,
                        cost_center_code: cost_center,
                        document_item_text,
                    })
                },
            )
            .collect::<GlobeliseResult<Vec<_>>>()?;

        // VALIDATIONS
        // NOTE: Consider parsing, not validate!

        if raw_payroll_journals.is_empty() {
            return Err(GlobeliseError::bad_request("File cannot be empty"));
        }

        // NOTE: Summation of floating points might not be accurate.
        // We might want to introduce some delta to compensate.
        if raw_payroll_journals.iter().map(|v| v.amount).sum::<f64>() != 0.0f64 {
            return Err(GlobeliseError::bad_request(
                "The total sum of amounts should be equal to 0",
            ));
        };

        if raw_payroll_journals
            .iter()
            .map(|v| &v.company_code)
            .unique()
            .count()
            != 1
        {
            return Err(GlobeliseError::bad_request(
                "All country codes must be the same",
            ));
        };

        let inferred_company_code = raw_payroll_journals
            .get(0)
            .expect("We already checked that there are at least 1 element in the vector")
            .company_code
            .clone();

        if !database
            .sap_mulesoft_payroll_journal_validate_company_code(&inferred_company_code)
            .await?
        {
            return Err(GlobeliseError::bad_request(format!(
                "The company code '{}' does not exist in the database",
                inferred_company_code
            )));
        }

        let inferred_country_code = database
            .sap_mulesoft_payroll_journal_get_country_code_from_company_code(&inferred_company_code)
            .await?
            .ok_or_else(|| {
                GlobeliseError::not_found(format!(
                    "The company code '{}' does not correspond to any countries",
                    inferred_company_code
                ))
            })?;

        if !database
            .sap_mulesoft_payroll_journal_validate_country_code(&inferred_country_code)
            .await?
        {
            return Err(GlobeliseError::bad_request(format!(
                "The country code '{}' does not exist in the database",
                inferred_country_code
            )));
        }

        if raw_payroll_journals
            .iter()
            .map(|v| &v.posting_date)
            .unique()
            .count()
            != 1
        {
            return Err(GlobeliseError::bad_request(
                "All posting dates should be the same",
            ));
        };

        if raw_payroll_journals
            .iter()
            .map(|v| &v.currency_code)
            .unique()
            .count()
            != 1
        {
            return Err(GlobeliseError::bad_request(
                "All rows must use the same currency",
            ));
        };

        if raw_payroll_journals
            .iter()
            .any(|v| v.gl_account == GlAccount::BankOutgoing && v.cost_center_code.is_some())
        {
            return Err(GlobeliseError::bad_request(
                "Cost center for the bank outgoing must be empty",
            ));
        }

        if raw_payroll_journals
            .iter()
            .any(|row| row.amount > 0.0f64 && row.debit_credit_code == DebitCreditCode::H)
        {
            return Err(GlobeliseError::bad_request(
                "When posting key 50 AKA H AKA credit then the amount must be negative",
            ));
        }

        if raw_payroll_journals
            .iter()
            .any(|row| row.amount < 0.0f64 && row.debit_credit_code == DebitCreditCode::S)
        {
            return Err(GlobeliseError::bad_request(
                "When posting key 40 AKA S AKA debit then the amount must be positive",
            ));
        }

        let cost_center_codes = raw_payroll_journals
            .iter()
            .filter_map(|v| v.cost_center_code.as_ref())
            .unique()
            .collect::<Vec<_>>();

        if !database
            .sap_mulesoft_payroll_journal_validate_cost_center_code(
                &inferred_company_code,
                &cost_center_codes,
            )
            .await?
        {
            return Err(GlobeliseError::bad_request(format!(
                "Company code '{}' and one of the cost center codes in '{:?}' does not match",
                inferred_company_code, cost_center_codes
            )));
        }

        let ulid = database
            .insert_sap_mulesoft_payroll_journal(
                body.client_ulid,
                &inferred_country_code,
                &raw_payroll_journals,
                &body.file,
            )
            .await?;

        let posting_date = raw_payroll_journals
            .iter()
            .map(|v| v.posting_date.clone())
            .next()
            .expect("We already checked that there is only 1 unique posting date");

        let payroll_journals = raw_payroll_journals
            .into_iter()
            .enumerate()
            .map(|(idx, row)| -> GlobeliseResult<B> {
                Ok(B {
                    reference_document_item: (idx + 1).to_string(),
                    gl_account: row.gl_account,
                    currency_code: row.currency_code,
                    debit_credit_code: row.debit_credit_code.as_str().to_string(),
                    cost_center: row.cost_center_code.unwrap_or_default(),
                    amount: row.amount.to_string(),
                    document_item_text: row.document_item_text,
                })
            })
            .collect::<GlobeliseResult<Vec<B>>>()?;

        let json_body = C {
            integration: A {
                id: ulid.to_string(),
                name: "payroll".to_string(),
            },
            country_iso: inferred_country_code,
            company_code: inferred_company_code,
            posting_date,
            details: payroll_journals,
        };

        let response = reqwest_client
            .post(format!(
                "{}/master-pub-v1/api/transaction/journalentry",
                *MULESOFT_API_URL
            ))
            .header("client_id", &*MULESOFT_CLIENT_ID)
            .header("client_secret", &*MULESOFT_CLIENT_SECRET)
            .json(&json_body)
            .send()
            .await?;

        if response.status() == StatusCode::CREATED {
            database
                .update_sap_mulesoft_payroll_journal_as_uploaded(ulid)
                .await?;
        }
        Ok(response.text().await?)
    } else {
        Err(GlobeliseError::bad_request("No worksheets provided"))
    }
}

#[derive(Debug, Clone)]
pub struct InsertPayrollJournalRowData {
    posting_date: String,
    doc_type: String,
    company_code: String,
    currency_code: String,
    reference: String,
    debit_credit_code: DebitCreditCode,
    document_header_text: String,
    gl_account: GlAccount,
    amount: f64,
    cost_center_code: Option<String>,
    document_item_text: Option<String>,
}

impl Database {
    pub async fn select_many_sap_mulesoft_payroll_journal_rows(
        &self,
        entry_ulid: Option<Uuid>,
    ) -> GlobeliseResult<Vec<SapMulesoftPayrollJournalRow>> {
        let result = sqlx::query_as(
            "
        SELECT
            ulid, entry_ulid, posting_date, doc_type, company_code, currency_code, 
            reference, debit_credit_code, document_header_text, gl_account, amount, 
            cost_center, uploaded
        FROM
            sap_mulesoft_payroll_journals_rows
        WHERE
            ($1 IS NULL OR entry_ulid = $1)",
        )
        .bind(entry_ulid)
        .fetch_all(&self.0)
        .await?;

        Ok(result)
    }

    pub async fn select_many_sap_mulesoft_payroll_journal_entries(
        &self,
        client_ulid: Option<Uuid>,
        country_code: Option<Country>,
    ) -> GlobeliseResult<Vec<SapMulesoftPayrollJournalEntry>> {
        let result = sqlx::query_as(
            "
        SELECT
            ulid, client_ulid, country_code
        FROM
            sap_mulesoft_payroll_journals_entries
        WHERE
            ($1 IS NULL OR client_ulid = $1) AND
            ($2 IS NULL OR country_code = $2)",
        )
        .bind(client_ulid)
        .bind(country_code)
        .fetch_all(&self.0)
        .await?;

        Ok(result)
    }

    pub async fn download_one_sap_mulesoft_payroll_journal_entry(
        &self,
        ulid: Uuid,
    ) -> GlobeliseResult<Option<Vec<u8>>> {
        let result = sqlx::query(
            "
        SELECT
            uploaded_file
        FROM
            sap_mulesoft_payroll_journals_entries
        WHERE
            ulid = $1",
        )
        .bind(ulid)
        .fetch_optional(&self.0)
        .await?
        .map(|r| r.try_get("uploaded_file"))
        .transpose()?;

        Ok(result)
    }

    pub async fn insert_sap_mulesoft_payroll_journal(
        &self,
        client_ulid: Uuid,
        country_code: &String,
        rows: &[InsertPayrollJournalRowData],
        file: &[u8],
    ) -> GlobeliseResult<Uuid> {
        let ulid = Uuid::new_v4();

        sqlx::query(
            "
        INSERT INTO sap_mulesoft_payroll_journals_entries (
            ulid, client_ulid, country_code, uploaded_file
        ) VALUES (
            $1, $2, $3, $4
        )",
        )
        .bind(ulid)
        .bind(client_ulid)
        .bind(country_code)
        .bind(file)
        .execute(&self.0)
        .await?;

        let rows_query = "
        INSERT INTO sap_mulesoft_payroll_journals_rows (
            ulid, entry_ulid, posting_date, doc_type, company_code, currency_code, 
            reference, debit_credit_code, document_header_text, gl_account, amount, 
            cost_center
        ) 
        SELECT
            *
        FROM UNNEST (
            $1, $2, $3, $4, $5,
            $6, $7, $8, $9, $10, 
            $11, $12
        ) RETURNING
            entry_ulid, posting_date, doc_type, company_code, currency_code, 
            reference, debit_credit_code, document_header_text, gl_account, amount, 
            cost_center";

        sqlx::query(rows_query)
            .bind(rows.iter().map(|_| Uuid::new_v4()).collect::<Vec<_>>())
            .bind(rows.iter().map(|_| ulid).collect::<Vec<_>>())
            .bind(
                rows.iter()
                    .cloned()
                    .map(|v| v.posting_date)
                    .collect::<Vec<_>>(),
            )
            .bind(rows.iter().cloned().map(|v| v.doc_type).collect::<Vec<_>>())
            .bind(
                rows.iter()
                    .map(|v| v.company_code.clone())
                    .collect::<Vec<_>>(),
            )
            .bind(
                rows.iter()
                    .map(|v| v.currency_code.clone())
                    .collect::<Vec<String>>(),
            )
            .bind(
                rows.iter()
                    .cloned()
                    .map(|v| v.reference)
                    .collect::<Vec<_>>(),
            )
            .bind(
                rows.iter()
                    .map(|v| v.debit_credit_code.as_str())
                    .collect::<Vec<&'static str>>(),
            )
            .bind(
                rows.iter()
                    .cloned()
                    .map(|v| v.document_header_text)
                    .collect::<Vec<_>>(),
            )
            .bind(
                rows.iter()
                    .map(|v| v.gl_account.as_str())
                    .collect::<Vec<&'static str>>(),
            )
            .bind(rows.iter().cloned().map(|v| v.amount).collect::<Vec<_>>())
            .bind(
                rows.iter()
                    .map(|v| v.cost_center_code.clone())
                    .collect::<Vec<Option<String>>>(),
            )
            .execute(&self.0)
            .await?;

        Ok(ulid)
    }

    pub async fn update_sap_mulesoft_payroll_journal_as_uploaded(
        &self,
        ulid: Uuid,
    ) -> GlobeliseResult<()> {
        sqlx::query(
            "
            UPDATE
                sap_mulesoft_payroll_journals_rows
            SET
                uploaded = 't'
            WHERE
                entry_ulid = $1",
        )
        .bind(ulid)
        .execute(&self.0)
        .await?;

        Ok(())
    }

    pub async fn sap_mulesoft_payroll_journal_validate_country_code(
        &self,
        country_code: &String,
    ) -> GlobeliseResult<bool> {
        let result = sqlx::query(
            "
            SELECT
                country_code
            FROM
                sap_mulesoft_payroll_journal_countries
            WHERE
                country_code = $1",
        )
        .bind(country_code)
        .fetch_optional(&self.0)
        .await?
        .is_some();

        Ok(result)
    }

    pub async fn sap_mulesoft_payroll_journal_get_country_code_from_company_code(
        &self,
        company_code: &String,
    ) -> GlobeliseResult<Option<String>> {
        let result = sqlx::query(
            "
            SELECT
                code, country_code
            FROM
                sap_mulesoft_payroll_journal_company_codes
            WHERE
                code = $1",
        )
        .bind(company_code)
        .fetch_optional(&self.0)
        .await?
        .map(|row| row.try_get("country_code"))
        .transpose()?;

        Ok(result)
    }

    pub async fn sap_mulesoft_payroll_journal_validate_company_code(
        &self,
        company_code: &String,
    ) -> GlobeliseResult<bool> {
        let result = sqlx::query(
            "
            SELECT
                code
            FROM
                sap_mulesoft_payroll_journal_company_codes
            WHERE
                code = $1",
        )
        .bind(company_code)
        .fetch_optional(&self.0)
        .await?
        .is_some();

        Ok(result)
    }

    pub async fn sap_mulesoft_payroll_journal_validate_cost_center_code(
        &self,
        company_code: &String,
        cost_center_codes: &[&String],
    ) -> GlobeliseResult<bool> {
        for cost_center_code in cost_center_codes {
            let result = sqlx::query(
                "
            SELECT
                code, company_code
            FROM
                sap_mulesoft_payroll_journal_cost_centers
            WHERE
                company_code = $1 AND
                code = $2",
            )
            .bind(company_code)
            .bind(cost_center_code)
            .fetch_optional(&self.0)
            .await?;

            if result.is_none() {
                return Ok(false);
            }
        }

        Ok(true)
    }
}
