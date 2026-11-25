\set ON_ERROR_STOP 1
SET search_path = public, test_tools;
BEGIN;
\ir ../fixture.sql
SELECT plan(7);
SELECT pg_temp.new_model('a6000000-0000-7000-8000-000000000001', 'review model', NULL);
SELECT pg_temp.add_member('20000000-0000-7000-8000-000000000001', '10000000-0000-7000-8000-000000000002');
INSERT INTO grants (organization_id, resource_id, grantee_id, grantee_kind, role)
VALUES ('20000000-0000-7000-8000-000000000001', '20000000-0000-7000-8000-000000000001', '10000000-0000-7000-8000-000000000002', 'user', 'writer');
SET CONSTRAINTS ALL IMMEDIATE;
SET CONSTRAINTS ALL DEFERRED;
SELECT set_config('app.user_id', '10000000-0000-7000-8000-000000000002', false);
INSERT INTO organizations (id, created_by_id) VALUES ('20000000-0000-7000-8000-000000000002', authz.caller_id());
INSERT INTO resources (id, kind, organization_id) VALUES ('20000000-0000-7000-8000-000000000002', 'organization', '20000000-0000-7000-8000-000000000002');
INSERT INTO memberships (organization_id, user_id, state) VALUES ('20000000-0000-7000-8000-000000000002', authz.caller_id(), 'active');
INSERT INTO grants (organization_id, resource_id, grantee_id, grantee_kind, role) VALUES ('20000000-0000-7000-8000-000000000002', '20000000-0000-7000-8000-000000000002', authz.caller_id(), 'user', 'admin');
SET CONSTRAINTS ALL IMMEDIATE;
SET CONSTRAINTS ALL DEFERRED;
INSERT INTO resource_parents (organization_id, id, resource_kind, parent_id, parent_kind) VALUES ('20000000-0000-7000-8000-000000000002', '50000000-0000-7000-8000-000000000002', 'project', '20000000-0000-7000-8000-000000000002', 'organization');
INSERT INTO resources (id, kind, organization_id) VALUES ('50000000-0000-7000-8000-000000000002', 'project', '20000000-0000-7000-8000-000000000002');
SET CONSTRAINTS ALL IMMEDIATE;
SET CONSTRAINTS ALL DEFERRED;
INSERT INTO ownables (id, organization_id, project_id, created_by_id)
SELECT identifier, '20000000-0000-7000-8000-000000000002', '50000000-0000-7000-8000-000000000002', authz.caller_id()
FROM unnest(ARRAY['c6000000-0000-7000-8000-000000000001'::uuid, 'c6000000-0000-7000-8000-000000000002'::uuid]) identifier;
INSERT INTO resources (id, kind, organization_id)
SELECT identifier, 'ownable', '20000000-0000-7000-8000-000000000002'::uuid
FROM unnest(ARRAY['c6000000-0000-7000-8000-000000000001'::uuid, 'c6000000-0000-7000-8000-000000000002'::uuid]) identifier;
INSERT INTO procedure_templates (id, organization_id, project_id, table_name_id, name)
SELECT id, organization_id, project_id, 'procedure_templates', id::text FROM ownables WHERE organization_id = '20000000-0000-7000-8000-000000000002';
INSERT INTO procedure_template_slots (id, organization_id, procedure_template_id, name, asset_model_id, role_id, minimum_count, maximum_count)
VALUES ('d6000000-0000-7000-8000-000000000001', '20000000-0000-7000-8000-000000000002', 'c6000000-0000-7000-8000-000000000001', 'input', 'a6000000-0000-7000-8000-000000000001', 'input', 0, 1);
UPDATE procedure_templates SET state_id = 'published' WHERE id = 'c6000000-0000-7000-8000-000000000001';
INSERT INTO procedure_template_steps (id, organization_id, parent_template_id, child_template_id, name, position)
VALUES ('e6000000-0000-7000-8000-000000000001', '20000000-0000-7000-8000-000000000002', 'c6000000-0000-7000-8000-000000000002', 'c6000000-0000-7000-8000-000000000001', 'first', 0),
('e6000000-0000-7000-8000-000000000002', '20000000-0000-7000-8000-000000000002', 'c6000000-0000-7000-8000-000000000002', 'c6000000-0000-7000-8000-000000000001', 'second', 1);
SELECT pg_temp.new_edge('e6000000-0000-7000-8000-000000000001', 'e6000000-0000-7000-8000-000000000002');
UPDATE procedure_templates SET state_id = 'published' WHERE id = 'c6000000-0000-7000-8000-000000000002';
INSERT INTO grants (organization_id, resource_id, grantee_id, grantee_kind, role)
VALUES ('20000000-0000-7000-8000-000000000002', '20000000-0000-7000-8000-000000000002', '10000000-0000-7000-8000-000000000001', 'user', 'reader');
SET CONSTRAINTS ALL IMMEDIATE;
SET CONSTRAINTS ALL DEFERRED;
SELECT set_config('app.user_id', '10000000-0000-7000-8000-000000000001', false);
SELECT pg_temp.new_procedure('f6000000-0000-7000-8000-000000000001', 'c6000000-0000-7000-8000-000000000002', 'in_progress', NULL, NULL);
UPDATE procedures SET started_at = now() - interval '3 hours' WHERE id = 'f6000000-0000-7000-8000-000000000001';
SELECT pg_temp.new_procedure('f6000000-0000-7000-8000-000000000002', 'c6000000-0000-7000-8000-000000000001', 'in_progress', 'f6000000-0000-7000-8000-000000000001', 'e6000000-0000-7000-8000-000000000001');
UPDATE procedures SET started_at = now() - interval '2 hours' WHERE id = 'f6000000-0000-7000-8000-000000000002';
UPDATE procedures SET status_id = 'completed', ended_at = now() - interval '1 hour' WHERE id = 'f6000000-0000-7000-8000-000000000002';
SET CONSTRAINTS ALL IMMEDIATE;
SET CONSTRAINTS ALL DEFERRED;
SAVEPOINT ordering;
SELECT throws_ok($statement$SELECT pg_temp.new_procedure('f6000000-0000-7000-8000-000000000003', 'c6000000-0000-7000-8000-000000000001', 'in_progress', 'f6000000-0000-7000-8000-000000000001', 'e6000000-0000-7000-8000-000000000002');
UPDATE procedures SET started_at = now() - interval '90 minutes' WHERE id = 'f6000000-0000-7000-8000-000000000003'; SET CONSTRAINTS ALL IMMEDIATE$statement$, '23514', 'procedure f6000000-0000-7000-8000-000000000003 starts before its predecessor run f6000000-0000-7000-8000-000000000002 ended', 'shared template predecessor times are enforced');
ROLLBACK TO ordering;
SET CONSTRAINTS ALL DEFERRED;
SELECT lives_ok($statement$SELECT pg_temp.new_procedure('f6000000-0000-7000-8000-000000000003', 'c6000000-0000-7000-8000-000000000001', 'in_progress', 'f6000000-0000-7000-8000-000000000001', 'e6000000-0000-7000-8000-000000000002'); SET CONSTRAINTS ALL IMMEDIATE$statement$, 'shared template successor starts after predecessor ends');
SET CONSTRAINTS ALL DEFERRED;
SELECT pg_temp.new_procedure('f6000000-0000-7000-8000-000000000040', 'c6000000-0000-7000-8000-000000000002', 'planned', NULL, NULL);
SELECT set_config('app.user_id', '10000000-0000-7000-8000-000000000002', false);
SELECT throws_ok($statement$
INSERT INTO ownables (id, organization_id, project_id, created_by_id) VALUES ('f6000000-0000-7000-8000-000000000004', '20000000-0000-7000-8000-000000000002', '50000000-0000-7000-8000-000000000002', authz.caller_id());
INSERT INTO resources (id, kind, organization_id) VALUES ('f6000000-0000-7000-8000-000000000004', 'ownable', '20000000-0000-7000-8000-000000000002');
INSERT INTO procedures (id, organization_id, project_id, table_name_id, procedure_template_id, parent_procedure_id, parent_template_id, step_id, root_procedure_id)
VALUES ('f6000000-0000-7000-8000-000000000004', '20000000-0000-7000-8000-000000000002', '50000000-0000-7000-8000-000000000002', 'procedures', 'c6000000-0000-7000-8000-000000000001', 'f6000000-0000-7000-8000-000000000040', 'c6000000-0000-7000-8000-000000000002', 'e6000000-0000-7000-8000-000000000002', 'f6000000-0000-7000-8000-000000000040'); SET CONSTRAINTS ALL IMMEDIATE$statement$, '23503', 'insert or update on table "procedures" violates foreign key constraint "procedures_parent_procedure_id_parent_template_id_organiza_fkey"', 'child procedure must share parent organization');
SELECT set_config('app.user_id', '10000000-0000-7000-8000-000000000001', false);
SET CONSTRAINTS ALL DEFERRED;
SELECT pg_temp.new_procedure('f6000000-0000-7000-8000-000000000020', 'c6000000-0000-7000-8000-000000000002', 'planned', NULL, NULL);
SELECT pg_temp.new_procedure('f6000000-0000-7000-8000-000000000021', 'c6000000-0000-7000-8000-000000000001', 'planned', 'f6000000-0000-7000-8000-000000000020', 'e6000000-0000-7000-8000-000000000001');
SAVEPOINT skip_reason;
SELECT throws_ok($statement$UPDATE procedures SET status_id = 'skipped', ended_at = now(), note = '   ' WHERE id = 'f6000000-0000-7000-8000-000000000021'; SET CONSTRAINTS ALL IMMEDIATE$statement$, '23514', 'procedure f6000000-0000-7000-8000-000000000021 skips a required step without a reason in its note', 'required skip rejects blank reason');
ROLLBACK TO skip_reason;
SET CONSTRAINTS ALL DEFERRED;
UPDATE procedures SET status_id = 'skipped', ended_at = now(), note = 'equipment unavailable' WHERE id = 'f6000000-0000-7000-8000-000000000021';
SELECT throws_ok($statement$UPDATE procedures SET note = '   ' WHERE id = 'f6000000-0000-7000-8000-000000000021'; SET CONSTRAINTS ALL IMMEDIATE$statement$, '23514', 'procedure f6000000-0000-7000-8000-000000000021 skips a required step without a reason in its note', 'required skipped step keeps nonblank reason');
SET CONSTRAINTS ALL DEFERRED;
SELECT pg_temp.new_template('c6000000-0000-7000-8000-000000000010', 'produce optionally');
SELECT pg_temp.new_slot('d6000000-0000-7000-8000-000000000010', 'c6000000-0000-7000-8000-000000000010', 'product', 'a6000000-0000-7000-8000-000000000001', 'produced', 0, 1);
UPDATE procedure_templates SET state_id = 'published' WHERE id = 'c6000000-0000-7000-8000-000000000010';
SELECT pg_temp.new_template('c6000000-0000-7000-8000-000000000011', 'composite production');
SELECT pg_temp.new_slot('d6000000-0000-7000-8000-000000000011', 'c6000000-0000-7000-8000-000000000011', 'product', 'a6000000-0000-7000-8000-000000000001', 'produced', 0, 1);
SELECT pg_temp.new_step('e6000000-0000-7000-8000-000000000010', 'c6000000-0000-7000-8000-000000000011', 'c6000000-0000-7000-8000-000000000010', 'produce', 0);
UPDATE procedure_template_steps SET optional = true WHERE id = 'e6000000-0000-7000-8000-000000000010';
SELECT pg_temp.new_binding('e6000000-0000-7000-8000-000000000010', 'd6000000-0000-7000-8000-000000000010', 'd6000000-0000-7000-8000-000000000011');
UPDATE procedure_templates SET state_id = 'published' WHERE id = 'c6000000-0000-7000-8000-000000000011';
SELECT pg_temp.new_procedure('f6000000-0000-7000-8000-000000000030', 'c6000000-0000-7000-8000-000000000011', 'in_progress', NULL, NULL);
SELECT pg_temp.bind_model('96000000-0000-7000-8000-000000000030', 'f6000000-0000-7000-8000-000000000030', 'd6000000-0000-7000-8000-000000000011', NULL, 'a6000000-0000-7000-8000-000000000001');
SAVEPOINT completion;
SELECT throws_ok($statement$UPDATE procedures SET status_id = 'completed', ended_at = now() WHERE id = 'f6000000-0000-7000-8000-000000000030'; SET CONSTRAINTS ALL IMMEDIATE$statement$, '23514', 'procedure f6000000-0000-7000-8000-000000000030 completes with an unfulfilled asset binding', 'completion requires child fulfillment of each parent effect binding');
ROLLBACK TO completion;
SET CONSTRAINTS ALL DEFERRED;
SELECT lives_ok($statement$SELECT pg_temp.new_procedure('f6000000-0000-7000-8000-000000000031', 'c6000000-0000-7000-8000-000000000010', 'in_progress', 'f6000000-0000-7000-8000-000000000030', 'e6000000-0000-7000-8000-000000000010');
SELECT pg_temp.bind_model('96000000-0000-7000-8000-000000000031', 'f6000000-0000-7000-8000-000000000031', 'd6000000-0000-7000-8000-000000000010', NULL, 'a6000000-0000-7000-8000-000000000001');
UPDATE procedures SET status_id = 'completed', ended_at = now() WHERE id = 'f6000000-0000-7000-8000-000000000031';
UPDATE procedures SET status_id = 'completed', ended_at = now() WHERE id = 'f6000000-0000-7000-8000-000000000030'; SET CONSTRAINTS ALL IMMEDIATE$statement$, 'completed child fulfills model-only parent effect binding');
SELECT * FROM finish();
ROLLBACK;
