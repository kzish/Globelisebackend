use common_utils::{calc_limit_and_offset, custom_serde::EmailWrapper, error::GlobeliseResult};
use uuid::Uuid;

use super::{
    ActivateContractRequest, ContractsAdditionalDocumentsResponse, ContractsIndexResponse,
    ContractsPayItemsResponse, ContractsRequest, ContractsResponse, GetContractsRequest,
    PermanantlyCancelContractRequest, RevokeSignContractRequest, SignContractRequest, UserResponse,
};
use crate::database::Database;

impl Database {
    pub async fn select_one_contract(
        &self,
        contract_ulid: Uuid,
    ) -> GlobeliseResult<ContractsIndexResponse> {
        let response = sqlx::query_as(
            "
            SELECT * FROM
                contracts_index
            WHERE 
                ulid = $1
            ",
        )
        .bind(contract_ulid)
        .fetch_one(&self.0)
        .await?;

        Ok(response)
    }

    pub async fn client_delete_contract(&self, contract_ulid: Uuid) -> GlobeliseResult<()> {
        sqlx::query(
            "
            DELETE FROM
                contracts
            WHERE 
                ulid = $1
            ",
        )
        .bind(contract_ulid)
        .execute(&self.0)
        .await?;

        Ok(())
    }

    pub async fn client_list_contracts(
        &self,
        request: GetContractsRequest,
    ) -> GlobeliseResult<Vec<ContractsIndexResponse>> {
        let (limit, offset) = calc_limit_and_offset(request.per_page, request.page);
        let response = sqlx::query_as(
            "
            SELECT * FROM
                contracts_index
            WHERE 
                ($1 IS NULL OR branch_ulid = $1)
            AND 
                ($2 IS NULL OR contractor_ulid = $2)
            AND
                client_ulid = $3
            AND 
                ($4 IS NULL OR (job_title ~* $4 OR client_name ~* $4 OR contractor_name ~* $4 OR contract_name ~* $4))
            AND 
                ($7 IS NULL OR ulid = $7)
            LIMIT $5
            OFFSET $6",
        )
        .bind(request.branch_ulid)
        .bind(request.contractor_ulid)
        .bind(request.client_ulid)
        .bind(request.query)
        .bind(limit)
        .bind(offset)
        .bind(request.contract_ulid)
        .fetch_all(&self.0)
        .await?;

        Ok(response)
    }

    pub async fn get_single_contract_index(
        &self,
        contract_ulid: Uuid,
    ) -> GlobeliseResult<ContractsIndexResponse> {
        let response = sqlx::query_as(
            "
            SELECT * FROM
                contracts_index
            WHERE 
                ulid = $1
            ",
        )
        .bind(contract_ulid)
        .fetch_one(&self.0)
        .await?;

        Ok(response)
    }

    pub async fn get_single_contract_index_additional_documents(
        &self,
        contract_ulid: Uuid,
    ) -> GlobeliseResult<Vec<ContractsAdditionalDocumentsResponse>> {
        let response = sqlx::query_as(
            "
            SELECT * FROM
                contracts_additional_documents
            WHERE 
                contract_ulid = $1
            ",
        )
        .bind(contract_ulid)
        .fetch_all(&self.0)
        .await?;

        Ok(response)
    }

    pub async fn get_single_contract_index_pay_items(
        &self,
        contract_ulid: Uuid,
    ) -> GlobeliseResult<Vec<ContractsPayItemsResponse>> {
        let response = sqlx::query_as(
            "
            SELECT * FROM
                contracts_index_pay_items
            WHERE 
                contract_ulid = $1
            ",
        )
        .bind(contract_ulid)
        .fetch_all(&self.0)
        .await?;

        Ok(response)
    }

    pub async fn contractor_list_contracts(
        &self,
        request: GetContractsRequest,
    ) -> GlobeliseResult<Vec<ContractsIndexResponse>> {
        let (limit, offset) = calc_limit_and_offset(request.per_page, request.page);
        let response = sqlx::query_as(
            "
            SELECT * FROM
                contracts_index
            WHERE 
                ($1 IS NULL OR branch_ulid = $1)
            AND 
                contractor_ulid = $2
            AND
                ($3 IS NULL OR client_ulid = $3)
            AND 
                ($4 IS NULL OR (job_title ~* $4 OR client_name ~* $4 OR contractor_name ~* $4 OR contract_name ~* $4))
            AND 
                ($7 IS NULL OR ulid = $7)
            LIMIT $5
            OFFSET $6",
        )
        .bind(request.branch_ulid)
        .bind(request.contractor_ulid)
        .bind(request.client_ulid)
        .bind(request.query)
        .bind(limit)
        .bind(offset)
        .bind(request.contract_ulid)
        .fetch_all(&self.0)
        .await?;

        Ok(response)
    }

    pub async fn client_post_update_contract(
        &self,
        request: ContractsRequest,
    ) -> GlobeliseResult<String> {
        sqlx::query(
            "
            INSERT INTO
                contracts(
                    ulid,
                    client_ulid,
                    contractor_ulid,
                    contract_name,
                    contract_type,
                    contract_status,
                    currency,
                    job_title,
                    seniority,
                    begin_at,
                    end_at,
                    branch_ulid,
                    client_signature,
                    contractor_signature,
                    client_date_signed,
                    contractor_date_signed,
                    team_ulid,
                    job_scope,
                    contract_amount,
                    country_of_contractors_tax_residence,
                    notice_period,
                    offer_stock_option,
                    special_clause,
                    cut_off,
                    pay_day,
                    due_date,
                    tax_settings,
                    statutory_fund_settings,
                    payment_calculation_settings
                ) VALUES($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22, $23, $24, $25, $26, $27, $28, $29)
                ON CONFLICT (ulid) DO UPDATE
                SET
                    client_ulid = $2,
                    contractor_ulid = $3,
                    contract_name = $4,
                    contract_type = $5,
                    contract_status = $6,
                    currency = $7,
                    job_title = $8,
                    seniority = $9,
                    begin_at = $10,
                    end_at = $11,
                    branch_ulid = $12,
                    client_signature = $13,
                    contractor_signature = $14,
                    client_date_signed = $15,
                    contractor_date_signed = $16,
                    team_ulid = $17,
                    job_scope = $18,
                    contract_amount = $19,
                    country_of_contractors_tax_residence = $20,
                    notice_period = $21,
                    offer_stock_option = $22,
                    special_clause = $23,
                    cut_off = $24,
                    pay_day = $25,
                    due_date = $26,
                    tax_settings = $27,
                    statutory_fund_settings = $28,
                    payment_calculation_settings = $29
                ",
        )
        .bind(request.ulid)
        .bind(request.client_ulid)
        .bind(request.contractor_ulid)
        .bind(request.contract_name)
        .bind(request.contract_type)
        .bind(request.contract_status)
        .bind(request.currency)
        .bind(request.job_title)
        .bind(request.seniority)
        .bind(request.begin_at)
        .bind(request.end_at)
        .bind(request.branch_ulid)
        .bind(request.client_signature)
        .bind(request.contractor_signature)
        .bind(request.client_date_signed)
        .bind(request.contractor_date_signed)
        .bind(request.team_ulid)
        .bind(request.job_scope)
        .bind(request.contract_amount)
        .bind(request.country_of_contractors_tax_residence)
        .bind(request.notice_period)
        .bind(request.offer_stock_option)
        .bind(request.special_clause)
        .bind(request.cut_off)
        .bind(request.pay_day)
        .bind(request.due_date)
        .bind(request.tax_settings)
        .bind(request.statutory_fund_settings)
        .bind(request.payment_calculation_settings)
        .execute(&self.0)
        .await?;

        //update table client_contractor_pairs
        if request.contractor_ulid.is_some() && request.client_ulid.is_some() {
            Self::update_client_contractor_pairs(
                self,
                request.client_ulid.unwrap(),
                request.contractor_ulid.unwrap(),
            )
            .await?;
        }

        //remove existing additional documents
        sqlx::query(
            "
            DELETE FROM 
                contracts_additional_documents
            WHERE 
                contract_ulid = $1",
        )
        .bind(request.ulid)
        .execute(&self.0)
        .await?;

        //remove existing claim items
        sqlx::query(
            "
            DELETE FROM 
                contracts_claim_items
            WHERE 
                contract_ulid = $1",
        )
        .bind(request.ulid)
        .execute(&self.0)
        .await?;

        //remove existing pay items
        sqlx::query(
            "
            DELETE FROM 
                contracts_pay_items
            WHERE 
                contract_ulid = $1",
        )
        .bind(request.ulid)
        .execute(&self.0)
        .await?;

        //add additional documents
        for item in request.additional_documents {
            sqlx::query(
                "
            INSERT INTO 
                contracts_additional_documents (
                    ulid,
                    contract_ulid,
                    file_name,
                    file_data
                ) values( $1, $2, $3, $4)
                ",
            )
            .bind(Uuid::new_v4())
            .bind(request.ulid)
            .bind(item.file_name)
            .bind(item.file_data)
            .execute(&self.0)
            .await?;
        }

        //add claim items
        for item in request.claim_items {
            sqlx::query(
                "
            INSERT INTO 
                contracts_claim_items (
                    contract_ulid,
                    claim_item_ulid
                ) values( $1, $2)
                ",
            )
            .bind(request.ulid)
            .bind(item.claim_item_ulid)
            .execute(&self.0)
            .await?;
        }

        //add pay items
        for item in request.pay_items {
            sqlx::query(
                "
            INSERT INTO 
                contracts_pay_items (
                    contract_ulid,
                    pay_item_ulid
                ) values( $1, $2)
                ",
            )
            .bind(request.ulid)
            .bind(item.pay_item_ulid)
            .execute(&self.0)
            .await?;
        }

        Ok(request.ulid.unwrap().to_string())
    }

    pub async fn update_client_contractor_pairs(
        &self,
        client_ulid: Uuid,
        contractor_ulid: Uuid,
    ) -> GlobeliseResult<()> {
        sqlx::query(
            "
                INSERT INTO 
                        client_contractor_pairs(client_ulid, contractor_ulid)
                VALUES ($1, $2)
                    ON CONFLICT (client_ulid, contractor_ulid) DO NOTHING
                ",
        )
        .bind(client_ulid)
        .bind(contractor_ulid)
        .execute(&self.0)
        .await?;

        Ok(())
    }

    pub async fn update_contractor_branch_pairs(
        &self,
        branch_ulid: Uuid,
        contractor_ulid: Uuid,
    ) -> GlobeliseResult<()> {
        sqlx::query(
            "
                INSERT INTO 
                        entity_contractor_branch_pairs(branch_ulid, contractor_ulid)
                VALUES ($1, $2)
                    ON CONFLICT (branch_ulid, contractor_ulid) DO NOTHING
                ",
        )
        .bind(branch_ulid)
        .bind(contractor_ulid)
        .execute(&self.0)
        .await?;

        Ok(())
    }

    pub async fn update_contract_add_contractor_ulid_to_contract(
        &self,
        client_ulid: Uuid,
        contractor_ulid: Uuid,
        contract_ulid: Uuid,
    ) -> GlobeliseResult<()> {
        sqlx::query(
            "
                UPDATE contracts SET
                    contractor_ulid = $3 
                WHERE
                    client_ulid = $1
                AND 
                    ulid = $2
                ",
        )
        .bind(client_ulid)
        .bind(contract_ulid)
        .bind(contractor_ulid)
        .execute(&self.0)
        .await?;

        Ok(())
    }

    pub async fn client_sign_contract(&self, request: SignContractRequest) -> GlobeliseResult<()> {
        let now = sqlx::types::time::OffsetDateTime::now_utc();
        sqlx::query(
            "
            UPDATE
                contracts
            SET
                client_signature = $1,
                client_date_signed = $2,
                contract_status = 'CONTRACTOR_SIGNATURE'
            WHERE 
                ulid = $3 
            AND
                client_ulid = $4",
        )
        .bind(request.signature)
        .bind(now)
        .bind(request.contract_ulid)
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
                contract_status = 'REJECTED',
                client_rejected_reason = $4
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
        .bind(request.reason)
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
                contract_status = 'REJECTED',
                contractor_rejected_reason = $4
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
        .bind(request.reason)
        .execute(&self.0)
        .await?;

        Ok(())
    }

    pub async fn get_user_by_email(
        &self,
        email: EmailWrapper,
    ) -> GlobeliseResult<Option<UserResponse>> {
        let response = sqlx::query_as(
            "
            SELECT * FROM
                users
            WHERE 
                email = $1
            ",
        )
        .bind(email)
        .fetch_optional(&self.0)
        .await?;

        Ok(response)
    }

    pub async fn get_contract_by_ulid(
        &self,
        contract_ulid: Uuid,
    ) -> GlobeliseResult<ContractsResponse> {
        let response = sqlx::query_as(
            "
                SELECT * FROM
                    contracts
                WHERE 
                    ulid = $1
                ",
        )
        .bind(contract_ulid)
        .fetch_one(&self.0)
        .await?;

        Ok(response)
    }

    //only client and admin
    pub async fn activate_contract_to_draft(
        &self,
        request: ActivateContractRequest,
    ) -> GlobeliseResult<()> {
        sqlx::query(
            "
            UPDATE
                contracts
            SET
                contract_status = 'DRAFT',
                activate_to_draft_reason = $4
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
        .bind(request.reason)
        .execute(&self.0)
        .await?;

        Ok(())
    }

    //only client and admin
    pub async fn permanantly_cancel_contract(
        &self,
        request: PermanantlyCancelContractRequest,
    ) -> GlobeliseResult<()> {
        sqlx::query(
            "
            UPDATE
                contracts
            SET
                contract_status = 'CANCELLED',
                cancelled_reason = $4
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
        .bind(request.reason)
        .execute(&self.0)
        .await?;

        Ok(())
    }
}
