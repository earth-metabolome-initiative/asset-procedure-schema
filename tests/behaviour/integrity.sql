\set ON_ERROR_STOP 1
SET search_path = public, test_tools;
BEGIN;
\ir ../fixture.sql
SELECT plan(23);

-- A second organization, founded by the second user, holding a model and an asset of its own.
RESET ROLE;
SELECT pg_temp.act('10000000-0000-7000-8000-000000000002');
SET ROLE app;
INSERT INTO organizations (id, created_by_id) VALUES ('20000000-0000-7000-8000-000000000002', authz.caller_id());
INSERT INTO resources (id, kind, organization_id)
  VALUES ('20000000-0000-7000-8000-000000000002', 'organization', '20000000-0000-7000-8000-000000000002');
INSERT INTO memberships (organization_id, user_id, state) VALUES ('20000000-0000-7000-8000-000000000002', authz.caller_id(), 'active');
INSERT INTO grants (organization_id, resource_id, grantee_id, grantee_kind, role)
  VALUES ('20000000-0000-7000-8000-000000000002', '20000000-0000-7000-8000-000000000002', authz.caller_id(), 'user', 'admin');
SET CONSTRAINTS ALL IMMEDIATE;
SET CONSTRAINTS ALL DEFERRED;
INSERT INTO resources (id, kind, organization_id)
  VALUES ('50000000-0000-7000-8000-000000000002', 'project', '20000000-0000-7000-8000-000000000002');
INSERT INTO resource_parents (organization_id, id, resource_kind, parent_id, parent_kind)
  VALUES ('20000000-0000-7000-8000-000000000002', '50000000-0000-7000-8000-000000000002', 'project',
    '20000000-0000-7000-8000-000000000002', 'organization');
SET CONSTRAINTS ALL IMMEDIATE;
SET CONSTRAINTS ALL DEFERRED;
INSERT INTO ownables (id, organization_id, project_id, created_by_id)
  VALUES ('a7000000-0000-7000-8000-000000000090', '20000000-0000-7000-8000-000000000002', '50000000-0000-7000-8000-000000000002', authz.caller_id()),
    ('b7000000-0000-7000-8000-000000000090', '20000000-0000-7000-8000-000000000002', '50000000-0000-7000-8000-000000000002', authz.caller_id());
INSERT INTO resources (id, kind, organization_id)
  VALUES ('a7000000-0000-7000-8000-000000000090', 'ownable', '20000000-0000-7000-8000-000000000002'),
    ('b7000000-0000-7000-8000-000000000090', 'ownable', '20000000-0000-7000-8000-000000000002');
INSERT INTO asset_models (id, organization_id, project_id, table_name_id, name)
  VALUES ('a7000000-0000-7000-8000-000000000090', '20000000-0000-7000-8000-000000000002', '50000000-0000-7000-8000-000000000002', 'asset_models', 'their tube');
INSERT INTO asset_model_ancestors (descendant_model_id, ancestor_model_id, organization_id, parent_model_id)
  VALUES ('a7000000-0000-7000-8000-000000000090', 'a7000000-0000-7000-8000-000000000090', '20000000-0000-7000-8000-000000000002', NULL);
INSERT INTO assets (id, organization_id, project_id, table_name_id, model_id, name)
  VALUES ('b7000000-0000-7000-8000-000000000090', '20000000-0000-7000-8000-000000000002', '50000000-0000-7000-8000-000000000002', 'assets',
    'a7000000-0000-7000-8000-000000000090', 'their tube one');
SET CONSTRAINTS ALL IMMEDIATE;
SET CONSTRAINTS ALL DEFERRED;
RESET ROLE;
SELECT pg_temp.act('10000000-0000-7000-8000-000000000001');
SET ROLE app;

-- A second project of the first organization.
INSERT INTO resources (id, kind, organization_id)
  VALUES ('50000000-0000-7000-8000-000000000003', 'project', '20000000-0000-7000-8000-000000000001');
INSERT INTO resource_parents (organization_id, id, resource_kind, parent_id, parent_kind)
  VALUES ('20000000-0000-7000-8000-000000000001', '50000000-0000-7000-8000-000000000003', 'project',
    '20000000-0000-7000-8000-000000000001', 'organization');
SET CONSTRAINTS ALL IMMEDIATE;
SET CONSTRAINTS ALL DEFERRED;

SELECT pg_temp.new_model('a7000000-0000-7000-8000-000000000001', 'tube', NULL);
SELECT pg_temp.new_model('a7000000-0000-7000-8000-000000000002', 'extract', NULL);
SELECT pg_temp.new_asset('b7000000-0000-7000-8000-000000000001', 'tube one', 'a7000000-0000-7000-8000-000000000001');
SELECT pg_temp.new_asset('b7000000-0000-7000-8000-000000000002', 'extract one', 'a7000000-0000-7000-8000-000000000002');
SELECT pg_temp.new_template('c7000000-0000-7000-8000-000000000001', 'label');
SELECT pg_temp.new_slot('d7000000-0000-7000-8000-000000000001', 'c7000000-0000-7000-8000-000000000001', 'tube', 'a7000000-0000-7000-8000-000000000001', 'input', 0, 1);
SELECT pg_temp.new_template('c7000000-0000-7000-8000-000000000002', 'extract');
SELECT pg_temp.new_slot('d7000000-0000-7000-8000-000000000002', 'c7000000-0000-7000-8000-000000000002', 'extract', 'a7000000-0000-7000-8000-000000000002', 'produced', 1, 1);
SELECT pg_temp.new_template('c7000000-0000-7000-8000-000000000003', 'use');
SELECT pg_temp.new_slot('d7000000-0000-7000-8000-000000000003', 'c7000000-0000-7000-8000-000000000003', 'extract', 'a7000000-0000-7000-8000-000000000002', 'input', 1, 1);
UPDATE procedure_templates SET state_id = 'published'
  WHERE id IN ('c7000000-0000-7000-8000-000000000001', 'c7000000-0000-7000-8000-000000000002', 'c7000000-0000-7000-8000-000000000003');
SELECT pg_temp.new_template('c7000000-0000-7000-8000-000000000010', 'two labels');
SELECT pg_temp.new_step('e7000000-0000-7000-8000-000000000001', 'c7000000-0000-7000-8000-000000000010', 'c7000000-0000-7000-8000-000000000001', 'first', 0);
SELECT pg_temp.new_step('e7000000-0000-7000-8000-000000000002', 'c7000000-0000-7000-8000-000000000010', 'c7000000-0000-7000-8000-000000000001', 'second', 1);
SELECT pg_temp.new_edge('e7000000-0000-7000-8000-000000000001', 'e7000000-0000-7000-8000-000000000002');
UPDATE procedure_templates SET state_id = 'published' WHERE id = 'c7000000-0000-7000-8000-000000000010';
SET CONSTRAINTS ALL IMMEDIATE;
SET CONSTRAINTS ALL DEFERRED;

SELECT isnt_empty($query$SELECT 1 FROM audit_events WHERE table_name = 'asset_models' AND operation = 'INSERT'
    AND new_row ->> 'id' = 'a7000000-0000-7000-8000-000000000001'$query$, 'inserting a model is audited');
SELECT pg_temp.new_procedure('f7000000-0000-7000-8000-000000000001', 'c7000000-0000-7000-8000-000000000001', 'planned', NULL, NULL);
UPDATE procedures SET note = 'audited' WHERE id = 'f7000000-0000-7000-8000-000000000001';
SELECT isnt_empty($query$SELECT 1 FROM audit_events WHERE table_name = 'procedures' AND operation = 'UPDATE'
    AND new_row ->> 'note' = 'audited'$query$, 'changing a procedure is audited');

SELECT throws_ok($statement$UPDATE procedure_templates SET state_id = 'deprecated' WHERE id = 'c7000000-0000-7000-8000-000000000001';
    UPDATE procedures SET status_id = 'in_progress', started_at = now() WHERE id = 'f7000000-0000-7000-8000-000000000001'; SET CONSTRAINTS ALL IMMEDIATE$statement$,
  '23514', 'procedure f7000000-0000-7000-8000-000000000001 cannot execute template c7000000-0000-7000-8000-000000000001 in its state',
  'a planned run of a deprecated template cannot start');

SELECT pg_temp.new_procedure('f7000000-0000-7000-8000-000000000010', 'c7000000-0000-7000-8000-000000000010', 'in_progress', NULL, NULL);
SELECT pg_temp.new_procedure('f7000000-0000-7000-8000-000000000011', 'c7000000-0000-7000-8000-000000000001', 'in_progress',
  'f7000000-0000-7000-8000-000000000010', 'e7000000-0000-7000-8000-000000000001');
UPDATE procedures SET status_id = 'completed', ended_at = now() WHERE id = 'f7000000-0000-7000-8000-000000000011';
SELECT pg_temp.new_procedure('f7000000-0000-7000-8000-000000000012', 'c7000000-0000-7000-8000-000000000001', 'in_progress',
  'f7000000-0000-7000-8000-000000000010', 'e7000000-0000-7000-8000-000000000002');
UPDATE procedures SET status_id = 'completed', ended_at = now() WHERE id = 'f7000000-0000-7000-8000-000000000012';
UPDATE procedures SET status_id = 'completed', ended_at = now() WHERE id = 'f7000000-0000-7000-8000-000000000010';
SET CONSTRAINTS ALL IMMEDIATE;
SET CONSTRAINTS ALL DEFERRED;
SELECT throws_ok($statement$DELETE FROM procedures WHERE id = 'f7000000-0000-7000-8000-000000000011'$statement$,
  '23514', 'parent procedure f7000000-0000-7000-8000-000000000010 is completed, so its step runs cannot be deleted',
  'a step run of a completed parent cannot be deleted');
SELECT lives_ok($statement$DELETE FROM procedures WHERE id = 'f7000000-0000-7000-8000-000000000010'; SET CONSTRAINTS ALL IMMEDIATE$statement$,
  'a completed run is deleted with its step runs');
SET CONSTRAINTS ALL DEFERRED;

SELECT throws_ok($statement$SELECT pg_temp.new_procedure('f7000000-0000-7000-8000-000000000020', 'c7000000-0000-7000-8000-000000000010', 'in_progress', NULL, NULL);
    SELECT pg_temp.new_procedure('f7000000-0000-7000-8000-000000000021', 'c7000000-0000-7000-8000-000000000001', 'in_progress',
      'f7000000-0000-7000-8000-000000000020', 'e7000000-0000-7000-8000-000000000001');
    UPDATE procedures SET started_at = now() - interval '1 hour' WHERE id = 'f7000000-0000-7000-8000-000000000021'; SET CONSTRAINTS ALL IMMEDIATE$statement$,
  '23514', 'procedure f7000000-0000-7000-8000-000000000021 starts before its parent f7000000-0000-7000-8000-000000000020 started',
  'a step run cannot start before its parent');
SELECT throws_ok($statement$SELECT pg_temp.new_procedure('f7000000-0000-7000-8000-000000000020', 'c7000000-0000-7000-8000-000000000010', 'in_progress', NULL, NULL);
    UPDATE procedures SET started_at = now() - interval '2 hours' WHERE id = 'f7000000-0000-7000-8000-000000000020';
    SELECT pg_temp.new_procedure('f7000000-0000-7000-8000-000000000021', 'c7000000-0000-7000-8000-000000000001', 'in_progress',
      'f7000000-0000-7000-8000-000000000020', 'e7000000-0000-7000-8000-000000000001');
    UPDATE procedures SET status_id = 'completed', ended_at = now() WHERE id = 'f7000000-0000-7000-8000-000000000021';
    SELECT pg_temp.new_procedure('f7000000-0000-7000-8000-000000000022', 'c7000000-0000-7000-8000-000000000001', 'in_progress',
      'f7000000-0000-7000-8000-000000000020', 'e7000000-0000-7000-8000-000000000002');
    UPDATE procedures SET started_at = now() - interval '1 hour' WHERE id = 'f7000000-0000-7000-8000-000000000022'; SET CONSTRAINTS ALL IMMEDIATE$statement$,
  '23514', 'procedure f7000000-0000-7000-8000-000000000022 starts before its predecessor run f7000000-0000-7000-8000-000000000021 ended',
  'a successor run cannot start before its predecessor run ended');
SELECT throws_ok($statement$SELECT pg_temp.new_procedure('f7000000-0000-7000-8000-000000000020', 'c7000000-0000-7000-8000-000000000010', 'in_progress', NULL, NULL);
    UPDATE procedures SET started_at = now() - interval '2 hours' WHERE id = 'f7000000-0000-7000-8000-000000000020';
    SELECT pg_temp.new_procedure('f7000000-0000-7000-8000-000000000021', 'c7000000-0000-7000-8000-000000000001', 'in_progress',
      'f7000000-0000-7000-8000-000000000020', 'e7000000-0000-7000-8000-000000000001');
    UPDATE procedures SET status_id = 'aborted', ended_at = now() WHERE id = 'f7000000-0000-7000-8000-000000000021';
    UPDATE procedures SET status_id = 'aborted', ended_at = now() - interval '1 hour' WHERE id = 'f7000000-0000-7000-8000-000000000020'; SET CONSTRAINTS ALL IMMEDIATE$statement$,
  '23514', 'procedure f7000000-0000-7000-8000-000000000021 ends after its parent f7000000-0000-7000-8000-000000000020 ended',
  'a step run cannot end after its parent');

SELECT throws_ok($statement$SELECT pg_temp.new_procedure('f7000000-0000-7000-8000-000000000030', 'c7000000-0000-7000-8000-000000000002', 'planned', NULL, NULL);
    SELECT pg_temp.bind('97000000-0000-7000-8000-000000000030', 'f7000000-0000-7000-8000-000000000030', 'd7000000-0000-7000-8000-000000000002', 'b7000000-0000-7000-8000-000000000002');
    SELECT pg_temp.new_procedure('f7000000-0000-7000-8000-000000000031', 'c7000000-0000-7000-8000-000000000003', 'in_progress', NULL, NULL);
    SELECT pg_temp.bind('97000000-0000-7000-8000-000000000031', 'f7000000-0000-7000-8000-000000000031', 'd7000000-0000-7000-8000-000000000003', 'b7000000-0000-7000-8000-000000000002'); SET CONSTRAINTS ALL IMMEDIATE$statement$,
  '23514', 'asset b7000000-0000-7000-8000-000000000002 is used outside the time between its production and its consumption',
  'an asset whose producer is still planned cannot be used');

SELECT throws_ok($statement$SELECT pg_temp.create_ownable('f7000000-0000-7000-8000-000000000040');
    INSERT INTO procedures (id, organization_id, project_id, table_name_id, procedure_template_id, status_id, root_procedure_id, started_at)
      VALUES ('f7000000-0000-7000-8000-000000000040', '20000000-0000-7000-8000-000000000001', '50000000-0000-7000-8000-000000000001',
        'procedures', 'c7000000-0000-7000-8000-000000000001', 'in_progress', 'f7000000-0000-7000-8000-000000000040', now())$statement$,
  '23514', NULL, 'a run in progress names who performs it');
-- The writer joins the performer's organization, so the performer passes the
-- policy as a co-member and only the membership guard refuses.
SAVEPOINT shared_performer;
RESET ROLE;
INSERT INTO memberships (organization_id, user_id, state)
  VALUES ('20000000-0000-7000-8000-000000000002', '10000000-0000-7000-8000-000000000001', 'active');
SET ROLE app;
SELECT throws_ok($statement$SELECT pg_temp.new_procedure('f7000000-0000-7000-8000-000000000041', 'c7000000-0000-7000-8000-000000000001', 'planned', NULL, NULL);
    UPDATE procedures SET performed_by_id = '10000000-0000-7000-8000-000000000002' WHERE id = 'f7000000-0000-7000-8000-000000000041'$statement$,
  '23514', 'user 10000000-0000-7000-8000-000000000002 is not an active member of organization 20000000-0000-7000-8000-000000000001',
  'a run is performed by an active member of its organization');
ROLLBACK TO shared_performer;

SELECT throws_ok($statement$SELECT pg_temp.new_procedure('f7000000-0000-7000-8000-000000000042', 'c7000000-0000-7000-8000-000000000001', 'in_progress', NULL, NULL);
    SELECT pg_temp.bind('97000000-0000-7000-8000-000000000042', 'f7000000-0000-7000-8000-000000000042', 'd7000000-0000-7000-8000-000000000001', 'b7000000-0000-7000-8000-000000000090')$statement$,
  '42501', NULL, 'an asset of another organization cannot be bound');
SELECT throws_ok($statement$SELECT pg_temp.new_asset('b7000000-0000-7000-8000-000000000043', 'borrowed model', 'a7000000-0000-7000-8000-000000000090')$statement$,
  '42501', NULL, 'an asset cannot instantiate a model of another organization');
SELECT throws_ok($statement$SELECT pg_temp.new_model('a7000000-0000-7000-8000-000000000044', 'borrowed parent', 'a7000000-0000-7000-8000-000000000090')$statement$,
  '42501', NULL, 'a model cannot specialise a model of another organization');

SELECT throws_ok($statement$UPDATE resources SET archived_at = now() WHERE id = 'a7000000-0000-7000-8000-000000000001';
    SELECT pg_temp.new_asset('b7000000-0000-7000-8000-000000000045', 'archived model', 'a7000000-0000-7000-8000-000000000001')$statement$,
  '42501', NULL, 'an asset cannot instantiate an archived model');

SELECT throws_ok($statement$SELECT pg_temp.create_ownable('c7000000-0000-7000-8000-000000000050');
    UPDATE ownables SET project_id = '50000000-0000-7000-8000-000000000003' WHERE id = 'c7000000-0000-7000-8000-000000000050';
    INSERT INTO procedure_templates (id, organization_id, project_id, table_name_id, name)
      VALUES ('c7000000-0000-7000-8000-000000000050', '20000000-0000-7000-8000-000000000001', '50000000-0000-7000-8000-000000000003', 'procedure_templates', 'label');
    SELECT pg_temp.new_slot('d7000000-0000-7000-8000-000000000050', 'c7000000-0000-7000-8000-000000000050', 'tube', 'a7000000-0000-7000-8000-000000000001', 'input', 0, 1);
    UPDATE procedure_templates SET state_id = 'published' WHERE id = 'c7000000-0000-7000-8000-000000000050';
    SELECT pg_temp.create_ownable('c7000000-0000-7000-8000-000000000051');
    INSERT INTO procedure_templates (id, organization_id, project_id, table_name_id, name, version, previous_version_id, previous_version)
      VALUES ('c7000000-0000-7000-8000-000000000051', '20000000-0000-7000-8000-000000000001', '50000000-0000-7000-8000-000000000001', 'procedure_templates', 'label',
        2, 'c7000000-0000-7000-8000-000000000050', 1); SET CONSTRAINTS ALL IMMEDIATE$statement$,
  '23503', NULL, 'a version cannot follow a version of another project');

SELECT throws_ok($statement$SELECT lock_procedure('f7000000-0000-7000-8000-000000000001')$statement$, '42501', NULL, 'app cannot lock a procedure directly');
SELECT throws_ok($statement$SELECT lock_asset('b7000000-0000-7000-8000-000000000090')$statement$, '42501', NULL, 'app cannot lock an asset directly');

SELECT throws_ok($statement$SELECT pg_temp.create_ownable('a7000000-0000-7000-8000-000000000060');
    SELECT pg_temp.create_ownable('a7000000-0000-7000-8000-000000000061');
    INSERT INTO asset_models (id, organization_id, project_id, table_name_id, name, parent_model_id)
      VALUES ('a7000000-0000-7000-8000-000000000060', '20000000-0000-7000-8000-000000000001', '50000000-0000-7000-8000-000000000001', 'asset_models', 'chicken', 'a7000000-0000-7000-8000-000000000061'),
        ('a7000000-0000-7000-8000-000000000061', '20000000-0000-7000-8000-000000000001', '50000000-0000-7000-8000-000000000001', 'asset_models', 'egg', 'a7000000-0000-7000-8000-000000000060');
    INSERT INTO asset_model_ancestors (descendant_model_id, ancestor_model_id, organization_id, parent_model_id)
      VALUES ('a7000000-0000-7000-8000-000000000060', 'a7000000-0000-7000-8000-000000000060', '20000000-0000-7000-8000-000000000001', NULL),
        ('a7000000-0000-7000-8000-000000000061', 'a7000000-0000-7000-8000-000000000061', '20000000-0000-7000-8000-000000000001', NULL);
    INSERT INTO asset_model_ancestors (descendant_model_id, ancestor_model_id, organization_id, parent_model_id)
      VALUES ('a7000000-0000-7000-8000-000000000060', 'a7000000-0000-7000-8000-000000000061', '20000000-0000-7000-8000-000000000001', 'a7000000-0000-7000-8000-000000000061'),
        ('a7000000-0000-7000-8000-000000000061', 'a7000000-0000-7000-8000-000000000060', '20000000-0000-7000-8000-000000000001', 'a7000000-0000-7000-8000-000000000060'); SET CONSTRAINTS ALL IMMEDIATE$statement$,
  '23514', NULL, 'two models cannot be each other''s parent');

SELECT pg_temp.create_ownable('a7000000-0000-7000-8000-000000000070');
INSERT INTO asset_models (id, organization_id, project_id, table_name_id, name)
  VALUES ('a7000000-0000-7000-8000-000000000070', '20000000-0000-7000-8000-000000000001', '50000000-0000-7000-8000-000000000001', 'physical_asset_models', 'balance');
INSERT INTO asset_model_ancestors (descendant_model_id, ancestor_model_id, organization_id, parent_model_id)
  VALUES ('a7000000-0000-7000-8000-000000000070', 'a7000000-0000-7000-8000-000000000070', '20000000-0000-7000-8000-000000000001', NULL);
INSERT INTO physical_asset_models (id, organization_id) VALUES ('a7000000-0000-7000-8000-000000000070', '20000000-0000-7000-8000-000000000001');
SELECT pg_temp.create_ownable('a7000000-0000-7000-8000-000000000071');
INSERT INTO asset_models (id, organization_id, project_id, table_name_id, name)
  VALUES ('a7000000-0000-7000-8000-000000000071', '20000000-0000-7000-8000-000000000001', '50000000-0000-7000-8000-000000000001', 'digital_asset_models', 'scan');
INSERT INTO asset_model_ancestors (descendant_model_id, ancestor_model_id, organization_id, parent_model_id)
  VALUES ('a7000000-0000-7000-8000-000000000071', 'a7000000-0000-7000-8000-000000000071', '20000000-0000-7000-8000-000000000001', NULL);
INSERT INTO digital_asset_models (id, organization_id, mime_type) VALUES ('a7000000-0000-7000-8000-000000000071', '20000000-0000-7000-8000-000000000001', 'image/png');
SET CONSTRAINTS ALL IMMEDIATE;
SET CONSTRAINTS ALL DEFERRED;
SELECT throws_ok($statement$INSERT INTO digital_asset_models (id, organization_id, mime_type)
      VALUES ('a7000000-0000-7000-8000-000000000070', '20000000-0000-7000-8000-000000000001', 'image/png')$statement$,
  '23514', 'row a7000000-0000-7000-8000-000000000070 of digital_asset_models is not on the table path of physical_asset_models',
  'a physical model cannot also be a digital model');
SELECT throws_ok($statement$SELECT pg_temp.create_ownable('a7000000-0000-7000-8000-000000000072');
    INSERT INTO asset_models (id, organization_id, project_id, table_name_id, name, parent_model_id)
      VALUES ('a7000000-0000-7000-8000-000000000072', '20000000-0000-7000-8000-000000000001', '50000000-0000-7000-8000-000000000001',
        'physical_asset_models', 'printed scan', 'a7000000-0000-7000-8000-000000000071')$statement$,
  '23514', 'asset model a7000000-0000-7000-8000-000000000072 of table physical_asset_models cannot have parent a7000000-0000-7000-8000-000000000071 of table digital_asset_models',
  'a physical model cannot specialise a digital model');

SELECT is_empty($query$
  SELECT function.oid::regprocedure
  FROM pg_proc function
  WHERE function.pronamespace = 'public'::regnamespace
    AND NOT EXISTS (
      SELECT 1 FROM pg_depend dependency
      WHERE dependency.classid = 'pg_proc'::regclass AND dependency.objid = function.oid AND dependency.deptype = 'e'
    )
    AND NOT coalesce('search_path=pg_catalog, pg_temp' = ANY (function.proconfig), FALSE)
$query$, 'every APS function searches pg_catalog then pg_temp last');

RESET ROLE;

-- Whether the checks of `table_name` accept `probe` in `column_name`, tried on
-- a copy of the table that keeps its checks and drops its NOT NULLs.
CREATE FUNCTION pg_temp.float_column_accepts(table_name NAME, column_name NAME, probe DOUBLE PRECISION) RETURNS BOOLEAN
LANGUAGE plpgsql AS $$
DECLARE
    nullable_column NAME;
    accepted BOOLEAN;
BEGIN
    EXECUTE format('CREATE TEMP TABLE float_probe (LIKE public.%I INCLUDING CONSTRAINTS)', table_name);
    FOR nullable_column IN
        SELECT attribute.attname FROM pg_attribute attribute
        WHERE attribute.attrelid = 'pg_temp.float_probe'::regclass AND attribute.attnum > 0 AND attribute.attnotnull
    LOOP
        EXECUTE format('ALTER TABLE pg_temp.float_probe ALTER COLUMN %I DROP NOT NULL', nullable_column);
    END LOOP;
    BEGIN
        EXECUTE format('INSERT INTO pg_temp.float_probe (%I) VALUES ($1)', column_name) USING probe;
        accepted := TRUE;
    EXCEPTION WHEN check_violation THEN
        accepted := FALSE;
    END;
    DROP TABLE pg_temp.float_probe;
    RETURN accepted;
END;
$$;

SELECT is_empty($query$
  SELECT relation.relname, attribute.attname, probe
  FROM pg_class relation
  JOIN pg_attribute attribute ON attribute.attrelid = relation.oid AND attribute.attnum > 0 AND NOT attribute.attisdropped
  CROSS JOIN (VALUES ('NaN'::double precision), ('Infinity'), ('-Infinity')) probes (probe)
  WHERE relation.relnamespace = 'public'::regnamespace AND relation.relkind = 'r'
    AND attribute.atttypid = 'double precision'::regtype
    AND NOT EXISTS (
      SELECT 1 FROM pg_depend dependency
      WHERE dependency.classid = 'pg_class'::regclass AND dependency.objid = relation.oid AND dependency.deptype = 'e'
    )
    AND pg_temp.float_column_accepts(relation.relname, attribute.attname, probe)
$query$, 'every APS DOUBLE PRECISION column refuses NaN and both infinities');
SELECT * FROM finish();
ROLLBACK;
