CREATE OR REPLACE VIEW public.onboarded_user_index AS 
WITH client_individual_info AS (
    SELECT
        users.created_at,
        users.ulid,
        users.email,
        concat(onboard_individual_clients.first_name, ' ', onboard_individual_clients.last_name) AS name,
        'client'::text AS user_role,
        'individual'::text AS user_type,
        (SELECT COUNT(*) FROM contracts WHERE contracts.client_ulid = users.ulid) AS contract_count
    FROM 
        onboard_individual_clients
    JOIN 
        users ON users.ulid = onboard_individual_clients.ulid
), client_entity_info AS (
    SELECT 
        users.created_at,
        users.ulid,
        users.email,
        onboard_entity_clients.company_name AS name,
        'client'::text AS user_role,
        'entity'::text AS user_type,
        (SELECT COUNT(*) FROM contracts WHERE contracts.client_ulid = users.ulid) AS contract_count
    FROM 
        onboard_entity_clients
    JOIN 
        users ON users.ulid = onboard_entity_clients.ulid
), contractor_individual_info AS (
    SELECT 
        users.created_at,
        users.ulid,
        users.email,
        concat(onboard_individual_contractors.first_name, ' ', onboard_individual_contractors.last_name) AS name,
        'contractor'::text AS user_role,
        'individual'::text AS user_type,
        (SELECT COUNT(*) FROM contracts WHERE contracts.contractor_ulid = users.ulid) AS contract_count
    FROM 
        onboard_individual_contractors
    JOIN 
        users ON users.ulid = onboard_individual_contractors.ulid
), contractor_entity_info AS (
    SELECT 
        users.created_at,
        users.ulid,
        users.email,
        onboard_entity_contractors.company_name AS name,
        'contractor'::text AS user_role,
        'entity'::text AS user_type,
        (SELECT COUNT(*) FROM contracts WHERE contracts.contractor_ulid = users.ulid) AS contract_count
    FROM 
        onboard_entity_contractors
    JOIN 
        users ON users.ulid = onboard_entity_contractors.ulid
)
SELECT
    client_individual_info.created_at,
    client_individual_info.ulid,
    client_individual_info.name,
    client_individual_info.email,
    client_individual_info.user_role,
    client_individual_info.user_type,
    client_individual_info.contract_count
FROM 
    client_individual_info
UNION
SELECT 
    client_entity_info.created_at,
    client_entity_info.ulid,
    client_entity_info.name,
    client_entity_info.email,
    client_entity_info.user_role,
    client_entity_info.user_type,
    client_entity_info.contract_count
FROM 
    client_entity_info
UNION
SELECT 
    contractor_individual_info.created_at,
    contractor_individual_info.ulid,
    contractor_individual_info.name,
    contractor_individual_info.email,
    contractor_individual_info.user_role,
    contractor_individual_info.user_type,
    contractor_individual_info.contract_count
FROM 
    contractor_individual_info
UNION
SELECT 
    contractor_entity_info.created_at,
    contractor_entity_info.ulid,
    contractor_entity_info.name,
    contractor_entity_info.email,
    contractor_entity_info.user_role,
    contractor_entity_info.user_type,
    contractor_entity_info.contract_count
FROM 
    contractor_entity_info;