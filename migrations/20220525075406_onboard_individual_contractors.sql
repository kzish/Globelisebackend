--
-- Name: onboard_individual_contractors; Type: VIEW; Schema: public; Owner: postgres
--

CREATE VIEW public.onboard_individual_contractors AS
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
    a.profile_picture,
    a.cv,
    b.bank_name,
    b.bank_account_name,
    b.bank_account_number
   FROM (public.individual_contractors_account_details a
     JOIN public.individual_contractors_bank_details b ON ((a.ulid = b.ulid)));


ALTER TABLE public.onboard_individual_contractors OWNER TO postgres;
