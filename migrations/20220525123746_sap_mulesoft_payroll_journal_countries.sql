--
-- Name: sap_mulesoft_payroll_journal_countries; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.sap_mulesoft_payroll_journal_countries (
    country_code text NOT NULL REFERENCES public.country_codes(code)
);


ALTER TABLE public.sap_mulesoft_payroll_journal_countries OWNER TO postgres;

INSERT INTO public.sap_mulesoft_payroll_journal_countries (country_code) VALUES ('SG');
INSERT INTO public.sap_mulesoft_payroll_journal_countries (country_code) VALUES ('VN');
