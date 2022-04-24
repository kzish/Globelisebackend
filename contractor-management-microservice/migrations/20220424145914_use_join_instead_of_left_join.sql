CREATE OR REPLACE VIEW public.contractors_index AS
 SELECT client_contractor_pairs.client_ulid,
    client_names.name AS client_name,
    client_contractor_pairs.contractor_ulid,
    contractor_names.name AS contractor_name,
    contracts.contract_name,
    contracts.contract_status,
    contracts.job_title,
    contracts.seniority
   FROM (((public.client_contractor_pairs
     JOIN public.client_names ON ((client_contractor_pairs.client_ulid = client_names.ulid)))
     JOIN public.contractor_names ON ((client_contractor_pairs.contractor_ulid = contractor_names.ulid)))
     JOIN public.contracts ON (((client_contractor_pairs.client_ulid = contracts.client_ulid) AND (client_contractor_pairs.contractor_ulid = contracts.contractor_ulid))));

CREATE OR REPLACE VIEW public.invoice_individual_index AS
 WITH total_amount AS (
         SELECT invoice_items.invoice_ulid,
            sum(((invoice_items.item_unit_quantity)::numeric * invoice_items.item_unit_price)) AS invoice_amount
           FROM (public.invoice_individual
             JOIN public.invoice_items ON ((invoice_individual.ulid = invoice_items.invoice_ulid)))
          GROUP BY invoice_items.invoice_ulid
        ), step_1 AS (
         SELECT invoice_individual.ulid,
            invoice_individual.invoice_group_ulid,
            invoice_individual.contractor_ulid,
            invoice_individual.client_ulid,
            invoice_individual.invoice_id,
            invoice_group.invoice_name,
            invoice_group.invoice_due,
            invoice_group.invoice_status
           FROM (public.invoice_group
             JOIN public.invoice_individual ON ((invoice_group.ulid = invoice_individual.invoice_group_ulid)))
        ), step_2 AS (
         SELECT step_1.ulid,
            step_1.invoice_group_ulid,
            step_1.contractor_ulid,
            step_1.client_ulid,
            step_1.invoice_id,
            step_1.invoice_name,
            step_1.invoice_due,
            step_1.invoice_status,
            COALESCE(total_amount.invoice_amount, (0)::numeric) AS invoice_amount
           FROM (step_1
             JOIN total_amount ON ((step_1.ulid = total_amount.invoice_ulid)))
        )
 SELECT step_2.ulid,
    step_2.invoice_group_ulid,
    step_2.contractor_ulid,
    step_2.client_ulid,
    step_2.invoice_id,
    step_2.invoice_name,
    step_2.invoice_due,
    step_2.invoice_status,
    step_2.invoice_amount
   FROM step_2;

CREATE OR REPLACE VIEW public.payslips_index AS
 SELECT payslips.ulid AS payslip_ulid,
    payslips.client_ulid,
    client_names.name AS client_name,
    payslips.contractor_ulid,
    contractor_names.name AS contractor_name,
    payslips.contract_ulid,
    contracts.contract_name,
    payslips.payslip_title,
    payslips.payment_date,
    payslips.begin_period,
    payslips.end_period,
    payslips.payslip_file
   FROM (((public.payslips
     JOIN public.client_names ON ((payslips.client_ulid = client_names.ulid)))
     JOIN public.contractor_names ON ((payslips.contractor_ulid = contractor_names.ulid)))
     JOIN public.contracts ON ((payslips.contract_ulid = contracts.ulid)));

CREATE OR REPLACE VIEW public.tax_reports_index AS
 SELECT tax_reports.ulid AS tax_report_ulid,
    tax_reports.client_ulid,
    client_names.name AS client_name,
    tax_reports.contractor_ulid,
    contractor_names.name AS contractor_name,
    contracts.contract_name,
    tax_reports.tax_interval,
    tax_reports.tax_name,
    tax_reports.begin_period,
    tax_reports.end_period,
    tax_reports.country,
    tax_reports.tax_report_file
   FROM (((public.tax_reports
     JOIN public.client_names ON ((tax_reports.client_ulid = client_names.ulid)))
     JOIN public.contractor_names ON ((tax_reports.contractor_ulid = contractor_names.ulid)))
     JOIN public.contracts ON ((tax_reports.contract_ulid = contracts.ulid)));
