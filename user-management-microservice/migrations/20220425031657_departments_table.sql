CREATE TABLE entity_client_branch_departments (
    ulid uuid NOT NULL PRIMARY KEY,
    branch_ulid uuid NOT NULL REFERENCES entity_client_branches(ulid),
    department_name TEXT NOT NULL,
    country TEXT NOT NULL,
    classification TEXT NOT NULL,
    currency CURRENCY,
    created_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL
);

CREATE TRIGGER mdt_entity_client_branch_departments BEFORE UPDATE ON entity_client_branch_departments FOR EACH ROW EXECUTE FUNCTION public.moddatetime('updated_at');

CREATE TABLE individual_contractor_department_pairs (
    individual_ulid uuid NOT NULL REFERENCES auth_individuals(ulid),
    department_ulid uuid NOT NULL REFERENCES entity_client_branch_departments(ulid),
    created_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    PRIMARY KEY (individual_ulid, department_ulid)
);

CREATE TRIGGER mdt_individual_contractor_department_pairs BEFORE UPDATE ON individual_contractor_department_pairs FOR EACH ROW EXECUTE FUNCTION public.moddatetime('updated_at');

CREATE TABLE entity_contractor_department_pairs (
    entity_ulid uuid NOT NULL REFERENCES auth_entities(ulid),
    department_ulid uuid NOT NULL REFERENCES entity_client_branch_departments(ulid),
    created_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    PRIMARY KEY (entity_ulid, department_ulid)
);

CREATE TRIGGER mdt_entity_contractor_department_pairs BEFORE UPDATE ON entity_contractor_department_pairs FOR EACH ROW EXECUTE FUNCTION public.moddatetime('updated_at');

CREATE VIEW entity_client_branch_departments_index AS
    WITH department_individual_contractor_count AS (
        SELECT
            department_ulid,
            COUNT(*) as c
        FROM
            individual_contractor_department_pairs
        GROUP BY
            department_ulid
    ),
    department_entity_contractor_count AS (
        SELECT
            department_ulid,
            COUNT(*) as c
        FROM
            entity_contractor_department_pairs
        GROUP BY
            department_ulid
    )
    SELECT
        a.ulid,
        a.branch_ulid,
        e.company_name AS branch_name,
        a.department_name,
        a.country,
        a.classification,
        a.currency,
        COALESCE(b.c, 0) + COALESCE(c.c, 0) AS total_member,
        d.client_ulid
    FROM
        entity_client_branch_departments a
    LEFT JOIN
        department_individual_contractor_count b
    ON
        a.ulid = b.department_ulid
    LEFT JOIN
        department_entity_contractor_count c
    ON
        a.ulid = c.department_ulid
    JOIN
        entity_client_branches d
    ON 
        a.branch_ulid = d.ulid
    JOIN
        entity_clients_branch_details e
    ON 
        a.branch_ulid = d.ulid;
