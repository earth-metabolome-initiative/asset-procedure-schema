\set ON_ERROR_STOP 1
SET search_path = public, test_tools;
BEGIN;
\ir ../fixture.sql
SELECT no_plan();
SELECT pg_temp.new_model('a8000000-0000-7000-8000-000000000001', 'ledger item family', NULL, 'physical_asset_models');
SELECT pg_temp.new_model('a8000000-0000-7000-8000-000000000002', 'ledger item', 'a8000000-0000-7000-8000-000000000001', 'physical_asset_models');
SELECT pg_temp.new_model('a8000000-0000-7000-8000-000000000003', 'ledger box', NULL, 'container_models');
SELECT pg_temp.new_model('a8000000-0000-7000-8000-000000000004', 'ledger freezer', NULL, 'container_models');
INSERT INTO physical_asset_models (id, organization_id) SELECT id, organization_id FROM asset_models WHERE id::text LIKE 'a8%';
INSERT INTO container_models (id, organization_id)
VALUES ('a8000000-0000-7000-8000-000000000003', '20000000-0000-7000-8000-000000000001'),
  ('a8000000-0000-7000-8000-000000000004', '20000000-0000-7000-8000-000000000001');
SELECT pg_temp.create_ownable('88000000-0000-7000-8000-000000000001');
SELECT pg_temp.create_ownable('88000000-0000-7000-8000-000000000002');
INSERT INTO container_compatibility_rules (id, organization_id, project_id, table_name_id, container_model_id, contained_asset_model_id, quantity)
VALUES ('88000000-0000-7000-8000-000000000001', '20000000-0000-7000-8000-000000000001', '50000000-0000-7000-8000-000000000001', 'container_compatibility_rules', 'a8000000-0000-7000-8000-000000000003', 'a8000000-0000-7000-8000-000000000001', 1),
  ('88000000-0000-7000-8000-000000000002', '20000000-0000-7000-8000-000000000001', '50000000-0000-7000-8000-000000000001', 'container_compatibility_rules', 'a8000000-0000-7000-8000-000000000004', 'a8000000-0000-7000-8000-000000000003', 1);
SELECT pg_temp.new_template('c8000000-0000-7000-8000-000000000001', 'ledger store');
SELECT pg_temp.new_slot('d8000000-0000-7000-8000-000000000001', 'c8000000-0000-7000-8000-000000000001', 'item', 'a8000000-0000-7000-8000-000000000001', 'input', 1, 1);
SELECT pg_temp.new_slot('d8000000-0000-7000-8000-000000000002', 'c8000000-0000-7000-8000-000000000001', 'box', 'a8000000-0000-7000-8000-000000000003', 'input', 1, 1);
UPDATE procedure_template_slots SET placed_into_slot_id = 'd8000000-0000-7000-8000-000000000002' WHERE id = 'd8000000-0000-7000-8000-000000000001';
UPDATE procedure_templates SET state_id = 'published' WHERE id = 'c8000000-0000-7000-8000-000000000001';
SELECT pg_temp.new_template('c8000000-0000-7000-8000-000000000002', 'ledger consume');
SELECT pg_temp.new_slot('d8000000-0000-7000-8000-000000000003', 'c8000000-0000-7000-8000-000000000002', 'consumed', 'a8000000-0000-7000-8000-000000000001', 'consumed', 1, 1);
UPDATE procedure_templates SET state_id = 'published' WHERE id = 'c8000000-0000-7000-8000-000000000002';
SELECT pg_temp.new_template('c8000000-0000-7000-8000-000000000003', 'ledger freeze');
SELECT pg_temp.new_slot('d8000000-0000-7000-8000-000000000004', 'c8000000-0000-7000-8000-000000000003', 'box', 'a8000000-0000-7000-8000-000000000003', 'input', 1, 1);
SELECT pg_temp.new_slot('d8000000-0000-7000-8000-000000000005', 'c8000000-0000-7000-8000-000000000003', 'freezer', 'a8000000-0000-7000-8000-000000000004', 'input', 1, 1);
UPDATE procedure_template_slots SET placed_into_slot_id = 'd8000000-0000-7000-8000-000000000005' WHERE id = 'd8000000-0000-7000-8000-000000000004';
UPDATE procedure_templates SET state_id = 'published' WHERE id = 'c8000000-0000-7000-8000-000000000003';
SELECT pg_temp.new_template('c8000000-0000-7000-8000-000000000011', 'ledger child');
SELECT pg_temp.new_slot('d8000000-0000-7000-8000-000000000011', 'c8000000-0000-7000-8000-000000000011', 'touched', 'a8000000-0000-7000-8000-000000000002', 'input', 0, 1);
UPDATE procedure_templates SET state_id = 'published' WHERE id = 'c8000000-0000-7000-8000-000000000011';
SELECT pg_temp.new_template('c8000000-0000-7000-8000-000000000010', 'ledger parent');
SELECT pg_temp.new_step('e8000000-0000-7000-8000-000000000001', 'c8000000-0000-7000-8000-000000000010', 'c8000000-0000-7000-8000-000000000011', 'touch', 0);
UPDATE procedure_templates SET state_id = 'published' WHERE id = 'c8000000-0000-7000-8000-000000000010';
SELECT pg_temp.new_asset('b8000000-0000-7000-8000-000000000001', 'item one', 'a8000000-0000-7000-8000-000000000002', 'physical_assets');
SELECT pg_temp.new_asset('b8000000-0000-7000-8000-000000000002', 'item two', 'a8000000-0000-7000-8000-000000000002', 'physical_assets');
SELECT pg_temp.new_asset('b8000000-0000-7000-8000-000000000003', 'item three', 'a8000000-0000-7000-8000-000000000002', 'physical_assets');
SELECT pg_temp.new_asset('b8000000-0000-7000-8000-000000000010', 'box', 'a8000000-0000-7000-8000-000000000003', 'containers');
SELECT pg_temp.new_asset('b8000000-0000-7000-8000-000000000011', 'freezer', 'a8000000-0000-7000-8000-000000000004', 'containers');
INSERT INTO physical_assets (id, organization_id, physical_asset_model_id) SELECT id, organization_id, model_id FROM assets WHERE id::text LIKE 'b8%';
INSERT INTO containers (id, organization_id, container_model_id) SELECT id, organization_id, model_id FROM assets WHERE table_name_id = 'containers' AND id::text LIKE 'b8%';
RESET ROLE;
-- Places `contained` into `destination` with the template given, binding them under ids derived from the run.
CREATE FUNCTION pg_temp.place(run UUID, template UUID, contained_slot UUID, destination_slot UUID, contained UUID, destination UUID,
    started TIMESTAMPTZ, ended TIMESTAMPTZ) RETURNS VOID LANGUAGE sql AS $$
SELECT pg_temp.new_procedure(run, template, 'in_progress', NULL, NULL);
UPDATE public.procedures SET started_at = started WHERE id = run;
SELECT pg_temp.bind(('9' || substring(run::text FROM 2))::uuid, run, contained_slot, contained);
SELECT pg_temp.bind(('8' || substring(run::text FROM 2))::uuid, run, destination_slot, destination);
UPDATE public.procedures SET status_id = 'completed', ended_at = ended WHERE id = run;
$$;
-- Starts consuming `consumed`, leaving the run in progress.
CREATE FUNCTION pg_temp.consume(run UUID, consumed UUID, started TIMESTAMPTZ) RETURNS VOID LANGUAGE sql AS $$
SELECT pg_temp.new_procedure(run, 'c8000000-0000-7000-8000-000000000002', 'in_progress', NULL, NULL);
UPDATE public.procedures SET started_at = started WHERE id = run;
SELECT pg_temp.bind(('9' || substring(run::text FROM 2))::uuid, run, 'd8000000-0000-7000-8000-000000000003', consumed);
$$;
GRANT EXECUTE ON FUNCTION pg_temp.place(UUID, UUID, UUID, UUID, UUID, UUID, TIMESTAMPTZ, TIMESTAMPTZ), pg_temp.consume(UUID, UUID, TIMESTAMPTZ),
  pg_temp.act(TEXT) TO app;
SET ROLE app;
SET CONSTRAINTS ALL IMMEDIATE;
SET CONSTRAINTS ALL DEFERRED;

SELECT pg_temp.place('f8000000-0000-7000-8000-000000000005', 'c8000000-0000-7000-8000-000000000003', 'd8000000-0000-7000-8000-000000000004',
  'd8000000-0000-7000-8000-000000000005', 'b8000000-0000-7000-8000-000000000010', 'b8000000-0000-7000-8000-000000000011', '2026-03-01 00:00+00', '2026-03-01 00:30+00');
SELECT pg_temp.place('f8000000-0000-7000-8000-000000000001', 'c8000000-0000-7000-8000-000000000001', 'd8000000-0000-7000-8000-000000000001',
  'd8000000-0000-7000-8000-000000000002', 'b8000000-0000-7000-8000-000000000001', 'b8000000-0000-7000-8000-000000000010', '2026-03-01 01:00+00', '2026-03-01 02:00+00');
SET CONSTRAINTS ALL IMMEDIATE;
SET CONSTRAINTS ALL DEFERRED;
SELECT ok((SELECT in_use FROM container_compatibility_rules WHERE id = '88000000-0000-7000-8000-000000000001'),
  'a placement marks the rule it counts against as used');
SELECT throws_ok($$UPDATE container_compatibility_rules SET quantity = 2 WHERE id = '88000000-0000-7000-8000-000000000001'$$, '23514',
  'definition 88000000-0000-7000-8000-000000000001 is used, so it is frozen', 'a used rule keeps its quantity');
SELECT throws_ok($$DELETE FROM container_compatibility_rules WHERE id = '88000000-0000-7000-8000-000000000001'$$, '23514',
  'definition 88000000-0000-7000-8000-000000000001 is used, so it is frozen', 'a used rule stays');
SELECT throws_ok($$INSERT INTO container_compatibility_rules (id, organization_id, project_id, table_name_id, container_model_id, contained_asset_model_id, quantity, in_use)
  VALUES ('88000000-0000-7000-8000-000000000009', '20000000-0000-7000-8000-000000000001', '50000000-0000-7000-8000-000000000001', 'container_compatibility_rules',
  'a8000000-0000-7000-8000-000000000003', 'a8000000-0000-7000-8000-000000000002', 1, TRUE)$$, '42501', 'permission denied for table container_compatibility_rules',
  'a client cannot create a rule already marked used');
SELECT throws_ok($$INSERT INTO volumetric_container_models (id, organization_id, volume_l, in_use)
  VALUES ('a8000000-0000-7000-8000-000000000003', '20000000-0000-7000-8000-000000000001', 1.0, TRUE)$$, '42501', 'permission denied for table volumetric_container_models',
  'a client cannot create a volumetric model already marked used');

SELECT pg_temp.consume('f8000000-0000-7000-8000-000000000002', 'b8000000-0000-7000-8000-000000000001', '2026-03-01 03:00+00');
SET CONSTRAINTS ALL IMMEDIATE;
SET CONSTRAINTS ALL DEFERRED;
SELECT lives_ok($$UPDATE procedures SET status_id = 'aborted', ended_at = '2026-03-01 03:30+00' WHERE id = 'f8000000-0000-7000-8000-000000000002'; SET CONSTRAINTS ALL IMMEDIATE$$,
  'aborting a started removal keeps the exit it recorded');
SET CONSTRAINTS ALL DEFERRED;
SELECT lives_ok($$SELECT pg_temp.place('f8000000-0000-7000-8000-000000000003', 'c8000000-0000-7000-8000-000000000001', 'd8000000-0000-7000-8000-000000000001',
  'd8000000-0000-7000-8000-000000000002', 'b8000000-0000-7000-8000-000000000002', 'b8000000-0000-7000-8000-000000000010', '2026-03-01 04:00+00', '2026-03-01 05:00+00'); SET CONSTRAINTS ALL IMMEDIATE$$,
  'the box admits a new item after the aborted removal of its last one');
SET CONSTRAINTS ALL DEFERRED;
SELECT lives_ok($$DELETE FROM procedures WHERE id = 'f8000000-0000-7000-8000-000000000002'; SET CONSTRAINTS ALL IMMEDIATE$$,
  'an aborted removal may be deleted');
SET CONSTRAINTS ALL DEFERRED;
SELECT pg_temp.consume('f8000000-0000-7000-8000-000000000004', 'b8000000-0000-7000-8000-000000000002', '2026-03-01 05:15+00');
SET CONSTRAINTS ALL IMMEDIATE;
SET CONSTRAINTS ALL DEFERRED;
SELECT lives_ok($$SELECT pg_temp.place('f8000000-0000-7000-8000-000000000009', 'c8000000-0000-7000-8000-000000000001', 'd8000000-0000-7000-8000-000000000001',
  'd8000000-0000-7000-8000-000000000002', 'b8000000-0000-7000-8000-000000000003', 'b8000000-0000-7000-8000-000000000010', '2026-03-01 05:30+00', '2026-03-01 05:45+00'); SET CONSTRAINTS ALL IMMEDIATE$$,
  'the box admits a third item once the second left, the deleted removal still having taken the first out');
SET CONSTRAINTS ALL DEFERRED;

SELECT throws_ok($$UPDATE procedure_assets SET asset_id = 'b8000000-0000-7000-8000-000000000003' WHERE id = '98000000-0000-7000-8000-000000000004'$$,
  '23514', 'binding 98000000-0000-7000-8000-000000000004 recorded its asset leaving a container, so its asset cannot change',
  'a started removal cannot rebind the asset it took out');
SELECT throws_ok($$DELETE FROM procedure_assets WHERE id = '98000000-0000-7000-8000-000000000004'$$,
  '23514', 'binding 98000000-0000-7000-8000-000000000004 recorded its asset leaving a container, so its asset cannot change',
  'a started removal cannot unbind the asset it took out');
SELECT throws_ok($$UPDATE procedures SET started_at = '2026-03-01 07:00+00' WHERE id = 'f8000000-0000-7000-8000-000000000004'$$,
  '23514', 'procedure f8000000-0000-7000-8000-000000000004 recorded an asset leaving a container, so it cannot start later',
  'a started removal cannot move its start later');
SELECT lives_ok($$UPDATE procedures SET started_at = '2026-03-01 05:10+00' WHERE id = 'f8000000-0000-7000-8000-000000000004'; SET CONSTRAINTS ALL IMMEDIATE$$,
  'a started removal may move its start earlier');
SET CONSTRAINTS ALL DEFERRED;
SELECT throws_ok($$DELETE FROM procedures WHERE id = 'f8000000-0000-7000-8000-000000000004'$$,
  '23514', 'procedure f8000000-0000-7000-8000-000000000004 is in_progress and uses an asset, so it cannot be deleted',
  'a started run using an asset cannot be deleted');

-- A second user, a member of the organization, reads ledgers through the runs and assets shared with them.
SELECT pg_temp.add_member('20000000-0000-7000-8000-000000000001', '10000000-0000-7000-8000-000000000002');
SAVEPOINT shared_reader;
INSERT INTO grants (organization_id, resource_id, grantee_id, grantee_kind, role)
VALUES ('20000000-0000-7000-8000-000000000001', 'f8000000-0000-7000-8000-000000000001', '10000000-0000-7000-8000-000000000002', 'user', 'reader');
SET CONSTRAINTS ALL IMMEDIATE;
SET CONSTRAINTS ALL DEFERRED;
SELECT pg_temp.act('10000000-0000-7000-8000-000000000002');
SELECT is((SELECT count(*) FROM asset_uses WHERE asset_id = 'b8000000-0000-7000-8000-000000000001'), 1::bigint,
  'a run reader reads the uses of the assets the run binds');
SELECT is((SELECT count(*) FROM asset_uses WHERE asset_id = 'b8000000-0000-7000-8000-000000000002'), 0::bigint,
  'a run reader reads no uses of other assets');
SELECT is((SELECT count(*) FROM container_occupancy WHERE container_id = 'b8000000-0000-7000-8000-000000000010'), 3::bigint,
  'a run reader reads the occupancy of the container the run binds');
SELECT is((SELECT count(*) FROM asset_moves WHERE container_id = 'b8000000-0000-7000-8000-000000000010'), 1::bigint,
  'a run reader reads the placements of its own assets into a container and no others');
SELECT results_eq($$SELECT id FROM asset_models WHERE id::text LIKE 'a8%' ORDER BY id$$,
  $$VALUES ('a8000000-0000-7000-8000-000000000001'::uuid), ('a8000000-0000-7000-8000-000000000002'::uuid), ('a8000000-0000-7000-8000-000000000003'::uuid)$$,
  'a run reader reads the models the run binds and their ancestors');
SELECT results_eq($$SELECT id FROM container_compatibility_rules WHERE id::text LIKE '88%'$$,
  $$VALUES ('88000000-0000-7000-8000-000000000001'::uuid)$$, 'a run reader reads the rules of the models the run binds');
SELECT pg_temp.act('10000000-0000-7000-8000-000000000001');
ROLLBACK TO shared_reader;

SAVEPOINT enclosure_reader;
INSERT INTO grants (organization_id, resource_id, grantee_id, grantee_kind, role)
VALUES ('20000000-0000-7000-8000-000000000001', 'b8000000-0000-7000-8000-000000000001', '10000000-0000-7000-8000-000000000002', 'user', 'reader');
SET CONSTRAINTS ALL IMMEDIATE;
SET CONSTRAINTS ALL DEFERRED;
SELECT pg_temp.act('10000000-0000-7000-8000-000000000002');
SELECT results_eq($$SELECT asset_id, container_id FROM asset_moves ORDER BY ended_at$$,
  $$VALUES ('b8000000-0000-7000-8000-000000000010'::uuid, 'b8000000-0000-7000-8000-000000000011'::uuid),
    ('b8000000-0000-7000-8000-000000000001'::uuid, 'b8000000-0000-7000-8000-000000000010'::uuid)$$,
  'an asset reader reads the chain of containers around the asset and nothing else inside them');
SELECT pg_temp.act('10000000-0000-7000-8000-000000000001');
ROLLBACK TO enclosure_reader;

SAVEPOINT template_reader;
INSERT INTO grants (organization_id, resource_id, grantee_id, grantee_kind, role)
VALUES ('20000000-0000-7000-8000-000000000001', 'c8000000-0000-7000-8000-000000000010', '10000000-0000-7000-8000-000000000002', 'user', 'reader');
SET CONSTRAINTS ALL IMMEDIATE;
SET CONSTRAINTS ALL DEFERRED;
SELECT pg_temp.act('10000000-0000-7000-8000-000000000002');
SELECT results_eq($$SELECT id FROM procedure_templates WHERE id::text LIKE 'c8%' ORDER BY id$$,
  $$VALUES ('c8000000-0000-7000-8000-000000000010'::uuid), ('c8000000-0000-7000-8000-000000000011'::uuid)$$,
  'a template reader reads the templates its steps run');
SELECT results_eq($$SELECT id FROM procedure_template_slots WHERE id::text LIKE 'd8%'$$,
  $$VALUES ('d8000000-0000-7000-8000-000000000011'::uuid)$$, 'a template reader reads the slots of the templates its steps run');
SELECT results_eq($$SELECT id FROM asset_models WHERE id::text LIKE 'a8%' ORDER BY id$$,
  $$VALUES ('a8000000-0000-7000-8000-000000000001'::uuid), ('a8000000-0000-7000-8000-000000000002'::uuid)$$,
  'a template reader reads the models its slots name and their ancestors');
SELECT pg_temp.act('10000000-0000-7000-8000-000000000001');
ROLLBACK TO template_reader;

-- Runs freeze for a writer who cannot read their template, here one of another project.
INSERT INTO resource_parents (organization_id, id, resource_kind, parent_id, parent_kind)
  VALUES ('20000000-0000-7000-8000-000000000001', '50000000-0000-7000-8000-000000000002', 'project',
    '20000000-0000-7000-8000-000000000001', 'organization');
INSERT INTO resources (id, kind, organization_id)
  VALUES ('50000000-0000-7000-8000-000000000002', 'project', '20000000-0000-7000-8000-000000000001');
SET CONSTRAINTS ALL IMMEDIATE;
SET CONSTRAINTS ALL DEFERRED;
INSERT INTO ownables (id, organization_id, project_id, created_by_id)
  VALUES ('c8000000-0000-7000-8000-000000000020', '20000000-0000-7000-8000-000000000001', '50000000-0000-7000-8000-000000000002', authz.caller_id());
INSERT INTO resources (id, kind, organization_id)
  VALUES ('c8000000-0000-7000-8000-000000000020', 'ownable', '20000000-0000-7000-8000-000000000001');
INSERT INTO procedure_templates (id, organization_id, project_id, table_name_id, name)
  VALUES ('c8000000-0000-7000-8000-000000000020', '20000000-0000-7000-8000-000000000001', '50000000-0000-7000-8000-000000000002',
    'procedure_templates', 'ledger elsewhere');
SELECT pg_temp.new_slot('d8000000-0000-7000-8000-000000000020', 'c8000000-0000-7000-8000-000000000020', 'touched',
  'a8000000-0000-7000-8000-000000000002', 'input', 0, 1);
UPDATE procedure_templates SET state_id = 'published' WHERE id = 'c8000000-0000-7000-8000-000000000020';
SELECT pg_temp.new_procedure('f8000000-0000-7000-8000-000000000007', 'c8000000-0000-7000-8000-000000000020', 'planned', NULL, NULL);
SET CONSTRAINTS ALL IMMEDIATE;
SET CONSTRAINTS ALL DEFERRED;
SAVEPOINT template_access;
INSERT INTO grants (organization_id, resource_id, grantee_id, grantee_kind, role)
VALUES ('20000000-0000-7000-8000-000000000001', '50000000-0000-7000-8000-000000000001', '10000000-0000-7000-8000-000000000002', 'user', 'writer');
SET CONSTRAINTS ALL IMMEDIATE;
SET CONSTRAINTS ALL DEFERRED;
SELECT pg_temp.act('10000000-0000-7000-8000-000000000002');
SELECT is_empty($$UPDATE procedures SET note = 'edited' WHERE id = 'f8000000-0000-7000-8000-000000000007' RETURNING id$$,
  'a run writer who cannot read the template cannot change the run');
SELECT pg_temp.act('10000000-0000-7000-8000-000000000001');
INSERT INTO grants (organization_id, resource_id, grantee_id, grantee_kind, role)
VALUES ('20000000-0000-7000-8000-000000000001', 'c8000000-0000-7000-8000-000000000020', '10000000-0000-7000-8000-000000000002', 'user', 'reader');
SET CONSTRAINTS ALL IMMEDIATE;
SET CONSTRAINTS ALL DEFERRED;
SELECT pg_temp.act('10000000-0000-7000-8000-000000000002');
SELECT isnt_empty($$UPDATE procedures SET note = 'edited' WHERE id = 'f8000000-0000-7000-8000-000000000007' RETURNING id$$,
  'a run writer who reads the template changes the run');
SELECT throws_ok($$UPDATE procedures SET status_id = 'in_progress', started_at = now(), performed_by_id = '10000000-0000-7000-8000-000000000001'
  WHERE id = 'f8000000-0000-7000-8000-000000000007'$$,
  '23514', 'procedure f8000000-0000-7000-8000-000000000007 names performer 10000000-0000-7000-8000-000000000001, who is not the caller',
  'a run starts only with its caller as performer');
SELECT lives_ok($$UPDATE procedures SET status_id = 'in_progress', started_at = now(), performed_by_id = '10000000-0000-7000-8000-000000000002'
  WHERE id = 'f8000000-0000-7000-8000-000000000007'; SET CONSTRAINTS ALL IMMEDIATE$$, 'a grant holder outside the organization performs a run');
SET CONSTRAINTS ALL DEFERRED;
SELECT pg_temp.act('10000000-0000-7000-8000-000000000001');
SELECT lives_ok($$UPDATE procedures SET note = 'reviewed' WHERE id = 'f8000000-0000-7000-8000-000000000007'; SET CONSTRAINTS ALL IMMEDIATE$$,
  'another writer edits a run someone else performs');
SET CONSTRAINTS ALL DEFERRED;
ROLLBACK TO template_access;
SELECT throws_ok($$UPDATE procedures SET performed_by_id = '10000000-0000-7000-8000-000000000002' WHERE id = 'f8000000-0000-7000-8000-000000000007'$$,
  '23514', 'procedure f8000000-0000-7000-8000-000000000007 names performer 10000000-0000-7000-8000-000000000002, who is not the caller',
  'a writer cannot name someone else as performer');
SELECT throws_ok($$UPDATE procedures SET performed_by_id = '10000000-0000-7000-8000-0000000000ff' WHERE id = 'f8000000-0000-7000-8000-000000000007'$$,
  '23514', 'procedure f8000000-0000-7000-8000-000000000007 names performer 10000000-0000-7000-8000-0000000000ff, who is not the caller',
  'naming a user who does not exist fails as naming a hidden one does');

-- Deleting a step run needs reader access to its parent.
SELECT pg_temp.new_procedure('f8000000-0000-7000-8000-000000000020', 'c8000000-0000-7000-8000-000000000010', 'planned', NULL, NULL);
SELECT pg_temp.new_procedure('f8000000-0000-7000-8000-000000000021', 'c8000000-0000-7000-8000-000000000011', 'planned',
  'f8000000-0000-7000-8000-000000000020', 'e8000000-0000-7000-8000-000000000001');
INSERT INTO grants (organization_id, resource_id, grantee_id, grantee_kind, role)
VALUES ('20000000-0000-7000-8000-000000000001', 'f8000000-0000-7000-8000-000000000021', '10000000-0000-7000-8000-000000000002', 'user', 'admin');
SET CONSTRAINTS ALL IMMEDIATE;
SET CONSTRAINTS ALL DEFERRED;
SELECT pg_temp.act('10000000-0000-7000-8000-000000000002');
SELECT is_empty($$DELETE FROM procedures WHERE id = 'f8000000-0000-7000-8000-000000000021' RETURNING id$$,
  'a step run admin who cannot read the parent cannot delete the step run');
SELECT pg_temp.act('10000000-0000-7000-8000-000000000001');
INSERT INTO grants (organization_id, resource_id, grantee_id, grantee_kind, role)
VALUES ('20000000-0000-7000-8000-000000000001', 'f8000000-0000-7000-8000-000000000020', '10000000-0000-7000-8000-000000000002', 'user', 'reader');
SET CONSTRAINTS ALL IMMEDIATE;
SET CONSTRAINTS ALL DEFERRED;
SELECT pg_temp.act('10000000-0000-7000-8000-000000000002');
SELECT isnt_empty($$DELETE FROM procedures WHERE id = 'f8000000-0000-7000-8000-000000000021' RETURNING id$$,
  'a step run admin who reads the parent deletes the step run');
SELECT pg_temp.act('10000000-0000-7000-8000-000000000001');

-- A blocking organization's rows open nothing the caller would otherwise
-- derive from the runs, templates and assets it reads.
INSERT INTO grants (organization_id, resource_id, grantee_id, grantee_kind, role)
VALUES ('20000000-0000-7000-8000-000000000001', 'f8000000-0000-7000-8000-000000000005', '10000000-0000-7000-8000-000000000002', 'user', 'reader'),
  ('20000000-0000-7000-8000-000000000001', 'c8000000-0000-7000-8000-000000000010', '10000000-0000-7000-8000-000000000002', 'user', 'reader'),
  ('20000000-0000-7000-8000-000000000001', 'b8000000-0000-7000-8000-000000000010', '10000000-0000-7000-8000-000000000002', 'user', 'reader');
SET CONSTRAINTS ALL IMMEDIATE;
SET CONSTRAINTS ALL DEFERRED;
SELECT pg_temp.act('10000000-0000-7000-8000-000000000002');
SELECT ok('b8000000-0000-7000-8000-000000000011' IN (SELECT ledger_assets()), 'a run reader reads the ledgers of the assets the run binds');
SELECT ok('c8000000-0000-7000-8000-000000000011' IN (SELECT readable_templates()), 'a template reader reads the templates its steps run');
SELECT ok('a8000000-0000-7000-8000-000000000001' IN (SELECT readable_models()), 'a template reader reads the models its step slots name and their ancestors');
SELECT ok('b8000000-0000-7000-8000-000000000011' IN (SELECT enclosing_assets()), 'an asset reader reads the container it was placed into');
SELECT ok('98000000-0000-7000-8000-000000000005' IN (SELECT enclosing_placements()), 'an asset reader reads the placement that put it into its container');
RESET ROLE;
INSERT INTO organization_blocks (organization_id, principal_id, principal_kind)
VALUES ('20000000-0000-7000-8000-000000000001', '10000000-0000-7000-8000-000000000002', 'user');
SET ROLE app;
SELECT ok('b8000000-0000-7000-8000-000000000011' NOT IN (SELECT ledger_assets()),
  'a run reader blocked by the run''s organization reads no ledger through its bindings');
SELECT ok('c8000000-0000-7000-8000-000000000011' NOT IN (SELECT readable_templates()),
  'a template reader blocked by the template''s organization reads no template through its steps');
SELECT ok('a8000000-0000-7000-8000-000000000001' NOT IN (SELECT readable_models()),
  'a reader blocked by the organization naming a model reads no model through its slots, bindings or ancestry');
SELECT ok('b8000000-0000-7000-8000-000000000011' NOT IN (SELECT enclosing_assets()),
  'an asset reader blocked by the placing organization reads no container through its placement');
SELECT ok('98000000-0000-7000-8000-000000000005' NOT IN (SELECT enclosing_placements()),
  'an asset reader blocked by the placing organization reads no placement of its own');
SELECT pg_temp.act('10000000-0000-7000-8000-000000000001');
SELECT * FROM finish();
ROLLBACK;
