--
-- Name: prefilled_entity_clients_bank_details; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.prefilled_entity_clients_bank_details (
    email text NOT NULL PRIMARY KEY,
    bank_name text NOT NULL,
    bank_account_name text NOT NULL,
    bank_account_number text NOT NULL,
    created_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL
);


ALTER TABLE public.prefilled_entity_clients_bank_details OWNER TO postgres;
