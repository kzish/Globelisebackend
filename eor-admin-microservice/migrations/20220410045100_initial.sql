--
-- PostgreSQL database dump
--

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
    updated_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
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
    updated_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    ulid uuid NOT NULL,
    first_name text NOT NULL,
    last_name text NOT NULL,
    dob date NOT NULL,
    dial_code text NOT NULL,
    phone_number text NOT NULL,
    country text NOT NULL,
    city text NOT NULL,
    address text NOT NULL,
    postal_code text NOT NULL,
    tax_id text,
    time_zone text NOT NULL,
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

