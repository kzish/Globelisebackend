--
-- Name: sap_mulesoft_payroll_journal_company_codes; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.sap_mulesoft_payroll_journal_company_codes (
    code text NOT NULL PRIMARY KEY,
    country_code text NOT NULL REFERENCES public.country_codes(code)
);


ALTER TABLE public.sap_mulesoft_payroll_journal_company_codes OWNER TO postgres;

INSERT INTO
    sap_mulesoft_payroll_journal_company_codes (code, country_code)
VALUES
    ('SG01', 'SG'),
    ('VN01', 'VN');
