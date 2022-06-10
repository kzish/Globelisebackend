--
-- Name: entity_client_branches; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.entity_client_branches (
    ulid uuid NOT NULL PRIMARY KEY,
    client_ulid uuid NOT NULL REFERENCES public.users(ulid)
);


ALTER TABLE public.entity_client_branches OWNER TO postgres;
