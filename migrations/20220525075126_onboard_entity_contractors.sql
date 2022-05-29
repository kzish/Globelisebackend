--
-- Name: onboard_entity_contractors; Type: VIEW; Schema: public; Owner: postgres
--

CREATE VIEW public.onboard_entity_contractors AS
 SELECT a.ulid,
    a.company_name,
    a.country,
    a.entity_type,
    a.registration_number,
    a.tax_id,
    a.company_address,
    a.city,
    a.postal_code,
    a.time_zone,
    a.logo,
    a.company_profile,
    b.first_name,
    b.last_name,
    b.dob,
    b.dial_code,
    b.phone_number,
    b.profile_picture,
    c.bank_name,
    c.bank_account_name,
    c.bank_account_number
   FROM ((public.entity_contractors_account_details a
     JOIN public.entity_contractors_pic_details b ON ((a.ulid = b.ulid)))
     JOIN public.entity_contractors_bank_details c ON ((a.ulid = c.ulid)));


ALTER TABLE public.onboard_entity_contractors OWNER TO postgres;
