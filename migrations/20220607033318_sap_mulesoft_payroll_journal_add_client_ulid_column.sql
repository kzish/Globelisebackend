ALTER TABLE 
    public.sap_mulesoft_payroll_journals_entries 
ADD COLUMN 
    client_ulid uuid NOT NULL REFERENCES public.users(ulid);