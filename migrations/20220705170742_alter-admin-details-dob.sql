-- Add migration script here
ALTER TABLE IF EXISTS public.onboard_eor_admins
ALTER COLUMN dob TYPE timestamp with time zone;