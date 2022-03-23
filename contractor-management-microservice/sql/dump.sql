--
-- PostgreSQL database dump
--

-- Dumped from database version 13.6
-- Dumped by pg_dump version 13.6

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

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
    'AOA',
    'ARS',
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
    'DOP',
    'DZD',
    'EGP',
    'ERN',
    'ETB',
    'FJD',
    'FKP',
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
    'NPR',
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
    'XAG',
    'XAU',
    'XBA',
    'XBB',
    'XBC',
    'XBD',
    'XDR',
    'XPD',
    'XPT',
    'XSU',
    'XTS',
    'XUA',
    'XXX',
    'YER',
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
-- Name: client_names; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.client_names (
    ulid uuid NOT NULL,
    name text NOT NULL,
    updated_at timestamp with time zone NOT NULL
);


ALTER TABLE public.client_names OWNER TO postgres;

--
-- Name: contractor_names; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.contractor_names (
    ulid uuid NOT NULL,
    name text NOT NULL,
    updated_at timestamp with time zone NOT NULL
);


ALTER TABLE public.contractor_names OWNER TO postgres;

--
-- Name: contracts; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.contracts (
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
    begin_at timestamp with time zone NOT NULL,
    end_at timestamp with time zone NOT NULL,
    created_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL
);


ALTER TABLE public.contracts OWNER TO postgres;

--
-- Name: contractors_index; Type: VIEW; Schema: public; Owner: postgres
--

CREATE VIEW public.contractors_index AS
 SELECT contractor_names.name AS contractor_name,
    contracts.client_ulid,
    contracts.contract_name,
    contracts.contract_status,
    contracts.contractor_ulid,
    contracts.job_title,
    contracts.seniority
   FROM (public.contracts
     JOIN public.contractor_names ON ((contracts.contractor_ulid = contractor_names.ulid)));


ALTER TABLE public.contractors_index OWNER TO postgres;

--
-- Name: contracts_index_for_client; Type: VIEW; Schema: public; Owner: postgres
--

CREATE VIEW public.contracts_index_for_client AS
 SELECT contractor_names.name AS contractor_name,
    contracts.client_ulid,
    contracts.contract_name,
    contracts.contract_type,
    contracts.contract_amount,
    contracts.contract_status,
    contracts.contractor_ulid,
    contracts.ulid,
    contracts.end_at,
    contracts.begin_at,
    contracts.currency,
    contracts.job_title,
    contracts.seniority
   FROM (public.contracts
     JOIN public.contractor_names ON ((contracts.contractor_ulid = contractor_names.ulid)));


ALTER TABLE public.contracts_index_for_client OWNER TO postgres;

--
-- Name: contracts_index_for_contractor; Type: VIEW; Schema: public; Owner: postgres
--

CREATE VIEW public.contracts_index_for_contractor AS
 SELECT client_names.name AS client_name,
    contracts.client_ulid,
    contracts.contract_name,
    contracts.contract_type,
    contracts.contract_amount,
    contracts.contract_status,
    contracts.contractor_ulid,
    contracts.ulid,
    contracts.end_at,
    contracts.begin_at,
    contracts.currency,
    contracts.job_title,
    contracts.seniority
   FROM (public.contracts
     JOIN public.client_names ON ((contracts.client_ulid = client_names.ulid)));


ALTER TABLE public.contracts_index_for_contractor OWNER TO postgres;

--
-- Name: invoice_group; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.invoice_group (
    ulid uuid NOT NULL,
    invoice_status text NOT NULL,
    invoice_due timestamp with time zone NOT NULL,
    invoice_date timestamp with time zone NOT NULL,
    created_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL
);


ALTER TABLE public.invoice_group OWNER TO postgres;

--
-- Name: invoice_group_name; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.invoice_group_name (
    invoice_ulid uuid,
    invoice_group_name text NOT NULL,
    created_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL
);


ALTER TABLE public.invoice_group_name OWNER TO postgres;

--
-- Name: invoice_individual; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.invoice_individual (
    ulid uuid NOT NULL,
    invoice_group_ulid uuid,
    contract_ulid uuid,
    invoice_id text NOT NULL,
    invoice_tax_amount integer NOT NULL,
    invoice_amount_paid integer NOT NULL,
    terms_and_instructions text NOT NULL,
    bill_to_name text NOT NULL,
    bill_to_address text NOT NULL,
    created_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL
);


ALTER TABLE public.invoice_individual OWNER TO postgres;

--
-- Name: invoice_items; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.invoice_items (
    ulid uuid NOT NULL,
    invoice_ulid uuid,
    item_name text NOT NULL,
    item_unit_price integer NOT NULL,
    item_unit_quantity integer NOT NULL,
    created_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL
);


ALTER TABLE public.invoice_items OWNER TO postgres;

--
-- Name: invoice_individual_index; Type: VIEW; Schema: public; Owner: postgres
--

CREATE VIEW public.invoice_individual_index AS
 WITH total_amount AS (
         SELECT invoice_items.invoice_ulid,
            sum((invoice_items.item_unit_quantity * invoice_items.item_unit_price)) AS invoice_amount
           FROM (public.invoice_individual
             JOIN public.invoice_items ON ((invoice_individual.ulid = invoice_items.invoice_ulid)))
          GROUP BY invoice_items.invoice_ulid
        ), step_1 AS (
         SELECT invoice_individual.ulid,
            invoice_individual.invoice_group_ulid,
            invoice_individual.contract_ulid,
            invoice_individual.invoice_id,
            invoice_group.invoice_due,
            invoice_group.invoice_status
           FROM (public.invoice_group
             JOIN public.invoice_individual ON ((invoice_group.ulid = invoice_individual.invoice_group_ulid)))
        ), step_2 AS (
         SELECT step_1.ulid,
            step_1.invoice_group_ulid,
            step_1.contract_ulid,
            step_1.invoice_id,
            step_1.invoice_due,
            step_1.invoice_status,
            COALESCE(total_amount.invoice_amount, (0)::bigint) AS invoice_amount
           FROM (step_1
             LEFT JOIN total_amount ON ((step_1.ulid = total_amount.invoice_ulid)))
        )
 SELECT step_2.ulid,
    step_2.invoice_group_ulid,
    step_2.contract_ulid,
    step_2.invoice_id,
    step_2.invoice_due,
    step_2.invoice_status,
    step_2.invoice_amount
   FROM step_2;


ALTER TABLE public.invoice_individual_index OWNER TO postgres;

--
-- Name: tax_report; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.tax_report (
    ulid uuid NOT NULL,
    tax_interval public.interval_type NOT NULL,
    tax_name text NOT NULL,
    begin_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    end_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    country text NOT NULL,
    tax_report_file bytea NOT NULL,
    created_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL
);


ALTER TABLE public.tax_report OWNER TO postgres;

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
-- Name: tax_report tax_report_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.tax_report
    ADD CONSTRAINT tax_report_pkey PRIMARY KEY (ulid);


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
-- Name: invoice_group_name mdt_invoice_group_name; Type: TRIGGER; Schema: public; Owner: postgres
--

CREATE TRIGGER mdt_invoice_group_name BEFORE UPDATE ON public.invoice_group_name FOR EACH ROW EXECUTE FUNCTION public.moddatetime('updated_at');


--
-- Name: invoice_items mdt_invoice_items; Type: TRIGGER; Schema: public; Owner: postgres
--

CREATE TRIGGER mdt_invoice_items BEFORE UPDATE ON public.invoice_items FOR EACH ROW EXECUTE FUNCTION public.moddatetime('updated_at');


--
-- Name: invoice_individual mdt_invoices; Type: TRIGGER; Schema: public; Owner: postgres
--

CREATE TRIGGER mdt_invoices BEFORE UPDATE ON public.invoice_individual FOR EACH ROW EXECUTE FUNCTION public.moddatetime('updated_at');


--
-- Name: tax_report mdt_tax_report; Type: TRIGGER; Schema: public; Owner: postgres
--

CREATE TRIGGER mdt_tax_report BEFORE UPDATE ON public.tax_report FOR EACH ROW EXECUTE FUNCTION public.moddatetime('updated_at');


--
-- Name: contracts contracts_client_ulid_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.contracts
    ADD CONSTRAINT contracts_client_ulid_fkey FOREIGN KEY (client_ulid) REFERENCES public.client_names(ulid) ON DELETE RESTRICT;


--
-- Name: contracts contracts_contractor_ulid_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.contracts
    ADD CONSTRAINT contracts_contractor_ulid_fkey FOREIGN KEY (contractor_ulid) REFERENCES public.contractor_names(ulid) ON DELETE RESTRICT;


--
-- Name: invoice_group_name invoice_group_name_invoice_ulid_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.invoice_group_name
    ADD CONSTRAINT invoice_group_name_invoice_ulid_fkey FOREIGN KEY (invoice_ulid) REFERENCES public.invoice_individual(ulid);


--
-- Name: invoice_individual invoice_individual_contract_ulid_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.invoice_individual
    ADD CONSTRAINT invoice_individual_contract_ulid_fkey FOREIGN KEY (contract_ulid) REFERENCES public.contracts(ulid);


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
-- PostgreSQL database dump complete
--

