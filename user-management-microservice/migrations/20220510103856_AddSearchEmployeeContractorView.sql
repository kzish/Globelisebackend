-- View: public.search_employee_contractors

-- DROP VIEW public.search_employee_contractors;

CREATE OR REPLACE VIEW public.search_employee_contractors
 AS
 WITH contractors AS (
         SELECT individual_contractors_account_details.ulid,
            concat(individual_contractors_account_details.first_name, ' ', individual_contractors_account_details.last_name) AS name,
            individual_contractors_account_details.time_zone
           FROM individual_contractors_account_details
        UNION
         SELECT entity_contractors_account_details.ulid,
            entity_contractors_account_details.company_name AS name,
            entity_contractors_account_details.time_zone
           FROM entity_contractors_account_details
        ), branches AS (
         SELECT entity_clients_branch_account_details.company_name AS sub_entity,
            entity_client_branches.ulid AS branch_ulid,
            entity_client_branches.client_ulid
           FROM entity_clients_branch_account_details
             JOIN entity_client_branches ON entity_clients_branch_account_details.ulid = entity_client_branches.ulid
        ), contractor_branches AS (
         SELECT individual_contractor_branch_pairs.contractor_ulid,
            individual_contractor_branch_pairs.branch_ulid
           FROM individual_contractor_branch_pairs
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

