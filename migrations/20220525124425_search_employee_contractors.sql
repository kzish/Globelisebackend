--
-- Name: search_employee_contractors; Type: VIEW; Schema: public; Owner: postgres
--

CREATE VIEW public.search_employee_contractors AS
 WITH contractors AS (
         SELECT individual_contractors_account_details.ulid,
            concat(individual_contractors_account_details.first_name, ' ', individual_contractors_account_details.last_name) AS name,
            individual_contractors_account_details.time_zone
           FROM public.individual_contractors_account_details
        UNION
         SELECT entity_contractors_account_details.ulid,
            entity_contractors_account_details.company_name AS name,
            entity_contractors_account_details.time_zone
           FROM public.entity_contractors_account_details
        ), branches AS (
         SELECT entity_clients_branch_account_details.branch_name AS sub_entity,
            entity_client_branches.ulid AS branch_ulid,
            entity_client_branches.client_ulid
           FROM (public.entity_clients_branch_account_details
             JOIN public.entity_client_branches ON ((entity_clients_branch_account_details.ulid = entity_client_branches.ulid)))
        ), contractor_branches AS (
         SELECT individual_contractor_branch_pairs.contractor_ulid,
            individual_contractor_branch_pairs.branch_ulid
           FROM public.individual_contractor_branch_pairs
        UNION
         SELECT entity_contractor_branch_pairs.contractor_ulid,
            entity_contractor_branch_pairs.branch_ulid
           FROM public.entity_contractor_branch_pairs
        )
 SELECT contractors.name,
    contractors.time_zone,
    public.contracts.job_title,
    branches.branch_ulid,
    branches.sub_entity,
    entity_client_branch_departments.classification,
    entity_client_branch_departments.department_name,
    contractors.ulid,
    branches.client_ulid,
    public.contracts.contract_name,
    public.contracts.contract_status
   FROM ((((contractors
     JOIN public.contracts ON ((contractors.ulid = public.contracts.contractor_ulid)))
     JOIN contractor_branches ON ((public.contracts.contractor_ulid = contractor_branches.contractor_ulid)))
     JOIN branches ON ((contractor_branches.branch_ulid = branches.branch_ulid)))
     JOIN public.entity_client_branch_departments ON ((branches.branch_ulid = entity_client_branch_departments.branch_ulid)));


ALTER TABLE public.search_employee_contractors OWNER TO postgres;
