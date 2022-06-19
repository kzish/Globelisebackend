use serde::{Deserialize, Serialize};
use serde_with::{serde_as, TryFromInto};
use sqlx::FromRow;
use uuid::Uuid;

use crate::{
    custom_serde::{Currency, OffsetDateWrapper, UserType},
    database::Database,
    error::GlobeliseResult,
};

#[serde_as]
#[derive(Debug, FromRow, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct OnboardClientPaymentDetails {
    pub currency: Currency,
    #[serde_as(as = "TryFromInto<OffsetDateWrapper>")]
    pub payment_date: sqlx::types::time::OffsetDateTime,
    #[serde_as(as = "TryFromInto<OffsetDateWrapper>")]
    pub cutoff_date: sqlx::types::time::OffsetDateTime,
}

impl Database {
    pub async fn insert_one_onboard_client_payment_details(
        &self,
        ulid: Uuid,
        user_type: UserType,
        currency: Currency,
        payment_date: &sqlx::types::time::OffsetDateTime,
        cutoff_date: &sqlx::types::time::OffsetDateTime,
    ) -> GlobeliseResult<()> {
        let table = match user_type {
            UserType::Individual => "individual_client_payment_details",
            UserType::Entity => "entity_client_payment_details",
        };

        let query = format!(
            "
            INSERT INTO {table} (
                ulid, currency, payment_date, cutoff_date
            ) VALUES (
                $1, $2, $3, $4
            ) ON CONFLICT(ulid) DO UPDATE SET 
                currency = $2, payment_date = $3, cutoff_date = $4",
        );

        sqlx::query(&query)
            .bind(ulid)
            .bind(currency)
            .bind(payment_date)
            .bind(cutoff_date)
            .execute(&self.0)
            .await?;

        Ok(())
    }

    pub async fn select_one_onboard_client_payment_details(
        &self,
        ulid: Uuid,
        user_type: UserType,
    ) -> GlobeliseResult<Option<OnboardClientPaymentDetails>> {
        let table = match user_type {
            UserType::Individual => "individual_client_payment_details",
            UserType::Entity => "entity_client_payment_details",
        };

        let query = format!(
            "
            SELECT
                ulid, currency, payment_date, cutoff_date
            FROM 
                {table}
            WHERE
                ulid = $1",
        );

        let result = sqlx::query_as(&query)
            .bind(ulid)
            .fetch_optional(&self.0)
            .await?;

        Ok(result)
    }
}
