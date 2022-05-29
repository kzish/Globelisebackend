--
-- Name: entity_clients_branch_bank_details; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.entity_clients_branch_bank_details (
    ulid uuid NOT NULL PRIMARY KEY REFERENCES public.users(ulid),
    currency TEXT NOT NULL REFERENCES public.currency_codes(code),
    bank_name text NOT NULL,
    bank_account_name text NOT NULL,
    bank_account_number text NOT NULL,
    swift_code text,
    bank_key text,
    iban text,
    bank_code text,
    branch_code text,
    created_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL
);

ALTER TABLE public.entity_clients_branch_bank_details OWNER TO postgres;
