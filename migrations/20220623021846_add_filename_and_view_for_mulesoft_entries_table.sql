-- Add file_name column

ALTER TABLE public.sap_mulesoft_payroll_journals_entries
ADD COLUMN file_name TEXT;

UPDATE public.sap_mulesoft_payroll_journals_entries
SET
    file_name = ''
WHERE
    file_name IS NULL;

ALTER TABLE public.sap_mulesoft_payroll_journals_entries
ALTER COLUMN file_name SET NOT NULL;

-- Make uploaded_file NOT NULL

UPDATE public.sap_mulesoft_payroll_journals_entries
SET
    uploaded_file = ''::BYTEA
WHERE
    uploaded_file IS NULL;

ALTER TABLE public.sap_mulesoft_payroll_journals_entries
ALTER COLUMN uploaded_file SET NOT NULL;

-- Make amount NOT NULL

UPDATE public.sap_mulesoft_payroll_journals_rows
SET
    amount = 0
WHERE
    amount IS NULL;

ALTER TABLE public.sap_mulesoft_payroll_journals_rows
ALTER COLUMN amount SET NOT NULL;

-- Create sap_mulesoft_payroll_journals_entry_index VIEW

CREATE OR REPLACE VIEW public.sap_mulesoft_payroll_journals_entry_index AS
    SELECT
        ulid,
        country_code,
        created_at,
        client_ulid,
        uploaded_file,
        file_name,
        (SELECT COUNT(*) FROM public.sap_mulesoft_payroll_journals_rows b WHERE b.entry_ulid = a.ulid) AS row_count
    FROM
        sap_mulesoft_payroll_journals_entries a;

