use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use crate::{custom_serde::UserType, database::Database, error::GlobeliseResult};

#[derive(Debug, FromRow, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ContractorBankDetails {
    pub bank_name: String,
    pub bank_account_name: String,
    pub bank_account_number: String,
    pub bank_code: String,
    pub branch_code: String,
}

impl Database {
    pub async fn insert_one_contractor_bank_details(
        &self,
        ulid: Uuid,
        user_type: UserType,
        details: ContractorBankDetails,
    ) -> GlobeliseResult<()> {
        let table = match user_type {
            UserType::Individual => "individual_contractor_bank_details",
            UserType::Entity => "entity_contractor_bank_details",
        };

        sqlx::query(&format!(
            "
        INSERT INTO {table} (
            ulid, bank_name, bank_account_name, bank_account_number, bank_code,
            branch_code
        ) VALUES (
            $1, $2, $3, $4, $5,
            $6
        ) ON CONFLICT(ulid) DO UPDATE SET 
            bank_name = $2, bank_account_name = $3, bank_account_number = $4, bank_code = $5,
            branch_code = $6",
        ))
        .bind(ulid)
        .bind(details.bank_name)
        .bind(details.bank_account_name)
        .bind(details.bank_account_number)
        .bind(details.bank_code)
        .bind(details.branch_code)
        .execute(&self.0)
        .await?;

        Ok(())
    }

    pub async fn select_one_contractor_bank_detail(
        &self,
        ulid: Uuid,
        user_type: UserType,
    ) -> GlobeliseResult<Option<ContractorBankDetails>> {
        let table = match user_type {
            UserType::Individual => "individual_contractor_bank_details",
            UserType::Entity => "entity_contractor_bank_details",
        };

        let result = sqlx::query_as(&format!(
            "
        SELECT
            ulid, bank_name, bank_account_name, bank_account_number, bank_code,
            branch_code
        FROM
            {table}
        WHERE
            ulid = $1",
        ))
        .bind(ulid)
        .fetch_optional(&self.0)
        .await?;

        Ok(result)
    }
}
