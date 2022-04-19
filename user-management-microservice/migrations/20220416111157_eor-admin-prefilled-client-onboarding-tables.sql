-- Table: public.prefilled_entity_clients_bank_details

DROP TABLE IF EXISTS public.prefilled_entity_clients_bank_details;

CREATE TABLE IF NOT EXISTS public.prefilled_entity_clients_bank_details
(
    created_at timestamp with time zone NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at timestamp with time zone NOT NULL DEFAULT CURRENT_TIMESTAMP,
    ulid uuid NOT NULL,
    bank_name text NOT NULL,
    bank_account_name text NOT NULL,
    bank_account_number text NOT NULL,
    CONSTRAINT prefilled_entity_clients_bank_details_pkey PRIMARY KEY (ulid),
    CONSTRAINT prefilled_entity_clients_bank_details_ulid_fkey FOREIGN KEY (ulid)
        REFERENCES public.auth_entities (ulid) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE CASCADE
)

TABLESPACE pg_default;

ALTER TABLE IF EXISTS public.prefilled_entity_clients_bank_details
    OWNER to postgres;

-- Trigger: mdt_prefilled_entity_clients_bank_details

-- DROP TRIGGER IF EXISTS mdt_prefilled_entity_clients_bank_details ON public.prefilled_entity_clients_bank_details;

CREATE TRIGGER mdt_prefilled_entity_clients_bank_details
    BEFORE UPDATE 
    ON public.prefilled_entity_clients_bank_details
    FOR EACH ROW
    EXECUTE FUNCTION public.moddatetime('updated_at');



-- Table: public.prefilled_entity_clients_payment_details

DROP TABLE IF EXISTS public.prefilled_entity_clients_payment_details;

CREATE TABLE IF NOT EXISTS public.prefilled_entity_clients_payment_details
(
    created_at timestamp with time zone NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at timestamp with time zone NOT NULL DEFAULT CURRENT_TIMESTAMP,
    ulid uuid NOT NULL,
    currency currency NOT NULL,
    payment_date date NOT NULL,
    cutoff_date date NOT NULL,
    CONSTRAINT prefilled_entity_clients_payment_details_pkey PRIMARY KEY (ulid),
    CONSTRAINT prefilled_entity_clients_payment_details_ulid_fkey FOREIGN KEY (ulid)
        REFERENCES public.auth_entities (ulid) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE CASCADE
)

TABLESPACE pg_default;

ALTER TABLE IF EXISTS public.prefilled_entity_clients_payment_details
    OWNER to postgres;
-- Index: fki_prefilled_entity_clients_payment_details_fkey

-- DROP INDEX IF EXISTS public.fki_prefilled_entity_clients_payment_details_fkey;

CREATE INDEX IF NOT EXISTS fki_prefilled_entity_clients_payment_details_fkey
    ON public.prefilled_entity_clients_payment_details USING btree
    (ulid ASC NULLS LAST)
    TABLESPACE pg_default;

-- Trigger: mdt_prefilled_entity_clients_payment_details

-- DROP TRIGGER IF EXISTS mdt_prefilled_entity_clients_payment_details ON public.prefilled_entity_clients_payment_details;

CREATE TRIGGER mdt_prefilled_entity_clients_payment_details
    BEFORE UPDATE 
    ON public.prefilled_entity_clients_payment_details
    FOR EACH ROW
    EXECUTE FUNCTION public.moddatetime('updated_at');


-- Table: public.prefilled_entity_clients_pic_details

DROP TABLE IF EXISTS public.prefilled_entity_clients_pic_details;

CREATE TABLE IF NOT EXISTS public.prefilled_entity_clients_pic_details
(
    created_at timestamp with time zone NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at timestamp with time zone NOT NULL DEFAULT CURRENT_TIMESTAMP,
    ulid uuid NOT NULL,
    first_name text NOT NULL,
    last_name text NOT NULL,
    dob date NOT NULL,
    dial_code text NOT NULL,
    phone_number text NOT NULL,
    profile_picture bytea,
    CONSTRAINT prefilled_entity_clients_pic_details_pkey PRIMARY KEY (ulid),
    CONSTRAINT prefilled_entity_clients_pic_details_ulid_fkey FOREIGN KEY (ulid)
        REFERENCES public.auth_entities (ulid) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE CASCADE
)

TABLESPACE pg_default;

ALTER TABLE IF EXISTS public.prefilled_entity_clients_pic_details
    OWNER to postgres;

-- Trigger: mdt_prefilled_entity_clients_pic_details

-- DROP TRIGGER IF EXISTS mdt_prefilled_entity_clients_pic_details ON public.prefilled_entity_clients_pic_details;

CREATE TRIGGER mdt_prefilled_entity_clients_pic_details
    BEFORE UPDATE 
    ON public.prefilled_entity_clients_pic_details
    FOR EACH ROW
    EXECUTE FUNCTION public.moddatetime('updated_at');



    -- Table: public.prefilled_entity_clients_account_details

DROP TABLE IF EXISTS public.prefilled_entity_clients_account_details;

CREATE TABLE IF NOT EXISTS public.prefilled_entity_clients_account_details
(
    created_at timestamp with time zone NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at timestamp with time zone NOT NULL DEFAULT CURRENT_TIMESTAMP,
    ulid uuid NOT NULL,
    company_name text NOT NULL,
    country text NOT NULL,
    entity_type text NOT NULL,
    registration_number text COLLATE pg_catalog."default",
    tax_id text COLLATE pg_catalog."default",
    company_address text NOT NULL,
    city text NOT NULL,
    postal_code text NOT NULL,
    time_zone text NOT NULL,
    logo bytea,
    CONSTRAINT prefilled_entity_clients_account_details_pkey PRIMARY KEY (ulid),
    CONSTRAINT prefilled_entity_clients_account_details_ulid_fkey FOREIGN KEY (ulid)
        REFERENCES public.auth_entities (ulid) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE CASCADE
)

TABLESPACE pg_default;

ALTER TABLE IF EXISTS public.prefilled_entity_clients_account_details
    OWNER to postgres;
-- Index: fki_prefilled_entity_clients_account_details_ulid_fkey

DROP INDEX IF EXISTS public.fki_prefilled_entity_clients_account_details_ulid_fkey;

CREATE INDEX IF NOT EXISTS fki_prefilled_entity_clients_account_details_ulid_fkey
    ON public.prefilled_entity_clients_account_details USING btree
    (ulid ASC NULLS LAST)
    TABLESPACE pg_default;

-- Trigger: mdt_prefilled_entity_clients_account_details

-- DROP TRIGGER IF EXISTS mdt_prefilled_entity_clients_account_details ON public.prefilled_entity_clients_account_details;

CREATE TRIGGER mdt_prefilled_entity_clients_account_details
    BEFORE UPDATE 
    ON public.prefilled_entity_clients_account_details
    FOR EACH ROW
    EXECUTE FUNCTION public.moddatetime('updated_at');
