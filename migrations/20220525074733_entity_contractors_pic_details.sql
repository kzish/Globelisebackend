--
-- Name: entity_contractors_pic_details; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.entity_contractors_pic_details (
    ulid uuid NOT NULL PRIMARY KEY REFERENCES public.users(ulid),
    first_name text NOT NULL,
    last_name text NOT NULL,
    dob timestamp with time zone NOT NULL,
    dial_code text NOT NULL,
    phone_number text NOT NULL,
    profile_picture bytea,
    created_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL
);


ALTER TABLE public.entity_contractors_pic_details OWNER TO postgres;
