use common_utils::{calc_limit_and_offset, error::GlobeliseResult};
use uuid::Uuid;

use super::{
    ContractDetails, ContractorEmailDetails, ContractorViewContractsRequest, ContractsPreview,
    ContractsPreviewCreateRequest, GenerateContractDto, ListContractsPreviewRequest,
    RevokeSignContractRequest, SignContractRequest,
};
use crate::database::Database;
impl Database {
    pub async fn client_sign_contract(&self, request: SignContractRequest) -> GlobeliseResult<()> {
        let now = sqlx::types::time::OffsetDateTime::now_utc();
        sqlx::query(
            "
            UPDATE
                contracts
            SET
                client_signature = $1,
                client_date_signed = $2,
                contract_status = 'CONTRACTOR TO SIGN'
            WHERE 
                ulid = $3 
            AND 
                contractor_ulid = $4 
            AND
                client_ulid = $5",
        )
        .bind(request.signature)
        .bind(now)
        .bind(request.contract_ulid)
        .bind(request.contractor_ulid)
        .bind(request.client_ulid)
        .execute(&self.0)
        .await?;

        Ok(())
    }

    pub async fn contractor_sign_contract(
        &self,
        request: SignContractRequest,
    ) -> GlobeliseResult<()> {
        let now = sqlx::types::time::OffsetDateTime::now_utc();
        sqlx::query(
            "
            UPDATE
                contracts
            SET
                contractor_signature = $1,
                contractor_date_signed = $2,
                contract_status = 'ACTIVE'
            WHERE 
                ulid = $3 
            AND 
                contractor_ulid = $4
            AND
                client_ulid = $5",
        )
        .bind(request.signature)
        .bind(now)
        .bind(request.contract_ulid)
        .bind(request.contractor_ulid)
        .bind(request.client_ulid)
        .execute(&self.0)
        .await?;

        Ok(())
    }

    pub async fn client_revoke_sign_contract(
        &self,
        request: RevokeSignContractRequest,
    ) -> GlobeliseResult<()> {
        sqlx::query(
            "
            UPDATE
                contracts
            SET
                client_signature = null,
                client_date_signed = null,
                contract_status = 'CANCELLED'
            WHERE 
                ulid = $1 
            AND 
                contractor_ulid = $2
            AND
                client_ulid = $3",
        )
        .bind(request.contract_ulid)
        .bind(request.contractor_ulid)
        .bind(request.client_ulid)
        .execute(&self.0)
        .await?;

        Ok(())
    }

    pub async fn contractor_revoke_sign_contract(
        &self,
        request: RevokeSignContractRequest,
    ) -> GlobeliseResult<()> {
        sqlx::query(
            "
            UPDATE
                contracts
            SET
                contractor_signature = null,
                contractor_date_signed = null,
                contract_status = 'CANCELLED'
            WHERE 
                ulid = $1 
            AND 
                contractor_ulid = $2
            AND
                client_ulid = $3",
        )
        .bind(request.contract_ulid)
        .bind(request.contractor_ulid)
        .bind(request.client_ulid)
        .execute(&self.0)
        .await?;

        Ok(())
    }

    pub async fn get_contractor_email_details(
        &self,
        contractor_ulid: Uuid,
    ) -> GlobeliseResult<ContractorEmailDetails> {
        let response = sqlx::query_as(
            "
            SELECT * 
            FROM
                contractor_index
            WHERE
                ulid = $1
            ",
        )
        .bind(contractor_ulid)
        .fetch_one(&self.0)
        .await?;

        Ok(response)
    }

    pub async fn get_contract_details(
        &self,
        contract_ulid: Uuid,
    ) -> GlobeliseResult<ContractDetails> {
        let response = sqlx::query_as(
            "
            SELECT * 
            FROM
                contracts_index
            WHERE
                contract_ulid = $1
            ",
        )
        .bind(contract_ulid)
        .fetch_one(&self.0)
        .await?;

        Ok(response)
    }

    pub async fn contractor_view_contracts(
        &self,
        request: ContractorViewContractsRequest,
    ) -> GlobeliseResult<Vec<ContractDetails>> {
        let (limit, offset) = calc_limit_and_offset(request.per_page, request.page);

        let response = sqlx::query_as(
            "
            SELECT * 
            FROM
                contracts_index
            WHERE
                contractor_ulid = $1
                AND ($2 IS NULL OR contract_name LIKE $2)
                LIMIT $3
                OFFSET $4
            ",
        )
        .bind(request.contractor_ulid)
        .bind(format!(
            "%{}%",
            request.contract_name.unwrap_or_else(|| "".to_string())
        ))
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.0)
        .await?;

        Ok(response)
    }

    pub async fn client_create_contract_preview(
        &self,
        request: ContractsPreviewCreateRequest,
    ) -> GlobeliseResult<()> {
        let ulid = Uuid::new_v4();
        sqlx::query(
            "
            INSERT INTO 
                    contract_preview (
                        ulid,
                        contract_preview_text,
                        client_ulid,
                        branch_ulid,
                        team_ulid,
                        contract_name,
                        job_title,
                        seniority_level,
                        job_scope,
                        start_date,
                        end_date
                    )
                values ($1, $2, $3 $4, $5, $6, $7, $8, $9, $10, $11)",
        )
        .bind(ulid)
        .bind(request.contract_preview_text)
        .bind(request.client_ulid)
        .bind(request.branch_ulid)
        .bind(request.team_ulid)
        .bind(request.contract_name)
        .bind(request.job_title)
        .bind(request.seniority_level)
        .bind(request.job_scope)
        .bind(request.start_date)
        .bind(request.end_date)
        .execute(&self.0)
        .await?;

        Ok(())
    }

    pub async fn client_update_contract_preview(
        &self,
        request: ContractsPreview,
    ) -> GlobeliseResult<()> {
        sqlx::query(
            "
            UPDATE 
                 contract_preview 
            SET
                    contract_preview_text = $2,
                    client_ulid = $3,
                    branch_ulid = $4,
                    team_ulid = $5,
                    contract_name = $6,
                    job_title = $7,
                    seniority_level = $8,
                    job_scope = $9,
                    start_date = $10,
                    end_date = $11,
                    )
            WHERE
                ulid = $1",
        )
        .bind(request.ulid)
        .bind(request.contract_preview_text)
        .bind(request.client_ulid)
        .bind(request.branch_ulid)
        .bind(request.team_ulid)
        .bind(request.contract_name)
        .bind(request.job_title)
        .bind(request.seniority_level)
        .bind(request.job_scope)
        .bind(request.start_date)
        .bind(request.end_date)
        .execute(&self.0)
        .await?;

        Ok(())
    }

    pub async fn client_get_contract_preview(
        &self,
        ulid: Uuid,
    ) -> GlobeliseResult<ContractsPreview> {
        let response = sqlx::query_as(
            "
            SELCT * FROM 
                 contract_preview 
            WHERE
                ulid = $1",
        )
        .bind(ulid)
        .fetch_one(&self.0)
        .await?;

        Ok(response)
    }

    pub async fn client_list_contracts_preview(
        &self,
        request: ListContractsPreviewRequest,
    ) -> GlobeliseResult<Vec<ContractsPreview>> {
        let (limit, offset) = calc_limit_and_offset(request.per_page, request.page);
        let response = sqlx::query_as(
            "
            SELCT * FROM 
                 contract_preview 
            WHERE
                branch_ulid = $1
            AND ($2 IS NULL OR contract_name LIKE $2)
            LIMIT $3
            OFFSET $4",
        )
        .bind(request.branch_ulid)
        .bind(format!(
            "%{}%",
            request.contract_name.unwrap_or_else(|| "".to_string())
        ))
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.0)
        .await?;

        Ok(response)
    }

    pub async fn client_delete_contract_preview(&self, ulid: Uuid) -> GlobeliseResult<()> {
        sqlx::query(
            "
            DELETE FROM 
                 contract_preview 
            WHERE
                ulid = $1",
        )
        .bind(ulid)
        .execute(&self.0)
        .await?;

        Ok(())
    }

    pub async fn client_generate_contract_from_preview(
        &self,
        request: GenerateContractDto,
    ) -> GlobeliseResult<()> {
        sqlx::query(
            "
            INSERT INTO
                contracts (
                    ulid,
                    client_ulid,
                    contractor_ulid,
                    contract_name,
                    contract_status,
                    contract_amount,
                    currency,
                    job_title,
                    seniority,
                    begin_at,
                    end_at,
                    branch_ulid,
                    contract_type,
                    contract_preview_text,
                    team_ulid,
                    job_scope
                )
                VALUES($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16)
            ",
        )
        .bind(request.ulid)
        .bind(request.client_ulid)
        .bind(request.contractor_ulid)
        .bind(request.contract_name)
        .bind(request.contract_status)
        .bind(request.contract_amount)
        .bind(request.currency)
        .bind(request.job_title)
        .bind(request.seniority)
        .bind(request.begin_at)
        .bind(request.end_at)
        .bind(request.branch_ulid)
        .bind(request.contract_type)
        .bind(request.contract_preview_text)
        .bind(request.team_ulid)
        .bind(request.job_scope)
        .execute(&self.0)
        .await?;

        Ok(())
    }
}
