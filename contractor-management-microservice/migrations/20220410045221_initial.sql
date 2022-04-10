--
-- PostgreSQL database dump
--

-- Dumped from database version 14.1 (Debian 14.1-1.pgdg110+1)
-- Dumped by pg_dump version 14.1 (Debian 14.1-1.pgdg110+1)

--
-- Name: moddatetime; Type: EXTENSION; Schema: -; Owner: -
--

CREATE EXTENSION IF NOT EXISTS moddatetime WITH SCHEMA public;


--
-- Name: EXTENSION moddatetime; Type: COMMENT; Schema: -; Owner: 
--

COMMENT ON EXTENSION moddatetime IS 'functions for tracking last modification time';


--
-- Name: currency; Type: TYPE; Schema: public; Owner: postgres
--

CREATE TYPE public.currency AS ENUM (
    'AED',
    'AFN',
    'ALL',
    'AMD',
    'ANG',
    'AOA',
    'ARS',
    'AUD',
    'AWG',
    'AZN',
    'BAM',
    'BBD',
    'BDT',
    'BGN',
    'BHD',
    'BIF',
    'BMD',
    'BND',
    'BOB',
    'BOV',
    'BRL',
    'BSD',
    'BTN',
    'BWP',
    'BYN',
    'BZD',
    'CAD',
    'CDF',
    'CHE',
    'CHF',
    'CHW',
    'CLF',
    'CLP',
    'CNY',
    'COP',
    'COU',
    'CRC',
    'CUC',
    'CUP',
    'CVE',
    'CZK',
    'DJF',
    'DKK',
    'DOP',
    'DZD',
    'EGP',
    'ERN',
    'ETB',
    'EUR',
    'FJD',
    'FKP',
    'GBP',
    'GEL',
    'GHS',
    'GIP',
    'GMD',
    'GNF',
    'GTQ',
    'GYD',
    'HKD',
    'HNL',
    'HRK',
    'HTG',
    'HUF',
    'IDR',
    'ILS',
    'INR',
    'IQD',
    'IRR',
    'ISK',
    'JMD',
    'JOD',
    'JPY',
    'KES',
    'KGS',
    'KHR',
    'KMF',
    'KPW',
    'KRW',
    'KWD',
    'KYD',
    'KZT',
    'LAK',
    'LBP',
    'LKR',
    'LRD',
    'LSL',
    'LYD',
    'MAD',
    'MDL',
    'MGA',
    'MKD',
    'MMK',
    'MNT',
    'MOP',
    'MRU',
    'MUR',
    'MVR',
    'MWK',
    'MXN',
    'MXV',
    'MYR',
    'MZN',
    'NAD',
    'NGN',
    'NIO',
    'NOK',
    'NPR',
    'NZD',
    'OMR',
    'PAB',
    'PEN',
    'PGK',
    'PHP',
    'PKR',
    'PLN',
    'PYG',
    'QAR',
    'RON',
    'RSD',
    'RUB',
    'RWF',
    'SAR',
    'SBD',
    'SCR',
    'SDG',
    'SEK',
    'SGD',
    'SHP',
    'SLL',
    'SOS',
    'SRD',
    'SSP',
    'STN',
    'SVC',
    'SYP',
    'SZL',
    'THB',
    'TJS',
    'TMT',
    'TND',
    'TOP',
    'TRY',
    'TTD',
    'TWD',
    'TZS',
    'UAH',
    'UGX',
    'USD',
    'USN',
    'UYI',
    'UYU',
    'UYW',
    'UZS',
    'VED',
    'VES',
    'VND',
    'VUV',
    'WST',
    'XAF',
    'XAG',
    'XAU',
    'XBA',
    'XBB',
    'XBC',
    'XBD',
    'XCD',
    'XDR',
    'XOF',
    'XPD',
    'XPF',
    'XPT',
    'XSU',
    'XTS',
    'XUA',
    'XXX',
    'YER',
    'ZAR',
    'ZMW',
    'ZWL'
);


ALTER TYPE public.currency OWNER TO postgres;

--
-- Name: interval_type; Type: TYPE; Schema: public; Owner: postgres
--

CREATE TYPE public.interval_type AS ENUM (
    'monthly',
    'yearly'
);


ALTER TYPE public.interval_type OWNER TO postgres;

SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- Name: client_contractor_pairs; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.client_contractor_pairs (
    created_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    client_ulid uuid NOT NULL,
    contractor_ulid uuid NOT NULL
);


ALTER TABLE public.client_contractor_pairs OWNER TO postgres;

--
-- Name: client_names; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.client_names (
    created_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    ulid uuid NOT NULL,
    name text NOT NULL
);


ALTER TABLE public.client_names OWNER TO postgres;

--
-- Name: contractor_names; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.contractor_names (
    created_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    ulid uuid NOT NULL,
    name text NOT NULL
);


ALTER TABLE public.contractor_names OWNER TO postgres;

--
-- Name: contracts; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.contracts (
    created_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    ulid uuid NOT NULL,
    client_ulid uuid NOT NULL,
    contractor_ulid uuid NOT NULL,
    contract_name text NOT NULL,
    contract_type text NOT NULL,
    contract_status text NOT NULL,
    contract_amount numeric NOT NULL,
    currency public.currency NOT NULL,
    job_title text NOT NULL,
    seniority text NOT NULL,
    begin_at date NOT NULL,
    end_at date NOT NULL,
    CONSTRAINT contracts_begin_at_end_at_check CHECK ((begin_at <= end_at)),
    CONSTRAINT contracts_contract_amount_check CHECK ((contract_amount >= (0)::numeric))
);


ALTER TABLE public.contracts OWNER TO postgres;

--
-- Name: contractors_index; Type: VIEW; Schema: public; Owner: postgres
--

CREATE VIEW public.contractors_index AS
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
     LEFT JOIN public.contracts ON (((client_contractor_pairs.client_ulid = contracts.client_ulid) AND (client_contractor_pairs.contractor_ulid = contracts.contractor_ulid))));


ALTER TABLE public.contractors_index OWNER TO postgres;

--
-- Name: contracts_index; Type: VIEW; Schema: public; Owner: postgres
--

CREATE VIEW public.contracts_index AS
 SELECT contracts.ulid AS contract_ulid,
    contracts.client_ulid,
    client_names.name AS client_name,
    contracts.contractor_ulid,
    contractor_names.name AS contractor_name,
    contracts.contract_name,
    contracts.contract_type,
    contracts.contract_status,
    contracts.contract_amount,
    contracts.currency,
    contracts.begin_at,
    contracts.end_at,
    contracts.job_title,
    contracts.seniority
   FROM ((public.contracts
     JOIN public.client_names ON ((contracts.client_ulid = client_names.ulid)))
     JOIN public.contractor_names ON ((contracts.contractor_ulid = contractor_names.ulid)));


ALTER TABLE public.contracts_index OWNER TO postgres;

--
-- Name: invoice_group; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.invoice_group (
    created_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    ulid uuid NOT NULL,
    invoice_name text NOT NULL,
    invoice_status text NOT NULL,
    invoice_due date NOT NULL,
    invoice_date date NOT NULL
);


ALTER TABLE public.invoice_group OWNER TO postgres;

--
-- Name: invoice_individual; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.invoice_individual (
    created_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    ulid uuid NOT NULL,
    invoice_group_ulid uuid NOT NULL,
    contractor_ulid uuid NOT NULL,
    client_ulid uuid NOT NULL,
    invoice_id bigint NOT NULL,
    invoice_tax_amount numeric NOT NULL,
    invoice_amount_paid numeric NOT NULL,
    terms_and_instructions text NOT NULL,
    bill_to_name text NOT NULL,
    bill_to_address text NOT NULL
);


ALTER TABLE public.invoice_individual OWNER TO postgres;

--
-- Name: invoice_items; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.invoice_items (
    created_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    ulid uuid NOT NULL,
    invoice_ulid uuid NOT NULL,
    item_name text NOT NULL,
    item_unit_price numeric NOT NULL,
    item_unit_quantity bigint NOT NULL
);


ALTER TABLE public.invoice_items OWNER TO postgres;

--
-- Name: invoice_individual_index; Type: VIEW; Schema: public; Owner: postgres
--

CREATE VIEW public.invoice_individual_index AS
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
             LEFT JOIN total_amount ON ((step_1.ulid = total_amount.invoice_ulid)))
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


ALTER TABLE public.invoice_individual_index OWNER TO postgres;

--
-- Name: invoice_group_index; Type: VIEW; Schema: public; Owner: postgres
--

CREATE VIEW public.invoice_group_index AS
 SELECT array_agg(a.ulid ORDER BY a.ulid) AS ulid,
    a.invoice_group_ulid,
    array_agg(a.contractor_ulid ORDER BY a.ulid) AS contractor_ulid,
    array_agg(a.client_ulid ORDER BY a.ulid) AS client_ulid,
    array_agg(a.invoice_id ORDER BY a.ulid) AS invoice_id,
    array_agg(a.invoice_name ORDER BY a.ulid) AS invoice_name,
    array_agg(a.invoice_due ORDER BY a.ulid) AS invoice_due,
    array_agg(a.invoice_status ORDER BY a.ulid) AS invoice_status,
    array_agg(a.invoice_amount ORDER BY a.ulid) AS invoice_amount
   FROM public.invoice_individual_index a
  GROUP BY a.invoice_group_ulid;


ALTER TABLE public.invoice_group_index OWNER TO postgres;

--
-- Name: payslips; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.payslips (
    created_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    ulid uuid NOT NULL,
    client_ulid uuid NOT NULL,
    contractor_ulid uuid NOT NULL,
    contract_ulid uuid,
    payslip_title text NOT NULL,
    payment_date date NOT NULL,
    begin_period date NOT NULL,
    end_period date NOT NULL,
    payslip_file bytea NOT NULL,
    CONSTRAINT payslips_begin_period_end_period_check CHECK ((begin_period <= end_period))
);


ALTER TABLE public.payslips OWNER TO postgres;

--
-- Name: payslips_index; Type: VIEW; Schema: public; Owner: postgres
--

CREATE VIEW public.payslips_index AS
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
     LEFT JOIN public.contracts ON ((payslips.contract_ulid = contracts.ulid)));


ALTER TABLE public.payslips_index OWNER TO postgres;

--
-- Name: tax_reports; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.tax_reports (
    created_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    ulid uuid NOT NULL,
    client_ulid uuid NOT NULL,
    contractor_ulid uuid NOT NULL,
    contract_ulid uuid,
    tax_interval public.interval_type NOT NULL,
    tax_name text NOT NULL,
    begin_period date NOT NULL,
    end_period date NOT NULL,
    country text NOT NULL,
    tax_report_file bytea NOT NULL,
    CONSTRAINT tax_reports_begin_period_end_period_check CHECK ((begin_period <= end_period))
);


ALTER TABLE public.tax_reports OWNER TO postgres;

--
-- Name: tax_reports_index; Type: VIEW; Schema: public; Owner: postgres
--

CREATE VIEW public.tax_reports_index AS
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
     LEFT JOIN public.contracts ON ((tax_reports.contract_ulid = contracts.ulid)));


ALTER TABLE public.tax_reports_index OWNER TO postgres;

--
-- Name: client_contractor_pairs client_contractor_pairs_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.client_contractor_pairs
    ADD CONSTRAINT client_contractor_pairs_pkey PRIMARY KEY (client_ulid, contractor_ulid);


--
-- Name: client_names client_names_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.client_names
    ADD CONSTRAINT client_names_pkey PRIMARY KEY (ulid);


--
-- Name: contractor_names contractor_names_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.contractor_names
    ADD CONSTRAINT contractor_names_pkey PRIMARY KEY (ulid);


--
-- Name: contracts contracts_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.contracts
    ADD CONSTRAINT contracts_pkey PRIMARY KEY (ulid);


--
-- Name: contracts contracts_ulid_client_ulid_contractor_ulid_key; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.contracts
    ADD CONSTRAINT contracts_ulid_client_ulid_contractor_ulid_key UNIQUE (ulid, client_ulid, contractor_ulid);


--
-- Name: invoice_group invoice_group_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.invoice_group
    ADD CONSTRAINT invoice_group_pkey PRIMARY KEY (ulid);


--
-- Name: invoice_individual invoice_individual_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.invoice_individual
    ADD CONSTRAINT invoice_individual_pkey PRIMARY KEY (ulid);


--
-- Name: invoice_items invoice_items_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.invoice_items
    ADD CONSTRAINT invoice_items_pkey PRIMARY KEY (ulid);


--
-- Name: payslips payslips_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.payslips
    ADD CONSTRAINT payslips_pkey PRIMARY KEY (ulid);


--
-- Name: tax_reports tax_reports_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.tax_reports
    ADD CONSTRAINT tax_reports_pkey PRIMARY KEY (ulid);


--
-- Name: client_contractor_pairs mdt_client_contractor_pairs; Type: TRIGGER; Schema: public; Owner: postgres
--

CREATE TRIGGER mdt_client_contractor_pairs BEFORE UPDATE ON public.client_contractor_pairs FOR EACH ROW EXECUTE FUNCTION public.moddatetime('updated_at');


--
-- Name: client_names mdt_client_names; Type: TRIGGER; Schema: public; Owner: postgres
--

CREATE TRIGGER mdt_client_names BEFORE UPDATE ON public.client_names FOR EACH ROW EXECUTE FUNCTION public.moddatetime('updated_at');


--
-- Name: contractor_names mdt_contractor_names; Type: TRIGGER; Schema: public; Owner: postgres
--

CREATE TRIGGER mdt_contractor_names BEFORE UPDATE ON public.contractor_names FOR EACH ROW EXECUTE FUNCTION public.moddatetime('updated_at');


--
-- Name: contracts mdt_contracts; Type: TRIGGER; Schema: public; Owner: postgres
--

CREATE TRIGGER mdt_contracts BEFORE UPDATE ON public.contracts FOR EACH ROW EXECUTE FUNCTION public.moddatetime('updated_at');


--
-- Name: invoice_group mdt_invoice_group; Type: TRIGGER; Schema: public; Owner: postgres
--

CREATE TRIGGER mdt_invoice_group BEFORE UPDATE ON public.invoice_group FOR EACH ROW EXECUTE FUNCTION public.moddatetime('updated_at');


--
-- Name: invoice_items mdt_invoice_items; Type: TRIGGER; Schema: public; Owner: postgres
--

CREATE TRIGGER mdt_invoice_items BEFORE UPDATE ON public.invoice_items FOR EACH ROW EXECUTE FUNCTION public.moddatetime('updated_at');


--
-- Name: invoice_individual mdt_invoices; Type: TRIGGER; Schema: public; Owner: postgres
--

CREATE TRIGGER mdt_invoices BEFORE UPDATE ON public.invoice_individual FOR EACH ROW EXECUTE FUNCTION public.moddatetime('updated_at');


--
-- Name: payslips mdt_payslips; Type: TRIGGER; Schema: public; Owner: postgres
--

CREATE TRIGGER mdt_payslips BEFORE UPDATE ON public.payslips FOR EACH ROW EXECUTE FUNCTION public.moddatetime('updated_at');


--
-- Name: tax_reports mdt_tax_reports; Type: TRIGGER; Schema: public; Owner: postgres
--

CREATE TRIGGER mdt_tax_reports BEFORE UPDATE ON public.tax_reports FOR EACH ROW EXECUTE FUNCTION public.moddatetime('updated_at');


--
-- Name: client_contractor_pairs client_contractor_pairs_client_ulid_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.client_contractor_pairs
    ADD CONSTRAINT client_contractor_pairs_client_ulid_fkey FOREIGN KEY (client_ulid) REFERENCES public.client_names(ulid) ON DELETE RESTRICT;


--
-- Name: client_contractor_pairs client_contractor_pairs_contractor_ulid_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.client_contractor_pairs
    ADD CONSTRAINT client_contractor_pairs_contractor_ulid_fkey FOREIGN KEY (contractor_ulid) REFERENCES public.contractor_names(ulid) ON DELETE RESTRICT;


--
-- Name: contracts contracts_client_ulid_contractor_ulid_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.contracts
    ADD CONSTRAINT contracts_client_ulid_contractor_ulid_fkey FOREIGN KEY (client_ulid, contractor_ulid) REFERENCES public.client_contractor_pairs(client_ulid, contractor_ulid) ON DELETE RESTRICT;


--
-- Name: invoice_individual invoice_individual_client_ulid_contractor_ulid_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.invoice_individual
    ADD CONSTRAINT invoice_individual_client_ulid_contractor_ulid_fkey FOREIGN KEY (client_ulid, contractor_ulid) REFERENCES public.client_contractor_pairs(client_ulid, contractor_ulid) ON DELETE RESTRICT;


--
-- Name: invoice_individual invoice_individual_invoice_group_ulid_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.invoice_individual
    ADD CONSTRAINT invoice_individual_invoice_group_ulid_fkey FOREIGN KEY (invoice_group_ulid) REFERENCES public.invoice_group(ulid);


--
-- Name: invoice_items invoice_items_invoice_ulid_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.invoice_items
    ADD CONSTRAINT invoice_items_invoice_ulid_fkey FOREIGN KEY (invoice_ulid) REFERENCES public.invoice_individual(ulid);


--
-- Name: payslips payslips_client_ulid_contractor_ulid_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.payslips
    ADD CONSTRAINT payslips_client_ulid_contractor_ulid_fkey FOREIGN KEY (client_ulid, contractor_ulid) REFERENCES public.client_contractor_pairs(client_ulid, contractor_ulid) ON DELETE RESTRICT;


--
-- Name: payslips payslips_contract_ulid_client_ulid_contractor_ulid_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.payslips
    ADD CONSTRAINT payslips_contract_ulid_client_ulid_contractor_ulid_fkey FOREIGN KEY (contract_ulid, client_ulid, contractor_ulid) REFERENCES public.contracts(ulid, client_ulid, contractor_ulid) ON DELETE RESTRICT;


--
-- Name: tax_reports tax_reports_client_ulid_contractor_ulid_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.tax_reports
    ADD CONSTRAINT tax_reports_client_ulid_contractor_ulid_fkey FOREIGN KEY (client_ulid, contractor_ulid) REFERENCES public.client_contractor_pairs(client_ulid, contractor_ulid) ON DELETE RESTRICT;


--
-- Name: tax_reports tax_reports_contract_ulid_client_ulid_contractor_ulid_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.tax_reports
    ADD CONSTRAINT tax_reports_contract_ulid_client_ulid_contractor_ulid_fkey FOREIGN KEY (contract_ulid, client_ulid, contractor_ulid) REFERENCES public.contracts(ulid, client_ulid, contractor_ulid) ON DELETE RESTRICT;


--
-- PostgreSQL database dump complete
--
