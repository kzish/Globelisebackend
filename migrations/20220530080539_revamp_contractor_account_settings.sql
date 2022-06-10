--- enity contractor bank details

ALTER TABLE IF EXISTS public.entity_contractors_bank_details
ADD COLUMN bank_code text NOT NULL;

ALTER TABLE IF EXISTS public.entity_contractors_bank_details
ADD COLUMN branch_code text NOT NULL;

--- individual contractor bank details

ALTER TABLE IF EXISTS public.individual_contractors_bank_details
ADD COLUMN bank_code text NOT NULL;


ALTER TABLE IF EXISTS public.individual_contractors_bank_details
ADD COLUMN branch_code text NOT NULL;


--individual contracter personal information

ALTER TABLE IF EXISTS public.individual_contractors_account_details
ADD COLUMN gender text COLLATE pg_catalog."default" NOT NULL;

ALTER TABLE IF EXISTS public.individual_contractors_account_details
ADD COLUMN marital_status text COLLATE pg_catalog."default" NOT NULL;

ALTER TABLE IF EXISTS public.individual_contractors_account_details
ADD COLUMN nationality text COLLATE pg_catalog."default";

ALTER TABLE IF EXISTS public.individual_contractors_account_details
ADD COLUMN email_address text;

ALTER TABLE IF EXISTS public.individual_contractors_account_details
ADD COLUMN national_id text;

ALTER TABLE IF EXISTS public.individual_contractors_account_details
ADD COLUMN passport_number text COLLATE pg_catalog."default";

ALTER TABLE IF EXISTS public.individual_contractors_account_details
ADD COLUMN passport_expiry_date text;

ALTER TABLE IF EXISTS public.individual_contractors_account_details
ADD COLUMN work_permit boolean;

ALTER TABLE IF EXISTS public.individual_contractors_account_details
ADD COLUMN added_related_pay_item_id uuid;

ALTER TABLE IF EXISTS public.individual_contractors_account_details
ADD COLUMN total_dependants bigint;

--entity contracter personal information

ALTER TABLE IF EXISTS public.entity_contractors_account_details
ADD COLUMN email_address text;

ALTER TABLE IF EXISTS public.entity_contractors_account_details
ADD COLUMN added_related_pay_item_id uuid;

ALTER TABLE IF EXISTS public.entity_contractors_account_details
ADD COLUMN total_dependants bigint;

---- payroll information
---- contractor entity

CREATE TABLE IF NOT EXISTS public.entity_contractor_payroll_information
(
    contractor_ulid uuid NOT NULL,
    client_ulid uuid NOT NULL,
    monthly_basic_salary_amount double precision NOT NULL,
    monthly_added_pay_items_for_addition_section double precision,
    monthly_added_pay_items_for_deduction_section double precision,
    monthly_added_pay_items_for_statement_only_section double precision,
    monthly_added_pay_items_for_employers_contribution_section double precision,
    CONSTRAINT entity_contractor_payroll_information_pkey PRIMARY KEY (client_ulid, contractor_ulid),
    CONSTRAINT entity_contractor_payroll_information FOREIGN KEY (contractor_ulid)
        REFERENCES users (ulid) MATCH SIMPLE
        ON UPDATE CASCADE
        ON DELETE NO ACTION
)

TABLESPACE pg_default;

ALTER TABLE IF EXISTS public.entity_contractor_payroll_information
    OWNER to postgres;


---- payroll information
---- contractor individuals

CREATE TABLE IF NOT EXISTS public.individual_contractor_payroll_information
(
    contractor_ulid uuid NOT NULL,
    client_ulid uuid NOT NULL,
    monthly_basic_salary_amount double precision NOT NULL,
    monthly_added_pay_items_for_addition_section double precision,
    monthly_added_pay_items_for_deduction_section double precision,
    monthly_added_pay_items_for_statement_only_section double precision,
    monthly_added_pay_items_for_employers_contribution_section double precision,
    CONSTRAINT individual_contractor_payroll_information_pkey PRIMARY KEY (client_ulid, contractor_ulid),
    CONSTRAINT individual_contractor_payroll_information FOREIGN KEY (contractor_ulid)
        REFERENCES users (ulid) MATCH SIMPLE
        ON UPDATE CASCADE
        ON DELETE NO ACTION
)

TABLESPACE pg_default;

ALTER TABLE IF EXISTS public.individual_contractor_payroll_information
    OWNER to postgres;



--- entity_contractor_employment_information 

CREATE TABLE IF NOT EXISTS public.entity_contractor_employment_information
(
    contractor_uuid uuid NOT NULL,
    team_uuid uuid NOT NULL,
    designation text COLLATE pg_catalog."default",
    start_date timestamp with time zone,
    end_date timestamp with time zone,
    employment_status boolean,
    CONSTRAINT entity_contractor_employment_information_pkey PRIMARY KEY (contractor_uuid, team_uuid),
    CONSTRAINT entity_contractor_employment_information FOREIGN KEY (contractor_uuid)
        REFERENCES users (ulid) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE NO ACTION
)

TABLESPACE pg_default;

ALTER TABLE IF EXISTS public.entity_contractor_employment_information
    OWNER to postgres;

--- individual_contractor_employment_information

CREATE TABLE IF NOT EXISTS public.individual_contractor_employment_information
(
    contractor_uuid uuid NOT NULL,
    team_uuid uuid NOT NULL,
    designation text COLLATE pg_catalog."default",
    start_date timestamp with time zone,
    end_date timestamp with time zone,
    employment_status boolean,
    CONSTRAINT individual_contractor_employment_information_pkey PRIMARY KEY (contractor_uuid, team_uuid),
    CONSTRAINT individual_contractor_employment_information FOREIGN KEY (contractor_uuid)
        REFERENCES users (ulid) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE NO ACTION
)

TABLESPACE pg_default;

ALTER TABLE IF EXISTS public.individual_contractor_employment_information
    OWNER to postgres; 

--- contractor_employment_information view
 
CREATE OR REPLACE VIEW public.contractor_employment_information
 AS
 SELECT individual_contractor_employment_information.contractor_uuid,
    individual_contractor_employment_information.team_uuid,
    individual_contractor_employment_information.designation,
    individual_contractor_employment_information.start_date,
    individual_contractor_employment_information.end_date,
    individual_contractor_employment_information.employment_status,
    'individual'::text AS contractor_type,
    client_contractor_pairs.client_ulid
   FROM individual_contractor_employment_information
     JOIN client_contractor_pairs ON individual_contractor_employment_information.contractor_uuid = client_contractor_pairs.contractor_ulid
UNION
 SELECT entity_contractor_employment_information.contractor_uuid,
    entity_contractor_employment_information.team_uuid,
    entity_contractor_employment_information.designation,
    entity_contractor_employment_information.start_date,
    entity_contractor_employment_information.end_date,
    entity_contractor_employment_information.employment_status,
    'entity'::text AS contractor_type,
    client_contractor_pairs.client_ulid
   FROM entity_contractor_employment_information
     JOIN client_contractor_pairs ON entity_contractor_employment_information.contractor_uuid = client_contractor_pairs.contractor_ulid;

ALTER TABLE public.contractor_employment_information
    OWNER TO postgres;

---- contractor_payroll_information
CREATE OR REPLACE VIEW public.contractor_payroll_information
 AS
 SELECT individual_contractor_payroll_information.contractor_ulid,
    individual_contractor_payroll_information.client_ulid,
    individual_contractor_payroll_information.monthly_basic_salary_amount,
    individual_contractor_payroll_information.monthly_added_pay_items_for_addition_section,
    individual_contractor_payroll_information.monthly_added_pay_items_for_deduction_section,
    individual_contractor_payroll_information.monthly_added_pay_items_for_statement_only_section,
    individual_contractor_payroll_information.monthly_added_pay_items_for_employers_contribution_section
   FROM individual_contractor_payroll_information
UNION
 SELECT entity_contractor_payroll_information.contractor_ulid,
    entity_contractor_payroll_information.client_ulid,
    entity_contractor_payroll_information.monthly_basic_salary_amount,
    entity_contractor_payroll_information.monthly_added_pay_items_for_addition_section,
    entity_contractor_payroll_information.monthly_added_pay_items_for_deduction_section,
    entity_contractor_payroll_information.monthly_added_pay_items_for_statement_only_section,
    entity_contractor_payroll_information.monthly_added_pay_items_for_employers_contribution_section
   FROM entity_contractor_payroll_information;

ALTER TABLE public.contractor_payroll_information
    OWNER TO postgres;

