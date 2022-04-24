CREATE OR REPLACE VIEW public.entity_clients_onboarded AS
 SELECT entity_clients_account_details.ulid
   FROM ((public.entity_clients_account_details
     JOIN public.entity_clients_payment_details ON ((entity_clients_account_details.ulid = entity_clients_payment_details.ulid)))
     JOIN public.entity_clients_pic_details ON ((entity_clients_account_details.ulid = entity_clients_pic_details.ulid)));

CREATE OR REPLACE VIEW public.onboarded_user_index AS
 WITH client_individual_info AS (
         SELECT auth_individuals.created_at,
            auth_individuals.ulid,
            auth_individuals.email,
            concat(onboard_individual_clients.first_name, ' ', onboard_individual_clients.last_name) AS name,
            'client'::text AS user_role,
            'individual'::text AS user_type
           FROM (public.onboard_individual_clients
             JOIN public.auth_individuals ON ((auth_individuals.ulid = onboard_individual_clients.ulid)))
        ), client_entity_info AS (
         SELECT auth_entities.created_at,
            auth_entities.ulid,
            auth_entities.email,
            onboard_entity_clients.company_name AS name,
            'client'::text AS user_role,
            'entity'::text AS user_type
           FROM (public.onboard_entity_clients
             JOIN public.auth_entities ON ((auth_entities.ulid = onboard_entity_clients.ulid)))
        ), contractor_individual_info AS (
         SELECT auth_individuals.created_at,
            auth_individuals.ulid,
            auth_individuals.email,
            concat(onboard_individual_contractors.first_name, ' ', onboard_individual_contractors.last_name) AS name,
            'contractor'::text AS user_role,
            'individual'::text AS user_type
           FROM (public.onboard_individual_contractors
             JOIN public.auth_individuals ON ((auth_individuals.ulid = onboard_individual_contractors.ulid)))
        ), contractor_entity_info AS (
         SELECT auth_entities.created_at,
            auth_entities.ulid,
            auth_entities.email,
            onboard_entity_contractors.company_name AS name,
            'contractor'::text AS user_role,
            'entity'::text AS user_type
           FROM (public.onboard_entity_contractors
             JOIN public.auth_entities ON ((auth_entities.ulid = onboard_entity_contractors.ulid)))
        )
 SELECT client_individual_info.created_at,
    client_individual_info.ulid,
    client_individual_info.name,
    client_individual_info.email,
    client_individual_info.user_role,
    client_individual_info.user_type
   FROM client_individual_info
UNION
 SELECT client_entity_info.created_at,
    client_entity_info.ulid,
    client_entity_info.name,
    client_entity_info.email,
    client_entity_info.user_role,
    client_entity_info.user_type
   FROM client_entity_info
UNION
 SELECT contractor_individual_info.created_at,
    contractor_individual_info.ulid,
    contractor_individual_info.name,
    contractor_individual_info.email,
    contractor_individual_info.user_role,
    contractor_individual_info.user_type
   FROM contractor_individual_info
UNION
 SELECT contractor_entity_info.created_at,
    contractor_entity_info.ulid,
    contractor_entity_info.name,
    contractor_entity_info.email,
    contractor_entity_info.user_role,
    contractor_entity_info.user_type
   FROM contractor_entity_info;
