DROP VIEW public.payslips_index;

CREATE OR REPLACE VIEW public.payslips_index AS
 SELECT payslips.ulid AS payslip_ulid,
    payslips.client_ulid,
    COALESCE(client_index.name, ''::text) AS client_name,
    payslips.contractor_ulid,
    COALESCE(contractor_index.name, ''::text) AS contractor_name,
    payslips.contract_ulid,
    contracts.contract_name,
    payslips.payslip_title,
    payslips.payment_date,
    payslips.begin_period,
    payslips.end_period
   FROM (((public.payslips
     LEFT JOIN public.client_index ON ((payslips.client_ulid = client_index.ulid)))
     LEFT JOIN public.contractor_index ON ((payslips.contractor_ulid = contractor_index.ulid)))
     LEFT JOIN public.contracts ON ((payslips.contract_ulid = contracts.ulid)));
