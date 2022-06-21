-- Add migration script here

ALTER TABLE prefilled_individual_contractor_details_for_bulk_upload
    DROP CONSTRAINT prefilled_individual_contractors_details_for_bulk_upload_pkey,
    DROP COLUMN ulid,
    ALTER COLUMN department_ulid DROP NOT NULL,
    ADD PRIMARY KEY (email, client_ulid, branch_ulid);