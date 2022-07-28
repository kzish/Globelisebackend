delete from contracts_additional_documents;

ALTER TABLE IF EXISTS public.contracts_additional_documents DROP COLUMN IF EXISTS contract_ulid;

ALTER TABLE IF EXISTS public.contracts_additional_documents
    ADD COLUMN contract_ulid uuid NOT NULL;


    
ALTER TABLE IF EXISTS public.contracts_additional_documents DROP CONSTRAINT IF EXISTS contracts_additional_documents_fkey;

ALTER TABLE IF EXISTS public.contracts_additional_documents
    ADD CONSTRAINT contracts_additional_documents_fkey FOREIGN KEY (contract_ulid)
    REFERENCES public.contracts (ulid) MATCH SIMPLE
    ON UPDATE CASCADE
    ON DELETE CASCADE
    NOT VALID;



ALTER TABLE IF EXISTS public.contracts_pay_items DROP CONSTRAINT IF EXISTS contracts_pay_items_fkey;

ALTER TABLE IF EXISTS public.contracts_pay_items
    ADD CONSTRAINT contracts_pay_items_fkey FOREIGN KEY (contract_ulid)
    REFERENCES public.contracts (ulid) MATCH SIMPLE
    ON UPDATE CASCADE
    ON DELETE CASCADE
    NOT VALID;


ALTER TABLE IF EXISTS public.contracts_pay_items DROP CONSTRAINT IF EXISTS pay_items_fkey;

ALTER TABLE IF EXISTS public.contracts_pay_items
    ADD CONSTRAINT pay_items_fkey FOREIGN KEY (pay_item_ulid)
    REFERENCES public.entity_client_branch_pay_items (ulid) MATCH SIMPLE
    ON UPDATE CASCADE
    ON DELETE CASCADE
    NOT VALID;