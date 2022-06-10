--
-- Name: entity_client_branch_individual_contractor_pairs; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.entity_client_branch_individual_contractor_pairs (
    contractor_ulid uuid NOT NULL REFERENCES public.users(ulid),
    branch_ulid uuid NOT NULL REFERENCES public.entity_client_branches(ulid),
    created_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL
);


ALTER TABLE public.entity_client_branch_individual_contractor_pairs OWNER TO postgres;
