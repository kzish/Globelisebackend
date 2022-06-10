--
-- Name: individual_contractors_fully_onboarded; Type: VIEW; Schema: public; Owner: postgres
--

CREATE VIEW public.individual_contractors_fully_onboarded AS
 SELECT individual_contractors_account_details.ulid
   FROM (public.individual_contractors_account_details
     JOIN public.individual_contractors_bank_details ON ((individual_contractors_account_details.ulid = individual_contractors_bank_details.ulid)));


ALTER TABLE public.individual_contractors_fully_onboarded OWNER TO postgres;
