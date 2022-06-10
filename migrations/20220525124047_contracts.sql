--
-- Name: contracts; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.contracts (
    ulid uuid NOT NULL PRIMARY KEY,
    client_ulid uuid NOT NULL REFERENCES public.users(ulid),
    contractor_ulid uuid NOT NULL REFERENCES public.users(ulid),
    contract_name text NOT NULL,
    contract_type text NOT NULL,
    contract_status text NOT NULL,
    contract_amount numeric NOT NULL,
    currency text NOT NULL REFERENCES public.currency_codes(code),
    job_title text NOT NULL,
    seniority text NOT NULL,
    begin_at timestamp with time zone NOT NULL,
    end_at timestamp with time zone NOT NULL,
    branch_ulid uuid REFERENCES public.entity_client_branches(ulid),
    created_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    CONSTRAINT contracts_begin_at_end_at_check CHECK ((begin_at <= end_at)),
    CONSTRAINT contracts_contract_amount_check CHECK ((contract_amount >= (0)::numeric))
);


ALTER TABLE public.contracts OWNER TO postgres;