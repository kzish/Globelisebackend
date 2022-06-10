--
-- Name: entity_contractors_fully_onboarded; Type: VIEW; Schema: public; Owner: postgres
--

CREATE VIEW public.entity_contractors_fully_onboarded AS
 SELECT entity_contractors_account_details.ulid
   FROM ((public.entity_contractors_account_details
     JOIN public.entity_contractors_bank_details ON ((entity_contractors_account_details.ulid = entity_contractors_bank_details.ulid)))
     JOIN public.entity_contractors_pic_details ON ((entity_contractors_account_details.ulid = entity_contractors_pic_details.ulid)));


ALTER TABLE public.entity_contractors_fully_onboarded OWNER TO postgres;
