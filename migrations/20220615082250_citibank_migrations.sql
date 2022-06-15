CREATE OR REPLACE VIEW public.search_client_branches
 AS
 SELECT entity_client_branches.ulid,
    entity_client_branches.client_ulid,
    entity_client_branch_departments.department_name,
    entity_client_branch_departments.country,
    entity_client_branch_departments.currency
   FROM entity_client_branches
     JOIN entity_client_branch_departments ON entity_client_branches.ulid = entity_client_branch_departments.branch_ulid;

ALTER TABLE public.search_client_branches
    OWNER TO postgres;

