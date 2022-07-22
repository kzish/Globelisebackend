
ALTER TABLE IF EXISTS public.contracts
ADD COLUMN client_signature text COLLATE pg_catalog."default";

ALTER TABLE IF EXISTS public.contracts
ADD COLUMN contractor_signature text COLLATE pg_catalog."default";

ALTER TABLE IF EXISTS public.contracts
ADD COLUMN client_date_signed timestamp with time zone;

ALTER TABLE IF EXISTS public.contracts
ADD COLUMN contractor_date_signed timestamp with time zone;