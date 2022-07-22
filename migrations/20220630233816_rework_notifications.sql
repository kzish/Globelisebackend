-- Drop all notification table

DROP TABLE public.entity_client_notifications;

DROP TABLE public.entity_contractor_notifications;

DROP TABLE public.individual_client_notifications;

DROP TABLE public.individual_contractor_notifications;

-- Create new notification table

CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS public.user_notification
(
    ulid uuid NOT NULL PRIMARY KEY DEFAULT uuid_generate_v4(),
    message text NOT NULL,
    created_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL
);

CREATE TABLE IF NOT EXISTS public.user_see_notification
(
    user_ulid uuid NOT NULL REFERENCES public.users(ulid),
    notification_ulid uuid NOT NULL REFERENCES public.user_notification(ulid),
    read boolean NOT NULL DEFAULT 'f',
    created_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    PRIMARY KEY (user_ulid, notification_ulid)
);

CREATE TABLE IF NOT EXISTS public.admin_see_notification
(
    admin_ulid uuid NOT NULL REFERENCES public.admin_users(ulid),
    notification_ulid uuid NOT NULL REFERENCES public.user_notification(ulid),
    read boolean NOT NULL DEFAULT 'f',
    created_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    PRIMARY KEY (admin_ulid, notification_ulid)
);

CREATE VIEW public.user_notification_index
AS
SELECT
    a.ulid AS notification_ulid,
    b.user_ulid,
    a.message,
    b.read,
    a.created_at
FROM
    public.user_notification a
JOIN
    public.user_see_notification b
ON
    a.ulid = b.notification_ulid;

CREATE VIEW public.admin_notification_index
AS
SELECT
    a.ulid AS notification_ulid,
    b.admin_ulid,
    a.message,
    b.read,
    a.created_at
FROM
    public.user_notification a
JOIN
    public.admin_see_notification b
ON
    a.ulid = b.notification_ulid;
