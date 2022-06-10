
--
-- Name: individual_contractors_bank_details; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.individual_contractors_bank_details (
    ulid uuid NOT NULL PRIMARY KEY REFERENCES public.users(ulid),
    bank_name text NOT NULL,
    bank_account_name text NOT NULL,
    bank_account_number text NOT NULL,
    created_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL
);


ALTER TABLE public.individual_contractors_bank_details OWNER TO postgres;
