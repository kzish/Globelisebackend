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
-- Name: auth_eor_admins; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.auth_eor_admins (
    created_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at timestamp without time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    ulid uuid NOT NULL,
    email character varying(254) NOT NULL,
    password text,
    is_google boolean NOT NULL,
    is_outlook boolean NOT NULL
);


ALTER TABLE public.auth_eor_admins OWNER TO postgres;

--
-- Name: onboard_eor_admins; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.onboard_eor_admins (
    created_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at timestamp without time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    ulid uuid NOT NULL,
    first_name character varying(50),
    last_name character varying(50),
    dob date,
    dial_code character varying(5),
    phone_number character varying(15),
    country character varying(100),
    city character varying(50),
    address character varying(250),
    postal_code character varying(20),
    tax_id character varying(50),
    time_zone character varying(50),
    profile_picture bytea
);


ALTER TABLE public.onboard_eor_admins OWNER TO postgres;

--
-- Name: auth_eor_admins auth_eor_admins_email_key; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.auth_eor_admins
    ADD CONSTRAINT auth_eor_admins_email_key UNIQUE (email);


--
-- Name: auth_eor_admins auth_eor_admins_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.auth_eor_admins
    ADD CONSTRAINT auth_eor_admins_pkey PRIMARY KEY (ulid);


--
-- Name: onboard_eor_admins onboard_eor_admins_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.onboard_eor_admins
    ADD CONSTRAINT onboard_eor_admins_pkey PRIMARY KEY (ulid);


--
-- Name: auth_eor_admins mdt_auth_eor_admins; Type: TRIGGER; Schema: public; Owner: postgres
--

CREATE TRIGGER mdt_auth_eor_admins BEFORE UPDATE ON public.auth_eor_admins FOR EACH ROW EXECUTE FUNCTION public.moddatetime('updated_at');


--
-- Name: onboard_eor_admins mdt_onboard_eor_admins; Type: TRIGGER; Schema: public; Owner: postgres
--

CREATE TRIGGER mdt_onboard_eor_admins BEFORE UPDATE ON public.onboard_eor_admins FOR EACH ROW EXECUTE FUNCTION public.moddatetime('updated_at');


--
-- Name: onboard_eor_admins onboard_eor_admins_ulid_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.onboard_eor_admins
    ADD CONSTRAINT onboard_eor_admins_ulid_fkey FOREIGN KEY (ulid) REFERENCES public.auth_eor_admins(ulid) ON DELETE CASCADE;


--
-- PostgreSQL database dump complete
--

