-- Add migration script here

ALTER TABLE prefilled_individual_contractor_details_for_bulk_upload
    DROP CONSTRAINT prefilled_individual_contractor_details_for_bulk_upload_pkey,
    DROP COLUMN branch_ulid,
    DROP COLUMN department_ulid,
    ADD PRIMARY KEY (email, client_ulid);
