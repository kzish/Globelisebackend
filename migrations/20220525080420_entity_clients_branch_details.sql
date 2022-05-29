--
-- Name: entity_clients_branch_details; Type: VIEW; Schema: public; Owner: postgres
--

CREATE VIEW public.entity_clients_branch_details AS
 SELECT a.client_ulid,
    b.ulid,
    b.branch_name,
    b.country,
    b.entity_type,
    b.registration_number,
    b.tax_id,
    b.statutory_contribution_submission_number,
    b.company_address,
    b.city,
    b.postal_code,
    b.time_zone,
    b.logo,
    c.currency,
    c.bank_name,
    c.bank_account_name,
    c.bank_account_number,
    c.swift_code,
    c.bank_key,
    c.iban,
    c.bank_code,
    c.branch_code,
    d.cutoff_date,
    d.payment_date
   FROM (((public.entity_client_branches a
     JOIN public.entity_clients_branch_account_details b ON ((a.ulid = b.ulid)))
     JOIN public.entity_clients_branch_bank_details c ON ((a.ulid = c.ulid)))
     JOIN public.entity_clients_branch_payroll_details d ON ((a.ulid = d.ulid)));


ALTER TABLE public.entity_clients_branch_details OWNER TO postgres;
