--
-- Name: prefilled_individual_contractors_bank_details; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.prefilled_individual_contractors_bank_details (
    email TEXT NOT NULL,
    client_ulid uuid NOT NULL REFERENCES public.users(ulid),
    bank_name text NOT NULL,
    bank_account_name text NOT NULL,
    bank_account_number text NOT NULL,
    created_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    PRIMARY KEY (email, client_ulid)
);


ALTER TABLE public.prefilled_individual_contractors_bank_details OWNER TO postgres;
