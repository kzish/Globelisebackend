use std::io::Cursor;

use axum::extract::{ContentLengthLimit, Extension, Json};
use calamine::Reader;
use common_utils::{
    custom_serde::FORM_DATA_LENGTH_LIMIT,
    error::{GlobeliseError, GlobeliseResult},
    token::Token,
};
use email_address::EmailAddress;
use eor_admin_microservice_sdk::token::AdminAccessToken;
use itertools::Itertools;
use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};
use serde_with::{base64::Base64, serde_as};
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
    pub country_code: String,
    pub company_code: String,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct GetPrefillIndividualContractorDetailsForBulkUpload {
    pub email: EmailAddress,
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

    if !database
        .sap_mulesoft_payroll_journal_validate_country_code(&body.country_code)
        .await?
    {
        return Err(GlobeliseError::bad_request(format!(
            "Unknown country code {}",
            body.country_code
        )));
    }

    if !database
        .sap_mulesoft_payroll_journal_validate_company_code(&body.company_code, &body.country_code)
        .await?
    {
        return Err(GlobeliseError::bad_request(format!(
            "Company code '{}' and country code '{}' does not match",
            body.company_code, body.country_code
        )));
    }

    let mut workbook = calamine::open_workbook_auto_from_rs(Cursor::new(body.file))?;
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
        }

        #[derive(Debug, Serialize)]
        struct C {
            #[serde(rename = "integration")]
            integration: A,
            #[serde(rename = "country_iso")]
            country_iso: String,
            #[serde(rename = "companyCode")]
            company_code: String,
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
                )>,
                _,
            >>()?
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
                    })
                },
            )
            .collect::<GlobeliseResult<Vec<_>>>()?;

        // VALIDATIONS
        // NOTE: Consider parsing, not validate!

        // NOTE: Summation of floating points might not be accurate.
        // We might want to introduce some delta to compensate.
        if raw_payroll_journals.iter().map(|v| v.amount).sum::<f64>() != 0.0f64 {
            return Err(GlobeliseError::bad_request(
                "The total sum of amounts should be equal to 0",
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
            .cloned()
            .collect::<Vec<_>>();

        if !database
            .sap_mulesoft_payroll_journal_validate_cost_center_code(
                &body.company_code,
                &cost_center_codes,
            )
            .await?
        {
            return Err(GlobeliseError::bad_request(format!(
                "Company code '{}' and one of the cost center codes in '{:?}' does not match",
                body.company_code, cost_center_codes
            )));
        }

        let ulid = database
            .insert_sap_mulesoft_payroll_journal(&body.country_code, &raw_payroll_journals)
            .await?;

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
                })
            })
            .collect::<GlobeliseResult<Vec<B>>>()?;

        let json_body = C {
            integration: A {
                id: ulid.to_string(),
                name: "payroll".to_string(),
            },
            country_iso: body.country_code,
            company_code: body.company_code,
            details: payroll_journals,
        };

        println!("json_body:\n{}", serde_json::to_string(&json_body).unwrap());

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
}

impl Database {
    pub async fn insert_sap_mulesoft_payroll_journal(
        &self,
        country_code: &String,
        rows: &[InsertPayrollJournalRowData],
    ) -> GlobeliseResult<Uuid> {
        let ulid = Uuid::new_v4();

        sqlx::query(
            "
        INSERT INTO sap_mulesoft_payroll_journals_entries (
            ulid, country_code
        ) VALUES (
            $1, $2
        )",
        )
        .bind(ulid)
        .bind(country_code)
        .execute(&self.0)
        .await?;

        let rows_query = "
        INSERT INTO sap_mulesoft_payroll_journals_rows (
            entry_ulid, posting_date, doc_type, company_code, currency_code, 
            reference, debit_credit_code, document_header_text, gl_account, amount, 
            cost_center
        ) 
        SELECT
            *
        FROM UNNEST (
            $1, $2, $3, $4, $5,
            $6, $7, $8, $9, $10, 
            $11
        ) RETURNING
            entry_ulid, posting_date, doc_type, company_code, currency_code, 
            reference, debit_credit_code, document_header_text, gl_account, amount, 
            cost_center";

        sqlx::query(rows_query)
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

    pub async fn sap_mulesoft_payroll_journal_validate_company_code(
        &self,
        company_code: &String,
        country_code: &String,
    ) -> GlobeliseResult<bool> {
        let result = sqlx::query(
            "
            SELECT
                code, country_code
            FROM
                sap_mulesoft_payroll_journal_company_codes
            WHERE
                code = $1 AND
                country_code = $2",
        )
        .bind(company_code)
        .bind(country_code)
        .fetch_optional(&self.0)
        .await?
        .is_some();

        Ok(result)
    }

    pub async fn sap_mulesoft_payroll_journal_validate_cost_center_code(
        &self,
        company_code: &String,
        cost_center_code: &[String],
    ) -> GlobeliseResult<bool> {
        let result = sqlx::query(
            "
            SELECT
                code, company_code
            FROM
                sap_mulesoft_payroll_journal_cost_centers
            WHERE
                company_code = $1 AND
                code = ANY($2::text[])",
        )
        .bind(company_code)
        .bind(cost_center_code)
        .fetch_optional(&self.0)
        .await?
        .is_some();

        Ok(result)
    }
}
