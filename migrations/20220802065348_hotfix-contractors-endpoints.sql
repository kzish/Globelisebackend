
DROP VIEW public.onboarded_user_index;
DROP VIEW public.contractor_index;
DROP VIEW public.client_index;
DROP VIEW public.payslips_index;
DROP VIEW public.contractors_index_for_clients;
DROP VIEW public.clients_index_for_contractors;
DROP VIEW public.search_employee_contractors;
DROP VIEW public.contractor_employment_information;
DROP VIEW public.entity_client_branch_department_individual_contractors_index;
DROP VIEW public.contractors_not_in_any_team_details;
DROP VIEW public.contractors_not_in_any_cost_center_details;
DROP VIEW public.team_contractors_details;
DROP VIEW public.contracts_index;
DROP VIEW public.cost_center_contractors_details;

ALTER TABLE IF EXISTS public.contracts DROP COLUMN IF EXISTS contractor_ulid;
ALTER TABLE IF EXISTS public.contracts
ADD COLUMN contractor_ulid uuid;

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
           FROM cost_center_contractor_pairs cost_center_contractor_pairs_1
          WHERE cost_center_contractor_pairs_1.contractor_ulid = contractors.contractor_ulid) AS cost_center_count,
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


CREATE OR REPLACE VIEW public.team_contractors_details
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
    teams.team_name,
    teams.ulid AS team_ulid,
    client_branches.country,
    client_branches.time_zone,
    contracts.job_title AS job_description
   FROM contractors
     JOIN entity_contractor_branch_pairs ON contractors.contractor_ulid = entity_contractor_branch_pairs.contractor_ulid
     JOIN client_branches ON entity_contractor_branch_pairs.branch_ulid = client_branches.branch_ulid
     JOIN teams ON client_branches.branch_ulid = teams.branch_ulid
     JOIN teams_contractors ON teams.ulid = teams_contractors.team_ulid
     JOIN contracts ON contracts.contractor_ulid = contractors.contractor_ulid;

ALTER TABLE public.team_contractors_details
    OWNER TO postgres;

CREATE OR REPLACE VIEW public.contractors_not_in_any_cost_center_details
 AS
 WITH contractors AS (
         SELECT entity_contractor_account_details.ulid AS contractor_ulid,
            entity_contractor_account_details.company_name AS contractor_name,
            entity_contractor_account_details.email_address,
            entity_contractor_account_details.time_zone,
            entity_contractor_account_details.country
           FROM entity_contractor_account_details
        UNION
         SELECT individual_contractor_account_details.ulid AS contractor_ulid,
            concat(individual_contractor_account_details.first_name, ' ', individual_contractor_account_details.last_name) AS contractor_name,
            individual_contractor_account_details.email_address,
            individual_contractor_account_details.time_zone,
            individual_contractor_account_details.country
           FROM individual_contractor_account_details
        )
 SELECT contractors.contractor_ulid,
    contractors.contractor_name,
    contractors.email_address,
    ( SELECT count(*) AS count
           FROM cost_center_contractor_pairs
          WHERE cost_center_contractor_pairs.contractor_ulid = contractors.contractor_ulid) AS cost_center_count,
    contractors.time_zone,
    contractors.country,
    contracts.job_title AS job_description,
    entity_client_branch_account_details.branch_name,
    entity_client_branch_account_details.ulid AS branch_ulid
   FROM contractors
     JOIN contracts ON contracts.contractor_ulid = contractors.contractor_ulid
     JOIN entity_contractor_branch_pairs ON entity_contractor_branch_pairs.contractor_ulid = contractors.contractor_ulid
     JOIN entity_client_branch_account_details ON entity_client_branch_account_details.ulid = contracts.branch_ulid;

ALTER TABLE public.contractors_not_in_any_cost_center_details
    OWNER TO postgres;


CREATE OR REPLACE VIEW public.contractors_not_in_any_team_details
 AS
 WITH contractors AS (
         SELECT entity_contractor_account_details.ulid AS contractor_ulid,
            entity_contractor_account_details.company_name AS contractor_name,
            entity_contractor_account_details.email_address,
            entity_contractor_account_details.time_zone,
            entity_contractor_account_details.country
           FROM entity_contractor_account_details
        UNION
         SELECT individual_contractor_account_details.ulid AS contractor_ulid,
            concat(individual_contractor_account_details.first_name, ' ', individual_contractor_account_details.last_name) AS contractor_name,
            individual_contractor_account_details.email_address,
            individual_contractor_account_details.time_zone,
            individual_contractor_account_details.country
           FROM individual_contractor_account_details
        )
 SELECT contractors.contractor_ulid,
    contractors.contractor_name,
    contractors.email_address,
    ( SELECT count(*) AS count
           FROM teams_contractors
          WHERE teams_contractors.contractor_ulid = contractors.contractor_ulid) AS teams_count,
    contractors.time_zone,
    contractors.country,
    contracts.job_title AS job_description,
    entity_client_branch_account_details.branch_name,
    entity_client_branch_account_details.ulid AS branch_ulid
   FROM contractors
     JOIN contracts ON contracts.contractor_ulid = contractors.contractor_ulid
     JOIN entity_contractor_branch_pairs ON entity_contractor_branch_pairs.contractor_ulid = contractors.contractor_ulid
     JOIN entity_client_branch_account_details ON entity_client_branch_account_details.ulid = contracts.branch_ulid;

ALTER TABLE public.contractors_not_in_any_team_details
    OWNER TO postgres;


CREATE OR REPLACE VIEW public.entity_client_branch_department_individual_contractors_index
 AS
 SELECT c.ulid,
    b.branch_ulid,
    b.branch_name,
    b.ulid AS department_ulid,
    b.department_name,
    b.classification
   FROM individual_contractor_department_pairs a
     JOIN entity_client_branch_departments_index b ON a.department_ulid = b.ulid
     JOIN onboarded_user_index c ON c.ulid = a.individual_ulid
     JOIN contracts d ON d.contractor_ulid = c.ulid AND d.branch_ulid = b.branch_ulid;

ALTER TABLE public.entity_client_branch_department_individual_contractors_index
    OWNER TO postgres;

CREATE OR REPLACE VIEW public.contractor_employment_information
 AS
 SELECT individual_contractor_employment_information.contractor_uuid,
    individual_contractor_employment_information.team_uuid,
    individual_contractor_employment_information.designation,
    individual_contractor_employment_information.start_date,
    individual_contractor_employment_information.end_date,
    individual_contractor_employment_information.employment_status,
    'individual'::text AS contractor_type,
    contracts.client_ulid
   FROM individual_contractor_employment_information
     JOIN contracts ON individual_contractor_employment_information.contractor_uuid = contracts.contractor_ulid
UNION
 SELECT entity_contractor_employment_information.contractor_uuid,
    entity_contractor_employment_information.team_uuid,
    entity_contractor_employment_information.designation,
    entity_contractor_employment_information.start_date,
    entity_contractor_employment_information.end_date,
    entity_contractor_employment_information.employment_status,
    'entity'::text AS contractor_type,
    contracts.client_ulid
   FROM entity_contractor_employment_information
     JOIN contracts ON entity_contractor_employment_information.contractor_uuid = contracts.contractor_ulid;

ALTER TABLE public.contractor_employment_information
    OWNER TO postgres;


CREATE OR REPLACE VIEW public.search_employee_contractors
 AS
 WITH contractors AS (
         SELECT individual_contractor_account_details.ulid,
            concat(individual_contractor_account_details.first_name, ' ', individual_contractor_account_details.last_name) AS name,
            individual_contractor_account_details.time_zone
           FROM individual_contractor_account_details
        UNION
         SELECT entity_contractor_account_details.ulid,
            entity_contractor_account_details.company_name AS name,
            entity_contractor_account_details.time_zone
           FROM entity_contractor_account_details
        ), branches AS (
         SELECT entity_client_branch_account_details.branch_name AS sub_entity,
            entity_client_branches.ulid AS branch_ulid,
            entity_client_branches.client_ulid
           FROM entity_client_branch_account_details
             JOIN entity_client_branches ON entity_client_branch_account_details.ulid = entity_client_branches.ulid
        ), contractor_branches AS (
         SELECT entity_client_branch_individual_contractor_pairs.contractor_ulid,
            entity_client_branch_individual_contractor_pairs.branch_ulid
           FROM entity_client_branch_individual_contractor_pairs
        UNION
         SELECT entity_contractor_branch_pairs.contractor_ulid,
            entity_contractor_branch_pairs.branch_ulid
           FROM entity_contractor_branch_pairs
        )
 SELECT contractors.name,
    contractors.time_zone,
    contracts.job_title,
    branches.branch_ulid,
    branches.sub_entity,
    entity_client_branch_departments.classification,
    entity_client_branch_departments.department_name,
    contractors.ulid,
    branches.client_ulid,
    contracts.contract_name,
    contracts.contract_status
   FROM contractors
     JOIN contracts ON contractors.ulid = contracts.contractor_ulid
     JOIN contractor_branches ON contracts.contractor_ulid = contractor_branches.contractor_ulid
     JOIN branches ON contractor_branches.branch_ulid = branches.branch_ulid
     JOIN entity_client_branch_departments ON branches.branch_ulid = entity_client_branch_departments.branch_ulid;

ALTER TABLE public.search_employee_contractors
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

CREATE OR REPLACE VIEW public.payslips_index
 AS
 SELECT payslips.ulid AS payslip_ulid,
    payslips.client_ulid,
    COALESCE(client_index.name, ''::text) AS client_name,
    payslips.contractor_ulid,
    COALESCE(contractor_index.name, ''::text) AS contractor_name,
    payslips.contract_ulid,
    contracts.contract_name,
    payslips.payslip_title,
    payslips.payment_date,
    payslips.begin_period,
    payslips.end_period
   FROM payslips
     LEFT JOIN client_index ON payslips.client_ulid = client_index.ulid
     LEFT JOIN contractor_index ON payslips.contractor_ulid = contractor_index.ulid
     LEFT JOIN contracts ON payslips.contract_ulid = contracts.ulid;

ALTER TABLE public.payslips_index
    OWNER TO postgres;


CREATE OR REPLACE VIEW public.client_index
 AS
 SELECT onboarded_user_index.created_at,
    onboarded_user_index.ulid,
    onboarded_user_index.name,
    onboarded_user_index.email,
    onboarded_user_index.user_type
   FROM onboarded_user_index
  WHERE onboarded_user_index.user_role = 'client'::text;

ALTER TABLE public.client_index
    OWNER TO postgres;


CREATE OR REPLACE VIEW public.contractor_index
 AS
 SELECT onboarded_user_index.created_at,
    onboarded_user_index.ulid,
    onboarded_user_index.name,
    onboarded_user_index.email,
    onboarded_user_index.user_type
   FROM onboarded_user_index
  WHERE onboarded_user_index.user_role = 'contractor'::text;

ALTER TABLE public.contractor_index
    OWNER TO postgres;



CREATE OR REPLACE VIEW public.onboarded_user_index
 AS
 WITH client_individual_info AS (
         SELECT users.created_at,
            users.ulid,
            users.email,
            concat(onboard_individual_clients.first_name, ' ', onboard_individual_clients.last_name) AS name,
            'client'::text AS user_role,
            'individual'::text AS user_type,
            ( SELECT count(*) AS count
                   FROM contracts
                  WHERE contracts.client_ulid = users.ulid) AS contract_count
           FROM onboard_individual_clients
             JOIN users ON users.ulid = onboard_individual_clients.ulid
        ), client_entity_info AS (
         SELECT users.created_at,
            users.ulid,
            users.email,
            onboard_entity_clients.company_name AS name,
            'client'::text AS user_role,
            'entity'::text AS user_type,
            ( SELECT count(*) AS count
                   FROM contracts
                  WHERE contracts.client_ulid = users.ulid) AS contract_count
           FROM onboard_entity_clients
             JOIN users ON users.ulid = onboard_entity_clients.ulid
        ), contractor_individual_info AS (
         SELECT users.created_at,
            users.ulid,
            users.email,
            concat(onboard_individual_contractors.first_name, ' ', onboard_individual_contractors.last_name) AS name,
            'contractor'::text AS user_role,
            'individual'::text AS user_type,
            ( SELECT count(*) AS count
                   FROM contracts
                  WHERE contracts.contractor_ulid = users.ulid) AS contract_count
           FROM onboard_individual_contractors
             JOIN users ON users.ulid = onboard_individual_contractors.ulid
        ), contractor_entity_info AS (
         SELECT users.created_at,
            users.ulid,
            users.email,
            onboard_entity_contractors.company_name AS name,
            'contractor'::text AS user_role,
            'entity'::text AS user_type,
            ( SELECT count(*) AS count
                   FROM contracts
                  WHERE contracts.contractor_ulid = users.ulid) AS contract_count
           FROM onboard_entity_contractors
             JOIN users ON users.ulid = onboard_entity_contractors.ulid
        )
 SELECT client_individual_info.created_at,
    client_individual_info.ulid,
    client_individual_info.name,
    client_individual_info.email,
    client_individual_info.user_role,
    client_individual_info.user_type,
    client_individual_info.contract_count
   FROM client_individual_info
UNION
 SELECT client_entity_info.created_at,
    client_entity_info.ulid,
    client_entity_info.name,
    client_entity_info.email,
    client_entity_info.user_role,
    client_entity_info.user_type,
    client_entity_info.contract_count
   FROM client_entity_info
UNION
 SELECT contractor_individual_info.created_at,
    contractor_individual_info.ulid,
    contractor_individual_info.name,
    contractor_individual_info.email,
    contractor_individual_info.user_role,
    contractor_individual_info.user_type,
    contractor_individual_info.contract_count
   FROM contractor_individual_info
UNION
 SELECT contractor_entity_info.created_at,
    contractor_entity_info.ulid,
    contractor_entity_info.name,
    contractor_entity_info.email,
    contractor_entity_info.user_role,
    contractor_entity_info.user_type,
    contractor_entity_info.contract_count
   FROM contractor_entity_info;

ALTER TABLE public.onboarded_user_index
    OWNER TO postgres;