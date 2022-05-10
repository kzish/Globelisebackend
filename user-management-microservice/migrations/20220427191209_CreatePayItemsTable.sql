-- Table: public.entity_clients_branch_pay_items

-- DROP TABLE IF EXISTS public.entity_clients_branch_pay_items;

CREATE TABLE IF NOT EXISTS public.entity_clients_branch_pay_items
(
    ulid uuid NOT NULL,
    created_at timestamp with time zone NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at timestamp with time zone NOT NULL DEFAULT CURRENT_TIMESTAMP,
    branch_ulid uuid NOT NULL,
    pay_item_type text COLLATE pg_catalog."default",
    pay_item_custom_name text COLLATE pg_catalog."default",
    use_pay_item_type_name boolean DEFAULT false,
    pay_item_method text COLLATE pg_catalog."default",
    employers_contribution text COLLATE pg_catalog."default",
    require_employee_id boolean DEFAULT false,
    CONSTRAINT entity_clients_branch_pay_items_pkey PRIMARY KEY (ulid),
    CONSTRAINT entity_clients_branch_pay_items__entity_clients_branches_fkey FOREIGN KEY (branch_ulid)
        REFERENCES public.entity_client_branches (ulid) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE CASCADE
)

TABLESPACE pg_default;

ALTER TABLE IF EXISTS public.entity_clients_branch_pay_items
    OWNER to postgres;

COMMENT ON CONSTRAINT entity_clients_branch_pay_items__entity_clients_branches_fkey ON public.entity_clients_branch_pay_items
    IS 'foreign key on branch ulid';
-- Index: fki_entity_clients_pay_items__auth_entities_fkey

-- DROP INDEX IF EXISTS public.fki_entity_clients_pay_items__auth_entities_fkey;

CREATE INDEX IF NOT EXISTS fki_entity_clients_pay_items__auth_entities_fkey
    ON public.entity_clients_branch_pay_items USING btree
    (branch_ulid ASC NULLS LAST)
    TABLESPACE pg_default;

-- Trigger: mdt_entity_clients_payitems

-- DROP TRIGGER IF EXISTS mdt_entity_clients_payitems ON public.entity_clients_branch_pay_items;

CREATE TRIGGER mdt_entity_clients_payitems
    BEFORE UPDATE 
    ON public.entity_clients_branch_pay_items
    FOR EACH ROW
    EXECUTE FUNCTION public.moddatetime('updated_at');

COMMENT ON TRIGGER mdt_entity_clients_payitems ON public.entity_clients_branch_pay_items
    IS 'update date modifieds';
