--
-- Name: clients_index_for_contractors; Type: VIEW; Schema: public; Owner: postgres
--

CREATE VIEW public.clients_index_for_contractors AS
 SELECT a.ulid AS contract_ulid,
    a.client_ulid,
    a.branch_ulid,
    COALESCE(b.name, ''::text) AS client_name,
    a.contractor_ulid,
    COALESCE(c.name, ''::text) AS contractor_name,
    a.contract_name,
    a.contract_type,
    a.contract_status,
    a.contract_amount,
    a.currency,
    a.begin_at,
    a.end_at,
    a.job_title,
    a.seniority
   FROM ((public.contracts a
     LEFT JOIN public.client_index b ON ((a.client_ulid = b.ulid)))
     LEFT JOIN public.contractor_index c ON ((a.contractor_ulid = c.ulid)));


ALTER TABLE public.clients_index_for_contractors OWNER TO postgres;