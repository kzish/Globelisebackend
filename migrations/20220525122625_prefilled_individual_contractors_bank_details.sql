--
-- Name: prefilled_individual_contractors_bank_details; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.prefilled_individual_contractors_bank_details (
    email TEXT NOT NULL PRIMARY KEY,
    client_ulid uuid REFERENCES public.users(ulid),
    bank_name text NOT NULL,
    bank_account_name text NOT NULL,
    bank_account_number text NOT NULL,
    created_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL
);


ALTER TABLE public.prefilled_individual_contractors_bank_details OWNER TO postgres;


--
-- Name: prefilled_individual_contractors_bank_details_by_admin; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.prefilled_individual_contractors_bank_details_by_admin (
    email text NOT NULL PRIMARY KEY,
    bank_name text NOT NULL,
    bank_account_name text NOT NULL,
    bank_account_number text NOT NULL,
    created_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL
);


ALTER TABLE public.prefilled_individual_contractors_bank_details_by_admin OWNER TO postgres;