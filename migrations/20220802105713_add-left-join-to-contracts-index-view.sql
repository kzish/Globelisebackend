-- View: public.contracts_index

DROP VIEW public.contracts_index;

CREATE OR REPLACE VIEW public.contracts_index
 AS
 WITH contractors AS (
         SELECT entity_contractor_account_details.ulid AS contractor_ulid,
            entity_contractor_account_details.company_name AS contractor_name
           FROM entity_contractor_account_details
        UNION
         SELECT individual_contractor_account_details.ulid AS contractor_ulid,
            concat(individual_contractor_account_details.first_name, ' ', individual_contractor_account_details.last_name) AS contractor_name
           FROM individual_contractor_account_details
        ), clients AS (
         SELECT entity_client_account_details.ulid AS client_ulid,
            entity_client_account_details.company_name AS client_name
           FROM entity_client_account_details
        UNION
         SELECT individual_client_account_details.ulid AS client_ulid,
            concat(individual_client_account_details.first_name, ' ', individual_client_account_details.last_name) AS client_name
           FROM individual_client_account_details
        )
 SELECT contracts.ulid,
    contracts.client_ulid,
    contracts.contractor_ulid,
    contracts.contract_name,
    contracts.contract_type,
    contracts.contract_status,
    contracts.currency,
    contracts.job_title,
    contracts.seniority,
    contracts.begin_at,
    contracts.end_at,
    contracts.branch_ulid,
    contracts.created_at,
    contracts.client_signature,
    contracts.contractor_signature,
    contracts.client_date_signed,
    contracts.contractor_date_signed,
    contracts.team_ulid,
    contracts.job_scope,
    contracts.contract_amount,
    contracts.country_of_contractors_tax_residence,
    contracts.notice_period,
    contracts.offer_stock_option,
    contracts.special_clause,
    contracts.cut_off,
    contracts.pay_day,
    contracts.due_date,
    contractors.contractor_name,
    clients.client_name,
    contracts.tax_settings,
    contracts.statutory_fund_settings,
    contracts.payment_calculation_settings,
    contracts.client_rejected_reason,
    contracts.contractor_rejected_reason,
    contracts.cancelled_reason,
    contracts.activate_to_draft_reason
   FROM contracts
     JOIN clients ON contracts.client_ulid = clients.client_ulid
     LEFT JOIN contractors ON contracts.contractor_ulid = contractors.contractor_ulid;

ALTER TABLE public.contracts_index
    OWNER TO postgres;

