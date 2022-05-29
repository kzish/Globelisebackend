--
-- Name: sap_mulesoft_payroll_journals_rows; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.sap_mulesoft_payroll_journals_rows (
    ulid uuid NOT NULL PRIMARY KEY,
    entry_ulid uuid NOT NULL REFERENCES public.sap_mulesoft_payroll_journals_entries(ulid),
    posting_date text NOT NULL,
    doc_type text NOT NULL,
    company_code text NOT NULL,
    currency_code text NOT NULL REFERENCES public.currency_codes(code),
    reference text NOT NULL,
    debit_credit_code text NOT NULL,
    document_header_text text NOT NULL,
    gl_account text NOT NULL,
    amount real NOT NULL,
    cost_center text,
    uploaded boolean DEFAULT false NOT NULL,
    created_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL
);


ALTER TABLE public.sap_mulesoft_payroll_journals_rows OWNER TO postgres;
