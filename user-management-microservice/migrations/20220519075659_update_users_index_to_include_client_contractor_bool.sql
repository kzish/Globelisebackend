CREATE
OR REPLACE VIEW users_index AS WITH client_individual_info AS (
    SELECT
        a.created_at,
        a.ulid,
        a.password,
        a.email,
        a.is_google,
        a.is_outlook,
        CASE
            WHEN b.ulid IS NULL THEN 'f'
            ELSE 't'
        END AS is_client,
        CASE
            WHEN b.ulid IS NULL THEN 'f'
            ELSE 't'
        END AS is_contractor
    FROM
        auth_individuals a
        LEFT JOIN individual_clients_fully_onboarded b ON a.ulid = b.ulid
        LEFT JOIN individual_contractors_fully_onboarded c ON a.ulid = c.ulid
),
client_entity_info AS (
    SELECT
        a.created_at,
        a.ulid,
        a.password,
        a.email,
        a.is_google,
        a.is_outlook,
        CASE
            WHEN b.ulid IS NULL THEN 'f'
            ELSE 't'
        END AS is_client,
        CASE
            WHEN b.ulid IS NULL THEN 'f'
            ELSE 't'
        END AS is_contractor
    FROM
        auth_entities a
        LEFT JOIN entity_clients_fully_onboarded b ON a.ulid = b.ulid
        LEFT JOIN entity_contractors_fully_onboarded c ON a.ulid = c.ulid
)
SELECT
    client_individual_info.created_at,
    client_individual_info.ulid,
    client_individual_info.password,
    client_individual_info.email,
    client_individual_info.is_google,
    client_individual_info.is_outlook,
    client_individual_info.is_client,
    client_individual_info.is_contractor,
    'individual' AS user_type
FROM
    client_individual_info
UNION
SELECT
    client_entity_info.created_at,
    client_entity_info.ulid,
    client_entity_info.password,
    client_entity_info.email,
    client_entity_info.is_google,
    client_entity_info.is_outlook,
    client_entity_info.is_client,
    client_entity_info.is_contractor,
    'entity' AS user_type
FROM
    client_entity_info;