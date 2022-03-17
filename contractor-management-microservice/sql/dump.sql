--
-- PostgreSQL database dump
--

-- Dumped from database version 14.1 (Debian 14.1-1.pgdg110+1)
-- Dumped by pg_dump version 14.1 (Debian 14.1-1.pgdg110+1)

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
-- Name: contracts; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.contracts (
    id uuid NOT NULL,
    client_ulid uuid NOT NULL,
    client_name text NOT NULL,
    contractor_ulid uuid NOT NULL,
    contractor_name text NOT NULL,
    contract_name text NOT NULL,
    contract_status text NOT NULL,
    contract_amount integer NOT NULL,
    job_title text NOT NULL,
    seniority text NOT NULL,
    begin_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    end_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    created_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL
);


ALTER TABLE public.contracts OWNER TO postgres;

--
-- Name: contract_index_for_client; Type: VIEW; Schema: public; Owner: postgres
--

CREATE VIEW public.contract_index_for_client AS
 SELECT contracts.client_ulid,
    contracts.contract_name,
    contracts.job_title,
    contracts.seniority,
    contracts.contractor_name,
    contracts.contract_status,
    contracts.contract_amount,
    contracts.end_at
   FROM public.contracts;


ALTER TABLE public.contract_index_for_client OWNER TO postgres;

--
-- Name: contract_index_for_contractor; Type: VIEW; Schema: public; Owner: postgres
--

CREATE VIEW public.contract_index_for_contractor AS
 SELECT contracts.contractor_ulid,
    contracts.contract_name,
    contracts.job_title,
    contracts.seniority,
    contracts.client_name,
    contracts.contract_status,
    contracts.contract_amount,
    contracts.end_at
   FROM public.contracts;


ALTER TABLE public.contract_index_for_contractor OWNER TO postgres;

--
-- Name: contract_index_for_eor_admin; Type: VIEW; Schema: public; Owner: postgres
--

CREATE VIEW public.contract_index_for_eor_admin AS
 SELECT contracts.contract_name,
    contracts.job_title,
    contracts.seniority,
    contracts.client_name,
    contracts.contract_status,
    contracts.contract_amount,
    contracts.end_at
   FROM public.contracts;


ALTER TABLE public.contract_index_for_eor_admin OWNER TO postgres;

--
-- Name: contractor_index; Type: VIEW; Schema: public; Owner: postgres
--

CREATE VIEW public.contractor_index AS
 SELECT contracts.client_ulid,
    contracts.contract_name,
    contracts.contract_status,
    contracts.contractor_name,
    contracts.job_title,
    contracts.seniority
   FROM public.contracts;


ALTER TABLE public.contractor_index OWNER TO postgres;

--
-- Name: tax_report; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.tax_report (
    id uuid,
    tax_interval public.interval_type NOT NULL,
    tax_name text NOT NULL,
    begin_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    end_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    country text NOT NULL,
    tax_report_file bytea,
    created_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL
);


ALTER TABLE public.tax_report OWNER TO postgres;

--
-- Name: tax_report_full; Type: VIEW; Schema: public; Owner: postgres
--

CREATE VIEW public.tax_report_full AS
 SELECT contracts.client_ulid,
    contracts.client_name,
    contracts.contractor_ulid,
    contracts.contractor_name,
    contracts.contract_name,
    tax_report.tax_interval,
    tax_report.tax_name,
    tax_report.country,
    tax_report.tax_report_file
   FROM (public.tax_report
     JOIN public.contracts ON ((tax_report.id = contracts.id)));


ALTER TABLE public.tax_report_full OWNER TO postgres;

--
-- Name: contracts contracts_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.contracts
    ADD CONSTRAINT contracts_pkey PRIMARY KEY (id);


--
-- Name: contracts mdt_contracts; Type: TRIGGER; Schema: public; Owner: postgres
--

CREATE TRIGGER mdt_contracts BEFORE UPDATE ON public.contracts FOR EACH ROW EXECUTE FUNCTION public.moddatetime('updated_at');


--
-- Name: tax_report mdt_tax_report; Type: TRIGGER; Schema: public; Owner: postgres
--

CREATE TRIGGER mdt_tax_report BEFORE UPDATE ON public.tax_report FOR EACH ROW EXECUTE FUNCTION public.moddatetime('updated_at');


--
-- Name: tax_report tax_report_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.tax_report
    ADD CONSTRAINT tax_report_id_fkey FOREIGN KEY (id) REFERENCES public.contracts(id);


--
-- PostgreSQL database dump complete
--