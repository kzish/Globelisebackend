-- Altering entity_clients_branch_details

ALTER TABLE entity_clients_branch_details
    RENAME TO entity_clients_branch_account_details;

ALTER TABLE ONLY entity_clients_branch_account_details
    ADD CONSTRAINT entity_clients_branch_account_details_pkey PRIMARY KEY (ulid);

ALTER TABLE ONLY entity_clients_branch_account_details 
    ADD COLUMN time_zone TEXT NOT NULL;
ALTER TABLE ONLY entity_clients_branch_account_details 
    ADD COLUMN logo BYTEA;

-- Altering entity_clients_bank_details

ALTER TABLE entity_clients_bank_details
    RENAME TO entity_clients_branch_bank_details;

ALTER TABLE ONLY entity_clients_branch_bank_details
    ADD CONSTRAINT entity_clients_branch_bank_details_pkey PRIMARY KEY (ulid);

-- Altering entity_clients_payroll_details

ALTER TABLE entity_clients_payroll_details
    RENAME TO entity_clients_branch_payroll_details;

ALTER TABLE ONLY entity_clients_branch_payroll_details
    ADD CONSTRAINT entity_clients_branch_payroll_details_pkey PRIMARY KEY (ulid);

-- Add a view that concates everything

CREATE VIEW public.entity_clients_branch_details AS
    SELECT
        a.client_ulid AS client_ulid,
        -- account details
        b.ulid, b.company_name, b.country, b.entity_type, b.registration_number, b.tax_id, 
        b.statutory_contribution_submission_number, b.company_address, b.city, b.postal_code, 
        b.time_zone, b.logo,
        -- bank details
        c.currency, c.bank_name, c.bank_account_name, c.bank_account_number,
        c.swift_code, c.bank_key, c.iban, c.bank_code, c.branch_code,
        -- payment details
        d.cutoff_date, d.payment_date
    FROM 
        entity_client_branches a
    JOIN
        entity_clients_branch_account_details b
    ON
        a.ulid = b.ulid
    JOIN 
        entity_clients_branch_bank_details c
    ON
        b.ulid = c.ulid
    JOIN 
        entity_clients_branch_payroll_details d
    ON 
        c.ulid = d.ulid;
