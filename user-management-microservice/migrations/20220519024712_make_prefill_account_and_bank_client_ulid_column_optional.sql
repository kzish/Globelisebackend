--- Drop constraints

ALTER TABLE ONLY
    prefilled_individual_contractors_bank_details
ALTER COLUMN
    client_ulid
DROP NOT NULL;

ALTER TABLE ONLY
    prefilled_individual_contractors_bank_details
DROP CONSTRAINT 
    prefilled_individual_contractors_bank_details_email_fkey;

ALTER TABLE ONLY 
    prefilled_individual_contractors_bank_details
DROP CONSTRAINT 
    prefilled_individual_contractors_bank_details_pkey;

ALTER TABLE ONLY
    prefilled_individual_contractors_account_details
ALTER COLUMN
    client_ulid
DROP NOT NULL;

ALTER TABLE ONLY 
    prefilled_individual_contractors_account_details
DROP CONSTRAINT 
    prefilled_individual_contractors_account_details_pkey;

--- Add constraints

ALTER TABLE ONLY
    prefilled_individual_contractors_bank_details
ADD CONSTRAINT 
    prefilled_individual_contractors_bank_details_unique
UNIQUE(email, client_ulid);

ALTER TABLE ONLY
    prefilled_individual_contractors_account_details
ADD CONSTRAINT 
    prefilled_individual_contractors_account_details_unique
UNIQUE(email, client_ulid);