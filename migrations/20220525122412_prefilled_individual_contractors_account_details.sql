--
-- Name: prefilled_individual_contractors_account_details; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.prefilled_individual_contractors_account_details (
    email TEXT NOT NULL,
    client_ulid uuid REFERENCES public.users(ulid),
    first_name text NOT NULL,
    last_name text NOT NULL,
    dob timestamp with time zone NOT NULL,
    dial_code text NOT NULL,
    phone_number text NOT NULL,
    country text NOT NULL REFERENCES public.country_codes(code),
    city text NOT NULL,
    address text NOT NULL,
    postal_code text NOT NULL,
    tax_id text,
    time_zone text NOT NULL,
    created_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    PRIMARY KEY (email, client_ulid)
);


ALTER TABLE public.prefilled_individual_contractors_account_details OWNER TO postgres;
