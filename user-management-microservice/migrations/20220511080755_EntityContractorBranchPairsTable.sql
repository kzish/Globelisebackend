-- Table: public.entity_contractor_branch_pairs

-- DROP TABLE IF EXISTS public.entity_contractor_branch_pairs;

CREATE TABLE IF NOT EXISTS public.entity_contractor_branch_pairs
(
    contractor_ulid uuid NOT NULL,
    branch_ulid uuid NOT NULL,
    created_at timestamp with time zone NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at timestamp with time zone NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT entity_contractor_branch_pairs_pkey PRIMARY KEY (contractor_ulid, branch_ulid),
    CONSTRAINT entity_contractor_branch_pairs_branch_ulid_fkey FOREIGN KEY (branch_ulid)
        REFERENCES public.entity_client_branches (ulid) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE NO ACTION,
    CONSTRAINT entity_contractor_branch_pairs_contractor_ulid_fkey FOREIGN KEY (contractor_ulid)
        REFERENCES public.auth_entities (ulid) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE NO ACTION
)

TABLESPACE pg_default;

ALTER TABLE IF EXISTS public.entity_contractor_branch_pairs
    OWNER to postgres;

-- Trigger: mdt_entity_contractor_branch_pairs

DROP TRIGGER IF EXISTS mdt_entity_contractor_branch_pairs ON public.entity_contractor_branch_pairs;

CREATE TRIGGER mdt_entity_contractor_branch_pairs
    BEFORE UPDATE 
    ON public.entity_contractor_branch_pairs
    FOR EACH ROW
    EXECUTE FUNCTION public.moddatetime('updated_at');
