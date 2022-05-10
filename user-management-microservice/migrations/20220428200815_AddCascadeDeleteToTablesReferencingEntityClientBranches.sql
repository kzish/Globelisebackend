-- Add migration script here
-- add cascading delete to tables referencing entity_client_branches

begin;

alter table entity_clients_branch_account_details
drop constraint entity_clients_branch_details_ulid_fkey;

alter table entity_clients_branch_account_details
add constraint entity_clients_branch_details_ulid_fkey
foreign key (ulid)
references entity_client_branches (ulid)
on delete cascade;

commit;


begin;

alter table entity_clients_branch_bank_details
drop constraint entity_clients_bank_details_ulid_fkey;

alter table entity_clients_branch_bank_details
add constraint entity_clients_bank_details_ulid_fkey
foreign key (ulid)
references entity_client_branches (ulid)
on delete cascade;

commit;


begin;

alter table entity_clients_branch_payroll_details
drop constraint entity_clients_payroll_details_ulid_fkey;

alter table entity_clients_branch_payroll_details
add constraint entity_clients_payroll_details_ulid_fkey
foreign key (ulid)
references entity_client_branches (ulid)
on delete cascade;

commit;

