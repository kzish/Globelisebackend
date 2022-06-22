use std::io::Cursor;

use axum::extract::{ContentLengthLimit, Extension, Json, Query};
use calamine::Reader;
use common_utils::{
    custom_serde::{
        Currency, EmailWrapper, OffsetDateWrapper, OptionOffsetDateWrapper, FORM_DATA_LENGTH_LIMIT,
    },
    error::{GlobeliseError, GlobeliseResult},
    token::Token,
};
use csv::{ReaderBuilder, StringRecord};
use eor_admin_microservice_sdk::token::AdminAccessToken;
use lettre::{message::Mailbox, Message, SmtpTransport, Transport};
use serde::{Deserialize, Serialize};
use serde_with::{base64::Base64, serde_as, TryFromInto};
use sqlx::FromRow;
use uuid::Uuid;

use crate::{
    database::{Database, SharedDatabase},
    env::{
        GLOBELISE_SENDER_EMAIL, GLOBELISE_SMTP_URL, SMTP_CREDENTIAL,
        USER_MANAGEMENT_MICROSERVICE_DOMAIN_URL,
    },
};

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct PostPrefillIndividualContractorDetailsForBulkUpload {
    #[serde_as(as = "Base64")]
    pub uploaded_file: Vec<u8>,
    pub debug: Option<bool>,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct GetPrefillIndividualContractorDetailsForBulkUpload {
    pub email: EmailWrapper,
}

#[serde_as]
#[derive(Debug, FromRow, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct PrefillIndividualContractorDetailsForBulkUpload {
    #[serde(rename = "Client")]
    pub client_ulid: Uuid,
    #[serde(rename = "Sub Entity")]
    pub branch_ulid: Uuid,
    #[serde(rename = "Cost Centre")]
    #[serde(default)]
    pub department_ulid: Option<Uuid>,
    #[serde(rename = "First Name")]
    pub first_name: String,
    #[serde(rename = "Last Name")]
    pub last_name: String,
    #[serde(rename = "Gender")]
    pub gender: Option<String>,
    #[serde(rename = "Marital Status")]
    pub marital_status: Option<String>,
    #[serde(rename = "Nationality")]
    pub nationality: String,
    #[serde(rename = "Date of Birth")]
    #[serde_as(as = "TryFromInto<OffsetDateWrapper>")]
    pub dob: sqlx::types::time::OffsetDateTime,
    #[serde(rename = "Dial Code")]
    pub dial_code: String,
    #[serde(rename = "Phone Number")]
    pub phone_number: String,
    #[serde(rename = "Email Address")]
    pub email: EmailWrapper,
    #[serde(rename = "Address")]
    pub address: Option<String>,
    #[serde(rename = "Country")]
    pub country: Option<String>,
    #[serde(rename = "City")]
    pub city: Option<String>,
    #[serde(rename = "Postal Code")]
    pub postal_code: Option<String>,
    #[serde(rename = "National ID")]
    pub national_id: Option<String>,
    #[serde(rename = "Passport Number")]
    pub passport_number: Option<String>,
    #[serde(rename = "Passport Expiry Date")]
    #[serde_as(as = "TryFromInto<OptionOffsetDateWrapper>")]
    #[serde(default)]
    pub passport_expiry_date: Option<sqlx::types::time::OffsetDateTime>,
    #[serde(rename = "Work Permit")]
    pub work_permit: Option<String>,
    #[serde(rename = "Tax ID")]
    pub tax_id: Option<String>,
    #[serde(rename = "Contribution ID #1")]
    pub contribution_id_1: Option<String>,
    #[serde(rename = "Contribution ID #2")]
    pub contribution_id_2: Option<String>,
    #[serde(rename = "Total Dependants")]
    pub total_dependants: Option<i64>,
    #[serde(rename = "Timezone")]
    pub time_zone: Option<String>,
    #[serde(rename = "Employee ID")]
    pub employee_id: Option<String>,
    #[serde(rename = "Designation")]
    pub designation: Option<String>,
    #[serde(rename = "Start Date")]
    #[serde_as(as = "TryFromInto<OptionOffsetDateWrapper>")]
    #[serde(default)]
    pub start_date: Option<sqlx::types::time::OffsetDateTime>,
    #[serde(rename = "End Date")]
    #[serde_as(as = "TryFromInto<OptionOffsetDateWrapper>")]
    #[serde(default)]
    pub end_date: Option<sqlx::types::time::OffsetDateTime>,
    #[serde(rename = "Employment Status")]
    pub employment_status: Option<String>,
    #[serde(rename = "Bank Name")]
    pub bank_name: Option<String>,
    #[serde(rename = "Bank Account Owner Name")]
    pub bank_account_owner_name: Option<String>,
    #[serde(rename = "Bank Account Number")]
    pub bank_account_number: Option<String>,
    #[serde(rename = "Bank Code")]
    pub bank_code: Option<String>,
    #[serde(rename = "Branch Code")]
    pub bank_branch_code: Option<String>,
    #[serde(rename = "Currency")]
    pub currency: Currency,
    #[serde(rename = "Basic Salary")]
    pub basic_salary: Option<f64>,
    #[serde(rename = "Additional Item 1")]
    pub additional_item_1: String,
    #[serde(rename = "Additional Item 2")]
    pub additional_item_2: Option<String>,
    #[serde(rename = "Deduction 1")]
    pub deduction_1: Option<String>,
    #[serde(rename = "Deduction 2")]
    pub deduction_2: Option<String>,
    #[serde(rename = "Other Pay Item 1")]
    pub other_pay_item_1: Option<String>,
    #[serde(rename = "Other Pay Item 2")]
    pub other_pay_item_2: Option<String>,
}

pub async fn post_one(
    // Only for validation
    _: Token<AdminAccessToken>,
    ContentLengthLimit(Json(body)): ContentLengthLimit<
        Json<PostPrefillIndividualContractorDetailsForBulkUpload>,
        FORM_DATA_LENGTH_LIMIT,
    >,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    let mut rows_to_enter = vec![];
    if let Ok(mut records) = ReaderBuilder::new()
        .has_headers(false)
        .from_reader(body.uploaded_file.as_slice())
        .records()
        .collect::<Result<Vec<StringRecord>, _>>()
    {
        if !records.is_empty() {
            // Get/remove the first row because its the header.
            let header = records.swap_remove(0);
            for record in records {
                let value = record.deserialize::<PrefillIndividualContractorDetailsForBulkUpload>(
                    Some(&header),
                )?;
                rows_to_enter.push(value);
            }
        }
    } else if let Ok(excel_sheet_des) =
        calamine::open_workbook_auto_from_rs(Cursor::new(body.uploaded_file.as_slice()))?
            .worksheets()
            .get(0)
            .ok_or_else(|| {
                GlobeliseError::bad_request(
                    "Please provide an excel file with at least 1 worksheet",
                )
            })?
            .1
            .deserialize()
    {
        let rows = excel_sheet_des
            .into_iter()
            .collect::<Result<Vec<PrefillIndividualContractorDetailsForBulkUpload>, _>>()?;
        rows_to_enter.extend(rows);
    } else {
        return Err(GlobeliseError::bad_request(
            "Please provide either CSV or Excel files",
        ));
    }

    for value in rows_to_enter {
        let receiver_email = value
            .email
            // TODO: Get the name of the person associated to this email address
            .0
            .to_display("")
            .parse::<Mailbox>()?;

        let database = database.lock().await;

        database
            .post_prefilll_individual_contractor_details_for_bulk_upload(value)
            .await?;

        if let Some(true) = body.debug {
            return Ok(());
        }

        // Send email to the contractor

        let email = Message::builder()
            .from(GLOBELISE_SENDER_EMAIL.clone())
            // TODO: Remove this because this is supposed to be a no-reply email?
            .reply_to(GLOBELISE_SENDER_EMAIL.clone())
            .to(receiver_email)
            .subject("Invitation to Globelise")
            .header(lettre::message::header::ContentType::TEXT_HTML)
            // TODO: Once designer have a template for this. Use a templating library to populate data.
            .body(format!(
                r##"
                <!DOCTYPE html>
                <html>
                <head>
                    <title>Globelise Invitation</title>
                </head>
                <body>
                    <p>
                   Click the <a href="{}">link</a> to sign up as a Globelise individual contractor.
                    </p>
                    <p>If you did not expect to receive this email. Please ignore!</p>
                </body>
                </html>
                "##,
                (*USER_MANAGEMENT_MICROSERVICE_DOMAIN_URL),
            ))?;

        // Open a remote connection to gmail
        let mailer = SmtpTransport::relay(&GLOBELISE_SMTP_URL)?
            .credentials(SMTP_CREDENTIAL.clone())
            .build();

        // Send the email
        mailer.send(&email)?;
    }

    Ok(())
}

pub async fn get_one(
    // Only for validation
    _: Token<AdminAccessToken>,
    Query(query): Query<GetPrefillIndividualContractorDetailsForBulkUpload>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<PrefillIndividualContractorDetailsForBulkUpload>> {
    let database = database.lock().await;

    let result = database
        .get_prefilll_individual_contractor_details_for_bulk_upload(query.email)
        .await?
        .ok_or(GlobeliseError::NotFound)?;

    Ok(Json(result))
}

impl Database {
    pub async fn post_prefilll_individual_contractor_details_for_bulk_upload(
        &self,
        details: PrefillIndividualContractorDetailsForBulkUpload,
    ) -> GlobeliseResult<()> {
        let query = "
            INSERT INTO prefilled_individual_contractor_details_for_bulk_upload (
                client_ulid, branch_ulid, department_ulid, first_name, last_name, 
                gender, marital_status, nationality, dob, dial_code,
                phone_number, email, address, country, city, 
                postal_code, national_id, passport_number, passport_expiry_date, work_permit, 
                tax_id, contribution_id_1, contribution_id_2, total_dependants, time_zone,
                employee_id, designation, start_date, end_date, employment_status, 
                bank_name, bank_account_owner_name, bank_account_number, bank_code, bank_branch_code, 
                currency, basic_salary, additional_item_1, additional_item_2, deduction_1, 
                deduction_2, other_pay_item_1, other_pay_item_2
            ) VALUES (
                $1, $2, $3, $4, $5, 
                $6, $7, $8, $9, $10,
                $11, $12, $13, $14, $15,
                $16, $17, $18, $19, $20,
                $21, $22, $23, $24, $25,
                $26, $27, $28, $29, $30,
                $31, $32, $33, $34, $35,
                $36, $37, $38, $39, $40,
                $41, $42, $43
            ) ON CONFLICT(email, client_ulid, branch_ulid) DO UPDATE SET 
                client_ulid = $1, branch_ulid = $2, department_ulid = $3, first_name = $4, last_name = $5, 
                gender = $6, marital_status = $7, nationality = $8, dob = $9, dial_code = $10,
                phone_number = $11, email = $12, address = $13, country = $14, city = $15, 
                postal_code = $16, national_id = $17, passport_number = $18, passport_expiry_date = $19, work_permit = $20, 
                tax_id = $21, contribution_id_1 = $22, contribution_id_2 = $23, total_dependants = $24, time_zone = $25,
                employee_id = $26, designation = $27, start_date = $28, end_date = $29, employment_status = $30, 
                bank_name = $31, bank_account_owner_name = $32, bank_account_number = $33, bank_code = $34, bank_branch_code = $35, 
                currency = $36, basic_salary = $37, additional_item_1 = $38, additional_item_2 = $39, deduction_1 = $40, 
                deduction_2 = $41, other_pay_item_1 = $42, other_pay_item_2 = $43";
        sqlx::query(query)
            .bind(details.client_ulid)
            .bind(details.branch_ulid)
            .bind(details.department_ulid)
            .bind(details.first_name)
            .bind(details.last_name)
            .bind(details.gender)
            .bind(details.marital_status)
            .bind(details.nationality)
            .bind(details.dob)
            .bind(details.dial_code)
            .bind(details.phone_number)
            .bind(details.email)
            .bind(details.address)
            .bind(details.country)
            .bind(details.city)
            .bind(details.postal_code)
            .bind(details.national_id)
            .bind(details.passport_number)
            .bind(details.passport_expiry_date)
            .bind(details.work_permit)
            .bind(details.tax_id)
            .bind(details.contribution_id_1)
            .bind(details.contribution_id_2)
            .bind(details.total_dependants)
            .bind(details.time_zone)
            .bind(details.employee_id)
            .bind(details.designation)
            .bind(details.start_date)
            .bind(details.end_date)
            .bind(details.employment_status)
            .bind(details.bank_name)
            .bind(details.bank_account_owner_name)
            .bind(details.bank_account_number)
            .bind(details.bank_code)
            .bind(details.bank_branch_code)
            .bind(details.currency)
            .bind(details.basic_salary)
            .bind(details.additional_item_1)
            .bind(details.additional_item_2)
            .bind(details.deduction_1)
            .bind(details.deduction_2)
            .bind(details.other_pay_item_1)
            .bind(details.other_pay_item_2)
            .execute(&self.0)
            .await?;
        Ok(())
    }

    pub async fn get_prefilll_individual_contractor_details_for_bulk_upload(
        &self,
        email: EmailWrapper,
    ) -> GlobeliseResult<Option<PrefillIndividualContractorDetailsForBulkUpload>> {
        let query = "
            SELECT
                *
            FROM
                prefilled_individual_contractor_details_for_bulk_upload
            WHERE
                email = $1";
        let result = sqlx::query_as(query)
            .bind(email)
            .fetch_optional(&self.0)
            .await?;
        Ok(result)
    }
}
