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


SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- Name: auth_entities; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.auth_entities (
    ulid uuid NOT NULL,
    email character varying(254) NOT NULL,
    password text,
    is_google boolean NOT NULL,
    is_outlook boolean NOT NULL,
    updated_at timestamp without time zone DEFAULT CURRENT_TIMESTAMP NOT NULL
);


ALTER TABLE public.auth_entities OWNER TO postgres;

--
-- Name: auth_individuals; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.auth_individuals (
    ulid uuid NOT NULL,
    email character varying(254) NOT NULL,
    password text,
    is_google boolean NOT NULL,
    is_outlook boolean NOT NULL,
    updated_at timestamp without time zone DEFAULT CURRENT_TIMESTAMP NOT NULL
);


ALTER TABLE public.auth_individuals OWNER TO postgres;

--
-- Name: onboard_entity_clients; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.onboard_entity_clients (
    ulid uuid NOT NULL,
    company_name character varying(120),
    country character varying(100),
    entity_type character varying(50),
    registration_number character varying(50),
    tax_id character varying(50),
    company_address character varying(250),
    city character varying(50),
    postal_code character varying(20),
    time_zone character varying(50),
    logo bytea,
    first_name character varying(50),
    last_name character varying(50),
    dob date,
    dial_code character varying(5),
    phone_number character varying(16),
    profile_picture bytea,
    updated_at timestamp without time zone DEFAULT CURRENT_TIMESTAMP NOT NULL
);


ALTER TABLE public.onboard_entity_clients OWNER TO postgres;

--
-- Name: onboard_entity_contractors; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.onboard_entity_contractors (
    ulid uuid NOT NULL,
    company_name character varying(120),
    country character varying(100),
    entity_type character varying(50),
    registration_number character varying(50),
    tax_id character varying(50),
    company_address character varying(250),
    city character varying(50),
    postal_code character varying(20),
    time_zone character varying(50),
    logo bytea,
    first_name character varying(50),
    last_name character varying(50),
    dob date,
    dial_code character varying(5),
    phone_number character varying(16),
    profile_picture bytea,
    bank_name character varying(120),
    bank_account_name character varying(50),
    bank_account_number character varying(20),
    updated_at timestamp without time zone DEFAULT CURRENT_TIMESTAMP NOT NULL
);


ALTER TABLE public.onboard_entity_contractors OWNER TO postgres;


--
-- Name: onboard_individual_clients; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.onboard_individual_clients (
    ulid uuid NOT NULL,
    first_name character varying(50),
    last_name character varying(50),
    dob date,
    dial_code character varying(5),
    phone_number character varying(16),
    country character varying(100),
    city character varying(50),
    address character varying(250),
    postal_code character varying(20),
    tax_id character varying(50),
    time_zone character varying(50),
    profile_picture bytea,
    updated_at timestamp without time zone DEFAULT CURRENT_TIMESTAMP NOT NULL
);


ALTER TABLE public.onboard_individual_clients OWNER TO postgres;

--
-- Name: onboard_individual_contractors; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.onboard_individual_contractors (
    ulid uuid NOT NULL,
    first_name character varying(50),
    last_name character varying(50),
    dob date,
    dial_code character varying(5),
    phone_number character varying(16),
    country character varying(100),
    city character varying(50),
    address character varying(250),
    postal_code character varying(20),
    tax_id character varying(50),
    time_zone character varying(50),
    profile_picture bytea,
    bank_name character varying(120),
    bank_account_name character varying(50),
    bank_account_number character varying(20),
    updated_at timestamp without time zone DEFAULT CURRENT_TIMESTAMP NOT NULL
);


ALTER TABLE public.onboard_individual_contractors OWNER TO postgres;

--
-- Name: prefilled_onboard_individual_contractors; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.prefilled_onboard_individual_contractors (
    email character varying(50) NOT NULL,
    first_name character varying(50),
    last_name character varying(50),
    dob date,
    dial_code character varying(5),
    phone_number character varying(16),
    country character varying(100),
    city character varying(50),
    address character varying(250),
    postal_code character varying(20),
    tax_id character varying(50),
    time_zone character varying(50),
    profile_picture bytea,
    bank_name character varying(120),
    bank_account_name character varying(50),
    bank_account_number character varying(20),
    updated_at timestamp without time zone DEFAULT CURRENT_TIMESTAMP NOT NULL
);


ALTER TABLE public.prefilled_onboard_individual_contractors OWNER TO postgres;

--
-- Name: auth_entities auth_entities_email_key; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.auth_entities
    ADD CONSTRAINT auth_entities_email_key UNIQUE (email);


--
-- Name: auth_entities auth_entities_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.auth_entities
    ADD CONSTRAINT auth_entities_pkey PRIMARY KEY (ulid);


--
-- Name: auth_individuals auth_individuals_email_key; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.auth_individuals
    ADD CONSTRAINT auth_individuals_email_key UNIQUE (email);


--
-- Name: auth_individuals auth_individuals_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.auth_individuals
    ADD CONSTRAINT auth_individuals_pkey PRIMARY KEY (ulid);


--
-- Name: onboard_entity_clients onboard_entity_clients_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.onboard_entity_clients
    ADD CONSTRAINT onboard_entity_clients_pkey PRIMARY KEY (ulid);


--
-- Name: onboard_entity_contractors onboard_entity_contractors_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.onboard_entity_contractors
    ADD CONSTRAINT onboard_entity_contractors_pkey PRIMARY KEY (ulid);


--
-- Name: onboard_eor_admins onboard_eor_admins_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.onboard_eor_admins
    ADD CONSTRAINT onboard_eor_admins_pkey PRIMARY KEY (ulid);


--
-- Name: onboard_individual_clients onboard_individual_clients_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.onboard_individual_clients
    ADD CONSTRAINT onboard_individual_clients_pkey PRIMARY KEY (ulid);


--
-- Name: onboard_individual_contractors onboard_individual_contractors_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.onboard_individual_contractors
    ADD CONSTRAINT onboard_individual_contractors_pkey PRIMARY KEY (ulid);


--
-- Name: prefilled_onboard_individual_contractors prefilled_onboard_individual_contractors_email_key; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.prefilled_onboard_individual_contractors
    ADD CONSTRAINT prefilled_onboard_individual_contractors_email_key UNIQUE (email);


--
-- Name: prefilled_onboard_individual_contractors prefilled_onboard_individual_contractors_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.prefilled_onboard_individual_contractors
    ADD CONSTRAINT prefilled_onboard_individual_contractors_pkey PRIMARY KEY (email);


--
-- Name: auth_entities mdt_auth_entities; Type: TRIGGER; Schema: public; Owner: postgres
--

CREATE TRIGGER mdt_auth_entities BEFORE UPDATE ON public.auth_entities FOR EACH ROW EXECUTE FUNCTION public.moddatetime('updated_at');


--
-- Name: auth_individuals mdt_auth_individuals; Type: TRIGGER; Schema: public; Owner: postgres
--

CREATE TRIGGER mdt_auth_individuals BEFORE UPDATE ON public.auth_individuals FOR EACH ROW EXECUTE FUNCTION public.moddatetime('updated_at');


--
-- Name: onboard_entity_clients mdt_onboard_entity_clients; Type: TRIGGER; Schema: public; Owner: postgres
--

CREATE TRIGGER mdt_onboard_entity_clients BEFORE UPDATE ON public.onboard_entity_clients FOR EACH ROW EXECUTE FUNCTION public.moddatetime('updated_at');


--
-- Name: onboard_entity_contractors mdt_onboard_entity_contractors; Type: TRIGGER; Schema: public; Owner: postgres
--

CREATE TRIGGER mdt_onboard_entity_contractors BEFORE UPDATE ON public.onboard_entity_contractors FOR EACH ROW EXECUTE FUNCTION public.moddatetime('timestamp_at');


--
-- Name: onboard_eor_admins mdt_onboard_eor_admins; Type: TRIGGER; Schema: public; Owner: postgres
--

CREATE TRIGGER mdt_onboard_eor_admins BEFORE UPDATE ON public.onboard_eor_admins FOR EACH ROW EXECUTE FUNCTION public.moddatetime('updated_at');


--
-- Name: onboard_individual_clients mdt_onboard_individual_clients; Type: TRIGGER; Schema: public; Owner: postgres
--

CREATE TRIGGER mdt_onboard_individual_clients BEFORE UPDATE ON public.onboard_individual_clients FOR EACH ROW EXECUTE FUNCTION public.moddatetime('updated_at');


--
-- Name: onboard_individual_contractors mdt_onboard_individual_contractors; Type: TRIGGER; Schema: public; Owner: postgres
--

CREATE TRIGGER mdt_onboard_individual_contractors BEFORE UPDATE ON public.onboard_individual_contractors FOR EACH ROW EXECUTE FUNCTION public.moddatetime('updated_at');


--
-- Name: prefilled_onboard_individual_contractors mdt_onboard_individual_contractors; Type: TRIGGER; Schema: public; Owner: postgres
--

CREATE TRIGGER mdt_onboard_individual_contractors BEFORE UPDATE ON public.prefilled_onboard_individual_contractors FOR EACH ROW EXECUTE FUNCTION public.moddatetime('updated_at');


--
-- Name: onboard_entity_clients onboard_entity_clients_ulid_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.onboard_entity_clients
    ADD CONSTRAINT onboard_entity_clients_ulid_fkey FOREIGN KEY (ulid) REFERENCES public.auth_entities(ulid) ON DELETE CASCADE;


--
-- Name: onboard_entity_contractors onboard_entity_contractors_ulid_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.onboard_entity_contractors
    ADD CONSTRAINT onboard_entity_contractors_ulid_fkey FOREIGN KEY (ulid) REFERENCES public.auth_entities(ulid) ON DELETE CASCADE;


--
-- Name: onboard_individual_clients onboard_individual_clients_ulid_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.onboard_individual_clients
    ADD CONSTRAINT onboard_individual_clients_ulid_fkey FOREIGN KEY (ulid) REFERENCES public.auth_individuals(ulid) ON DELETE CASCADE;


--
-- Name: onboard_individual_contractors onboard_individual_contractors_ulid_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.onboard_individual_contractors
    ADD CONSTRAINT onboard_individual_contractors_ulid_fkey FOREIGN KEY (ulid) REFERENCES public.auth_individuals(ulid) ON DELETE CASCADE;


--
-- PostgreSQL database dump complete
--

