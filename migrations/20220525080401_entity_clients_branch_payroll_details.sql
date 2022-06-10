--
-- Name: entity_clients_branch_payroll_details; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.entity_clients_branch_payroll_details (
    ulid uuid NOT NULL PRIMARY KEY REFERENCES public.entity_client_branches(ulid),
    cutoff_date timestamp with time zone NOT NULL,
    payment_date timestamp with time zone NOT NULL,
    created_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL
);


ALTER TABLE public.entity_clients_branch_payroll_details OWNER TO postgres;
