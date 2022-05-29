--
-- Name: entity_clients_branch_account_details; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.entity_clients_branch_account_details (
    ulid uuid NOT NULL PRIMARY KEY REFERENCES public.users(ulid),
    branch_name text NOT NULL,
    country text NOT NULL REFERENCES public.country_codes(code),
    entity_type text NOT NULL,
    registration_number text,
    tax_id text,
    statutory_contribution_submission_number text,
    company_address text NOT NULL,
    city text NOT NULL,
    postal_code text NOT NULL,
    time_zone text NOT NULL,
    logo bytea,
    created_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL
);


ALTER TABLE public.entity_clients_branch_account_details OWNER TO postgres;
