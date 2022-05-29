--
-- Name: users; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.admin_users (
    ulid uuid NOT NULL PRIMARY KEY,
    email text NOT NULL UNIQUE,
    password text,
    is_google boolean NOT NULL DEFAULT 'f',
    is_outlook boolean NOT NULL DEFAULT 'f',
    created_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL
);


ALTER TABLE public.users OWNER TO postgres;


--
-- Name: onboard_eor_admins; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.onboard_eor_admins (
    ulid uuid NOT NULL PRIMARY KEY REFERENCES public.admin_users(ulid),
    first_name text NOT NULL,
    last_name text NOT NULL,
    dob date NOT NULL,
    dial_code text NOT NULL,
    phone_number text NOT NULL,
    country text NOT NULL REFERENCES public.country_codes(code),
    city text NOT NULL,
    address text NOT NULL,
    postal_code text NOT NULL,
    tax_id text,
    time_zone text NOT NULL,
    profile_picture bytea,
    created_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL
);


ALTER TABLE public.onboard_eor_admins OWNER TO postgres;