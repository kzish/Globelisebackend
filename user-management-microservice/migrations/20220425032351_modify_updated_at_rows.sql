CREATE TRIGGER mdt_client_contractor_pairs BEFORE UPDATE ON client_contractor_pairs FOR EACH ROW EXECUTE FUNCTION public.moddatetime('updated_at');

CREATE TRIGGER mdt_entity_client_branches BEFORE UPDATE ON entity_client_branches FOR EACH ROW EXECUTE FUNCTION public.moddatetime('updated_at');

CREATE TRIGGER mdt_entity_clients_branch_account_details BEFORE UPDATE ON entity_clients_branch_account_details FOR EACH ROW EXECUTE FUNCTION public.moddatetime('updated_at');

CREATE TRIGGER mdt_entity_clients_branch_bank_details BEFORE UPDATE ON entity_clients_branch_bank_details FOR EACH ROW EXECUTE FUNCTION public.moddatetime('updated_at');

CREATE TRIGGER mdt_entity_clients_branch_payroll_details BEFORE UPDATE ON entity_clients_branch_payroll_details FOR EACH ROW EXECUTE FUNCTION public.moddatetime('updated_at');

