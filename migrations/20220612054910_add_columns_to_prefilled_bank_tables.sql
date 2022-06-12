ALTER TABLE ONLY public.prefilled_individual_contractor_bank_details
ADD COLUMN bank_code TEXT NOT NULL,
ADD COLUMN branch_code TEXT NOT NULL;
