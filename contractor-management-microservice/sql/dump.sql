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
-- Name: contractors; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.contractors (
    client_ulid uuid NOT NULL,
    client_name character varying(120) NOT NULL,
    contractor_ulid uuid NOT NULL,
    contractor_name character varying(120) NOT NULL,
    contract_name character varying(50) NOT NULL,
    contract_status character varying(50) NOT NULL,
    job_title character varying(50) NOT NULL,
    seniority character varying(50) NOT NULL,
    created_at timestamp without time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at timestamp without time zone DEFAULT CURRENT_TIMESTAMP NOT NULL
);


ALTER TABLE public.contractors OWNER TO postgres;

--
-- Name: contractor_index; Type: VIEW; Schema: public; Owner: postgres
--

CREATE VIEW public.contractor_index AS
 SELECT contractors.client_ulid,
    contractors.contract_name,
    contractors.contract_status,
    contractors.contractor_name,
    contractors.job_title,
    contractors.seniority
   FROM public.contractors;


ALTER TABLE public.contractor_index OWNER TO postgres;

--
-- Name: tax_report; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.tax_report (
    client_ulid uuid NOT NULL,
    contractor_ulid uuid NOT NULL,
    tax_interval public.interval_type NOT NULL,
    tax_name character varying(120) NOT NULL,
    begin_period timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    end_period timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    country character varying(50) NOT NULL,
    tax_report_file bytea,
    created_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL
);


ALTER TABLE public.tax_report OWNER TO postgres;

--
-- Name: tax_report_full; Type: VIEW; Schema: public; Owner: postgres
--

CREATE VIEW public.tax_report_full AS
 SELECT contractors.client_ulid,
    contractors.client_name,
    contractors.contractor_ulid,
    contractors.contractor_name,
    contractors.contract_name,
    tax_report.tax_interval,
    tax_report.tax_name,
    tax_report.country,
    tax_report.tax_report_file
   FROM (public.tax_report
     JOIN public.contractors ON (((tax_report.client_ulid = contractors.client_ulid) AND (tax_report.contractor_ulid = contractors.contractor_ulid))));


ALTER TABLE public.tax_report_full OWNER TO postgres;

--
-- Name: contractors mdt_contracts; Type: TRIGGER; Schema: public; Owner: postgres
--

CREATE TRIGGER mdt_contracts BEFORE UPDATE ON public.contractors FOR EACH ROW EXECUTE FUNCTION public.moddatetime('updated_at');


--
-- Name: tax_report mdt_tax_report; Type: TRIGGER; Schema: public; Owner: postgres
--

CREATE TRIGGER mdt_tax_report BEFORE UPDATE ON public.tax_report FOR EACH ROW EXECUTE FUNCTION public.moddatetime('updated_at');


--
-- PostgreSQL database dump complete
--