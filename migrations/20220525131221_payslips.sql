--
-- Name: payslips; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.payslips (
    ulid uuid NOT NULL PRIMARY KEY,
    client_ulid uuid NOT NULL REFERENCES public.users(ulid),
    contractor_ulid uuid NOT NULL REFERENCES public.users(ulid),
    contract_ulid uuid REFERENCES public.contracts(ulid),
    payslip_title text NOT NULL,
    payment_date timestamp with time zone NOT NULL,
    begin_period timestamp with time zone NOT NULL,
    end_period timestamp with time zone NOT NULL,
    payslip_file bytea NOT NULL,
    created_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    CONSTRAINT payslips_begin_period_end_period_check CHECK ((begin_period <= end_period))
);


ALTER TABLE public.payslips OWNER TO postgres;

--
-- Name: payslips_index; Type: VIEW; Schema: public; Owner: postgres
--

CREATE VIEW public.payslips_index AS
 SELECT payslips.ulid AS payslip_ulid,
    payslips.client_ulid,
    COALESCE(client_index.name, ''::text) AS client_name,
    payslips.contractor_ulid,
    COALESCE(contractor_index.name, ''::text) AS contractor_name,
    payslips.contract_ulid,
    contracts.contract_name,
    payslips.payslip_title,
    payslips.payment_date,
    payslips.begin_period,
    payslips.end_period,
    payslips.payslip_file
   FROM (((public.payslips
     LEFT JOIN public.client_index ON ((payslips.client_ulid = client_index.ulid)))
     LEFT JOIN public.contractor_index ON ((payslips.contractor_ulid = contractor_index.ulid)))
     LEFT JOIN public.contracts ON ((payslips.contract_ulid = contracts.ulid)));


ALTER TABLE public.payslips_index OWNER TO postgres;
