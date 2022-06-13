ALTER TABLE ONLY public.prefilled_entity_client_bank_details
ADD COLUMN bank_code TEXT,
ADD COLUMN branch_code TEXT;

UPDATE public.prefilled_entity_client_bank_details
SET bank_code = '', branch_code = '';

ALTER TABLE public.prefilled_entity_client_bank_details
ALTER COLUMN bank_code SET NOT NULL,
ALTER COLUMN branch_code SET NOT NULL;
