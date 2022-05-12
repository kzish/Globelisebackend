CREATE OR REPLACE VIEW public.payslips_index AS
 SELECT payslips.ulid AS payslip_ulid,
    payslips.client_ulid,
    COALESCE(client_names.name, '') AS client_name,
    payslips.contractor_ulid,
    COALESCE(contractor_names.name, '') AS contractor_name,
    payslips.contract_ulid,
    contracts.contract_name,
    payslips.payslip_title,
    payslips.payment_date,
    payslips.begin_period,
    payslips.end_period,
    payslips.payslip_file
   FROM (((public.payslips
     LEFT JOIN public.client_names ON ((payslips.client_ulid = client_names.ulid)))
     LEFT JOIN public.contractor_names ON ((payslips.contractor_ulid = contractor_names.ulid)))
     LEFT JOIN public.contracts ON ((payslips.contract_ulid = contracts.ulid)));