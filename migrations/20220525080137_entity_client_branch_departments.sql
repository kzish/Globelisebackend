--
-- Name: entity_client_branch_departments; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.entity_client_branch_departments (
    ulid uuid NOT NULL PRIMARY KEY,
    branch_ulid uuid NOT NULL REFERENCES public.entity_client_branches(ulid),
    department_name text NOT NULL,
    country text NOT NULL REFERENCES public.country_codes(code),
    classification text NOT NULL,
    currency TEXT NOT NULL REFERENCES public.currency_codes(code),
    created_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL
);


ALTER TABLE public.entity_client_branch_departments OWNER TO postgres;
