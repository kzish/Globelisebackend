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

CREATE OR REPLACE VIEW public.contractors_index_for_clients AS
 SELECT a.client_ulid,
    b.name AS client_name,
    a.contractor_ulid,
    c.name AS contractor_name,
    a.contract_name,
    a.contract_status,
    a.job_title,
    a.seniority
   FROM ((public.contracts a
     JOIN public.client_index b ON ((a.client_ulid = b.ulid)))
     JOIN public.contractor_index c ON ((a.contractor_ulid = c.ulid)));


ALTER TABLE public.contractors_index_for_clients OWNER TO postgres;


DROP TABLE client_contractor_pairs;
