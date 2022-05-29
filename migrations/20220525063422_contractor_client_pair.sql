--
-- Name: client_contractor_pairs; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.client_contractor_pairs (
    client_ulid uuid NOT NULL REFERENCES public.users(ulid),
    contractor_ulid uuid NOT NULL REFERENCES public.users(ulid),
    created_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL
);


ALTER TABLE public.client_contractor_pairs OWNER TO postgres;
