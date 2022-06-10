--
-- Name: onboard_individual_clients; Type: VIEW; Schema: public; Owner: postgres
--

CREATE VIEW public.onboard_individual_clients AS
 SELECT a.ulid,
    a.first_name,
    a.last_name,
    a.dob,
    a.dial_code,
    a.phone_number,
    a.country,
    a.city,
    a.address,
    a.postal_code,
    a.tax_id,
    a.time_zone,
    a.profile_picture
   FROM public.individual_clients_account_details a;


ALTER TABLE public.onboard_individual_clients OWNER TO postgres;
