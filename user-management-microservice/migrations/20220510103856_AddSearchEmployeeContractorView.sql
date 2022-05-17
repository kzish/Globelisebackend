CREATE TABLE public.pubsub_contracts (
    created_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    ulid uuid NOT NULL PRIMARY KEY,
    client_ulid uuid NOT NULL,
    contractor_ulid uuid NOT NULL,
    contract_name text NOT NULL,
    contract_type text NOT NULL,
    contract_status text NOT NULL,
    contract_amount numeric NOT NULL,
    currency public.currency NOT NULL,
    job_title text NOT NULL,
    seniority text NOT NULL,
    begin_at date NOT NULL,
    end_at date NOT NULL,
    branch_ulid uuid,
    CONSTRAINT pubsub_contracts_begin_at_end_at_check CHECK ((begin_at <= end_at)),
    CONSTRAINT pubsub_contracts_contract_amount_check CHECK ((contract_amount >= (0)::numeric))
);

ALTER TABLE public.pubsub_contracts OWNER TO postgres;

CREATE TRIGGER mdt_pubsub_contracts BEFORE UPDATE ON public.pubsub_contracts FOR EACH ROW EXECUTE FUNCTION public.moddatetime('updated_at');

-- Table: public.entity_contractor_branch_pairs

-- DROP TABLE IF EXISTS public.entity_contractor_branch_pairs;

CREATE TABLE public.entity_contractor_branch_pairs
(
    contractor_ulid uuid NOT NULL,
    branch_ulid uuid NOT NULL,
    created_at timestamp with time zone NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at timestamp with time zone NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT entity_contractor_branch_pairs_pkey PRIMARY KEY (contractor_ulid, branch_ulid),
    CONSTRAINT entity_contractor_branch_pairs_branch_ulid_fkey FOREIGN KEY (branch_ulid)
        REFERENCES public.entity_client_branches (ulid) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE NO ACTION,
    CONSTRAINT entity_contractor_branch_pairs_contractor_ulid_fkey FOREIGN KEY (contractor_ulid)
        REFERENCES public.auth_entities (ulid) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE NO ACTION
);

ALTER TABLE public.entity_contractor_branch_pairs
    OWNER to postgres;

CREATE TRIGGER mdt_entity_contractor_branch_pairs
    BEFORE UPDATE 
    ON public.entity_contractor_branch_pairs
    FOR EACH ROW
    EXECUTE FUNCTION public.moddatetime('updated_at');

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
    pubsub_contracts.job_title,
    branches.branch_ulid,
    branches.sub_entity,
    entity_client_branch_departments.classification,
    entity_client_branch_departments.department_name,
    contractors.ulid,
    branches.client_ulid,
    pubsub_contracts.contract_name,
    pubsub_contracts.contract_status
   FROM contractors
     JOIN pubsub_contracts ON contractors.ulid = pubsub_contracts.contractor_ulid
     JOIN contractor_branches ON pubsub_contracts.contractor_ulid = contractor_branches.contractor_ulid
     JOIN branches ON contractor_branches.branch_ulid = branches.branch_ulid
     JOIN entity_client_branch_departments ON branches.branch_ulid = entity_client_branch_departments.branch_ulid;

ALTER TABLE public.search_employee_contractors
    OWNER TO postgres;

