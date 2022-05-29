--
-- Name: prefilled_individual_contractors_details_for_bulk_upload; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.prefilled_individual_contractors_details_for_bulk_upload (
    ulid uuid NOT NULL PRIMARY KEY,
    client_ulid uuid NOT NULL REFERENCES public.users(ulid),
    branch_ulid uuid NOT NULL REFERENCES public.entity_client_branches(ulid),
    department_ulid uuid NOT NULL REFERENCES public.entity_client_branch_departments(ulid),
    first_name text NOT NULL,
    last_name text NOT NULL,
    gender text,
    marital_status text,
    nationality text NOT NULL,
    dob timestamp with time zone NOT NULL,
    dial_code text NOT NULL,
    phone_number text NOT NULL,
    email text NOT NULL,
    address text,
    country text REFERENCES public.country_codes(code),
    city text,
    postal_code text,
    national_id text,
    passport_number text,
    passport_expiry_date text,
    work_permit text,
    tax_id text,
    contribution_id_1 text,
    contribution_id_2 text,
    total_dependants smallint,
    time_zone text,
    employee_id text,
    designation text,
    start_date timestamp with time zone,
    end_date timestamp with time zone,
    employment_status text,
    bank_name text,
    bank_account_owner_name text,
    bank_account_number text,
    bank_code text,
    bank_branch_code text,
    currency TEXT NOT NULL REFERENCES public.currency_codes(code),
    basic_salary numeric,
    additional_item_1 text,
    additional_item_2 text,
    deduction_1 text,
    deduction_2 text,
    other_pay_item_1 text,
    other_pay_item_2 text,
    created_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL
);


ALTER TABLE public.prefilled_individual_contractors_details_for_bulk_upload OWNER TO postgres;
