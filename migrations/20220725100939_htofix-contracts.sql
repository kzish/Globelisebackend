ALTER TABLE IF EXISTS public.contract_preview
    ADD COLUMN client_ulid uuid NOT NULL;

ALTER TABLE IF EXISTS public.contract_preview
    ADD COLUMN currency text COLLATE pg_catalog."default";

DROP VIEW public.contracts_index;

ALTER TABLE IF EXISTS public.contracts DROP COLUMN IF EXISTS contract_amount;

ALTER TABLE IF EXISTS public.contracts
    ADD COLUMN contract_amount double precision NOT NULL DEFAULT 0;

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
 SELECT contracts.ulid AS contract_ulid,
    contracts.client_ulid,
    contracts.contractor_ulid,
    contracts.contract_name,
    contracts.contract_type,
    contracts.contract_status,
    contracts.contract_amount,
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
    contracts.contract_preview_text,
    contracts.team_ulid,
    contracts.job_scope,
    contractors.contractor_name,
    clients.client_name
   FROM contracts
     JOIN clients ON contracts.client_ulid = clients.client_ulid
     JOIN contractors ON contracts.contractor_ulid = contractors.contractor_ulid;

ALTER TABLE public.contracts_index
    OWNER TO postgres;

