-- Add migration script here


    drop table contract_preview

    ALTER TABLE IF EXISTS public.contracts
    ADD COLUMN country_of_contractors_tax_residence text NOT NULL DEFAULT 'SG';

    ALTER TABLE IF EXISTS public.contracts
    ADD COLUMN notice_period integer NOT NULL DEFAULT '0';

    ALTER TABLE IF EXISTS public.contracts
    ADD COLUMN offer_stock_option bool NOT NULL DEFAULT false;

    ALTER TABLE IF EXISTS public.contracts
    ADD COLUMN special_clause text;

    ALTER TABLE IF EXISTS public.contracts
    ADD COLUMN cut_off integer NOT NULL DEFAULT '0';

    ALTER TABLE IF EXISTS public.contracts
    ADD COLUMN pay_day integer NOT NULL DEFAULT '0';

    ALTER TABLE IF EXISTS public.contracts
    ADD COLUMN due_date timestamp with time zone;


    
CREATE TABLE IF NOT EXISTS public.contracts_claim_items
(
    contract_ulid uuid NOT NULL,
    claim_item_ulid uuid NOT NULL,
    CONSTRAINT contracts_claim_items_pkey PRIMARY KEY (contract_ulid, claim_item_ulid)
)

TABLESPACE pg_default;

ALTER TABLE IF EXISTS public.contracts_claim_items
    OWNER to postgres;


CREATE TABLE IF NOT EXISTS public.contracts_pay_items
(
    contract_ulid uuid NOT NULL,
    pay_item_ulid uuid NOT NULL,
    CONSTRAINT contracts_pay_items_pkey PRIMARY KEY (contract_ulid, pay_item_ulid)
)

TABLESPACE pg_default;

ALTER TABLE IF EXISTS public.contracts_pay_items
    OWNER to postgres;

CREATE TABLE IF NOT EXISTS public.contracts_additional_documents
(
    ulid uuid NOT NULL,
    contract_ulid uuid NOT NULL,
    file_name text COLLATE pg_catalog."default" NOT NULL,
    file_data bytea NOT NULL,
    CONSTRAINT contracts_additional_documents_pkey PRIMARY KEY (ulid),
    CONSTRAINT contracts_additiona_items_fkey FOREIGN KEY (contract_ulid)
        REFERENCES public.contracts (ulid) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE NO ACTION
)

TABLESPACE pg_default;

ALTER TABLE IF EXISTS public.contracts_additional_documents
    OWNER to postgres;


DROP VIEW public.contracts_index;
ALTER TABLE IF EXISTS public.contracts
DROP COLUMN contract_preview_text;
    
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
    clients.client_name
   FROM contracts
     JOIN clients ON contracts.client_ulid = clients.client_ulid
     JOIN contractors ON contracts.contractor_ulid = contractors.contractor_ulid;

ALTER TABLE public.contracts_index
    OWNER TO postgres;

CREATE OR REPLACE VIEW public.contracts_index_pay_items
 AS
 SELECT entity_client_branch_pay_items.ulid,
    entity_client_branch_pay_items.branch_ulid,
    entity_client_branch_pay_items.pay_item_type,
    entity_client_branch_pay_items.pay_item_custom_name,
    entity_client_branch_pay_items.use_pay_item_type_name,
    entity_client_branch_pay_items.pay_item_method,
    entity_client_branch_pay_items.employers_contribution,
    entity_client_branch_pay_items.require_employee_id,
    entity_client_branch_pay_items.created_at,
    contracts_pay_items.contract_ulid
   FROM entity_client_branch_pay_items
     JOIN contracts_pay_items ON entity_client_branch_pay_items.ulid = contracts_pay_items.pay_item_ulid;

ALTER TABLE public.contracts_index_pay_items
    OWNER TO postgres;