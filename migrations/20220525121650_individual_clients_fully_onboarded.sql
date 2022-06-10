--
-- Name: individual_clients_fully_onboarded; Type: VIEW; Schema: public; Owner: postgres
--

CREATE VIEW public.individual_clients_fully_onboarded AS
 SELECT individual_clients_account_details.ulid
   FROM (public.individual_clients_account_details
     JOIN public.individual_clients_payment_details ON ((individual_clients_account_details.ulid = individual_clients_payment_details.ulid)));


ALTER TABLE public.individual_clients_fully_onboarded OWNER TO postgres;
