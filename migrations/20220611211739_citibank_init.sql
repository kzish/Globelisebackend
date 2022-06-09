-- Add migration script here


CREATE TABLE IF NOT EXISTS public.uploaded_citibank_transfer_initiation_files
(
    ulid uuid NOT NULL,
    title_identifier text COLLATE pg_catalog."default" NOT NULL,
    status text COLLATE pg_catalog."default" NOT NULL,
    created_at time with time zone NOT NULL DEFAULT CURRENT_TIMESTAMP,
    client_ulid uuid NOT NULL,
    CONSTRAINT uploaded_citibank_transfer_initiation_files_pkey PRIMARY KEY (ulid),
    CONSTRAINT client_ulid_fkey FOREIGN KEY (client_ulid)
        REFERENCES public.users (ulid) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE NO ACTION
)

TABLESPACE pg_default;

ALTER TABLE IF EXISTS public.uploaded_citibank_transfer_initiation_files
    OWNER to postgres;


    
CREATE TABLE IF NOT EXISTS public.uploaded_citibank_transfer_initiation_files_records
(
    ulid uuid NOT NULL,
    currency_code text COLLATE pg_catalog."default" NOT NULL,
    country_code text COLLATE pg_catalog."default" NOT NULL,
    employee_id uuid NOT NULL,
    employee_name text COLLATE pg_catalog."default" NOT NULL,
    bank_name text COLLATE pg_catalog."default" NOT NULL,
    bank_account_number text COLLATE pg_catalog."default" NOT NULL,
    bank_code text COLLATE pg_catalog."default" NOT NULL,
    swift_code text COLLATE pg_catalog."default" NOT NULL,
    amount double precision NOT NULL,
    file_ulid uuid NOT NULL,
    transaction_status text COLLATE pg_catalog."default" NOT NULL,
    transaction_status_description text COLLATE pg_catalog."default",
    CONSTRAINT uploaded_citibank_transfer_initiation_files_records_pkey PRIMARY KEY (ulid),
    CONSTRAINT uploaded_citibank_transfer_initiation_files_records_fkey FOREIGN KEY (file_ulid)
        REFERENCES public.uploaded_citibank_transfer_initiation_files (ulid) MATCH SIMPLE
        ON UPDATE CASCADE
        ON DELETE CASCADE
)

TABLESPACE pg_default;

ALTER TABLE IF EXISTS public.uploaded_citibank_transfer_initiation_files_records
    OWNER to postgres;


    
CREATE OR REPLACE VIEW public.contractor_bank_account_details_citibank_template
 AS
 WITH contractor_account_details AS (
         SELECT entity_contractors_account_details.ulid AS contractor_ulid,
            entity_contractors_account_details.company_name AS name
           FROM entity_contractors_account_details
        UNION
         SELECT individual_contractors_account_details.ulid AS contractor_ulid,
            concat(individual_contractors_account_details.first_name, ' ', individual_contractors_account_details.last_name) AS name
           FROM individual_contractors_account_details
        ), contractor_bank_details AS (
         SELECT individual_contractors_bank_details.ulid,
            individual_contractors_bank_details.bank_name,
            individual_contractors_bank_details.bank_account_number,
            individual_contractors_bank_details.bank_code,
            individual_contractors_bank_details.branch_code
           FROM individual_contractors_bank_details
        UNION
         SELECT entity_contractors_bank_details.ulid,
            entity_contractors_bank_details.bank_name,
            entity_contractors_bank_details.bank_account_number,
            entity_contractors_bank_details.bank_code,
            entity_contractors_bank_details.branch_code
           FROM entity_contractors_bank_details
        )
 SELECT contractor_account_details.contractor_ulid AS employee_id,
    contractor_account_details.name AS employee_name,
    contractor_bank_details.bank_name,
    contractor_bank_details.bank_account_number,
    contractor_bank_details.bank_code,
    contractor_bank_details.branch_code AS bank_branch_code,
    contractor_branch_pairs.branch_ulid
   FROM contractor_account_details
     JOIN contractor_bank_details ON contractor_account_details.contractor_ulid = contractor_bank_details.ulid
     JOIN contractor_branch_pairs ON contractor_branch_pairs.contractor_ulid = contractor_account_details.contractor_ulid;

ALTER TABLE public.contractor_bank_account_details_citibank_template
    OWNER TO postgres;


