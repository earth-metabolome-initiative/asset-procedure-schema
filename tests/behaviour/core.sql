\set ON_ERROR_STOP 1
SET search_path = public, test_tools;
BEGIN;
\ir ../fixture.sql
SELECT plan(58);


SELECT pg_temp.new_model('a0000000-0000-7000-8000-000000000001', 'tube', NULL);
SELECT pg_temp.new_model('a0000000-0000-7000-8000-000000000002', 'eppendorf', 'a0000000-0000-7000-8000-000000000001');
SELECT pg_temp.new_model('a0000000-0000-7000-8000-000000000003', 'balance', NULL);
SELECT pg_temp.new_model('a0000000-0000-7000-8000-000000000004', 'sample', NULL);
SELECT pg_temp.new_model('a0000000-0000-7000-8000-000000000005', 'extract', NULL);
SELECT pg_temp.new_asset('b0000000-0000-7000-8000-000000000001', 'tube one', 'a0000000-0000-7000-8000-000000000002');
SELECT pg_temp.new_asset('b0000000-0000-7000-8000-000000000002', 'tube two', 'a0000000-0000-7000-8000-000000000002');
SELECT pg_temp.new_asset('b0000000-0000-7000-8000-000000000003', 'balance one', 'a0000000-0000-7000-8000-000000000003');
SELECT pg_temp.new_asset('b0000000-0000-7000-8000-000000000004', 'sample one', 'a0000000-0000-7000-8000-000000000004');
SELECT pg_temp.new_asset('b0000000-0000-7000-8000-000000000005', 'extract one', 'a0000000-0000-7000-8000-000000000005');
SELECT pg_temp.new_asset('b0000000-0000-7000-8000-000000000006', 'extract two', 'a0000000-0000-7000-8000-000000000005');
SET CONSTRAINTS ALL IMMEDIATE;
SET CONSTRAINTS ALL DEFERRED;

SELECT throws_ok($statement$SELECT pg_temp.create_ownable('a0000000-0000-7000-8000-000000000009');
    INSERT INTO asset_models (id, organization_id, project_id, table_name_id, name)
      VALUES ('a0000000-0000-7000-8000-000000000009', '20000000-0000-7000-8000-000000000001', '50000000-0000-7000-8000-000000000001', 'asset_models', 'orphan'); SET CONSTRAINTS ALL IMMEDIATE$statement$, '23514', 'asset model a0000000-0000-7000-8000-000000000009 lacks rows in asset_model_ancestors', 'model without its own ancestor row is rejected at commit');
SELECT throws_ok($statement$SELECT pg_temp.create_ownable('a0000000-0000-7000-8000-000000000009');
    INSERT INTO asset_models (id, organization_id, project_id, table_name_id, name, parent_model_id)
      VALUES ('a0000000-0000-7000-8000-000000000009', '20000000-0000-7000-8000-000000000001', '50000000-0000-7000-8000-000000000001',
        'asset_models', 'microtube', 'a0000000-0000-7000-8000-000000000002');
    INSERT INTO asset_model_ancestors VALUES ('a0000000-0000-7000-8000-000000000009', 'a0000000-0000-7000-8000-000000000009', '20000000-0000-7000-8000-000000000001', NULL);
    INSERT INTO asset_model_ancestors VALUES ('a0000000-0000-7000-8000-000000000009', 'a0000000-0000-7000-8000-000000000002', '20000000-0000-7000-8000-000000000001', 'a0000000-0000-7000-8000-000000000002'); SET CONSTRAINTS ALL IMMEDIATE$statement$, '23514', 'asset model a0000000-0000-7000-8000-000000000009 lacks rows in asset_model_ancestors', 'model missing an inherited ancestor row is rejected at commit');
SELECT throws_ok($statement$INSERT INTO asset_model_ancestors VALUES ('a0000000-0000-7000-8000-000000000002', 'a0000000-0000-7000-8000-000000000003', '20000000-0000-7000-8000-000000000001', 'a0000000-0000-7000-8000-000000000001'); SET CONSTRAINTS ALL IMMEDIATE$statement$, '23503', 'insert or update on table "asset_model_ancestors" violates foreign key constraint "asset_model_ancestors_parent_model_id_ancestor_model_id_fkey"', 'ancestor row naming a model that is not an ancestor is rejected');
SELECT throws_ok($statement$UPDATE asset_models SET parent_model_id = NULL WHERE id = 'a0000000-0000-7000-8000-000000000002'; SET CONSTRAINTS ALL IMMEDIATE$statement$, '42501', 'permission denied for table asset_models', 'parent model cannot change');
SELECT throws_ok($statement$DELETE FROM asset_models WHERE id = 'a0000000-0000-7000-8000-000000000003'; SET CONSTRAINTS ALL IMMEDIATE$statement$, '23503', 'update or delete on table "asset_models" violates foreign key constraint "assets_model_id_fkey" on table "assets"', 'model with assets cannot be deleted');


SELECT pg_temp.new_template('c0000000-0000-7000-8000-000000000001', 'weigh');
SELECT pg_temp.new_slot('d0000000-0000-7000-8000-000000000001', 'c0000000-0000-7000-8000-000000000001', 'weighed',
  'a0000000-0000-7000-8000-000000000001', 'input', 1, 1);
SELECT pg_temp.new_slot('d0000000-0000-7000-8000-000000000002', 'c0000000-0000-7000-8000-000000000001', 'balance',
  'a0000000-0000-7000-8000-000000000003', 'instrument', 1, 1);
SELECT pg_temp.new_template('c0000000-0000-7000-8000-000000000002', 'centrifuge');
SELECT pg_temp.new_slot('d0000000-0000-7000-8000-000000000003', 'c0000000-0000-7000-8000-000000000002', 'tube',
  'a0000000-0000-7000-8000-000000000001', 'input', 1, NULL);
SELECT pg_temp.new_template('c0000000-0000-7000-8000-000000000004', 'extraction');
SELECT pg_temp.new_slot('d0000000-0000-7000-8000-000000000006', 'c0000000-0000-7000-8000-000000000004', 'sample',
  'a0000000-0000-7000-8000-000000000004', 'consumed', 1, 1);
SELECT pg_temp.new_slot('d0000000-0000-7000-8000-000000000007', 'c0000000-0000-7000-8000-000000000004', 'extract',
  'a0000000-0000-7000-8000-000000000005', 'produced', 1, 1);
SELECT pg_temp.new_template('c0000000-0000-7000-8000-000000000003', 'protocol');
SELECT pg_temp.new_slot('d0000000-0000-7000-8000-000000000004', 'c0000000-0000-7000-8000-000000000003', 'tube',
  'a0000000-0000-7000-8000-000000000002', 'input', 1, 1);
SELECT pg_temp.new_slot('d0000000-0000-7000-8000-000000000005', 'c0000000-0000-7000-8000-000000000003', 'balance',
  'a0000000-0000-7000-8000-000000000003', 'instrument', 1, 1);
SET CONSTRAINTS ALL IMMEDIATE;
SET CONSTRAINTS ALL DEFERRED;

SELECT throws_ok($statement$SELECT pg_temp.create_ownable('c0000000-0000-7000-8000-000000000009');
    INSERT INTO procedure_templates (id, organization_id, project_id, table_name_id, name, state_id)
      VALUES ('c0000000-0000-7000-8000-000000000009', '20000000-0000-7000-8000-000000000001', '50000000-0000-7000-8000-000000000001',
        'procedure_templates', 'eager', 'published'); SET CONSTRAINTS ALL IMMEDIATE$statement$, '23514', 'procedure template c0000000-0000-7000-8000-000000000009 must be inserted as draft', 'template inserted as published is rejected');
SELECT throws_ok($statement$SELECT pg_temp.new_step('e0000000-0000-7000-8000-000000000009', 'c0000000-0000-7000-8000-000000000003',
      'c0000000-0000-7000-8000-000000000001', 'weigh early', 0); SET CONSTRAINTS ALL IMMEDIATE$statement$, '23514', 'step e0000000-0000-7000-8000-000000000009 runs template c0000000-0000-7000-8000-000000000001, which is not published', 'step running a draft template is rejected');

UPDATE procedure_templates SET state_id = 'published'
  WHERE id IN ('c0000000-0000-7000-8000-000000000001', 'c0000000-0000-7000-8000-000000000002', 'c0000000-0000-7000-8000-000000000004');

SELECT throws_ok($statement$SELECT pg_temp.new_step('e0000000-0000-7000-8000-000000000009', 'c0000000-0000-7000-8000-000000000003',
      'c0000000-0000-7000-8000-000000000003', 'recurse', 0); SET CONSTRAINTS ALL IMMEDIATE$statement$, '23514', 'new row for relation "procedure_template_steps" violates check constraint "procedure_template_steps_check"', 'template cannot run itself as a step');


SELECT pg_temp.new_step('e0000000-0000-7000-8000-000000000001', 'c0000000-0000-7000-8000-000000000003',
  'c0000000-0000-7000-8000-000000000001', 'weigh before', 0);
SELECT pg_temp.new_step('e0000000-0000-7000-8000-000000000002', 'c0000000-0000-7000-8000-000000000003',
  'c0000000-0000-7000-8000-000000000002', 'spin', 1);
SELECT pg_temp.new_step('e0000000-0000-7000-8000-000000000003', 'c0000000-0000-7000-8000-000000000003',
  'c0000000-0000-7000-8000-000000000001', 'weigh after', 2);
SELECT pg_temp.new_edge('e0000000-0000-7000-8000-000000000001', 'e0000000-0000-7000-8000-000000000002');
SELECT pg_temp.new_edge('e0000000-0000-7000-8000-000000000002', 'e0000000-0000-7000-8000-000000000003');
SELECT pg_temp.new_binding('e0000000-0000-7000-8000-000000000001', 'd0000000-0000-7000-8000-000000000001', 'd0000000-0000-7000-8000-000000000004');
SELECT pg_temp.new_binding('e0000000-0000-7000-8000-000000000002', 'd0000000-0000-7000-8000-000000000003', 'd0000000-0000-7000-8000-000000000004');
SELECT pg_temp.new_binding('e0000000-0000-7000-8000-000000000003', 'd0000000-0000-7000-8000-000000000001', 'd0000000-0000-7000-8000-000000000004');
SELECT pg_temp.new_binding('e0000000-0000-7000-8000-000000000001', 'd0000000-0000-7000-8000-000000000002', 'd0000000-0000-7000-8000-000000000005');
SELECT pg_temp.new_binding('e0000000-0000-7000-8000-000000000003', 'd0000000-0000-7000-8000-000000000002', 'd0000000-0000-7000-8000-000000000005');
SET CONSTRAINTS ALL IMMEDIATE;
SET CONSTRAINTS ALL DEFERRED;
SELECT results_eq($query$SELECT count(*) FROM procedure_template_steps WHERE child_template_id = 'c0000000-0000-7000-8000-000000000001'$query$, $expected$VALUES (2::bigint)$expected$, 'one template runs in two steps that share slot names');
SELECT throws_ok($statement$SELECT pg_temp.new_edge('e0000000-0000-7000-8000-000000000003', 'e0000000-0000-7000-8000-000000000001'); SET CONSTRAINTS ALL IMMEDIATE$statement$, '23514', 'new row for relation "procedure_template_step_edges" violates check constraint "procedure_template_step_edges_check"', 'edge against step positions is rejected');
SELECT throws_ok($statement$UPDATE procedure_template_steps SET position = 5 WHERE id = 'e0000000-0000-7000-8000-000000000001'; SET CONSTRAINTS ALL IMMEDIATE$statement$, '23514', 'new row for relation "procedure_template_step_edges" violates check constraint "procedure_template_step_edges_check"', 'moving a step behind its successor is rejected');
SELECT throws_ok($statement$SELECT pg_temp.new_slot('d0000000-0000-7000-8000-000000000009', 'c0000000-0000-7000-8000-000000000003', 'any tube',
      'a0000000-0000-7000-8000-000000000001', 'input', 1, 1);
    DELETE FROM procedure_template_slot_bindings WHERE step_id = 'e0000000-0000-7000-8000-000000000002';
    SELECT pg_temp.new_binding('e0000000-0000-7000-8000-000000000002', 'd0000000-0000-7000-8000-000000000003', 'd0000000-0000-7000-8000-000000000009');
    UPDATE procedure_template_slots SET asset_model_id = 'a0000000-0000-7000-8000-000000000002' WHERE id = 'd0000000-0000-7000-8000-000000000003'; SET CONSTRAINTS ALL IMMEDIATE$statement$, '23503', 'update or delete on table "procedure_template_slots" violates foreign key constraint "procedure_template_slot_bindi_child_slot_id_child_template_fkey" on table "procedure_template_slot_bindings"', 'parent slot widening the child slot model is rejected');
SELECT throws_ok($statement$INSERT INTO procedure_template_slot_bindings VALUES ('e0000000-0000-7000-8000-000000000002', 'd0000000-0000-7000-8000-000000000001',
      'c0000000-0000-7000-8000-000000000003', 'c0000000-0000-7000-8000-000000000002', 'd0000000-0000-7000-8000-000000000004',
      'a0000000-0000-7000-8000-000000000001', 'a0000000-0000-7000-8000-000000000002', 'input', 'input',
      '20000000-0000-7000-8000-000000000001'); SET CONSTRAINTS ALL IMMEDIATE$statement$, '23503', 'insert or update on table "procedure_template_slot_bindings" violates foreign key constraint "procedure_template_slot_bindi_child_slot_id_child_template_fkey"', 'binding from a slot of another template is rejected');
SELECT throws_ok($statement$SELECT pg_temp.new_procedure('f0000000-0000-7000-8000-000000000009', 'c0000000-0000-7000-8000-000000000003', 'planned', NULL, NULL); SET CONSTRAINTS ALL IMMEDIATE$statement$, '23514', 'procedure f0000000-0000-7000-8000-000000000009 cannot execute template c0000000-0000-7000-8000-000000000003 in its state', 'procedure of a draft template is rejected');

UPDATE procedure_templates SET state_id = 'published' WHERE id = 'c0000000-0000-7000-8000-000000000003';

SELECT throws_ok($statement$SELECT pg_temp.new_slot('d0000000-0000-7000-8000-000000000009', 'c0000000-0000-7000-8000-000000000003', 'late',
      'a0000000-0000-7000-8000-000000000001', 'input', 0, NULL); SET CONSTRAINTS ALL IMMEDIATE$statement$, '23514', 'procedure template c0000000-0000-7000-8000-000000000003 is not a draft, so its definition cannot change', 'published template cannot gain a slot');
SELECT throws_ok($statement$UPDATE procedure_templates SET name = 'renamed' WHERE id = 'c0000000-0000-7000-8000-000000000003'; SET CONSTRAINTS ALL IMMEDIATE$statement$, '23514', 'procedure template c0000000-0000-7000-8000-000000000003 is published, so it cannot change', 'published template cannot be renamed');
SELECT throws_ok($statement$DELETE FROM procedure_template_steps WHERE id = 'e0000000-0000-7000-8000-000000000003'; SET CONSTRAINTS ALL IMMEDIATE$statement$, '23514', 'procedure template c0000000-0000-7000-8000-000000000003 is not a draft, so its definition cannot change', 'published template cannot lose a step');
SELECT throws_ok($statement$UPDATE procedure_templates SET state_id = 'draft' WHERE id = 'c0000000-0000-7000-8000-000000000003'; SET CONSTRAINTS ALL IMMEDIATE$statement$, '23514', 'procedure template c0000000-0000-7000-8000-000000000003 cannot go from published to draft', 'published template cannot return to draft');
SELECT pg_temp.create_ownable('c0000000-0000-7000-8000-000000000009');
    INSERT INTO procedure_templates (id, organization_id, project_id, table_name_id, name, version, previous_version_id, previous_version)
      VALUES ('c0000000-0000-7000-8000-000000000009', '20000000-0000-7000-8000-000000000001', '50000000-0000-7000-8000-000000000001',
        'procedure_templates', 'protocol', 2, 'c0000000-0000-7000-8000-000000000003', 1);
SET CONSTRAINTS ALL IMMEDIATE;
SET CONSTRAINTS ALL DEFERRED;
SELECT results_eq($query$SELECT state_id FROM procedure_templates WHERE id = 'c0000000-0000-7000-8000-000000000009'$query$, $expected$VALUES ('draft'::text)$expected$, 'new version follows the published one');
SELECT throws_ok($statement$SELECT pg_temp.create_ownable('c0000000-0000-7000-8000-000000000008');
    INSERT INTO procedure_templates (id, organization_id, project_id, table_name_id, name, version, previous_version_id, previous_version)
      VALUES ('c0000000-0000-7000-8000-000000000008', '20000000-0000-7000-8000-000000000001', '50000000-0000-7000-8000-000000000001',
        'procedure_templates', 'protocol', 3, 'c0000000-0000-7000-8000-000000000003', 1); SET CONSTRAINTS ALL IMMEDIATE$statement$, '23514', 'new row for relation "procedure_templates" violates check constraint "procedure_templates_check2"', 'version that skips a number is rejected');

SELECT throws_ok($statement$SELECT pg_temp.create_ownable('f0000000-0000-7000-8000-000000000009');
INSERT INTO procedures (id, organization_id, project_id, table_name_id, procedure_template_id, root_procedure_id, status_id, started_at, ended_at, performed_by_id)
VALUES ('f0000000-0000-7000-8000-000000000009', '20000000-0000-7000-8000-000000000001', '50000000-0000-7000-8000-000000000001', 'procedures', 'c0000000-0000-7000-8000-000000000001', 'f0000000-0000-7000-8000-000000000009', 'completed', now(), now(), authz.caller_id());
SET CONSTRAINTS ALL IMMEDIATE$statement$, '23514', 'procedure f0000000-0000-7000-8000-000000000009 must be inserted as planned or in_progress', 'procedure inserted as completed is rejected');


SELECT pg_temp.new_procedure('f0000000-0000-7000-8000-000000000001', 'c0000000-0000-7000-8000-000000000003', 'planned', NULL, NULL);
SELECT pg_temp.new_procedure('f0000000-0000-7000-8000-000000000002', 'c0000000-0000-7000-8000-000000000001', 'planned',
  'f0000000-0000-7000-8000-000000000001', 'e0000000-0000-7000-8000-000000000001');
SET CONSTRAINTS ALL IMMEDIATE;
SET CONSTRAINTS ALL DEFERRED;
SELECT throws_ok($statement$UPDATE procedures SET status_id = 'in_progress', started_at = now() WHERE id = 'f0000000-0000-7000-8000-000000000002'; SET CONSTRAINTS ALL IMMEDIATE$statement$, '23514', 'procedure f0000000-0000-7000-8000-000000000002 starts while its parent is not in progress', 'child started under a planned parent is rejected');
SELECT throws_ok($statement$SELECT pg_temp.new_procedure('f0000000-0000-7000-8000-000000000003', 'c0000000-0000-7000-8000-000000000002', 'in_progress',
      'f0000000-0000-7000-8000-000000000001', 'e0000000-0000-7000-8000-000000000002'); SET CONSTRAINTS ALL IMMEDIATE$statement$, '23514', 'procedure f0000000-0000-7000-8000-000000000003 cannot be in_progress under a parent that is planned', 'in-progress child under a planned parent is rejected');
SELECT throws_ok($statement$SELECT pg_temp.new_procedure('f0000000-0000-7000-8000-000000000009', 'c0000000-0000-7000-8000-000000000002', 'planned',
      'f0000000-0000-7000-8000-000000000001', 'e0000000-0000-7000-8000-000000000003'); SET CONSTRAINTS ALL IMMEDIATE$statement$, '23503', 'insert or update on table "procedures" violates foreign key constraint "procedures_step_id_parent_template_id_procedure_template_i_fkey"', 'procedure whose template differs from its step child is rejected');

SELECT throws_ok($statement$UPDATE procedures SET status_id = 'in_progress', started_at = now() + interval '1 hour' WHERE id = 'f0000000-0000-7000-8000-000000000001'; SET CONSTRAINTS ALL IMMEDIATE$statement$, '23514', 'new row for relation "procedures" violates check constraint "procedures_started_at_check"', 'start beyond the clock tolerance ahead is rejected');
SELECT pg_temp.new_procedure('f0000000-0000-7000-8000-000000000026', 'c0000000-0000-7000-8000-000000000001', 'planned', NULL, NULL);
UPDATE procedures SET status_id = 'in_progress', started_at = now() + interval '1 minute' WHERE id = 'f0000000-0000-7000-8000-000000000026';
SET CONSTRAINTS ALL IMMEDIATE;
SET CONSTRAINTS ALL DEFERRED;
SELECT results_eq($query$SELECT started_at FROM procedures WHERE id = 'f0000000-0000-7000-8000-000000000026'$query$, $expected$SELECT now() + interval '1 minute'$expected$, 'start within the clock tolerance ahead is accepted');
SELECT pg_temp.new_procedure('f0000000-0000-7000-8000-000000000027', 'c0000000-0000-7000-8000-000000000001', 'planned', NULL, NULL);
UPDATE procedures SET status_id = 'in_progress', started_at = now() - interval '3 hours' WHERE id = 'f0000000-0000-7000-8000-000000000027';
SET CONSTRAINTS ALL IMMEDIATE;
SET CONSTRAINTS ALL DEFERRED;
SELECT results_eq($query$SELECT started_at FROM procedures WHERE id = 'f0000000-0000-7000-8000-000000000027'$query$, $expected$SELECT now() - interval '3 hours'$expected$, 'start logged hours later is accepted');
UPDATE procedures SET status_id = 'in_progress', started_at = now() WHERE id = 'f0000000-0000-7000-8000-000000000001';
SELECT pg_temp.bind('90000000-0000-7000-8000-000000000001', 'f0000000-0000-7000-8000-000000000001',
  'd0000000-0000-7000-8000-000000000004', 'b0000000-0000-7000-8000-000000000001');
SELECT pg_temp.bind('90000000-0000-7000-8000-000000000002', 'f0000000-0000-7000-8000-000000000001',
  'd0000000-0000-7000-8000-000000000005', 'b0000000-0000-7000-8000-000000000003');

SELECT throws_ok($statement$SELECT pg_temp.new_procedure('f0000000-0000-7000-8000-000000000009', 'c0000000-0000-7000-8000-000000000002', 'in_progress', NULL, NULL);
    SELECT pg_temp.bind('90000000-0000-7000-8000-000000000009', 'f0000000-0000-7000-8000-000000000009',
      'd0000000-0000-7000-8000-000000000003', 'b0000000-0000-7000-8000-000000000003'); SET CONSTRAINTS ALL IMMEDIATE$statement$, '23503', 'insert or update on table "procedure_assets" violates foreign key constraint "procedure_assets_asset_model_id_slot_asset_model_id_fkey"', 'asset outside the slot model is rejected');
SELECT throws_ok($statement$SELECT pg_temp.new_procedure('f0000000-0000-7000-8000-000000000003', 'c0000000-0000-7000-8000-000000000002', 'in_progress',
      'f0000000-0000-7000-8000-000000000001', 'e0000000-0000-7000-8000-000000000002'); SET CONSTRAINTS ALL IMMEDIATE$statement$, '23514', 'procedure f0000000-0000-7000-8000-000000000003 starts before a predecessor step closed', 'step started before its predecessor completed is rejected');

UPDATE procedures SET status_id = 'in_progress', started_at = now() WHERE id = 'f0000000-0000-7000-8000-000000000002';
SELECT throws_ok($statement$SELECT pg_temp.bind('90000000-0000-7000-8000-000000000009', 'f0000000-0000-7000-8000-000000000002',
      'd0000000-0000-7000-8000-000000000001', 'b0000000-0000-7000-8000-000000000002'); SET CONSTRAINTS ALL IMMEDIATE$statement$, '23514', 'procedure f0000000-0000-7000-8000-000000000002 must bind the asset its parent binds to slot d0000000-0000-7000-8000-000000000001', 'child binding another asset than its parent slot is rejected');
SELECT pg_temp.bind('90000000-0000-7000-8000-000000000003', 'f0000000-0000-7000-8000-000000000002',
  'd0000000-0000-7000-8000-000000000001', 'b0000000-0000-7000-8000-000000000001');
SELECT throws_ok($statement$UPDATE procedures SET status_id = 'completed', ended_at = now() WHERE id = 'f0000000-0000-7000-8000-000000000002'; SET CONSTRAINTS ALL IMMEDIATE$statement$, '23514', 'procedure f0000000-0000-7000-8000-000000000002 completes with a slot below its minimum count', 'completion with a slot below its minimum is rejected');
SELECT pg_temp.bind('90000000-0000-7000-8000-000000000004', 'f0000000-0000-7000-8000-000000000002',
  'd0000000-0000-7000-8000-000000000002', 'b0000000-0000-7000-8000-000000000003');
SELECT throws_ok($statement$
    INSERT INTO procedure_assets (id, organization_id, procedure_id, procedure_template_id, slot_id, slot_asset_model_id,
        slot_role_id, asset_model_id)
      VALUES ('90000000-0000-7000-8000-000000000009', '20000000-0000-7000-8000-000000000001', 'f0000000-0000-7000-8000-000000000002',
        'c0000000-0000-7000-8000-000000000001', 'd0000000-0000-7000-8000-000000000001', 'a0000000-0000-7000-8000-000000000001',
        'input', 'a0000000-0000-7000-8000-000000000002'); SET CONSTRAINTS ALL IMMEDIATE$statement$, '23514', 'procedure f0000000-0000-7000-8000-000000000002 binds more assets than slot d0000000-0000-7000-8000-000000000001 admits', 'binding past the slot maximum is rejected');
SELECT throws_ok($statement$UPDATE procedures SET status_id = 'completed' WHERE id = 'f0000000-0000-7000-8000-000000000002'; SET CONSTRAINTS ALL IMMEDIATE$statement$, '23514', 'new row for relation "procedures" violates check constraint "procedures_check6"', 'completion before ended_at is rejected');
SELECT throws_ok($statement$UPDATE procedures SET status_id = 'completed', ended_at = now() + interval '1 hour' WHERE id = 'f0000000-0000-7000-8000-000000000002'; SET CONSTRAINTS ALL IMMEDIATE$statement$, '23514', 'new row for relation "procedures" violates check constraint "procedures_ended_at_check"', 'end beyond the clock tolerance ahead is rejected');
UPDATE procedures SET status_id = 'completed', ended_at = now() WHERE id = 'f0000000-0000-7000-8000-000000000002';

SELECT results_eq($query$SELECT status_id FROM procedure_assets WHERE procedure_id = 'f0000000-0000-7000-8000-000000000002' ORDER BY id$query$, $expected$VALUES ('completed'::text), ('completed'::text)$expected$, 'completion copies the status onto bindings');
SELECT throws_ok($statement$UPDATE procedure_assets SET asset_id = NULL WHERE id = '90000000-0000-7000-8000-000000000004'; SET CONSTRAINTS ALL IMMEDIATE$statement$, '23514', 'procedure f0000000-0000-7000-8000-000000000002 is completed, so its assets cannot change', 'binding on a completed procedure cannot change');
SELECT throws_ok($statement$DELETE FROM procedure_assets WHERE id = '90000000-0000-7000-8000-000000000004'; SET CONSTRAINTS ALL IMMEDIATE$statement$, '23514', 'procedure f0000000-0000-7000-8000-000000000002 is completed, so its assets cannot change', 'binding on a completed procedure cannot be removed');
SELECT throws_ok($statement$UPDATE procedures SET status_id = 'in_progress', ended_at = NULL WHERE id = 'f0000000-0000-7000-8000-000000000002'; SET CONSTRAINTS ALL IMMEDIATE$statement$, '23514', 'procedure f0000000-0000-7000-8000-000000000002 is completed, so only its note can change', 'completed procedure cannot reopen');
SELECT throws_ok($statement$UPDATE procedures SET started_at = started_at - interval '1 hour' WHERE id = 'f0000000-0000-7000-8000-000000000002'; SET CONSTRAINTS ALL IMMEDIATE$statement$, '23514', 'procedure f0000000-0000-7000-8000-000000000002 is completed, so only its note can change', 'completed procedure cannot change its times');
SELECT throws_ok($statement$UPDATE procedures SET performed_by_id = '10000000-0000-7000-8000-0000000000ff' WHERE id = 'f0000000-0000-7000-8000-000000000002'$statement$, '23514', 'procedure f0000000-0000-7000-8000-000000000002 names performer 10000000-0000-7000-8000-0000000000ff, who is not the caller', 'a completed procedure cannot name a missing performer either');
UPDATE procedures SET note = 'balance recalibrated afterwards' WHERE id = 'f0000000-0000-7000-8000-000000000002';
SET CONSTRAINTS ALL IMMEDIATE;
SET CONSTRAINTS ALL DEFERRED;
SELECT results_eq($query$SELECT note FROM procedures WHERE id = 'f0000000-0000-7000-8000-000000000002'$query$, $expected$VALUES ('balance recalibrated afterwards'::text)$expected$, 'completed procedure accepts a note');
SELECT throws_ok($statement$UPDATE procedures SET status_id = 'completed', ended_at = now() WHERE id = 'f0000000-0000-7000-8000-000000000001'; SET CONSTRAINTS ALL IMMEDIATE$statement$, '23514', 'procedure f0000000-0000-7000-8000-000000000001 completes with a required step neither completed nor skipped', 'parent completion with a required step missing is rejected');

SELECT pg_temp.new_procedure('f0000000-0000-7000-8000-000000000003', 'c0000000-0000-7000-8000-000000000002', 'in_progress',
  'f0000000-0000-7000-8000-000000000001', 'e0000000-0000-7000-8000-000000000002');
SELECT throws_ok($statement$SELECT pg_temp.new_procedure('f0000000-0000-7000-8000-000000000009', 'c0000000-0000-7000-8000-000000000002', 'planned',
      'f0000000-0000-7000-8000-000000000001', 'e0000000-0000-7000-8000-000000000002'); SET CONSTRAINTS ALL IMMEDIATE$statement$, '23505', 'duplicate key value violates unique constraint "procedures_step_once"', 'second run of a step under one parent is rejected');
SELECT throws_ok($statement$UPDATE procedures SET status_id = 'aborted', ended_at = now() WHERE id = 'f0000000-0000-7000-8000-000000000001'; SET CONSTRAINTS ALL IMMEDIATE$statement$, '23514', 'procedure f0000000-0000-7000-8000-000000000001 has children that are still open', 'abort with an open child is rejected');
SELECT pg_temp.bind('90000000-0000-7000-8000-000000000005', 'f0000000-0000-7000-8000-000000000003',
  'd0000000-0000-7000-8000-000000000003', 'b0000000-0000-7000-8000-000000000001');
UPDATE procedures SET status_id = 'completed', ended_at = now() WHERE id = 'f0000000-0000-7000-8000-000000000003';

SELECT pg_temp.new_procedure('f0000000-0000-7000-8000-000000000004', 'c0000000-0000-7000-8000-000000000001', 'in_progress',
  'f0000000-0000-7000-8000-000000000001', 'e0000000-0000-7000-8000-000000000003');
SELECT pg_temp.bind('90000000-0000-7000-8000-000000000006', 'f0000000-0000-7000-8000-000000000004',
  'd0000000-0000-7000-8000-000000000001', 'b0000000-0000-7000-8000-000000000001');
SELECT pg_temp.bind('90000000-0000-7000-8000-000000000007', 'f0000000-0000-7000-8000-000000000004',
  'd0000000-0000-7000-8000-000000000002', 'b0000000-0000-7000-8000-000000000003');
UPDATE procedures SET status_id = 'completed', ended_at = now() WHERE id = 'f0000000-0000-7000-8000-000000000004';
SET CONSTRAINTS ALL IMMEDIATE;
SET CONSTRAINTS ALL DEFERRED;
UPDATE procedures SET status_id = 'completed', ended_at = now() WHERE id = 'f0000000-0000-7000-8000-000000000001';
SELECT throws_ok($statement$
SELECT pg_temp.new_template('c0000000-0000-7000-8000-000000000010', 'optional parent');
SELECT pg_temp.new_step('e0000000-0000-7000-8000-000000000010', 'c0000000-0000-7000-8000-000000000010', 'c0000000-0000-7000-8000-000000000002', 'optional', 0);
UPDATE procedure_template_steps SET optional = true WHERE id = 'e0000000-0000-7000-8000-000000000010';
UPDATE procedure_templates SET state_id = 'published' WHERE id = 'c0000000-0000-7000-8000-000000000010';
SELECT pg_temp.new_procedure('f0000000-0000-7000-8000-000000000010', 'c0000000-0000-7000-8000-000000000010', 'in_progress', NULL, NULL);
UPDATE procedures SET status_id = 'completed', ended_at = now() WHERE id = 'f0000000-0000-7000-8000-000000000010';
SELECT pg_temp.new_procedure('f0000000-0000-7000-8000-000000000009', 'c0000000-0000-7000-8000-000000000002', 'planned',
      'f0000000-0000-7000-8000-000000000010', 'e0000000-0000-7000-8000-000000000010'); SET CONSTRAINTS ALL IMMEDIATE$statement$, '23514', 'procedure f0000000-0000-7000-8000-000000000009 cannot be planned under a parent that is completed', 'child under a completed parent is rejected');
SELECT results_eq($query$SELECT count(DISTINCT procedure_id) FROM procedure_assets WHERE asset_id = 'b0000000-0000-7000-8000-000000000001'$query$, $expected$VALUES (4::bigint)$expected$, 'procedures touching an asset come from one table');
SELECT throws_ok($statement$DELETE FROM assets WHERE id = 'b0000000-0000-7000-8000-000000000001'; SET CONSTRAINTS ALL IMMEDIATE$statement$, '23503', 'update or delete on table "assets" violates foreign key constraint "procedure_assets_asset_id_asset_model_id_fkey" on table "procedure_assets"', 'asset bound to a procedure cannot be deleted');
DELETE FROM procedure_templates WHERE id = 'c0000000-0000-7000-8000-000000000009';
SELECT throws_ok($statement$DELETE FROM procedure_templates WHERE id = 'c0000000-0000-7000-8000-000000000003'; SET CONSTRAINTS ALL IMMEDIATE$statement$, '23503', NULL, 'template with procedures cannot be deleted');
SELECT throws_ok($statement$DELETE FROM procedures WHERE id = 'f0000000-0000-7000-8000-000000000001'; SET CONSTRAINTS ALL IMMEDIATE$statement$, '23514',
  'procedure f0000000-0000-7000-8000-000000000001 is completed and uses an asset, so it cannot be deleted', 'a completed run that used an asset stays on the ledger');
SET CONSTRAINTS ALL DEFERRED;


SELECT pg_temp.new_procedure('f0000000-0000-7000-8000-000000000005', 'c0000000-0000-7000-8000-000000000004', 'in_progress', NULL, NULL);
SELECT pg_temp.bind('90000000-0000-7000-8000-000000000011', 'f0000000-0000-7000-8000-000000000005',
  'd0000000-0000-7000-8000-000000000006', 'b0000000-0000-7000-8000-000000000004');
SELECT pg_temp.bind('90000000-0000-7000-8000-000000000012', 'f0000000-0000-7000-8000-000000000005',
  'd0000000-0000-7000-8000-000000000007', 'b0000000-0000-7000-8000-000000000005');
SELECT pg_temp.new_procedure('f0000000-0000-7000-8000-000000000006', 'c0000000-0000-7000-8000-000000000004', 'in_progress', NULL, NULL);
SET CONSTRAINTS ALL IMMEDIATE;
SET CONSTRAINTS ALL DEFERRED;
SELECT throws_ok($statement$SELECT pg_temp.bind('90000000-0000-7000-8000-000000000013', 'f0000000-0000-7000-8000-000000000006',
      'd0000000-0000-7000-8000-000000000006', 'b0000000-0000-7000-8000-000000000004'); SET CONSTRAINTS ALL IMMEDIATE$statement$, '23505', 'duplicate key value violates unique constraint "procedure_assets_consumed_once"', 'asset consumed by two procedures is rejected');
SELECT throws_ok($statement$SELECT pg_temp.bind('90000000-0000-7000-8000-000000000013', 'f0000000-0000-7000-8000-000000000006',
      'd0000000-0000-7000-8000-000000000007', 'b0000000-0000-7000-8000-000000000005'); SET CONSTRAINTS ALL IMMEDIATE$statement$, '23505', 'duplicate key value violates unique constraint "procedure_assets_produced_once"', 'asset produced by two procedures is rejected');
SELECT throws_ok($statement$UPDATE procedures SET status_id = 'aborted' WHERE id = 'f0000000-0000-7000-8000-000000000005'; SET CONSTRAINTS ALL IMMEDIATE$statement$, '23514', 'new row for relation "procedures" violates check constraint "procedures_check7"', 'abort without ended_at is rejected');
UPDATE procedures SET status_id = 'aborted', ended_at = now() WHERE id = 'f0000000-0000-7000-8000-000000000005';
    SELECT pg_temp.bind('90000000-0000-7000-8000-000000000013', 'f0000000-0000-7000-8000-000000000006',
      'd0000000-0000-7000-8000-000000000006', 'b0000000-0000-7000-8000-000000000004');
    SELECT pg_temp.bind('90000000-0000-7000-8000-000000000014', 'f0000000-0000-7000-8000-000000000006',
      'd0000000-0000-7000-8000-000000000007', 'b0000000-0000-7000-8000-000000000005');
SET CONSTRAINTS ALL IMMEDIATE;
SET CONSTRAINTS ALL DEFERRED;
SELECT results_eq($query$SELECT count(*) FROM procedure_assets WHERE procedure_id = 'f0000000-0000-7000-8000-000000000006'$query$, $expected$VALUES (2::bigint)$expected$, 'aborting a run frees what it consumed and produced');
SELECT pg_temp.new_asset('b0000000-0000-7000-8000-000000000010', 'sample two', 'a0000000-0000-7000-8000-000000000004');
    SELECT pg_temp.new_template('c0000000-0000-7000-8000-000000000010', 'preparation');
    SELECT pg_temp.new_slot('d0000000-0000-7000-8000-000000000010', 'c0000000-0000-7000-8000-000000000010', 'sample',
      'a0000000-0000-7000-8000-000000000004', 'consumed', 1, 1);
    SELECT pg_temp.new_slot('d0000000-0000-7000-8000-000000000011', 'c0000000-0000-7000-8000-000000000010', 'extract',
      'a0000000-0000-7000-8000-000000000005', 'produced', 1, 1);
    SELECT pg_temp.new_step('e0000000-0000-7000-8000-000000000010', 'c0000000-0000-7000-8000-000000000010',
      'c0000000-0000-7000-8000-000000000004', 'extract', 0);
    SELECT pg_temp.new_binding('e0000000-0000-7000-8000-000000000010', 'd0000000-0000-7000-8000-000000000006', 'd0000000-0000-7000-8000-000000000010');
    SELECT pg_temp.new_binding('e0000000-0000-7000-8000-000000000010', 'd0000000-0000-7000-8000-000000000007', 'd0000000-0000-7000-8000-000000000011');
    UPDATE procedure_templates SET state_id = 'published' WHERE id = 'c0000000-0000-7000-8000-000000000010';
    SELECT pg_temp.new_procedure('f0000000-0000-7000-8000-000000000010', 'c0000000-0000-7000-8000-000000000010', 'in_progress', NULL, NULL);
    SELECT pg_temp.bind('90000000-0000-7000-8000-000000000020', 'f0000000-0000-7000-8000-000000000010',
      'd0000000-0000-7000-8000-000000000010', 'b0000000-0000-7000-8000-000000000010');
    SELECT pg_temp.bind('90000000-0000-7000-8000-000000000021', 'f0000000-0000-7000-8000-000000000010',
      'd0000000-0000-7000-8000-000000000011', 'b0000000-0000-7000-8000-000000000006');
    SELECT pg_temp.new_procedure('f0000000-0000-7000-8000-000000000011', 'c0000000-0000-7000-8000-000000000004', 'in_progress',
      'f0000000-0000-7000-8000-000000000010', 'e0000000-0000-7000-8000-000000000010');
    SELECT pg_temp.bind('90000000-0000-7000-8000-000000000022', 'f0000000-0000-7000-8000-000000000011',
      'd0000000-0000-7000-8000-000000000006', 'b0000000-0000-7000-8000-000000000010');
    SELECT pg_temp.bind('90000000-0000-7000-8000-000000000023', 'f0000000-0000-7000-8000-000000000011',
      'd0000000-0000-7000-8000-000000000007', 'b0000000-0000-7000-8000-000000000006');
SET CONSTRAINTS ALL IMMEDIATE;
SET CONSTRAINTS ALL DEFERRED;
SELECT results_eq($query$SELECT count(*) FROM procedure_assets WHERE procedure_id = 'f0000000-0000-7000-8000-000000000011'$query$, $expected$VALUES (2::bigint)$expected$, 'extraction run as a step consumes and produces through its parent');

SELECT set_config('app.user_id', '10000000-0000-7000-8000-000000000002', true);
SET CONSTRAINTS ALL IMMEDIATE;
SET CONSTRAINTS ALL DEFERRED;
SELECT is_empty($query$SELECT id FROM asset_models UNION ALL SELECT id FROM procedure_templates UNION ALL SELECT id FROM procedure_template_slots UNION ALL SELECT id FROM procedures UNION ALL SELECT id FROM procedure_assets$query$, 'stranger sees no APS rows');
SELECT set_config('app.user_id', '10000000-0000-7000-8000-000000000001', true);
SELECT pg_temp.add_member('20000000-0000-7000-8000-000000000001', '10000000-0000-7000-8000-000000000002');
SELECT throws_ok($statement$INSERT INTO grants (organization_id, resource_id, grantee_id, grantee_kind, role)
      VALUES ('20000000-0000-7000-8000-000000000001', '50000000-0000-7000-8000-000000000001', '10000000-0000-7000-8000-000000000002', 'user', 'reader');
    SELECT set_config('app.user_id', '10000000-0000-7000-8000-000000000002', true);
    SELECT pg_temp.new_procedure('f0000000-0000-7000-8000-000000000009', 'c0000000-0000-7000-8000-000000000001', 'planned', NULL, NULL); SET CONSTRAINTS ALL IMMEDIATE$statement$, '42501', 'new row violates row-level security policy for table "resources"', 'reader cannot record a procedure');
INSERT INTO grants (organization_id, resource_id, grantee_id, grantee_kind, role)
      VALUES ('20000000-0000-7000-8000-000000000001', '50000000-0000-7000-8000-000000000001', '10000000-0000-7000-8000-000000000002', 'user', 'reader');
    SELECT set_config('app.user_id', '10000000-0000-7000-8000-000000000002', true);
SET CONSTRAINTS ALL IMMEDIATE;
SET CONSTRAINTS ALL DEFERRED;
SELECT results_eq($query$SELECT count(*) FROM procedure_template_slots$query$, $expected$VALUES (9::bigint)$expected$, 'reader sees slots of templates in the project');
SELECT set_config('app.user_id', '10000000-0000-7000-8000-000000000001', true);
UPDATE resources SET archived_at = now() WHERE id = '50000000-0000-7000-8000-000000000001';
    UPDATE procedures SET note = 'late' WHERE id = 'f0000000-0000-7000-8000-000000000006';
SET CONSTRAINTS ALL IMMEDIATE;
SET CONSTRAINTS ALL DEFERRED;
SELECT results_eq($query$SELECT note FROM procedures WHERE id = 'f0000000-0000-7000-8000-000000000006'$query$, $expected$VALUES (NULL::text)$expected$, 'archived project refuses new bindings');
RESET ROLE;
SELECT * FROM finish();
ROLLBACK;
