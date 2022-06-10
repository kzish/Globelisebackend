--
-- Name: user_detail_types; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.user_detail_types (
    code text NOT NULL PRIMARY KEY,
    created_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL
);


ALTER TABLE public.user_detail_types OWNER TO postgres;

--
-- Data for Name: user_detail_types; Type: TABLE DATA; Schema: public; Owner: postgres
--

INSERT INTO public.user_detail_types (code) VALUES ('PERSONAL');
INSERT INTO public.user_detail_types (code) VALUES ('EMPLOYMENT');
INSERT INTO public.user_detail_types (code) VALUES ('BANK');
INSERT INTO public.user_detail_types (code) VALUES ('PAYROLL');


--
-- Name: user_detail_formats; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.user_detail_formats (
    code text NOT NULL PRIMARY KEY,
    created_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL
);


ALTER TABLE public.user_detail_formats OWNER TO postgres;

--
-- Data for Name: user_detail_formats; Type: TABLE DATA; Schema: public; Owner: postgres
--

INSERT INTO public.user_detail_formats (code) VALUES ('SHORT_TEXT');
INSERT INTO public.user_detail_formats (code) VALUES ('LONG_TEXT');
INSERT INTO public.user_detail_formats (code) VALUES ('NUMBER');
INSERT INTO public.user_detail_formats (code) VALUES ('ACCOUNTING_NUMBER');
INSERT INTO public.user_detail_formats (code) VALUES ('DD_MM_YY');
INSERT INTO public.user_detail_formats (code) VALUES ('SINGLE_SELECT');
INSERT INTO public.user_detail_formats (code) VALUES ('MULTIPLE_SELECT');


--
-- Name: entity_client_custom_fields; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.entity_client_custom_fields (
    ulid uuid NOT NULL PRIMARY KEY,
    client_ulid uuid NOT NULL REFERENCES public.users(ulid),
    field_name text NOT NULL,
    field_type text NOT NULL REFERENCES public.user_detail_types(code),
    field_format text NOT NULL REFERENCES public.user_detail_formats(code),
    created_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL
);


ALTER TABLE public.entity_client_custom_fields OWNER TO postgres;


--
-- Name: entity_client_custom_field_config_single_select_choices; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.entity_client_custom_field_config_single_select_choices (
    ulid uuid NOT NULL REFERENCES public.entity_client_custom_fields(ulid),
    choice text NOT NULL,
    created_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL
);


ALTER TABLE public.entity_client_custom_field_config_single_select_choices OWNER TO postgres;

--
-- Name: entity_client_custom_field_config_single_mutiple_choices; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.entity_client_custom_field_config_single_mutiple_choices (
    ulid uuid NOT NULL REFERENCES public.entity_client_custom_fields(ulid),
    choice text NOT NULL,
    created_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL
);


ALTER TABLE public.entity_client_custom_field_config_single_mutiple_choices OWNER TO postgres;
