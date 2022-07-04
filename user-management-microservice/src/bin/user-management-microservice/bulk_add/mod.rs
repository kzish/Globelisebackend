use std::{io::Cursor, str::FromStr};

use axum::extract::{ContentLengthLimit, Extension, Json, Query};
use calamine::Reader;
use common_utils::{
    custom_serde::{EmailWrapper, FORM_DATA_LENGTH_LIMIT},
    database::{bulk_add::PrefillIndividualContractorDetailsForBulkUpload, CommonDatabase},
    error::{GlobeliseError, GlobeliseResult},
    token::Token,
};
use csv::{ReaderBuilder, StringRecord};
use eor_admin_microservice_sdk::token::AdminAccessToken;
use lettre::{message::Mailbox, Message, SmtpTransport, Transport};
use serde::{Deserialize, Serialize};
use serde_with::{base64::Base64, serde_as};
use uuid::Uuid;

use crate::env::{
    GLOBELISE_SENDER_EMAIL, GLOBELISE_SMTP_URL, SMTP_CREDENTIAL,
    USER_MANAGEMENT_MICROSERVICE_DOMAIN_URL,
};

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct PostOneAddBulkEmployee {
    pub file_name: String,
    #[serde_as(as = "Base64")]
    pub file_data: Vec<u8>,
    pub client_ulid: Uuid,
    pub debug: Option<bool>,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct GetPrefillIndividualContractorDetailsForBulkUpload {
    pub email: EmailWrapper,
}

pub async fn post_one(
    // Only for validation
    _: Token<AdminAccessToken>,
    ContentLengthLimit(Json(body)): ContentLengthLimit<
        Json<PostOneAddBulkEmployee>,
        FORM_DATA_LENGTH_LIMIT,
    >,
    Extension(database): Extension<CommonDatabase>,
) -> GlobeliseResult<()> {
    let mut rows_to_enter = vec![];

    let file_name = std::path::PathBuf::from_str(&body.file_name)?;

    enum FileType {
        Csv,
        Excel,
    }

    let file_extension = match file_name.extension().and_then(|s| s.to_str()) {
        Some("xls") | Some("xlsx") | Some("xlsb") | Some("ods") => Ok(FileType::Excel),
        Some("csv") => Ok(FileType::Csv),
        _ => Err(GlobeliseError::bad_request(
            "Cannot determine the file type from the file name",
        )),
    }?;

    match file_extension {
        FileType::Csv => {
            let mut records = ReaderBuilder::new()
                .has_headers(false)
                .from_reader(body.file_data.as_slice())
                .records()
                .collect::<Result<Vec<StringRecord>, _>>()?;

            if !records.is_empty() {
                // Get/remove the first row because its the header.
                let header = records.swap_remove(0);
                for record in records {
                    let value = record
                        .deserialize::<PrefillIndividualContractorDetailsForBulkUpload>(Some(
                            &header,
                        ))
                        .map_err(|_| {
                            GlobeliseError::bad_request(
                                "Please provide a CSV file that follows the template",
                            )
                        })?;
                    rows_to_enter.push(value);
                }
            }
        }
        FileType::Excel => {
            let excel_workbook =
                calamine::open_workbook_auto_from_rs(Cursor::new(body.file_data.as_slice()))?
                    .worksheets();

            let first_worksheet = excel_workbook.first().ok_or_else(|| {
                GlobeliseError::bad_request(
                    "Please provide an excel file with at least 1 worksheet",
                )
            })?;

            let row_deserializer = first_worksheet.1.deserialize()?;

            let rows = row_deserializer
                .into_iter()
                .collect::<Result<Vec<PrefillIndividualContractorDetailsForBulkUpload>, _>>()?;

            rows_to_enter.extend(rows);
        }
    };

    for value in rows_to_enter {
        let receiver_email = value
            .email
            // TODO: Get the name of the person associated to this email address
            .0
            .to_display("")
            .parse::<Mailbox>()?;

        let database = database.lock().await;

        database
            .post_prefilll_individual_contractor_details_for_bulk_upload(body.client_ulid, &value)
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
    Extension(database): Extension<CommonDatabase>,
) -> GlobeliseResult<Json<PrefillIndividualContractorDetailsForBulkUpload>> {
    let database = database.lock().await;

    let result = database
        .get_prefilll_individual_contractor_details_for_bulk_upload(query.email)
        .await?
        .ok_or_else(|| {
            GlobeliseError::not_found(
                "Cannot find prefilled individual contractor details from the query",
            )
        })?;

    Ok(Json(result))
}
