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
-- Name: contracts_index_for_clients; Type: VIEW; Schema: public; Owner: postgres
--

CREATE VIEW public.contracts_index_for_clients AS
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


ALTER TABLE public.contracts_index_for_clients OWNER TO postgres;

--
-- Name: contracts_index_for_contractors; Type: VIEW; Schema: public; Owner: postgres
--

CREATE VIEW public.contracts_index_for_contractors AS
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


ALTER TABLE public.contracts_index_for_contractors OWNER TO postgres;

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
-- PostgreSQL database dump complete
--

