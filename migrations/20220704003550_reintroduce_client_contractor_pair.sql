--
-- Name: client_contractor_pairs; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.client_contractor_pairs (
    client_ulid uuid NOT NULL,
    contractor_ulid uuid NOT NULL,
    contract_ulid uuid,
    created_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    PRIMARY KEY (client_ulid, contractor_ulid)
);

ALTER TABLE public.client_contractor_pairs OWNER TO postgres;

INSERT INTO public.client_contractor_pairs (
    client_ulid, contractor_ulid, contract_ulid
) SELECT
    client_ulid, contractor_ulid, ulid AS contract_ulid
FROM public.contracts;

-- Create views

CREATE OR REPLACE VIEW public.contractors_index_for_clients
AS
SELECT
    b.name,
    b.email,
    b.user_role,
    b.user_type,
    b.contract_count,
    b.created_at,
    a.client_ulid,
    a.contractor_ulid AS ulid
FROM
    public.client_contractor_pairs a
LEFT JOIN
    public.onboarded_user_index b 
ON
    a.contractor_ulid = b.ulid;

CREATE OR REPLACE VIEW public.clients_index_for_contractors
AS
SELECT
    b.name,
    b.email,
    b.user_role,
    b.user_type,
    b.contract_count,
    b.created_at,
    a.client_ulid AS ulid,
    a.contractor_ulid
FROM
    public.client_contractor_pairs a
LEFT JOIN
    public.onboarded_user_index b
ON
    a.client_ulid = b.ulid;