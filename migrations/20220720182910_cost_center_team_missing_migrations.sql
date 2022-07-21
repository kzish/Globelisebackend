DROP VIEW public.contractors_not_in_any_team_details;

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





DROP VIEW public.contractors_not_in_any_cost_center_details;

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
           FROM cost_center_contractors
          WHERE cost_center_contractors.contractor_ulid = contractors.contractor_ulid) AS cost_center_count,
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



DROP VIEW public.team_contractors_details;

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



ALTER TABLE entity_contractor_branch_pairs ADD PRIMARY KEY (contractor_ulid, branch_ulid);


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
           FROM cost_center_contractors cost_center_contractors_1
          WHERE cost_center_contractors_1.contractor_ulid = contractors.contractor_ulid) AS cost_center_count,
    client_branches.time_zone,
    contracts.job_title AS job_description
   FROM contractors
     JOIN entity_contractor_branch_pairs ON entity_contractor_branch_pairs.contractor_ulid = contractors.contractor_ulid
     JOIN client_branches ON client_branches.branch_ulid = entity_contractor_branch_pairs.branch_ulid
     JOIN cost_center ON cost_center.branch_ulid = client_branches.branch_ulid
     JOIN cost_center_contractors ON cost_center_contractors.cost_center_ulid = cost_center.ulid
     JOIN contracts ON contracts.contractor_ulid = contractors.contractor_ulid;

ALTER TABLE public.cost_center_contractors_details
    OWNER TO postgres;