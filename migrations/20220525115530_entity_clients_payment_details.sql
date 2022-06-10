--
-- Name: entity_clients_payment_details; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.entity_clients_payment_details (
    ulid uuid NOT NULL PRIMARY KEY,
    currency TEXT NOT NULL REFERENCES public.currency_codes(code),
    payment_date timestamp with time zone NOT NULL,
    cutoff_date timestamp with time zone NOT NULL,
    created_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL
);


ALTER TABLE public.entity_clients_payment_details OWNER TO postgres;
