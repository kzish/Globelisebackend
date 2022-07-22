-- Add migration script here

ALTER TABLE public.payslips
    ADD COLUMN payslip_file_name TEXT;
