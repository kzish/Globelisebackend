CREATE TABLE IF NOT EXISTS public.entity_client_notifications
(
    ulid uuid NOT NULL PRIMARY KEY,
    user_ulid uuid NOT NULL REFERENCES public.users(ulid),
    message text NOT NULL,
    read boolean NOT NULL DEFAULT 'f',
    created_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL
);

CREATE TABLE IF NOT EXISTS public.entity_contractor_notifications
(
    ulid uuid NOT NULL PRIMARY KEY,
    user_ulid uuid NOT NULL REFERENCES public.users(ulid),
    message text NOT NULL,
    read boolean NOT NULL DEFAULT 'f',
    created_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL
);

CREATE TABLE IF NOT EXISTS public.individual_client_notifications
(
    ulid uuid NOT NULL PRIMARY KEY,
    user_ulid uuid NOT NULL REFERENCES public.users(ulid),
    message text NOT NULL,
    read boolean NOT NULL DEFAULT 'f',
    created_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL
);

CREATE TABLE IF NOT EXISTS public.individual_contractor_notifications
(
    ulid uuid NOT NULL PRIMARY KEY,
    user_ulid uuid NOT NULL REFERENCES public.users(ulid),
    message text NOT NULL,
    read boolean NOT NULL DEFAULT 'f',
    created_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL
);
