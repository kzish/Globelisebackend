--
-- Name: users; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.users (
    ulid uuid NOT NULL PRIMARY KEY,
    email text NOT NULL UNIQUE,
    password text,
    is_google boolean NOT NULL DEFAULT 'f',
    is_outlook boolean NOT NULL DEFAULT 'f',
    is_entity boolean NOT NULL DEFAULT 'f',
    is_individual boolean NOT NULL DEFAULT 'f',
    is_client boolean NOT NULL DEFAULT 'f',
    is_contractor boolean NOT NULL DEFAULT 'f',
    created_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL
);


ALTER TABLE public.users OWNER TO postgres;
