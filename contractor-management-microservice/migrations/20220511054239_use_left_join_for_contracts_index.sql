DROP VIEW contracts_index;

CREATE VIEW contracts_index AS
SELECT contracts.ulid AS contract_ulid,
    contracts.client_ulid,
    contracts.branch_ulid,
    COALESCE(client_names.name, '') AS client_name,
    contracts.contractor_ulid,
    COALESCE(contractor_names.name, '') AS contractor_name,
    contracts.contract_name,
    contracts.contract_type,
    contracts.contract_status,
    contracts.contract_amount,
    contracts.currency,
    contracts.begin_at,
    contracts.end_at,
    contracts.job_title,
    contracts.seniority
FROM 
    contracts
LEFT JOIN 
    client_names 
ON 
    contracts.client_ulid = client_names.ulid
LEFT JOIN 
    contractor_names 
ON 
    contracts.contractor_ulid = contractor_names.ulid;