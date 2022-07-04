-- View: public.contractors_not_in_any_cost_center_details

-- DROP VIEW public.contractors_not_in_any_cost_center_details;

CREATE OR REPLACE VIEW public.contractors_not_in_any_cost_center_details
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
        )
 SELECT contractors.contractor_ulid,
    contractors.contractor_name,
    contractors.email_address,
    ( SELECT count(*) AS count
           FROM cost_center_contractors
          WHERE cost_center_contractors.contractor_ulid = contractors.contractor_ulid) AS cost_center_count
   FROM contractors;

ALTER TABLE public.contractors_not_in_any_cost_center_details
    OWNER TO postgres;



-- View: public.contractors_not_in_any_team_details

-- DROP VIEW public.contractors_not_in_any_team_details;

CREATE OR REPLACE VIEW public.contractors_not_in_any_team_details
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
        )
 SELECT contractors.contractor_ulid,
    contractors.contractor_name,
    contractors.email_address,
    ( SELECT count(*) AS count
           FROM teams_contractors
          WHERE teams_contractors.contractor_ulid = contractors.contractor_ulid) AS teams_count
   FROM contractors;

ALTER TABLE public.contractors_not_in_any_team_details
    OWNER TO postgres;

