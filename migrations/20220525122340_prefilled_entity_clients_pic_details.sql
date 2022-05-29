--
-- Name: prefilled_entity_clients_pic_details; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.prefilled_entity_clients_pic_details (
    email text NOT NULL PRIMARY KEY,
    first_name text NOT NULL,
    last_name text NOT NULL,
    dob timestamp with time zone NOT NULL,
    dial_code text NOT NULL,
    phone_number text NOT NULL,
    profile_picture bytea,
    created_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL
);


ALTER TABLE public.prefilled_entity_clients_pic_details OWNER TO postgres;
