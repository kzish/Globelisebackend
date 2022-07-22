ALTER TABLE IF EXISTS public.uploaded_citibank_transfer_initiation_files_records
    ADD COLUMN bank_branch_code text COLLATE pg_catalog."default" NOT NULL;