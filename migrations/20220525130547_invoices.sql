--
-- Name: invoice_group; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.invoice_group (
    ulid uuid NOT NULL PRIMARY KEY,
    invoice_name text NOT NULL,
    invoice_status text NOT NULL,
    invoice_due timestamp with time zone NOT NULL,
    invoice_date timestamp with time zone NOT NULL,
    created_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL
);


ALTER TABLE public.invoice_group OWNER TO postgres;


--
-- Name: invoice_individual; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.invoice_individual (
    ulid uuid NOT NULL PRIMARY KEY,
    invoice_group_ulid uuid NOT NULL REFERENCES public.invoice_group(ulid),
    contractor_ulid uuid NOT NULL REFERENCES public.users(ulid),
    client_ulid uuid NOT NULL REFERENCES public.users(ulid),
    invoice_id bigint NOT NULL,
    invoice_tax_amount numeric NOT NULL,
    invoice_amount_paid numeric NOT NULL,
    terms_and_instructions text NOT NULL,
    bill_to_name text NOT NULL,
    bill_to_address text NOT NULL,
    created_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL
);


ALTER TABLE public.invoice_individual OWNER TO postgres;

--
-- Name: invoice_items; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.invoice_items (
    ulid uuid NOT NULL PRIMARY KEY,
    invoice_individual_ulid uuid NOT NULL REFERENCES public.invoice_individual(ulid),
    item_name text NOT NULL,
    item_unit_price numeric NOT NULL,
    item_unit_quantity bigint NOT NULL,
    created_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL
);


ALTER TABLE public.invoice_items OWNER TO postgres;

--
-- Name: invoice_individual_index; Type: VIEW; Schema: public; Owner: postgres
--

CREATE VIEW public.invoice_individual_index AS
 WITH total_amount AS (
         SELECT invoice_items.invoice_individual_ulid,
            sum(((invoice_items.item_unit_quantity)::numeric * invoice_items.item_unit_price)) AS invoice_amount
           FROM (public.invoice_individual
             JOIN public.invoice_items ON ((invoice_individual.ulid = invoice_items.invoice_individual_ulid)))
          GROUP BY invoice_items.invoice_individual_ulid
        ), step_1 AS (
         SELECT invoice_individual.ulid,
            invoice_individual.invoice_group_ulid,
            invoice_individual.contractor_ulid,
            invoice_individual.client_ulid,
            invoice_individual.invoice_id,
            invoice_group.invoice_name,
            invoice_group.invoice_due,
            invoice_group.invoice_status
           FROM (public.invoice_group
             JOIN public.invoice_individual ON ((invoice_group.ulid = invoice_individual.invoice_group_ulid)))
        ), step_2 AS (
         SELECT step_1.ulid,
            step_1.invoice_group_ulid,
            step_1.contractor_ulid,
            step_1.client_ulid,
            step_1.invoice_id,
            step_1.invoice_name,
            step_1.invoice_due,
            step_1.invoice_status,
            COALESCE(total_amount.invoice_amount, (0)::numeric) AS invoice_amount
           FROM (step_1
             JOIN total_amount ON ((step_1.ulid = total_amount.invoice_individual_ulid)))
        )
 SELECT step_2.ulid,
    step_2.invoice_group_ulid,
    step_2.contractor_ulid,
    step_2.client_ulid,
    step_2.invoice_id,
    step_2.invoice_name,
    step_2.invoice_due,
    step_2.invoice_status,
    step_2.invoice_amount
   FROM step_2;


ALTER TABLE public.invoice_individual_index OWNER TO postgres;

--
-- Name: invoice_group_index; Type: VIEW; Schema: public; Owner: postgres
--

CREATE VIEW public.invoice_group_index AS
 SELECT array_agg(a.ulid ORDER BY a.ulid) AS ulid,
    a.invoice_group_ulid,
    array_agg(a.contractor_ulid ORDER BY a.ulid) AS contractor_ulid,
    array_agg(a.client_ulid ORDER BY a.ulid) AS client_ulid,
    array_agg(a.invoice_id ORDER BY a.ulid) AS invoice_id,
    array_agg(a.invoice_name ORDER BY a.ulid) AS invoice_name,
    array_agg(a.invoice_due ORDER BY a.ulid) AS invoice_due,
    array_agg(a.invoice_status ORDER BY a.ulid) AS invoice_status,
    array_agg(a.invoice_amount ORDER BY a.ulid) AS invoice_amount
   FROM public.invoice_individual_index a
  GROUP BY a.invoice_group_ulid;


ALTER TABLE public.invoice_group_index OWNER TO postgres;
