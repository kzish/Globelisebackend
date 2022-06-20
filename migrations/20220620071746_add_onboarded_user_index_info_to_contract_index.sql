CREATE OR REPLACE VIEW public.clients_index_for_contractors
AS
SELECT DISTINCT ON (client_ulid)
    b.name,
    b.email,
    b.user_role,
    b.user_type,
    b.contract_count,
    b.created_at,
    a.client_ulid AS ulid,
    a.contractor_ulid
FROM
    contracts a
LEFT JOIN onboarded_user_index b ON a.client_ulid = b.ulid;

CREATE OR REPLACE VIEW public.contractors_index_for_clients
AS
SELECT DISTINCT ON (contractor_ulid)
    b.name,
    b.email,
    b.user_role,
    b.user_type,
    b.contract_count,
    b.created_at,
    a.client_ulid,
    a.contractor_ulid AS ulid
FROM
    contracts a
LEFT JOIN onboarded_user_index b ON a.contractor_ulid = b.ulid;
