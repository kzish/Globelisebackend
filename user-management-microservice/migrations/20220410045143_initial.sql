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

SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- Name: auth_entities; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.auth_entities (
    created_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    ulid uuid NOT NULL,
    email character varying(254) NOT NULL,
    password text,
    is_google boolean NOT NULL,
    is_outlook boolean NOT NULL
);


ALTER TABLE public.auth_entities OWNER TO postgres;

--
-- Name: auth_individuals; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.auth_individuals (
    created_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    ulid uuid NOT NULL,
    email character varying(254) NOT NULL,
    password text,
    is_google boolean NOT NULL,
    is_outlook boolean NOT NULL
);


ALTER TABLE public.auth_individuals OWNER TO postgres;

--
-- Name: entity_clients_account_details; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.entity_clients_account_details (
    created_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    ulid uuid NOT NULL,
    company_name text NOT NULL,
    country text NOT NULL,
    entity_type text NOT NULL,
    registration_number text,
    tax_id text,
    company_address text NOT NULL,
    city text NOT NULL,
    postal_code text NOT NULL,
    time_zone text NOT NULL,
    logo bytea
);


ALTER TABLE public.entity_clients_account_details OWNER TO postgres;

--
-- Name: entity_clients_payment_details; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.entity_clients_payment_details (
    created_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    ulid uuid NOT NULL,
    currency public.currency NOT NULL,
    payment_date date NOT NULL,
    cutoff_date date NOT NULL
);


ALTER TABLE public.entity_clients_payment_details OWNER TO postgres;

--
-- Name: entity_clients_pic_details; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.entity_clients_pic_details (
    created_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    ulid uuid NOT NULL,
    first_name text NOT NULL,
    last_name text NOT NULL,
    dob date NOT NULL,
    dial_code text NOT NULL,
    phone_number text NOT NULL,
    profile_picture bytea
);


ALTER TABLE public.entity_clients_pic_details OWNER TO postgres;

--
-- Name: entity_clients_fully_onboarded; Type: VIEW; Schema: public; Owner: postgres
--

CREATE VIEW public.entity_clients_fully_onboarded AS
 SELECT entity_clients_account_details.ulid
   FROM ((public.entity_clients_account_details
     JOIN public.entity_clients_payment_details ON ((entity_clients_account_details.ulid = entity_clients_payment_details.ulid)))
     JOIN public.entity_clients_pic_details ON ((entity_clients_account_details.ulid = entity_clients_pic_details.ulid)));


ALTER TABLE public.entity_clients_fully_onboarded OWNER TO postgres;

--
-- Name: entity_clients_onboarded; Type: VIEW; Schema: public; Owner: postgres
--

CREATE VIEW public.entity_clients_onboarded AS
 SELECT entity_clients_account_details.ulid
   FROM ((public.entity_clients_account_details
     LEFT JOIN public.entity_clients_payment_details ON ((entity_clients_account_details.ulid = entity_clients_payment_details.ulid)))
     LEFT JOIN public.entity_clients_pic_details ON ((entity_clients_account_details.ulid = entity_clients_pic_details.ulid)));


ALTER TABLE public.entity_clients_onboarded OWNER TO postgres;

--
-- Name: entity_contractors_account_details; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.entity_contractors_account_details (
    created_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    ulid uuid NOT NULL,
    company_name text NOT NULL,
    country text NOT NULL,
    entity_type text NOT NULL,
    registration_number text,
    tax_id text,
    company_address text NOT NULL,
    city text NOT NULL,
    postal_code text NOT NULL,
    time_zone text NOT NULL,
    logo bytea,
    company_profile bytea
);


ALTER TABLE public.entity_contractors_account_details OWNER TO postgres;

--
-- Name: entity_contractors_bank_details; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.entity_contractors_bank_details (
    created_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    ulid uuid NOT NULL,
    bank_name text NOT NULL,
    bank_account_name text NOT NULL,
    bank_account_number text NOT NULL
);


ALTER TABLE public.entity_contractors_bank_details OWNER TO postgres;

--
-- Name: entity_contractors_pic_details; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.entity_contractors_pic_details (
    created_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    ulid uuid NOT NULL,
    first_name text NOT NULL,
    last_name text NOT NULL,
    dob date NOT NULL,
    dial_code text NOT NULL,
    phone_number text NOT NULL,
    profile_picture bytea
);


ALTER TABLE public.entity_contractors_pic_details OWNER TO postgres;

--
-- Name: entity_contractors_fully_onboarded; Type: VIEW; Schema: public; Owner: postgres
--

CREATE VIEW public.entity_contractors_fully_onboarded AS
 SELECT entity_contractors_account_details.ulid
   FROM ((public.entity_contractors_account_details
     JOIN public.entity_contractors_bank_details ON ((entity_contractors_account_details.ulid = entity_contractors_bank_details.ulid)))
     JOIN public.entity_contractors_pic_details ON ((entity_contractors_account_details.ulid = entity_contractors_pic_details.ulid)));


ALTER TABLE public.entity_contractors_fully_onboarded OWNER TO postgres;

--
-- Name: individual_clients_account_details; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.individual_clients_account_details (
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


ALTER TABLE public.individual_clients_account_details OWNER TO postgres;

--
-- Name: individual_clients_payment_details; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.individual_clients_payment_details (
    created_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    ulid uuid NOT NULL,
    currency public.currency NOT NULL,
    payment_date date NOT NULL,
    cutoff_date date NOT NULL
);


ALTER TABLE public.individual_clients_payment_details OWNER TO postgres;

--
-- Name: individual_clients_fully_onboarded; Type: VIEW; Schema: public; Owner: postgres
--

CREATE VIEW public.individual_clients_fully_onboarded AS
 SELECT individual_clients_account_details.ulid
   FROM (public.individual_clients_account_details
     JOIN public.individual_clients_payment_details ON ((individual_clients_account_details.ulid = individual_clients_payment_details.ulid)));


ALTER TABLE public.individual_clients_fully_onboarded OWNER TO postgres;

--
-- Name: individual_contractors_account_details; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.individual_contractors_account_details (
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
    profile_picture bytea,
    cv bytea
);


ALTER TABLE public.individual_contractors_account_details OWNER TO postgres;

--
-- Name: individual_contractors_bank_details; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.individual_contractors_bank_details (
    created_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    ulid uuid NOT NULL,
    bank_name text NOT NULL,
    bank_account_name text NOT NULL,
    bank_account_number text NOT NULL
);


ALTER TABLE public.individual_contractors_bank_details OWNER TO postgres;

--
-- Name: individual_contractors_fully_onboarded; Type: VIEW; Schema: public; Owner: postgres
--

CREATE VIEW public.individual_contractors_fully_onboarded AS
 SELECT individual_contractors_account_details.ulid
   FROM (public.individual_contractors_account_details
     JOIN public.individual_contractors_bank_details ON ((individual_contractors_account_details.ulid = individual_contractors_bank_details.ulid)));


ALTER TABLE public.individual_contractors_fully_onboarded OWNER TO postgres;

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
    entity_contractors_account_details.company_profile,
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
    individual_contractors_account_details.cv,
    individual_contractors_bank_details.bank_name,
    individual_contractors_bank_details.bank_account_name,
    individual_contractors_bank_details.bank_account_number
   FROM (public.individual_contractors_bank_details
     JOIN public.individual_contractors_account_details ON ((individual_contractors_bank_details.ulid = individual_contractors_account_details.ulid)));


ALTER TABLE public.onboard_individual_contractors OWNER TO postgres;

--
-- Name: prefilled_individual_contractors_account_details; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.prefilled_individual_contractors_account_details (
    created_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    email character varying(254) NOT NULL,
    client_ulid uuid NOT NULL,
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
    time_zone text NOT NULL
);


ALTER TABLE public.prefilled_individual_contractors_account_details OWNER TO postgres;

--
-- Name: prefilled_individual_contractors_bank_details; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.prefilled_individual_contractors_bank_details (
    created_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    email character varying(254) NOT NULL,
    client_ulid uuid NOT NULL,
    bank_name text NOT NULL,
    bank_account_name text NOT NULL,
    bank_account_number text NOT NULL
);


ALTER TABLE public.prefilled_individual_contractors_bank_details OWNER TO postgres;

--
-- Name: onboarded_user_index; Type: VIEW; Schema: public; Owner: postgres
--

CREATE VIEW public.onboarded_user_index AS
 WITH client_individual_info AS (
         SELECT auth_individuals.created_at,
            auth_individuals.ulid,
            auth_individuals.email,
            concat(onboard_individual_clients.first_name, ' ', onboard_individual_clients.last_name) AS name,
            'client'::text AS user_role,
            'individual'::text AS user_type
           FROM (public.onboard_individual_clients
             LEFT JOIN public.auth_individuals ON ((auth_individuals.ulid = onboard_individual_clients.ulid)))
        ), client_entity_info AS (
         SELECT auth_entities.created_at,
            auth_entities.ulid,
            auth_entities.email,
            onboard_entity_clients.company_name AS name,
            'client'::text AS user_role,
            'entity'::text AS user_type
           FROM (public.onboard_entity_clients
             LEFT JOIN public.auth_entities ON ((auth_entities.ulid = onboard_entity_clients.ulid)))
        ), contractor_individual_info AS (
         SELECT auth_individuals.created_at,
            auth_individuals.ulid,
            auth_individuals.email,
            concat(onboard_individual_contractors.first_name, ' ', onboard_individual_contractors.last_name) AS name,
            'contractor'::text AS user_role,
            'individual'::text AS user_type
           FROM (public.onboard_individual_contractors
             LEFT JOIN public.auth_individuals ON ((auth_individuals.ulid = onboard_individual_contractors.ulid)))
        ), contractor_entity_info AS (
         SELECT auth_entities.created_at,
            auth_entities.ulid,
            auth_entities.email,
            onboard_entity_contractors.company_name AS name,
            'contractor'::text AS user_role,
            'entity'::text AS user_type
           FROM (public.onboard_entity_contractors
             LEFT JOIN public.auth_entities ON ((auth_entities.ulid = onboard_entity_contractors.ulid)))
        )
 SELECT client_individual_info.created_at,
    client_individual_info.ulid,
    client_individual_info.name,
    client_individual_info.email,
    client_individual_info.user_role,
    client_individual_info.user_type
   FROM client_individual_info
UNION
 SELECT client_entity_info.created_at,
    client_entity_info.ulid,
    client_entity_info.name,
    client_entity_info.email,
    client_entity_info.user_role,
    client_entity_info.user_type
   FROM client_entity_info
UNION
 SELECT contractor_individual_info.created_at,
    contractor_individual_info.ulid,
    contractor_individual_info.name,
    contractor_individual_info.email,
    contractor_individual_info.user_role,
    contractor_individual_info.user_type
   FROM contractor_individual_info
UNION
 SELECT contractor_entity_info.created_at,
    contractor_entity_info.ulid,
    contractor_entity_info.name,
    contractor_entity_info.email,
    contractor_entity_info.user_role,
    contractor_entity_info.user_type
   FROM contractor_entity_info;


ALTER TABLE public.onboarded_user_index OWNER TO postgres;

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
-- Name: entity_clients_payment_details entity_clients_payment_details_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.entity_clients_payment_details
    ADD CONSTRAINT entity_clients_payment_details_pkey PRIMARY KEY (ulid);


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
-- Name: individual_clients_payment_details individual_clients_payment_details_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.individual_clients_payment_details
    ADD CONSTRAINT individual_clients_payment_details_pkey PRIMARY KEY (ulid);


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
-- Name: prefilled_individual_contractors_account_details prefilled_individual_contractors_account_details_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.prefilled_individual_contractors_account_details
    ADD CONSTRAINT prefilled_individual_contractors_account_details_pkey PRIMARY KEY (email);


--
-- Name: prefilled_individual_contractors_bank_details prefilled_individual_contractors_bank_details_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.prefilled_individual_contractors_bank_details
    ADD CONSTRAINT prefilled_individual_contractors_bank_details_pkey PRIMARY KEY (email);


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
-- Name: entity_clients_payment_details mdt_entity_clients_payment_details; Type: TRIGGER; Schema: public; Owner: postgres
--

CREATE TRIGGER mdt_entity_clients_payment_details BEFORE UPDATE ON public.entity_clients_payment_details FOR EACH ROW EXECUTE FUNCTION public.moddatetime('updated_at');


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
-- Name: individual_clients_payment_details mdt_individual_clients_payment_details; Type: TRIGGER; Schema: public; Owner: postgres
--

CREATE TRIGGER mdt_individual_clients_payment_details BEFORE UPDATE ON public.individual_clients_payment_details FOR EACH ROW EXECUTE FUNCTION public.moddatetime('updated_at');


--
-- Name: individual_contractors_account_details mdt_individual_contractors_account_details; Type: TRIGGER; Schema: public; Owner: postgres
--

CREATE TRIGGER mdt_individual_contractors_account_details BEFORE UPDATE ON public.individual_contractors_account_details FOR EACH ROW EXECUTE FUNCTION public.moddatetime('updated_at');


--
-- Name: individual_contractors_bank_details mdt_individual_contractors_bank_details; Type: TRIGGER; Schema: public; Owner: postgres
--

CREATE TRIGGER mdt_individual_contractors_bank_details BEFORE UPDATE ON public.individual_contractors_bank_details FOR EACH ROW EXECUTE FUNCTION public.moddatetime('updated_at');


--
-- Name: prefilled_individual_contractors_account_details mdt_prefilled_individual_contractors_account_details; Type: TRIGGER; Schema: public; Owner: postgres
--

CREATE TRIGGER mdt_prefilled_individual_contractors_account_details BEFORE UPDATE ON public.prefilled_individual_contractors_account_details FOR EACH ROW EXECUTE FUNCTION public.moddatetime('updated_at');


--
-- Name: prefilled_individual_contractors_bank_details mdt_prefilled_individual_contractors_bank_details; Type: TRIGGER; Schema: public; Owner: postgres
--

CREATE TRIGGER mdt_prefilled_individual_contractors_bank_details BEFORE UPDATE ON public.prefilled_individual_contractors_bank_details FOR EACH ROW EXECUTE FUNCTION public.moddatetime('updated_at');


--
-- Name: entity_clients_account_details entity_clients_account_details_ulid_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.entity_clients_account_details
    ADD CONSTRAINT entity_clients_account_details_ulid_fkey FOREIGN KEY (ulid) REFERENCES public.auth_entities(ulid) ON DELETE CASCADE;


--
-- Name: entity_clients_payment_details entity_clients_payment_details_ulid_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.entity_clients_payment_details
    ADD CONSTRAINT entity_clients_payment_details_ulid_fkey FOREIGN KEY (ulid) REFERENCES public.auth_entities(ulid) ON DELETE CASCADE;


--
-- Name: entity_clients_pic_details entity_clients_pic_details_ulid_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.entity_clients_pic_details
    ADD CONSTRAINT entity_clients_pic_details_ulid_fkey FOREIGN KEY (ulid) REFERENCES public.auth_entities(ulid) ON DELETE CASCADE;


--
-- Name: entity_contractors_account_details entity_contractors_account_details_ulid_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.entity_contractors_account_details
    ADD CONSTRAINT entity_contractors_account_details_ulid_fkey FOREIGN KEY (ulid) REFERENCES public.auth_entities(ulid) ON DELETE CASCADE;


--
-- Name: entity_contractors_bank_details entity_contractors_bank_details_ulid_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.entity_contractors_bank_details
    ADD CONSTRAINT entity_contractors_bank_details_ulid_fkey FOREIGN KEY (ulid) REFERENCES public.auth_entities(ulid) ON DELETE CASCADE;


--
-- Name: entity_contractors_pic_details entity_contractors_pic_details_ulid_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.entity_contractors_pic_details
    ADD CONSTRAINT entity_contractors_pic_details_ulid_fkey FOREIGN KEY (ulid) REFERENCES public.auth_entities(ulid) ON DELETE CASCADE;


--
-- Name: individual_clients_account_details individual_clients_account_details_ulid_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.individual_clients_account_details
    ADD CONSTRAINT individual_clients_account_details_ulid_fkey FOREIGN KEY (ulid) REFERENCES public.auth_individuals(ulid) ON DELETE CASCADE;


--
-- Name: individual_clients_payment_details individual_clients_payment_details_ulid_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.individual_clients_payment_details
    ADD CONSTRAINT individual_clients_payment_details_ulid_fkey FOREIGN KEY (ulid) REFERENCES public.auth_individuals(ulid) ON DELETE CASCADE;


--
-- Name: individual_contractors_account_details individual_contractors_account_details_ulid_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.individual_contractors_account_details
    ADD CONSTRAINT individual_contractors_account_details_ulid_fkey FOREIGN KEY (ulid) REFERENCES public.auth_individuals(ulid) ON DELETE CASCADE;


--
-- Name: individual_contractors_bank_details individual_contractors_bank_details_ulid_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.individual_contractors_bank_details
    ADD CONSTRAINT individual_contractors_bank_details_ulid_fkey FOREIGN KEY (ulid) REFERENCES public.auth_individuals(ulid) ON DELETE CASCADE;


--
-- Name: prefilled_individual_contractors_bank_details prefilled_individual_contractors_bank_details_email_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.prefilled_individual_contractors_bank_details
    ADD CONSTRAINT prefilled_individual_contractors_bank_details_email_fkey FOREIGN KEY (email) REFERENCES public.prefilled_individual_contractors_account_details(email) ON DELETE CASCADE;


--
-- Name: user_index; Type: VIEW; Schema: public; Owner: postgres
--

CREATE VIEW public.user_index AS
 WITH client_individual_info AS (
         SELECT auth_individuals.created_at,
            auth_individuals.ulid,
            auth_individuals.email
           FROM public.auth_individuals 
        ), client_entity_info AS (
         SELECT auth_entities.created_at,
            auth_entities.ulid,
            auth_entities.email
           FROM public.auth_entities
        )
 SELECT client_individual_info.created_at,
    client_individual_info.ulid,
    client_individual_info.email
   FROM client_individual_info
UNION
 SELECT client_entity_info.created_at,
    client_entity_info.ulid,
    client_entity_info.email
   FROM client_entity_info;


ALTER TABLE public.user_index OWNER TO postgres;


--
-- PostgreSQL database dump complete
--
