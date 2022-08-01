ALTER TABLE cost_center_contractors
RENAME TO cost_center_contractor_pairs;

DROP VIEW public.contractors_index_for_clients;
DROP VIEW public.clients_index_for_contractors;
DROP TABLE IF EXISTS public.client_contractor_pairs;

CREATE TABLE IF NOT EXISTS public.client_contractor_pairs
(
    client_ulid uuid NOT NULL,
    contractor_ulid uuid NOT NULL,
    CONSTRAINT client_contractor_pairs_pkey PRIMARY KEY (client_ulid, contractor_ulid),
    CONSTRAINT client_ulid_fkey FOREIGN KEY (client_ulid)
        REFERENCES public.users (ulid) MATCH SIMPLE
        ON UPDATE CASCADE
        ON DELETE CASCADE,
    CONSTRAINT contractor_ulid_fkey FOREIGN KEY (contractor_ulid)
        REFERENCES public.users (ulid) MATCH SIMPLE
        ON UPDATE CASCADE
        ON DELETE CASCADE
)

TABLESPACE pg_default;

ALTER TABLE IF EXISTS public.client_contractor_pairs
    OWNER to postgres;

ALTER TABLE IF EXISTS public.client_contractor_pairs
    OWNER to postgres;


CREATE OR REPLACE VIEW public.contractors_index_for_clients
 AS
 WITH contractor_cost_centers AS (
         SELECT cost_center_contractor_pairs.cost_center_ulid,
            cost_center_contractor_pairs.contractor_ulid,
            cost_center.cost_center_name,
            cost_center.branch_ulid
           FROM cost_center_contractor_pairs
             JOIN cost_center ON cost_center_contractor_pairs.cost_center_ulid = cost_center.ulid
        )
 SELECT onboarded_user_index.name,
    onboarded_user_index.email,
    onboarded_user_index.user_role,
    onboarded_user_index.user_type,
    onboarded_user_index.contract_count,
    onboarded_user_index.created_at,
    client_contractor_pairs.client_ulid,
    client_contractor_pairs.contractor_ulid AS ulid,
    entity_contractor_branch_pairs.branch_ulid,
    entity_client_branch_account_details.branch_name,
    contractor_cost_centers.cost_center_ulid,
    contractor_cost_centers.cost_center_name
   FROM client_contractor_pairs
     LEFT JOIN onboarded_user_index ON client_contractor_pairs.contractor_ulid = onboarded_user_index.ulid
     LEFT JOIN entity_contractor_branch_pairs ON client_contractor_pairs.contractor_ulid = entity_contractor_branch_pairs.contractor_ulid
     LEFT JOIN entity_client_branch_account_details ON entity_contractor_branch_pairs.branch_ulid = entity_client_branch_account_details.ulid
     LEFT JOIN contractor_cost_centers ON entity_contractor_branch_pairs.branch_ulid = contractor_cost_centers.branch_ulid;

ALTER TABLE public.contractors_index_for_clients
    OWNER TO postgres;


CREATE OR REPLACE VIEW public.clients_index_for_contractors
 AS
 SELECT b.name,
    b.email,
    b.user_role,
    b.user_type,
    b.contract_count,
    b.created_at,
    a.client_ulid AS ulid,
    a.contractor_ulid
   FROM client_contractor_pairs a
     LEFT JOIN onboarded_user_index b ON a.client_ulid = b.ulid;

ALTER TABLE public.clients_index_for_contractors
    OWNER TO postgres;

DROP VIEW public.cost_center_contractors_details;

CREATE OR REPLACE VIEW public.cost_center_contractors_details
 AS
 WITH contractors AS (
         SELECT entity_contractor_account_details.ulid AS contractor_ulid,
            entity_contractor_account_details.company_name AS contractor_name,
            entity_contractor_account_details.email_address
           FROM entity_contractor_account_details
        UNION
         SELECT individual_contractor_account_details.ulid AS contractor_ulid,
            concat(individual_contractor_account_details.first_name, ' ', individual_contractor_account_details.last_name) AS contractor_name,
            individual_contractor_account_details.email_address
           FROM individual_contractor_account_details
        ), client_branches AS (
         SELECT entity_client_branch_account_details.ulid AS branch_ulid,
            entity_client_branch_account_details.branch_name,
            entity_client_branch_account_details.country,
            entity_client_branch_account_details.time_zone
           FROM entity_client_branch_account_details
        )
 SELECT contractors.contractor_ulid,
    contractors.contractor_name,
    client_branches.branch_ulid,
    client_branches.branch_name,
    cost_center.cost_center_name,
    cost_center.ulid AS cost_center_ulid,
    cost_center.currency,
    client_branches.country,
    contractors.email_address,
    ( SELECT count(*) AS count
           FROM cost_center_contractor_pairs
          WHERE cost_center_contractor_pairs.contractor_ulid = contractors.contractor_ulid) AS cost_center_count,
    client_branches.time_zone,
    contracts.job_title AS job_description
   FROM contractors
     JOIN entity_contractor_branch_pairs ON entity_contractor_branch_pairs.contractor_ulid = contractors.contractor_ulid
     JOIN client_branches ON client_branches.branch_ulid = entity_contractor_branch_pairs.branch_ulid
     JOIN cost_center ON cost_center.branch_ulid = client_branches.branch_ulid
     JOIN cost_center_contractor_pairs ON cost_center_contractor_pairs.cost_center_ulid = cost_center.ulid
     JOIN contracts ON contracts.contractor_ulid = contractors.contractor_ulid;

ALTER TABLE public.cost_center_contractors_details
    OWNER TO postgres;


ALTER TABLE IF EXISTS public.contracts
ADD COLUMN client_rejected_reason text;

ALTER TABLE IF EXISTS public.contracts
ADD COLUMN contractor_rejected_reason text;

ALTER TABLE IF EXISTS public.contracts
ADD COLUMN cancelled_reason text;

ALTER TABLE IF EXISTS public.contracts
ADD COLUMN activate_to_draft_reason text;


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
     JOIN contractors ON contracts.contractor_ulid = contractors.contractor_ulid;

ALTER TABLE public.contracts_index
    OWNER TO postgres;