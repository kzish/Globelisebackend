CREATE TABLE public.entity_client_branches (
    ulid UUID NOT NULL PRIMARY KEY,
    client_ulid UUID NOT NULL REFERENCES auth_entities(ulid)
);

CREATE TABLE public.entity_clients_branch_details (
    ulid uuid NOT NULL REFERENCES entity_client_branches(ulid),
    company_name TEXT NOT NULL,
    country TEXT NOT NULL,
    entity_type TEXT NOT NULL,
    registration_number TEXT,
    tax_id TEXT,
    statutory_contribution_submission_number TEXT,
    company_address TEXT NOT NULL,
    city TEXT NOT NULL,
    postal_code TEXT NOT NULL,
    created_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL
);

CREATE TABLE public.entity_clients_bank_details (
    ulid uuid NOT NULL REFERENCES entity_client_branches(ulid),
    currency CURRENCY NOT NULL,
    bank_name TEXT NOT NULL,
    bank_account_name TEXT NOT NULL,
    bank_account_number TEXT NOT NULL,
    swift_code TEXT,
    bank_key TEXT,
    iban TEXT,
    bank_code TEXT,
    branch_code TEXT,
    created_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL
);

CREATE TABLE public.entity_clients_payroll_details (
    ulid uuid NOT NULL REFERENCES entity_client_branches(ulid),
    cutoff_date DATE NOT NULL,
    payment_date DATE NOT NULL,
    created_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL
);

