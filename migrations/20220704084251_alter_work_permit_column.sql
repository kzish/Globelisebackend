ALTER TABLE IF EXISTS public.individual_contractor_account_details
DROP COLUMN work_permit;

ALTER TABLE IF EXISTS public.individual_contractor_account_details
ADD COLUMN work_permit TEXT;