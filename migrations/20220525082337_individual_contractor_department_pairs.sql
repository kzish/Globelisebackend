--
-- Name: individual_contractor_department_pairs; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.individual_contractor_department_pairs (
    individual_ulid uuid NOT NULL REFERENCES public.users(ulid),
    department_ulid uuid NOT NULL REFERENCES public.entity_client_branch_departments(ulid),
    created_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL
);


ALTER TABLE public.individual_contractor_department_pairs OWNER TO postgres;