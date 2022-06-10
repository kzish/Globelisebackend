CREATE VIEW contracts_index AS
 SELECT a.ulid AS contract_ulid,
    a.client_ulid,
    a.branch_ulid,
    b.company_name AS client_name,
    a.contractor_ulid,
    CONCAT(c.first_name, ' ', c.last_name) AS contractor_name,
    a.contract_name,
    a.contract_type,
    a.contract_status,
    a.contract_amount,
    a.currency,
    a.begin_at,
    a.end_at,
    a.job_title,
    a.seniority
   FROM contracts a
     LEFT JOIN onboard_entity_clients b ON a.client_ulid = b.ulid
     LEFT JOIN onboard_individual_contractors c ON a.contractor_ulid = c.ulid;