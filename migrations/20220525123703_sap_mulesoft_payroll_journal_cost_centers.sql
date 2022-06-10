--
-- Name: sap_mulesoft_payroll_journal_cost_centers; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.sap_mulesoft_payroll_journal_cost_centers (
    code text NOT NULL PRIMARY KEY,
    company_code text NOT NULL REFERENCES public.sap_mulesoft_payroll_journal_company_codes(code),
    long_name text NOT NULL
);


ALTER TABLE public.sap_mulesoft_payroll_journal_cost_centers OWNER TO postgres;

INSERT INTO
    sap_mulesoft_payroll_journal_cost_centers (code, company_code, long_name)
VALUES
    --- Singapore
    ('SG00010000', 'SG01', 'General Management'),
    ('SG00020000', 'SG01', 'Finance and Accounting'),
    ('SG00030000', 'SG01', 'Marketing'),
    ('SG00040000', 'SG01', 'Human Resources'),
    ('SG00050000', 'SG01', 'Supply Chain'),
    ('SG00060000', 'SG01', 'Information Technology'),
    ('SG00070000', 'SG01', 'Research and Development'),
    --- Vietnam
    ('VN00010000', 'VN01', 'General Management'),
    ('VN00020000', 'VN01', 'Finance and Accounting'),
    ('VN00030000', 'VN01', 'Marketing'),
    ('VN00040000', 'VN01', 'Human Resources'),
    ('VN00050000', 'VN01', 'Supply Chain'),
    ('VN00060000', 'VN01', 'Information Technology'),
    ('VN00070000', 'VN01', 'Research and Development'),
    ('VN01010000', 'VN01', 'General Management'),
    ('VN01020000', 'VN01', 'Finance and Accounting'),
    ('VN01030000', 'VN01', 'Marketing'),
    ('VN01040000', 'VN01', 'Human Resources'),
    ('VN01050000', 'VN01', 'Supply Chain'),
    ('VN01060000', 'VN01', 'Information Technology'),
    ('VN01070000', 'VN01', 'Research and Development');
