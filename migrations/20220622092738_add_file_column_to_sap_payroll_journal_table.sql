ALTER TABLE sap_mulesoft_payroll_journals_entries
ADD COLUMN uploaded_file BYTEA;

ALTER TABLE sap_mulesoft_payroll_journals_rows
ALTER COLUMN amount TYPE NUMERIC;
