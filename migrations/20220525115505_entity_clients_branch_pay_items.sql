--
-- Name: entity_clients_branch_pay_items; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.entity_clients_branch_pay_items (
    ulid uuid NOT NULL PRIMARY KEY,
    branch_ulid uuid NOT NULL REFERENCES entity_client_branches(ulid),
    pay_item_type text,
    pay_item_custom_name text,
    use_pay_item_type_name boolean DEFAULT false,
    pay_item_method text,
    employers_contribution text,
    require_employee_id boolean DEFAULT false,
    created_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL
);


ALTER TABLE public.entity_clients_branch_pay_items OWNER TO postgres;
