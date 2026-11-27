\set ON_ERROR_STOP 1
SET search_path = public, test_tools;
BEGIN;
\ir ../fixture.sql
SELECT no_plan();
SELECT ok(NOT has_function_privilege('app', 'guard_asset()', 'EXECUTE'), 'N4 app cannot execute an asset guard');
SELECT pg_temp.new_model('a7100000-0000-7000-8000-000000000001', 'physical model', NULL, 'physical_asset_models');
INSERT INTO physical_asset_models (id, organization_id) VALUES ('a7100000-0000-7000-8000-000000000001', '20000000-0000-7000-8000-000000000001');
SELECT pg_temp.new_template('c7100000-0000-7000-8000-000000000001', 'bounded number');
SELECT pg_temp.new_slot('d7100000-0000-7000-8000-000000000010', 'c7100000-0000-7000-8000-000000000001', 'optional input', 'a7100000-0000-7000-8000-000000000001', 'input', 0, 1);
INSERT INTO procedure_template_parameters (id, organization_id, procedure_template_id, name, value_kind_id, quantity_kind_id, minimum_number, maximum_number)
VALUES ('d7100000-0000-7000-8000-000000000001', '20000000-0000-7000-8000-000000000001', 'c7100000-0000-7000-8000-000000000001', 'mass', 'number', 'mass', 1, 3);
UPDATE procedure_templates SET state_id = 'published' WHERE id = 'c7100000-0000-7000-8000-000000000001';
SET CONSTRAINTS ALL IMMEDIATE;
SET CONSTRAINTS ALL DEFERRED;
SELECT set_config('app.user_id', '10000000-0000-7000-8000-000000000002', false);
SELECT throws_ok($statement$INSERT INTO procedure_template_parameters (id, organization_id, procedure_template_id, name, value_kind_id)
VALUES ('d7100000-0000-7000-8000-000000000002', '20000000-0000-7000-8000-000000000001', 'c7100000-0000-7000-8000-000000000001', 'hidden', 'text')$statement$, '42501', 'new row violates row-level security policy for table "procedure_template_parameters"', 'N1 hidden template state is not disclosed');
SELECT throws_ok($statement$INSERT INTO digital_asset_models (id, organization_id, mime_type) VALUES ('a7100000-0000-7000-8000-000000000001', '20000000-0000-7000-8000-000000000001', 'image/png')$statement$, '42501', 'new row violates row-level security policy for table "digital_asset_models"', 'N1 hidden model path is not disclosed');
SELECT throws_ok($statement$INSERT INTO procedures (id, organization_id, project_id, table_name_id, procedure_template_id, root_procedure_id, performed_by_id)
VALUES ('f7100000-0000-7000-8000-000000000099', '20000000-0000-7000-8000-000000000001', '50000000-0000-7000-8000-000000000001', 'procedures', 'c7100000-0000-7000-8000-000000000001', 'f7100000-0000-7000-8000-000000000099', '10000000-0000-7000-8000-000000000002')$statement$, '42501', 'new row violates row-level security policy for table "procedures"', 'N1 hidden performer membership is not disclosed');
SELECT set_config('app.user_id', '10000000-0000-7000-8000-000000000001', false);
SAVEPOINT missing_model;
SELECT throws_ok($statement$SELECT pg_temp.new_model('a7100000-0000-7000-8000-000000000002', 'missing physical model', NULL, 'physical_asset_models'); SET CONSTRAINTS ALL IMMEDIATE$statement$, '23503', 'a7100000-0000-7000-8000-000000000002 names a table extending physical_asset_models but has no row there', 'N18 physical model requires its named row');
ROLLBACK TO missing_model;
SET CONSTRAINTS ALL DEFERRED;
SAVEPOINT deleted_model;
SELECT throws_ok($statement$DELETE FROM physical_asset_models WHERE id = 'a7100000-0000-7000-8000-000000000001'; SET CONSTRAINTS ALL IMMEDIATE$statement$, '23503', 'a7100000-0000-7000-8000-000000000001 names a table extending physical_asset_models but has no row there', 'N18 physical model keeps its named row');
ROLLBACK TO deleted_model;
SET CONSTRAINTS ALL DEFERRED;
SELECT throws_ok($statement$SELECT pg_temp.new_asset('b7100000-0000-7000-8000-000000000001', 'wrong instance', 'a7100000-0000-7000-8000-000000000001');$statement$, '23514', 'asset b7100000-0000-7000-8000-000000000001 does not name its model instance table', 'N19 physical model cannot instantiate a plain asset');
SAVEPOINT missing_asset;
SELECT throws_ok($statement$SELECT pg_temp.new_asset('b7100000-0000-7000-8000-000000000002', 'missing physical asset', 'a7100000-0000-7000-8000-000000000001', 'physical_assets'); SET CONSTRAINTS ALL IMMEDIATE$statement$, '23503', 'b7100000-0000-7000-8000-000000000002 names a table extending physical_assets but has no row there', 'N18 physical asset requires its named row');
ROLLBACK TO missing_asset;
SET CONSTRAINTS ALL DEFERRED;
SELECT pg_temp.new_procedure('f7100000-0000-7000-8000-000000000001', 'c7100000-0000-7000-8000-000000000001', 'in_progress', NULL, NULL);
SELECT pg_temp.record_number('f7100000-0000-7000-8000-000000000001', 'd7100000-0000-7000-8000-000000000001', 0);
SAVEPOINT deviation;
SELECT throws_ok($statement$UPDATE procedures SET status_id = 'completed', ended_at = now() WHERE id = 'f7100000-0000-7000-8000-000000000001'; SET CONSTRAINTS ALL IMMEDIATE$statement$, '23514', 'procedure f7100000-0000-7000-8000-000000000001 requires a note for its deviations', 'N25 below-minimum value requires a note');
ROLLBACK TO deviation;
SELECT lives_ok($statement$UPDATE procedures SET status_id = 'completed', ended_at = now(), note = 'measured outside range' WHERE id = 'f7100000-0000-7000-8000-000000000001'; SET CONSTRAINTS ALL IMMEDIATE$statement$, 'N25 noted deviation may complete');
SELECT throws_ok($statement$UPDATE procedures SET note = '   ' WHERE id = 'f7100000-0000-7000-8000-000000000001'; SET CONSTRAINTS ALL IMMEDIATE$statement$, '23514', 'procedure f7100000-0000-7000-8000-000000000001 requires a note for its deviations', 'N25 completed deviation keeps a nonblank note');
SET CONSTRAINTS ALL DEFERRED;
SELECT lives_ok($statement$
SELECT pg_temp.new_model('a7100000-0000-7000-8000-000000000003', 'transient physical model', NULL, 'physical_asset_models');
DELETE FROM asset_models WHERE id = 'a7100000-0000-7000-8000-000000000003';
SET CONSTRAINTS ALL IMMEDIATE$statement$, 'N18 inserted then deleted root needs no subtype row');
SET CONSTRAINTS ALL DEFERRED;
SELECT lives_ok($statement$
DELETE FROM physical_asset_models WHERE id = 'a7100000-0000-7000-8000-000000000001';
INSERT INTO physical_asset_models (id, organization_id) VALUES ('a7100000-0000-7000-8000-000000000001', '20000000-0000-7000-8000-000000000001');
SET CONSTRAINTS ALL IMMEDIATE$statement$, 'N18 subtype deleted then restored survives commit');
SET CONSTRAINTS ALL DEFERRED;
SELECT throws_ok($statement$
SELECT pg_temp.new_template('c7100000-0000-7000-8000-000000000002', 'typed weighing', 'weighing_procedure_templates');
SELECT pg_temp.new_slot('d7100000-0000-7000-8000-000000000020', 'c7100000-0000-7000-8000-000000000002', 'optional input', 'a7100000-0000-7000-8000-000000000001', 'input', 0, 1);
UPDATE procedure_templates SET state_id = 'published' WHERE id = 'c7100000-0000-7000-8000-000000000002';
SELECT pg_temp.new_procedure('f7100000-0000-7000-8000-000000000002', 'c7100000-0000-7000-8000-000000000002', 'planned', NULL, NULL);
$statement$, '23514', 'procedure f7100000-0000-7000-8000-000000000002 does not name its template instance table', 'N19 weighing template cannot run as a plain procedure');
SELECT pg_temp.new_procedure('f7100000-0000-7000-8000-000000000003', 'c7100000-0000-7000-8000-000000000001', 'in_progress', NULL, NULL);
SELECT pg_temp.record_number('f7100000-0000-7000-8000-000000000003', 'd7100000-0000-7000-8000-000000000001', 4);
SELECT throws_ok($statement$UPDATE procedures SET status_id = 'completed', ended_at = now() WHERE id = 'f7100000-0000-7000-8000-000000000003'; SET CONSTRAINTS ALL IMMEDIATE$statement$, '23514', 'procedure f7100000-0000-7000-8000-000000000003 requires a note for its deviations', 'N25 above-maximum value requires a note');
UPDATE procedure_parameter_values SET number_value = 3 WHERE procedure_id = 'f7100000-0000-7000-8000-000000000003';
SELECT lives_ok($statement$UPDATE procedures SET status_id = 'completed', ended_at = now() WHERE id = 'f7100000-0000-7000-8000-000000000003'; SET CONSTRAINTS ALL IMMEDIATE$statement$, 'N25 inclusive maximum needs no note');
SELECT results_eq($query$SELECT * FROM procedure_deviations('f7100000-0000-7000-8000-000000000001')$query$, $expected$VALUES ('mass'::text)$expected$, 'N25 deviations expose the bounded parameter name');
SELECT * FROM finish();
ROLLBACK;
