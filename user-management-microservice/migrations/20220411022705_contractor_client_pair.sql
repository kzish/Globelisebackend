CREATE VIEW public.client_index AS
    SELECT
        created_at,
        ulid,
        name,
        email,
        user_type
    FROM 
        onboarded_user_index
    WHERE
        user_role = 'client'::text;


CREATE VIEW public.contractor_index AS
    SELECT
        created_at,
        ulid,
        name,
        email,
        user_type
    FROM 
        onboarded_user_index
    WHERE
        user_role = 'contractor'::text;


CREATE TABLE public.client_contractor_pairs (
    created_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    client_ulid uuid NOT NULL,
    contractor_ulid uuid NOT NULL
);
