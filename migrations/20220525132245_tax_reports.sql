--
-- Name: a; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.tax_reports (
    ulid uuid NOT NULL PRIMARY KEY,
    client_ulid uuid NOT NULL REFERENCES public.users(ulid),
    contractor_ulid uuid NOT NULL REFERENCES public.users(ulid),
    contract_ulid uuid REFERENCES public.contracts(ulid),
    tax_interval TEXT NOT NULL,
    tax_name text NOT NULL,
    begin_period timestamp with time zone NOT NULL,
    end_period timestamp with time zone NOT NULL,
    country text NOT NULL,
    tax_report_file bytea NOT NULL,
    created_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    CONSTRAINT tax_reports_begin_period_end_period_check CHECK ((begin_period <= end_period))
);


ALTER TABLE public.tax_reports OWNER TO postgres;

--
-- Name: tax_reports_index; Type: VIEW; Schema: public; Owner: postgres
--

CREATE VIEW public.tax_reports_index AS
 SELECT a.ulid AS tax_report_ulid,
    a.client_ulid,
    a.contractor_ulid,
    a.tax_interval,
    a.tax_name,
    a.begin_period,
    a.end_period,
    a.country,
    a.tax_report_file,
    b.name AS client_name,
    c.name AS contractor_name,
    d.contract_name
   FROM (((public.tax_reports a
     JOIN public.client_index b ON ((a.client_ulid = b.ulid)))
     JOIN public.contractor_index c ON ((a.contractor_ulid = c.ulid)))
     JOIN public.contracts d ON ((a.contract_ulid = d.ulid)));


ALTER TABLE public.tax_reports_index OWNER TO postgres;
