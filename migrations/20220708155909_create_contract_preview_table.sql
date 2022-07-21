-- Table: public.contract_preview

-- DROP TABLE IF EXISTS public.contract_preview;

CREATE TABLE IF NOT EXISTS public.contract_preview
(
    ulid uuid NOT NULL,
    contract_preview_text text COLLATE pg_catalog."default",
    team_ulid uuid,
    contract_name text COLLATE pg_catalog."default" NOT NULL,
    job_title text COLLATE pg_catalog."default" NOT NULL,
    seniority_level text COLLATE pg_catalog."default" NOT NULL,
    job_scope text COLLATE pg_catalog."default",
    start_date timestamp with time zone,
    end_date timestamp with time zone,
    contract_type text COLLATE pg_catalog."default",
    contract_amount double precision,
    branch_ulid uuid,
    CONSTRAINT contract_preview_pkey PRIMARY KEY (ulid)
)

TABLESPACE pg_default;

ALTER TABLE IF EXISTS public.contract_preview
OWNER to postgres;


ALTER TABLE IF EXISTS public.contracts
ADD COLUMN contract_preview_text text COLLATE pg_catalog."default";


ALTER TABLE IF EXISTS public.contracts
ADD COLUMN team_ulid uuid;


ALTER TABLE IF EXISTS public.contracts
ADD COLUMN job_scope text COLLATE pg_catalog."default";



DROP VIEW public.contracts_index;

CREATE OR REPLACE VIEW public.contracts_index
 AS
 SELECT a.ulid AS contract_ulid,
    a.client_ulid,
    a.branch_ulid,
    b.company_name AS client_name,
    a.contractor_ulid,
    concat(c.first_name, ' ', c.last_name) AS contractor_name,
    a.contract_name,
    a.contract_type,
    a.contract_status,
    a.contract_amount,
    a.currency,
    a.begin_at,
    a.end_at,
    a.job_title,
    a.seniority,
    a.contract_preview_text,
    a.team_ulid,
    a.job_scope
   FROM contracts a
     LEFT JOIN onboard_entity_clients b ON a.client_ulid = b.ulid
     LEFT JOIN onboard_individual_contractors c ON a.contractor_ulid = c.ulid;

ALTER TABLE public.contracts_index
    OWNER TO postgres;

