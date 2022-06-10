--
-- Name: entity_clients_fully_onboarded; Type: VIEW; Schema: public; Owner: postgres
--

CREATE VIEW public.entity_clients_fully_onboarded AS
 SELECT entity_clients_account_details.ulid
   FROM ((public.entity_clients_account_details
     JOIN public.entity_clients_payment_details ON ((entity_clients_account_details.ulid = entity_clients_payment_details.ulid)))
     JOIN public.entity_clients_pic_details ON ((entity_clients_account_details.ulid = entity_clients_pic_details.ulid)));


ALTER TABLE public.entity_clients_fully_onboarded OWNER TO postgres;
