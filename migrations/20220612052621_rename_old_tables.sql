DROP VIEW entity_client_branch_deparment_individual_contractors_index;

CREATE VIEW entity_client_branch_department_individual_contractors_index AS
SELECT
    c.ulid,
    b.branch_ulid,
    b.branch_name,
    b.ulid as department_ulid,
    b.department_name,
    b.classification
FROM
    individual_contractor_department_pairs a
    JOIN entity_client_branch_departments_index b ON a.department_ulid = b.ulid
    JOIN onboarded_user_index c ON c.ulid = a.individual_ulid
    JOIN contracts d ON d.contractor_ulid = c.ulid
    AND d.branch_ulid = b.branch_ulid;

ALTER TABLE entity_clients_account_details
RENAME TO entity_client_account_details;

ALTER TABLE entity_clients_branch_account_details
RENAME TO entity_client_branch_account_details;

ALTER TABLE entity_clients_branch_details
RENAME TO entity_client_branch_details;

ALTER TABLE entity_clients_branch_pay_items
RENAME TO entity_client_branch_pay_items;

ALTER TABLE entity_clients_branch_payroll_details
RENAME TO entity_client_branch_payroll_details;

ALTER TABLE entity_clients_fully_onboarded
RENAME TO entity_client_fully_onboarded;

ALTER TABLE entity_clients_payment_details
RENAME TO entity_client_payment_details;

ALTER TABLE entity_clients_pic_details
RENAME TO entity_client_pic_details;

ALTER TABLE entity_contractors_account_details
RENAME TO entity_contractor_account_details;

ALTER TABLE entity_contractors_bank_details
RENAME TO entity_contractor_bank_details;

ALTER TABLE entity_contractors_fully_onboarded
RENAME TO entity_contractor_fully_onboarded;

ALTER TABLE entity_contractors_pic_details
RENAME TO entity_contractor_pic_details;

ALTER TABLE individual_clients_account_details
RENAME TO individual_client_account_details;

ALTER TABLE individual_clients_payment_details
RENAME TO individual_client_payment_details;

ALTER TABLE individual_contractors_account_details
RENAME TO individual_contractor_account_details;

ALTER TABLE individual_contractors_bank_details
RENAME TO individual_contractor_bank_details;

ALTER TABLE individual_contractors_fully_onboarded
RENAME TO individual_contractor_fully_onboarded;

ALTER TABLE prefilled_entity_clients_account_details
RENAME TO prefilled_entity_client_account_details;

ALTER TABLE prefilled_entity_clients_bank_details
RENAME TO prefilled_entity_client_bank_details;

ALTER TABLE prefilled_entity_clients_payment_details
RENAME TO prefilled_entity_client_payment_details;

ALTER TABLE prefilled_entity_clients_pic_details
RENAME TO prefilled_entity_client_pic_details;

ALTER TABLE prefilled_individual_contractors_account_details
RENAME TO prefilled_individual_contractor_account_details;

ALTER TABLE prefilled_individual_contractors_bank_details
RENAME TO prefilled_individual_contractor_bank_details;

ALTER TABLE prefilled_individual_contractors_details_for_bulk_upload
RENAME TO prefilled_individual_contractor_details_for_bulk_upload;

ALTER TABLE entity_clients_branch_bank_details
RENAME TO entity_client_branch_bank_details;

ALTER TABLE individual_clients_fully_onboarded
RENAME TO individual_client_fully_onboarded;