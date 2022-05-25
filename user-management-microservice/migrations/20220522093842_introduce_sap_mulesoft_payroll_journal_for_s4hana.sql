CREATE TABLE sap_mulesoft_payroll_journals_entries (
    ulid UUID NOT NULL PRIMARY KEY,
    country_code TEXT NOT NULL REFERENCES country_codes(code),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL
);

CREATE TRIGGER mdt_sap_mulesoft_payroll_journals_entries BEFORE
UPDATE
    ON sap_mulesoft_payroll_journals_entries FOR EACH ROW EXECUTE FUNCTION public.moddatetime('updated_at');

CREATE TABLE sap_mulesoft_payroll_journals_rows (
    ulid UUID NOT NULL PRIMARY KEY DEFAULT gen_random_uuid(),
    entry_ulid UUID NOT NULL REFERENCES sap_mulesoft_payroll_journals_entries(ulid),
    posting_date TEXT NOT NULL,
    doc_type TEXT NOT NULL,
    company_code TEXT NOT NULL,
    currency_code TEXT REFERENCES currency_codes(code),
    reference TEXT NOT NULL,
    debit_credit_code TEXT NOT NULL,
    document_header_text TEXT NOT NULL,
    gl_account TEXT NOT NULL,
    amount REAL NOT NULL,
    cost_center TEXT,
    uploaded BOOLEAN NOT NULL DEFAULT 'f',
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL
);

CREATE TRIGGER mdt_sap_mulesoft_payroll_journals_rows BEFORE
UPDATE
    ON sap_mulesoft_payroll_journals_rows FOR EACH ROW EXECUTE FUNCTION public.moddatetime('updated_at');

CREATE TABLE sap_mulesoft_payroll_journal_countries (
    country_code TEXT PRIMARY KEY REFERENCES country_codes(code)
);

INSERT INTO
    sap_mulesoft_payroll_journal_countries (country_code)
VALUES
    ('SG'),
    ('VN');

CREATE TABLE sap_mulesoft_payroll_journal_company_codes (
    code TEXT NOT NULL PRIMARY KEY,
    country_code TEXT NOT NULL REFERENCES country_codes(code)
);

INSERT INTO
    sap_mulesoft_payroll_journal_company_codes (code, country_code)
VALUES
    ('SG01', 'SG'),
    ('VN01', 'VN');

CREATE TABLE sap_mulesoft_payroll_journal_cost_centers (
    code TEXT NOT NULL PRIMARY KEY,
    company_code TEXT NOT NULL REFERENCES sap_mulesoft_payroll_journal_company_codes(code),
    long_name TEXT NOT NULL
);

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