--
-- Name: entity_client_branch_departments_index; Type: VIEW; Schema: public; Owner: postgres
--

CREATE VIEW public.entity_client_branch_departments_index AS
 WITH department_individual_contractor_count AS (
         SELECT individual_contractor_department_pairs.department_ulid,
            count(*) AS c
           FROM public.individual_contractor_department_pairs
          GROUP BY individual_contractor_department_pairs.department_ulid
        ), department_entity_contractor_count AS (
         SELECT entity_contractor_department_pairs.department_ulid,
            count(*) AS c
           FROM public.entity_contractor_department_pairs
          GROUP BY entity_contractor_department_pairs.department_ulid
        )
 SELECT a.ulid,
    a.branch_ulid,
    e.branch_name,
    a.department_name,
    a.country,
    a.classification,
    a.currency,
    (COALESCE(b.c, (0)::bigint) + COALESCE(c.c, (0)::bigint)) AS total_member,
    d.client_ulid
   FROM ((((public.entity_client_branch_departments a
     LEFT JOIN department_individual_contractor_count b ON ((a.ulid = b.department_ulid)))
     LEFT JOIN department_entity_contractor_count c ON ((a.ulid = c.department_ulid)))
     JOIN public.entity_client_branches d ON ((a.branch_ulid = d.ulid)))
     JOIN public.entity_clients_branch_details e ON ((a.branch_ulid = d.ulid)));


ALTER TABLE public.entity_client_branch_departments_index OWNER TO postgres;
