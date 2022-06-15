-- Table: public.cost_center

-- DROP TABLE IF EXISTS public.cost_center;

CREATE TABLE IF NOT EXISTS public.cost_center
(
    ulid uuid NOT NULL,
    branch_ulid uuid NOT NULL,
    cost_center_name text COLLATE pg_catalog."default" NOT NULL,
    currency text COLLATE pg_catalog."default" NOT NULL,
    CONSTRAINT cost_center_pkey PRIMARY KEY (ulid),
    CONSTRAINT currency_fkey FOREIGN KEY (currency)
        REFERENCES public.currency_codes (code) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE NO ACTION
        NOT VALID
)

TABLESPACE pg_default;

ALTER TABLE IF EXISTS public.cost_center
    OWNER to postgres;


-- Table: public.cost_center_contractors

-- DROP TABLE IF EXISTS public.cost_center_contractors;

CREATE TABLE IF NOT EXISTS public.cost_center_contractors
(
    cost_center_ulid uuid NOT NULL,
    contractor_ulid uuid NOT NULL,
    CONSTRAINT cost_center_contractors_pkey PRIMARY KEY (cost_center_ulid, contractor_ulid),
    CONSTRAINT cost_center_contractors_fkey FOREIGN KEY (cost_center_ulid)
        REFERENCES public.cost_center (ulid) MATCH SIMPLE
        ON UPDATE CASCADE
        ON DELETE CASCADE
)

TABLESPACE pg_default;

ALTER TABLE IF EXISTS public.cost_center_contractors
    OWNER to postgres;



-- View: public.cost_center_index

-- DROP VIEW public.cost_center_index;

CREATE OR REPLACE VIEW public.cost_center_index
 AS
 SELECT cost_center.ulid,
    cost_center.branch_ulid,
    cost_center.cost_center_name,
    cost_center.currency,
    ( SELECT count(*) AS count
           FROM cost_center_contractors
          WHERE cost_center_contractors.cost_center_ulid = cost_center.ulid) AS member
   FROM cost_center;

ALTER TABLE public.cost_center_index
    OWNER TO postgres;




-- View: public.cost_center_contractors_details

-- DROP VIEW public.cost_center_contractors_details;

CREATE OR REPLACE VIEW public.cost_center_contractors_details
 AS
 WITH contractors AS (
         SELECT entity_contractor_account_details.ulid AS contractor_ulid,
            entity_contractor_account_details.company_name AS contractor_name,
            entity_contractor_account_details.email_address
           FROM entity_contractor_account_details
        UNION
         SELECT individual_contractor_account_details.ulid AS contractor_ulid,
            concat(individual_contractor_account_details.first_name, ' ', individual_contractor_account_details.last_name) AS contractor_name,
            individual_contractor_account_details.email_address
           FROM individual_contractor_account_details
        ), client_branches AS (
         SELECT entity_client_branch_account_details.ulid AS branch_ulid,
            entity_client_branch_account_details.branch_name,
            entity_client_branch_account_details.country
           FROM entity_client_branch_account_details
        )
 SELECT contractors.contractor_ulid,
    contractors.contractor_name,
    client_branches.branch_ulid,
    client_branches.branch_name,
    cost_center.cost_center_name,
    cost_center.ulid AS cost_center_ulid,
    cost_center.currency,
    client_branches.country
   FROM contractors
     JOIN entity_contractor_branch_pairs ON contractors.contractor_ulid = entity_contractor_branch_pairs.contractor_ulid
     JOIN client_branches ON entity_contractor_branch_pairs.branch_ulid = client_branches.branch_ulid
     JOIN cost_center ON client_branches.branch_ulid = cost_center.branch_ulid
     JOIN cost_center_contractors ON cost_center.ulid = cost_center_contractors.cost_center_ulid;

ALTER TABLE public.cost_center_contractors_details
    OWNER TO postgres;



-- Table: public.teams

-- DROP TABLE IF EXISTS public.teams;

CREATE TABLE IF NOT EXISTS public.teams
(
    ulid uuid NOT NULL,
    branch_ulid uuid NOT NULL,
    team_name text COLLATE pg_catalog."default" NOT NULL,
    schedule_type text COLLATE pg_catalog."default" NOT NULL,
    time_zone text COLLATE pg_catalog."default" NOT NULL,
    working_days_sun boolean NOT NULL,
    working_days_mon boolean NOT NULL,
    working_days_tue boolean NOT NULL,
    working_days_wed boolean NOT NULL,
    working_days_thu boolean NOT NULL,
    working_days_fri boolean NOT NULL,
    working_days_sat boolean NOT NULL,
    working_hours_sun_start timestamp with time zone NOT NULL,
    working_hours_mon_start timestamp with time zone NOT NULL,
    working_hours_tue_start timestamp with time zone NOT NULL,
    working_hours_wed_start timestamp with time zone NOT NULL,
    working_hours_thu_start timestamp with time zone NOT NULL,
    working_hours_fri_start timestamp with time zone NOT NULL,
    working_hours_sat_start timestamp with time zone NOT NULL,
    working_hours_sun_end timestamp with time zone NOT NULL,
    working_hours_mon_end timestamp with time zone NOT NULL,
    working_hours_tue_end timestamp with time zone NOT NULL,
    working_hours_wed_end timestamp with time zone NOT NULL,
    working_hours_thu_end timestamp with time zone NOT NULL,
    working_hours_fri_end timestamp with time zone NOT NULL,
    working_hours_sat_end timestamp with time zone NOT NULL,
    CONSTRAINT teams_pkey PRIMARY KEY (ulid),
    CONSTRAINT branch_ulid_fkey FOREIGN KEY (branch_ulid)
        REFERENCES public.entity_client_branches (ulid) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE NO ACTION
)

TABLESPACE pg_default;

ALTER TABLE IF EXISTS public.teams
    OWNER to postgres;



-- Table: public.teams_contractors

-- DROP TABLE IF EXISTS public.teams_contractors;

CREATE TABLE IF NOT EXISTS public.teams_contractors
(
    team_ulid uuid NOT NULL,
    contractor_ulid uuid NOT NULL,
    CONSTRAINT teams_contractors_pkey PRIMARY KEY (team_ulid, contractor_ulid),
    CONSTRAINT team_ulid_fkey FOREIGN KEY (team_ulid)
        REFERENCES public.teams (ulid) MATCH SIMPLE
        ON UPDATE CASCADE
        ON DELETE CASCADE
)

TABLESPACE pg_default;

ALTER TABLE IF EXISTS public.teams_contractors
    OWNER to postgres;


-- View: public.teams_index

-- DROP VIEW public.teams_index;

CREATE OR REPLACE VIEW public.teams_index
 AS
 SELECT teams.ulid AS team_ulid,
    teams.branch_ulid,
    teams.team_name,
    teams.schedule_type,
    teams.time_zone,
    teams.working_days_sun,
    teams.working_days_mon,
    teams.working_days_tue,
    teams.working_days_wed,
    teams.working_days_thu,
    teams.working_days_fri,
    teams.working_days_sat,
    teams.working_hours_sun_start,
    teams.working_hours_mon_start,
    teams.working_hours_tue_start,
    teams.working_hours_wed_start,
    teams.working_hours_thu_start,
    teams.working_hours_fri_start,
    teams.working_hours_sat_start,
    teams.working_hours_sun_end,
    teams.working_hours_mon_end,
    teams.working_hours_tue_end,
    teams.working_hours_wed_end,
    teams.working_hours_thu_end,
    teams.working_hours_fri_end,
    teams.working_hours_sat_end,
    ( SELECT count(*) AS count
           FROM teams_contractors
          WHERE teams_contractors.team_ulid = teams.ulid) AS member
   FROM teams;

ALTER TABLE public.teams_index
    OWNER TO postgres;



-- View: public.team_contractors_details

-- DROP VIEW public.team_contractors_details;

CREATE OR REPLACE VIEW public.team_contractors_details
 AS
 WITH contractors AS (
         SELECT entity_contractor_account_details.ulid AS contractor_ulid,
            entity_contractor_account_details.company_name AS contractor_name,
            entity_contractor_account_details.email_address
           FROM entity_contractor_account_details
        UNION
         SELECT individual_contractor_account_details.ulid AS contractor_ulid,
            concat(individual_contractor_account_details.first_name, ' ', individual_contractor_account_details.last_name) AS contractor_name,
            individual_contractor_account_details.email_address
           FROM individual_contractor_account_details
        ), client_branches AS (
         SELECT entity_client_branch_account_details.ulid AS branch_ulid,
            entity_client_branch_account_details.branch_name,
            entity_client_branch_account_details.country
           FROM entity_client_branch_account_details
        )
 SELECT contractors.contractor_ulid,
    contractors.contractor_name,
    client_branches.branch_ulid,
    client_branches.branch_name,
    teams.team_name,
    teams.ulid AS team_ulid,
    client_branches.country
   FROM contractors
     JOIN entity_contractor_branch_pairs ON contractors.contractor_ulid = entity_contractor_branch_pairs.contractor_ulid
     JOIN client_branches ON entity_contractor_branch_pairs.branch_ulid = client_branches.branch_ulid
     JOIN teams ON client_branches.branch_ulid = teams.branch_ulid
     JOIN teams_contractors ON teams.ulid = teams_contractors.team_ulid;

ALTER TABLE public.team_contractors_details
    OWNER TO postgres;

