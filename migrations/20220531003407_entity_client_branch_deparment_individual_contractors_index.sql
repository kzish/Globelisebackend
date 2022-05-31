CREATE VIEW entity_client_branch_deparment_individual_contractors_index AS
SELECT
    c.ulid,
    b.branch_ulid,
    b.branch_name,
    b.ulid as department_ulid,
    b.department_name,
    b.classification
FROM
    individual_contractor_department_pairs a
    JOIN entity_client_branch_departments_index b ON a.department_ulid = b.ulid
    JOIN onboarded_user_index c ON c.ulid = a.individual_ulid
    JOIN contracts d ON d.contractor_ulid = c.ulid
    AND d.branch_ulid = b.branch_ulid;
