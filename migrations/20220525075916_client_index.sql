--
-- Name: client_index; Type: VIEW; Schema: public; Owner: postgres
--

CREATE VIEW public.client_index AS
 SELECT onboarded_user_index.created_at,
    onboarded_user_index.ulid,
    onboarded_user_index.name,
    onboarded_user_index.email,
    onboarded_user_index.user_type
   FROM public.onboarded_user_index
  WHERE (onboarded_user_index.user_role = 'client'::text);


ALTER TABLE public.client_index OWNER TO postgres;
