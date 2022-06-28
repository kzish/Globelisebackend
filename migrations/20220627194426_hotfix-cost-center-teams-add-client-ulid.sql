-- View: public.cost_center_index

DROP VIEW public.cost_center_index;

CREATE OR REPLACE VIEW public.cost_center_index
 AS
 SELECT cost_center.ulid,
    cost_center.branch_ulid,
    cost_center.cost_center_name,
    cost_center.currency,
    entity_client_branches.client_ulid,
    ( SELECT count(*) AS count
           FROM cost_center_contractors
          WHERE cost_center_contractors.cost_center_ulid = cost_center.ulid) AS member
   FROM cost_center
     JOIN entity_client_branches ON cost_center.branch_ulid = entity_client_branches.ulid;

ALTER TABLE public.cost_center_index
    OWNER TO postgres;


-- View: public.teams_index

DROP VIEW public.teams_index;

CREATE OR REPLACE VIEW public.teams_index
 AS
 SELECT teams.ulid AS team_ulid,
    teams.branch_ulid,
    entity_client_branches.client_ulid,
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
   FROM teams
     JOIN entity_client_branches ON teams.branch_ulid = entity_client_branches.ulid;

ALTER TABLE public.teams_index
    OWNER TO postgres;



