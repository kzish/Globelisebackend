--
-- Name: onboard_entity_clients; Type: VIEW; Schema: public; Owner: postgres
--

CREATE VIEW public.onboard_entity_clients AS
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
    b.first_name,
    b.last_name,
    b.dob,
    b.dial_code,
    b.phone_number,
    b.profile_picture
   FROM (public.entity_clients_account_details a
     JOIN public.entity_clients_pic_details b ON ((a.ulid = b.ulid)));


ALTER TABLE public.onboard_entity_clients OWNER TO postgres;
