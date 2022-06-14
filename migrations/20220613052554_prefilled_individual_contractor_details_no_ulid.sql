CREATE TABLE IF NOT EXISTS public.prefilled_individual_contractor_account_details_no_client_ulid
(
    email text NOT NULL PRIMARY KEY,
    first_name text NOT NULL,
    last_name text NOT NULL,
    dob timestamp with time zone NOT NULL,
    dial_code text NOT NULL,
    phone_number text NOT NULL,
    country text NOT NULL REFERENCES public.country_codes (code),
    city text NOT NULL,
    address text NOT NULL,
    postal_code text NOT NULL,
    tax_id text,
    time_zone text NOT NULL,
    created_at timestamp with time zone NOT NULL DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE IF EXISTS public.prefilled_individual_contractor_account_details_no_client_ulid
    OWNER to postgres;

CREATE TABLE IF NOT EXISTS public.prefilled_individual_contractor_bank_details_no_client_ulid
(
    email text NOT NULL PRIMARY KEY,
    bank_name text NOT NULL,
    bank_account_name text NOT NULL,
    bank_account_number text NOT NULL,
    bank_code text NOT NULL,
    branch_code text NOT NULL,
    created_at timestamp with time zone NOT NULL DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE IF EXISTS public.prefilled_individual_contractor_bank_details_no_client_ulid
    OWNER to postgres;

CREATE VIEW prefilled_individual_contractor_account_details_index AS
    SELECT
        email, client_ulid, first_name, last_name, dob, 
        dial_code, phone_number, country, city, address, 
        postal_code, tax_id, time_zone, created_at
    FROM
        prefilled_individual_contractor_account_details
    UNION
    SELECT
        email, NULL AS client_ulid, first_name, last_name, dob, 
        dial_code, phone_number, country, city, address, 
        postal_code, tax_id, time_zone, created_at
    FROM
        prefilled_individual_contractor_account_details_no_client_ulid;

CREATE VIEW prefilled_individual_contractor_bank_details_index AS
    SELECT
        email, client_ulid, bank_name, bank_account_name, bank_account_number,
        bank_code, branch_code, created_at 
    FROM
        prefilled_individual_contractor_bank_details
    UNION
    SELECT
        email, NULL AS client_ulid, bank_name, bank_account_name, bank_account_number,
        bank_code, branch_code, created_at 
    FROM
        prefilled_individual_contractor_bank_details_no_client_ulid;
