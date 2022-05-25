CREATE TABLE user_detail_types (
    ulid UUID NOT NULL PRIMARY KEY,
    detail_type TEXT NOT NULL,
    created_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL
);

INSERT INTO
    user_detail_types
VALUES
    (gen_random_uuid(), 'PERSONAL');

INSERT INTO
    user_detail_types
VALUES
    (gen_random_uuid(), 'EMPLOYMENT');

INSERT INTO
    user_detail_types
VALUES
    (gen_random_uuid(), 'BANK');

INSERT INTO
    user_detail_types
VALUES
    (gen_random_uuid(), 'PAYROLL');

CREATE TABLE entity_client_custom_fields (
    ulid UUID NOT NULL PRIMARY KEY,
    client_ulid UUID NOT NULL REFERENCES auth_entities(ulid),
    field_name TEXT NOT NULL,
    field_detail_type_ulid UUID NOT NULL REFERENCES user_detail_types(ulid),
    field_format TEXT NOT NULL,
    field_option_1 TEXT NOT NULL,
    field_option_2 TEXT NOT NULL,
    created_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL
);

CREATE VIEW entity_client_custom_fields_index AS
SELECT
    a.ulid,
    a.client_ulid,
    a.field_name,
    b.detail_type AS field_detail_type,
    a.field_format,
    a.field_option_1,
    a.field_option_2
FROM
    entity_client_custom_fields a
    JOIN user_detail_types b ON a.field_detail_type_ulid = b.ulid;