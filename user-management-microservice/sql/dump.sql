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
-- Name: auth_entities; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.auth_entities (
    ulid uuid NOT NULL,
    email character varying(254) NOT NULL,
    password text,
    is_google boolean NOT NULL,
    is_outlook boolean NOT NULL,
    updated_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL
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
    updated_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL
);


ALTER TABLE public.auth_individuals OWNER TO postgres;

--
-- Name: entity_clients_account_details; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.entity_clients_account_details (
    ulid uuid NOT NULL,
    company_name character varying(120) NOT NULL,
    country character varying(100) NOT NULL,
    entity_type character varying(50) NOT NULL,
    registration_number character varying(50),
    tax_id character varying(50),
    company_address character varying(250) NOT NULL,
    city character varying(50) NOT NULL,
    postal_code character varying(20) NOT NULL,
    time_zone character varying(50) NOT NULL,
    logo bytea,
    updated_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL
);


ALTER TABLE public.entity_clients_account_details OWNER TO postgres;

--
-- Name: entity_clients_pic_details; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.entity_clients_pic_details (
    ulid uuid NOT NULL,
    first_name character varying(50) NOT NULL,
    last_name character varying(50) NOT NULL,
    dob date NOT NULL,
    dial_code character varying(5) NOT NULL,
    phone_number character varying(16) NOT NULL,
    profile_picture bytea,
    updated_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL
);


ALTER TABLE public.entity_clients_pic_details OWNER TO postgres;

--
-- Name: entity_contractors_account_details; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.entity_contractors_account_details (
    ulid uuid NOT NULL,
    company_name character varying(120) NOT NULL,
    country character varying(100) NOT NULL,
    entity_type character varying(50) NOT NULL,
    registration_number character varying(50),
    tax_id character varying(50),
    company_address character varying(250) NOT NULL,
    city character varying(50) NOT NULL,
    postal_code character varying(20) NOT NULL,
    time_zone character varying(50) NOT NULL,
    logo bytea,
    updated_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL
);


ALTER TABLE public.entity_contractors_account_details OWNER TO postgres;

--
-- Name: entity_contractors_bank_details; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.entity_contractors_bank_details (
    ulid uuid NOT NULL,
    bank_name character varying(120) NOT NULL,
    bank_account_name character varying(50) NOT NULL,
    bank_account_number character varying(20) NOT NULL,
    updated_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL
);


ALTER TABLE public.entity_contractors_bank_details OWNER TO postgres;

--
-- Name: entity_contractors_pic_details; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.entity_contractors_pic_details (
    ulid uuid NOT NULL,
    first_name character varying(50) NOT NULL,
    last_name character varying(50) NOT NULL,
    dob date NOT NULL,
    dial_code character varying(5) NOT NULL,
    phone_number character varying(16) NOT NULL,
    profile_picture bytea,
    updated_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL
);


ALTER TABLE public.entity_contractors_pic_details OWNER TO postgres;

--
-- Name: individual_clients_account_details; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.individual_clients_account_details (
    ulid uuid NOT NULL,
    first_name character varying(50) NOT NULL,
    last_name character varying(50) NOT NULL,
    dob date NOT NULL,
    dial_code character varying(5) NOT NULL,
    phone_number character varying(16) NOT NULL,
    country character varying(100) NOT NULL,
    city character varying(50) NOT NULL,
    address character varying(250) NOT NULL,
    postal_code character varying(20) NOT NULL,
    tax_id character varying(50),
    time_zone character varying(50) NOT NULL,
    profile_picture bytea,
    updated_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL
);


ALTER TABLE public.individual_clients_account_details OWNER TO postgres;

--
-- Name: individual_contractors_account_details; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.individual_contractors_account_details (
    ulid uuid NOT NULL,
    first_name character varying(50) NOT NULL,
    last_name character varying(50) NOT NULL,
    dob date NOT NULL,
    dial_code character varying(5) NOT NULL,
    phone_number character varying(16) NOT NULL,
    country character varying(100) NOT NULL,
    city character varying(50) NOT NULL,
    address character varying(250) NOT NULL,
    postal_code character varying(20) NOT NULL,
    tax_id character varying(50),
    time_zone character varying(50) NOT NULL,
    profile_picture bytea,
    updated_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL
);


ALTER TABLE public.individual_contractors_account_details OWNER TO postgres;

--
-- Name: individual_contractors_bank_details; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.individual_contractors_bank_details (
    ulid uuid NOT NULL,
    bank_name character varying(120) NOT NULL,
    bank_account_name character varying(50) NOT NULL,
    bank_account_number character varying(20) NOT NULL,
    updated_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL
);


ALTER TABLE public.individual_contractors_bank_details OWNER TO postgres;

--
-- Name: onboard_entity_clients; Type: VIEW; Schema: public; Owner: postgres
--

CREATE VIEW public.onboard_entity_clients AS
 SELECT entity_clients_account_details.ulid,
    entity_clients_account_details.company_name,
    entity_clients_account_details.country,
    entity_clients_account_details.entity_type,
    entity_clients_account_details.registration_number,
    entity_clients_account_details.tax_id,
    entity_clients_account_details.company_address,
    entity_clients_account_details.city,
    entity_clients_account_details.postal_code,
    entity_clients_account_details.time_zone,
    entity_clients_account_details.logo,
    entity_clients_pic_details.first_name,
    entity_clients_pic_details.last_name,
    entity_clients_pic_details.dob,
    entity_clients_pic_details.dial_code,
    entity_clients_pic_details.phone_number,
    entity_clients_pic_details.profile_picture
   FROM (public.entity_clients_pic_details
     JOIN public.entity_clients_account_details ON ((entity_clients_pic_details.ulid = entity_clients_account_details.ulid)));


ALTER TABLE public.onboard_entity_clients OWNER TO postgres;

--
-- Name: onboard_entity_contractors; Type: VIEW; Schema: public; Owner: postgres
--

CREATE VIEW public.onboard_entity_contractors AS
 SELECT entity_contractors_account_details.ulid,
    entity_contractors_account_details.company_name,
    entity_contractors_account_details.country,
    entity_contractors_account_details.entity_type,
    entity_contractors_account_details.registration_number,
    entity_contractors_account_details.tax_id,
    entity_contractors_account_details.company_address,
    entity_contractors_account_details.city,
    entity_contractors_account_details.postal_code,
    entity_contractors_account_details.time_zone,
    entity_contractors_account_details.logo,
    entity_contractors_pic_details.first_name,
    entity_contractors_pic_details.last_name,
    entity_contractors_pic_details.dob,
    entity_contractors_pic_details.dial_code,
    entity_contractors_pic_details.phone_number,
    entity_contractors_pic_details.profile_picture,
    entity_contractors_bank_details.bank_name,
    entity_contractors_bank_details.bank_account_name,
    entity_contractors_bank_details.bank_account_number
   FROM ((public.entity_contractors_bank_details
     JOIN public.entity_contractors_pic_details ON ((entity_contractors_bank_details.ulid = entity_contractors_pic_details.ulid)))
     JOIN public.entity_contractors_account_details ON ((entity_contractors_pic_details.ulid = entity_contractors_account_details.ulid)));


ALTER TABLE public.onboard_entity_contractors OWNER TO postgres;

--
-- Name: onboard_individual_clients; Type: VIEW; Schema: public; Owner: postgres
--

CREATE VIEW public.onboard_individual_clients AS
 SELECT individual_clients_account_details.ulid,
    individual_clients_account_details.first_name,
    individual_clients_account_details.last_name,
    individual_clients_account_details.dob,
    individual_clients_account_details.dial_code,
    individual_clients_account_details.phone_number,
    individual_clients_account_details.country,
    individual_clients_account_details.city,
    individual_clients_account_details.address,
    individual_clients_account_details.postal_code,
    individual_clients_account_details.tax_id,
    individual_clients_account_details.time_zone,
    individual_clients_account_details.profile_picture
   FROM public.individual_clients_account_details;


ALTER TABLE public.onboard_individual_clients OWNER TO postgres;

--
-- Name: onboard_individual_contractors; Type: VIEW; Schema: public; Owner: postgres
--

CREATE VIEW public.onboard_individual_contractors AS
 SELECT individual_contractors_account_details.ulid,
    individual_contractors_account_details.first_name,
    individual_contractors_account_details.last_name,
    individual_contractors_account_details.dob,
    individual_contractors_account_details.dial_code,
    individual_contractors_account_details.phone_number,
    individual_contractors_account_details.country,
    individual_contractors_account_details.city,
    individual_contractors_account_details.address,
    individual_contractors_account_details.postal_code,
    individual_contractors_account_details.tax_id,
    individual_contractors_account_details.time_zone,
    individual_contractors_account_details.profile_picture,
    individual_contractors_bank_details.bank_name,
    individual_contractors_bank_details.bank_account_name,
    individual_contractors_bank_details.bank_account_number
   FROM (public.individual_contractors_bank_details
     JOIN public.individual_contractors_account_details ON ((individual_contractors_bank_details.ulid = individual_contractors_account_details.ulid)));


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
    updated_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL
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
-- Name: entity_clients_account_details entity_clients_account_details_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.entity_clients_account_details
    ADD CONSTRAINT entity_clients_account_details_pkey PRIMARY KEY (ulid);


--
-- Name: entity_clients_pic_details entity_clients_pic_details_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.entity_clients_pic_details
    ADD CONSTRAINT entity_clients_pic_details_pkey PRIMARY KEY (ulid);


--
-- Name: entity_contractors_account_details entity_contractors_account_details_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.entity_contractors_account_details
    ADD CONSTRAINT entity_contractors_account_details_pkey PRIMARY KEY (ulid);


--
-- Name: entity_contractors_bank_details entity_contractors_bank_details_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.entity_contractors_bank_details
    ADD CONSTRAINT entity_contractors_bank_details_pkey PRIMARY KEY (ulid);


--
-- Name: entity_contractors_pic_details entity_contractors_pic_details_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.entity_contractors_pic_details
    ADD CONSTRAINT entity_contractors_pic_details_pkey PRIMARY KEY (ulid);


--
-- Name: individual_clients_account_details individual_clients_account_details_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.individual_clients_account_details
    ADD CONSTRAINT individual_clients_account_details_pkey PRIMARY KEY (ulid);


--
-- Name: individual_contractors_account_details individual_contractors_account_details_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.individual_contractors_account_details
    ADD CONSTRAINT individual_contractors_account_details_pkey PRIMARY KEY (ulid);


--
-- Name: individual_contractors_bank_details individual_contractors_bank_details_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.individual_contractors_bank_details
    ADD CONSTRAINT individual_contractors_bank_details_pkey PRIMARY KEY (ulid);


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
-- Name: entity_clients_account_details mdt_entity_clients_account_details; Type: TRIGGER; Schema: public; Owner: postgres
--

CREATE TRIGGER mdt_entity_clients_account_details BEFORE UPDATE ON public.entity_clients_account_details FOR EACH ROW EXECUTE FUNCTION public.moddatetime('updated_at');


--
-- Name: entity_clients_pic_details mdt_entity_clients_pic_details; Type: TRIGGER; Schema: public; Owner: postgres
--

CREATE TRIGGER mdt_entity_clients_pic_details BEFORE UPDATE ON public.entity_clients_pic_details FOR EACH ROW EXECUTE FUNCTION public.moddatetime('updated_at');


--
-- Name: entity_contractors_account_details mdt_entity_contractors_account_details; Type: TRIGGER; Schema: public; Owner: postgres
--

CREATE TRIGGER mdt_entity_contractors_account_details BEFORE UPDATE ON public.entity_contractors_account_details FOR EACH ROW EXECUTE FUNCTION public.moddatetime('updated_at');


--
-- Name: entity_contractors_bank_details mdt_entity_contractors_bank_details; Type: TRIGGER; Schema: public; Owner: postgres
--

CREATE TRIGGER mdt_entity_contractors_bank_details BEFORE UPDATE ON public.entity_contractors_bank_details FOR EACH ROW EXECUTE FUNCTION public.moddatetime('updated_at');


--
-- Name: entity_contractors_pic_details mdt_entity_contractors_pic_details; Type: TRIGGER; Schema: public; Owner: postgres
--

CREATE TRIGGER mdt_entity_contractors_pic_details BEFORE UPDATE ON public.entity_contractors_pic_details FOR EACH ROW EXECUTE FUNCTION public.moddatetime('updated_at');


--
-- Name: individual_clients_account_details mdt_individual_clients_account_details; Type: TRIGGER; Schema: public; Owner: postgres
--

CREATE TRIGGER mdt_individual_clients_account_details BEFORE UPDATE ON public.individual_clients_account_details FOR EACH ROW EXECUTE FUNCTION public.moddatetime('updated_at');


--
-- Name: individual_contractors_account_details mdt_individual_contractors_account_details; Type: TRIGGER; Schema: public; Owner: postgres
--

CREATE TRIGGER mdt_individual_contractors_account_details BEFORE UPDATE ON public.individual_contractors_account_details FOR EACH ROW EXECUTE FUNCTION public.moddatetime('updated_at');


--
-- Name: individual_contractors_bank_details mdt_individual_contractors_bank_details; Type: TRIGGER; Schema: public; Owner: postgres
--

CREATE TRIGGER mdt_individual_contractors_bank_details BEFORE UPDATE ON public.individual_contractors_bank_details FOR EACH ROW EXECUTE FUNCTION public.moddatetime('updated_at');


--
-- Name: prefilled_onboard_individual_contractors mdt_prefilled_onboard_individual_contractors; Type: TRIGGER; Schema: public; Owner: postgres
--

CREATE TRIGGER mdt_prefilled_onboard_individual_contractors BEFORE UPDATE ON public.prefilled_onboard_individual_contractors FOR EACH ROW EXECUTE FUNCTION public.moddatetime('updated_at');


--
-- Name: entity_clients_account_details entity_clients_account_details_ulid_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.entity_clients_account_details
    ADD CONSTRAINT entity_clients_account_details_ulid_fkey FOREIGN KEY (ulid) REFERENCES public.auth_entities(ulid) ON DELETE CASCADE;


--
-- Name: entity_clients_pic_details entity_clients_pic_details_ulid_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.entity_clients_pic_details
    ADD CONSTRAINT entity_clients_pic_details_ulid_fkey FOREIGN KEY (ulid) REFERENCES public.entity_clients_account_details(ulid) ON DELETE CASCADE;


--
-- Name: entity_contractors_account_details entity_contractors_account_details_ulid_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.entity_contractors_account_details
    ADD CONSTRAINT entity_contractors_account_details_ulid_fkey FOREIGN KEY (ulid) REFERENCES public.auth_entities(ulid) ON DELETE CASCADE;


--
-- Name: entity_contractors_bank_details entity_contractors_bank_details_ulid_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.entity_contractors_bank_details
    ADD CONSTRAINT entity_contractors_bank_details_ulid_fkey FOREIGN KEY (ulid) REFERENCES public.entity_contractors_pic_details(ulid) ON DELETE CASCADE;


--
-- Name: entity_contractors_pic_details entity_contractors_pic_details_ulid_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.entity_contractors_pic_details
    ADD CONSTRAINT entity_contractors_pic_details_ulid_fkey FOREIGN KEY (ulid) REFERENCES public.entity_contractors_account_details(ulid) ON DELETE CASCADE;


--
-- Name: individual_clients_account_details individual_clients_account_details_ulid_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.individual_clients_account_details
    ADD CONSTRAINT individual_clients_account_details_ulid_fkey FOREIGN KEY (ulid) REFERENCES public.auth_individuals(ulid) ON DELETE CASCADE;


--
-- Name: individual_contractors_account_details individual_contractors_account_details_ulid_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.individual_contractors_account_details
    ADD CONSTRAINT individual_contractors_account_details_ulid_fkey FOREIGN KEY (ulid) REFERENCES public.auth_individuals(ulid) ON DELETE CASCADE;


--
-- Name: individual_contractors_bank_details individual_contractors_bank_details_ulid_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.individual_contractors_bank_details
    ADD CONSTRAINT individual_contractors_bank_details_ulid_fkey FOREIGN KEY (ulid) REFERENCES public.individual_contractors_account_details(ulid) ON DELETE CASCADE;


--
-- PostgreSQL database dump complete
--

