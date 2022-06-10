--
-- Name: individual_contractors_account_details; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.individual_contractors_account_details (
    ulid uuid NOT NULL PRIMARY KEY REFERENCES public.users(ulid),
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
    profile_picture bytea,
    cv bytea,
    created_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL
);


ALTER TABLE public.individual_contractors_account_details OWNER TO postgres;
