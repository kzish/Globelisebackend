--
-- Name: prefilled_entity_clients_account_details; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.prefilled_entity_clients_account_details (
    email text NOT NULL PRIMARY KEY,
    company_name text NOT NULL,
    country text NOT NULL,
    entity_type text NOT NULL,
    registration_number text,
    tax_id text,
    company_address text NOT NULL,
    city text NOT NULL,
    postal_code text NOT NULL,
    time_zone text NOT NULL,
    logo bytea,
    created_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL
);


ALTER TABLE public.prefilled_entity_clients_account_details OWNER TO postgres;