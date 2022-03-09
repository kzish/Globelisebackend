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
-- Name: contractors mdt_contracts; Type: TRIGGER; Schema: public; Owner: postgres
--

CREATE TRIGGER mdt_contracts BEFORE UPDATE ON public.contractors FOR EACH ROW EXECUTE FUNCTION public.moddatetime('updated_at');


--
-- PostgreSQL database dump complete
--

