--
-- Name: sap_mulesoft_payroll_journals_entries; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.sap_mulesoft_payroll_journals_entries (
    ulid uuid NOT NULL PRIMARY KEY,
    country_code text NOT NULL REFERENCES public.country_codes(code),
    created_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL
);


ALTER TABLE public.sap_mulesoft_payroll_journals_entries OWNER TO postgres;
