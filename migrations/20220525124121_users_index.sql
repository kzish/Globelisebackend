--
-- Name: users_index; Type: VIEW; Schema: public; Owner: postgres
--

CREATE VIEW public.users_index AS
 WITH client_individual_info AS (
         SELECT a.created_at,
            a.ulid,
            a.password,
            a.email,
            a.is_google,
            a.is_outlook,
            a.is_entity,
            a.is_individual
           FROM ((public.users a
             LEFT JOIN public.individual_clients_fully_onboarded b ON ((a.ulid = b.ulid)))
             LEFT JOIN public.individual_contractors_fully_onboarded c ON ((a.ulid = c.ulid)))
            WHERE
                a.is_individual = 't'
        ), client_entity_info AS (
         SELECT a.created_at,
            a.ulid,
            a.password,
            a.email,
            a.is_google,
            a.is_outlook,
            a.is_entity,
            a.is_individual
           FROM ((public.users a
             LEFT JOIN public.entity_clients_fully_onboarded b ON ((a.ulid = b.ulid)))
             LEFT JOIN public.entity_contractors_fully_onboarded c ON ((a.ulid = c.ulid)))
            WHERE
                a.is_entity = 't'
        )
 SELECT client_individual_info.created_at,
    client_individual_info.ulid,
    client_individual_info.password,
    client_individual_info.email,
    client_individual_info.is_google,
    client_individual_info.is_outlook,
    'individual'::text AS user_type
   FROM client_individual_info
UNION
 SELECT client_entity_info.created_at,
    client_entity_info.ulid,
    client_entity_info.password,
    client_entity_info.email,
    client_entity_info.is_google,
    client_entity_info.is_outlook,
    'entity'::text AS user_type
   FROM client_entity_info;


ALTER TABLE public.users_index OWNER TO postgres;
