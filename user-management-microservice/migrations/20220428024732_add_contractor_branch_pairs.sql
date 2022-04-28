CREATE TABLE individual_contractor_branch_pairs (
    contractor_ulid UUID NOT NULL REFERENCES auth_individuals(ulid),
    branch_ulid UUID NOT NULL REFERENCES entity_client_branches(ulid),
    created_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    PRIMARY KEY (contractor_ulid, branch_ulid)
);

CREATE TRIGGER mdt_individual_contractor_branch_pairs BEFORE UPDATE ON individual_contractor_branch_pairs FOR EACH ROW EXECUTE FUNCTION public.moddatetime('updated_at');
