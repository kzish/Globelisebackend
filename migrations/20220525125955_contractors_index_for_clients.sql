--
-- Name: contractors_index_for_clients; Type: VIEW; Schema: public; Owner: postgres
--

CREATE VIEW public.contractors_index_for_clients AS
 SELECT a.client_ulid,
    b.name AS client_name,
    a.contractor_ulid,
    c.name AS contractor_name,
    d.contract_name,
    d.contract_status,
    d.job_title,
    d.seniority
   FROM (((public.client_contractor_pairs a
     JOIN public.client_index b ON ((a.client_ulid = b.ulid)))
     JOIN public.contractor_index c ON ((a.contractor_ulid = c.ulid)))
     JOIN public.contracts d ON (((a.client_ulid = d.client_ulid) AND (a.contractor_ulid = d.contractor_ulid))));


ALTER TABLE public.contractors_index_for_clients OWNER TO postgres;
