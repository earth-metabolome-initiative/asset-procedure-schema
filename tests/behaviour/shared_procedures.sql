\set ON_ERROR_STOP 1
SET search_path = public, test_tools;
BEGIN;
\ir ../fixture.sql
SELECT no_plan();
SELECT pg_temp.new_model('60000000-0000-7000-8000-000000000001', 'shared sample', NULL, 'sample_models');
INSERT INTO physical_asset_models (id, organization_id) VALUES ('60000000-0000-7000-8000-000000000001', '20000000-0000-7000-8000-000000000001');
SELECT pg_temp.new_model('60000000-0000-7000-8000-000000000002', 'shared balance', NULL, 'weighing_device_models');
INSERT INTO physical_asset_models (id, organization_id) VALUES ('60000000-0000-7000-8000-000000000002', '20000000-0000-7000-8000-000000000001');
SELECT pg_temp.new_model('60000000-0000-7000-8000-000000000003', 'shared tube', NULL, 'volumetric_container_models');
INSERT INTO physical_asset_models (id, organization_id) VALUES ('60000000-0000-7000-8000-000000000003', '20000000-0000-7000-8000-000000000001');
SELECT pg_temp.new_model('60000000-0000-7000-8000-000000000004', 'shared pipette', NULL, 'volume_measuring_device_models');
INSERT INTO physical_asset_models (id, organization_id) VALUES ('60000000-0000-7000-8000-000000000004', '20000000-0000-7000-8000-000000000001');
INSERT INTO weighing_device_models (id, organization_id) VALUES ('60000000-0000-7000-8000-000000000002', '20000000-0000-7000-8000-000000000001');
INSERT INTO container_models (id, organization_id) VALUES ('60000000-0000-7000-8000-000000000003', '20000000-0000-7000-8000-000000000001');
INSERT INTO volume_measuring_device_models (id, organization_id) VALUES ('60000000-0000-7000-8000-000000000004', '20000000-0000-7000-8000-000000000001');
INSERT INTO volumetric_container_models (id, organization_id, volume_l) VALUES ('60000000-0000-7000-8000-000000000003', '20000000-0000-7000-8000-000000000001', 1.0);
SELECT pg_temp.new_model('60000000-0000-7000-8000-000000000005', 'shared source', NULL, 'sample_source_models');
INSERT INTO physical_asset_models (id, organization_id) VALUES ('60000000-0000-7000-8000-000000000005', '20000000-0000-7000-8000-000000000001');
INSERT INTO sample_source_models (id, organization_id) VALUES ('60000000-0000-7000-8000-000000000005', '20000000-0000-7000-8000-000000000001');
INSERT INTO sample_models (id, organization_id, sample_source_model_id) VALUES ('60000000-0000-7000-8000-000000000001', '20000000-0000-7000-8000-000000000001', '60000000-0000-7000-8000-000000000005');
SELECT pg_temp.new_template('60000000-0000-7000-8000-000000000010', 'shared weighing', 'weighing_procedure_templates');
SELECT pg_temp.new_slot('60000000-0000-7000-8000-000000000020', '60000000-0000-7000-8000-000000000010', 'weighed_asset', '60000000-0000-7000-8000-000000000001', 'input', 1, 1);
SELECT pg_temp.new_slot('60000000-0000-7000-8000-000000000021', '60000000-0000-7000-8000-000000000010', 'weighed_with', '60000000-0000-7000-8000-000000000002', 'instrument', 1, 1);
SELECT pg_temp.new_slot('60000000-0000-7000-8000-000000000022', '60000000-0000-7000-8000-000000000010', 'wrong_role', '60000000-0000-7000-8000-000000000001', 'consumed', 0, 1);

SELECT throws_ok($statement$INSERT INTO weighing_procedure_templates (id, organization_id, weighed_asset_slot_id, weighed_asset_model_id, weighed_with_slot_id, weighed_with_model_id) VALUES ('60000000-0000-7000-8000-000000000010', '20000000-0000-7000-8000-000000000001', '60000000-0000-7000-8000-000000000022', '60000000-0000-7000-8000-000000000001', '60000000-0000-7000-8000-000000000021', '60000000-0000-7000-8000-000000000002'); SET CONSTRAINTS ALL IMMEDIATE$statement$, '23503',
  NULL, 'weighing rejects a slot with another role');
SELECT throws_ok($statement$INSERT INTO weighing_procedure_templates (id, organization_id, weighed_asset_slot_id, weighed_asset_model_id, weighed_with_slot_id, weighed_with_model_id, weighed_asset_slot_role_id) VALUES ('60000000-0000-7000-8000-000000000010', '20000000-0000-7000-8000-000000000001', '60000000-0000-7000-8000-000000000020', '60000000-0000-7000-8000-000000000001', '60000000-0000-7000-8000-000000000021', '60000000-0000-7000-8000-000000000002', 'consumed'); SET CONSTRAINTS ALL IMMEDIATE$statement$, '428C9',
  'cannot insert a non-DEFAULT value into column "weighed_asset_slot_role_id"', 'weighing rejects an overridden typed role');
INSERT INTO weighing_procedure_templates (id, organization_id, weighed_asset_slot_id, weighed_asset_model_id, weighed_with_slot_id, weighed_with_model_id) VALUES ('60000000-0000-7000-8000-000000000010', '20000000-0000-7000-8000-000000000001', '60000000-0000-7000-8000-000000000020', '60000000-0000-7000-8000-000000000001', '60000000-0000-7000-8000-000000000021', '60000000-0000-7000-8000-000000000002');
SELECT pg_temp.new_template('60000000-0000-7000-8000-000000000011', 'published weighing', 'weighing_procedure_templates');
SELECT pg_temp.new_slot('60000000-0000-7000-8000-000000000023', '60000000-0000-7000-8000-000000000011', 'weighed_asset', '60000000-0000-7000-8000-000000000001', 'input', 1, 1);
SELECT pg_temp.new_slot('60000000-0000-7000-8000-000000000024', '60000000-0000-7000-8000-000000000011', 'weighed_with', '60000000-0000-7000-8000-000000000002', 'instrument', 1, 1);
INSERT INTO weighing_procedure_templates (id, organization_id, weighed_asset_slot_id, weighed_asset_model_id, weighed_with_slot_id, weighed_with_model_id) VALUES ('60000000-0000-7000-8000-000000000011', '20000000-0000-7000-8000-000000000001', '60000000-0000-7000-8000-000000000023', '60000000-0000-7000-8000-000000000001', '60000000-0000-7000-8000-000000000024', '60000000-0000-7000-8000-000000000002');
UPDATE procedure_templates SET state_id = 'published' WHERE id = '60000000-0000-7000-8000-000000000011';
SELECT throws_ok($statement$DELETE FROM weighing_procedure_templates WHERE id = '60000000-0000-7000-8000-000000000011'; SET CONSTRAINTS ALL IMMEDIATE$statement$, '23514',
  'procedure template 60000000-0000-7000-8000-000000000011 is not a draft, so its definition cannot change', 'published weighing rejects its subtype row');
UPDATE procedure_templates SET state_id = 'published' WHERE id = '60000000-0000-7000-8000-000000000010';
SELECT pg_temp.new_asset('60000000-0000-7000-8000-000000000040', 'shared sample one', '60000000-0000-7000-8000-000000000001', 'samples');
INSERT INTO physical_assets (id, organization_id, physical_asset_model_id) VALUES ('60000000-0000-7000-8000-000000000040', '20000000-0000-7000-8000-000000000001', '60000000-0000-7000-8000-000000000001');
INSERT INTO samples (id, organization_id, sample_model_id, sample_source_model_id) VALUES ('60000000-0000-7000-8000-000000000040', '20000000-0000-7000-8000-000000000001', '60000000-0000-7000-8000-000000000001', '60000000-0000-7000-8000-000000000005');
SELECT pg_temp.new_asset('60000000-0000-7000-8000-000000000041', 'shared balance one', '60000000-0000-7000-8000-000000000002', 'weighing_devices');
INSERT INTO physical_assets (id, organization_id, physical_asset_model_id) VALUES ('60000000-0000-7000-8000-000000000041', '20000000-0000-7000-8000-000000000001', '60000000-0000-7000-8000-000000000002');
INSERT INTO weighing_devices (id, organization_id, weighing_device_model_id) VALUES ('60000000-0000-7000-8000-000000000041', '20000000-0000-7000-8000-000000000001', '60000000-0000-7000-8000-000000000002');
SELECT pg_temp.new_procedure('60000000-0000-7000-8000-000000000050', '60000000-0000-7000-8000-000000000010', 'in_progress', NULL, NULL, NULL, 'weighing_procedures');
UPDATE procedures SET started_at = '2026-01-01 00:00:00+00' WHERE id = '60000000-0000-7000-8000-000000000050';
INSERT INTO weighing_procedures (id, organization_id, weighing_procedure_template_id, status_id) VALUES ('60000000-0000-7000-8000-000000000050', '20000000-0000-7000-8000-000000000001', '60000000-0000-7000-8000-000000000010', 'in_progress');
SELECT pg_temp.bind('60000000-0000-7000-8000-000000000060', '60000000-0000-7000-8000-000000000050', '60000000-0000-7000-8000-000000000020', '60000000-0000-7000-8000-000000000040');
SELECT pg_temp.bind('60000000-0000-7000-8000-000000000061', '60000000-0000-7000-8000-000000000050', '60000000-0000-7000-8000-000000000021', '60000000-0000-7000-8000-000000000041');
SET CONSTRAINTS ALL IMMEDIATE;
SET CONSTRAINTS ALL DEFERRED;
SELECT throws_ok($statement$UPDATE procedures SET status_id = 'completed', ended_at = '2026-01-01 01:00:00+00' WHERE id = '60000000-0000-7000-8000-000000000050'; SET CONSTRAINTS ALL IMMEDIATE$statement$, '23514',
  'new row for relation "weighing_procedures" violates check constraint "weighing_procedures_check"', 'weighing completion requires mass');
UPDATE weighing_procedures SET mass_kg = 0.01 WHERE id = '60000000-0000-7000-8000-000000000050'; UPDATE procedures SET status_id = 'completed', ended_at = '2026-01-01 01:00:00+00' WHERE id = '60000000-0000-7000-8000-000000000050';
SET CONSTRAINTS ALL IMMEDIATE;
SET CONSTRAINTS ALL DEFERRED;
SELECT results_eq($query$SELECT mass_kg, status_id FROM weighing_procedures WHERE id = '60000000-0000-7000-8000-000000000050'$query$,
  $expected$VALUES (0.01::double precision, 'completed'::text)$expected$, 'weighing completes with mass');
SELECT throws_ok($statement$UPDATE weighing_procedures SET mass_kg = 0.02 WHERE id = '60000000-0000-7000-8000-000000000050'; SET CONSTRAINTS ALL IMMEDIATE$statement$, '23514',
  'procedure 60000000-0000-7000-8000-000000000050 is completed, so it cannot change', 'completed weighing rejects measurement updates');
SELECT pg_temp.new_template('60000000-0000-7000-8000-000000000012', 'shared storage', 'storage_procedure_templates');
SELECT pg_temp.new_slot('60000000-0000-7000-8000-000000000025', '60000000-0000-7000-8000-000000000012', 'stored_asset', '60000000-0000-7000-8000-000000000001', 'input', 1, 1);
SELECT pg_temp.new_slot('60000000-0000-7000-8000-000000000026', '60000000-0000-7000-8000-000000000012', 'stored_into', '60000000-0000-7000-8000-000000000003', 'input', 1, 1);
SELECT throws_ok($statement$UPDATE procedure_template_slots SET placed_into_slot_id = '60000000-0000-7000-8000-000000000026' WHERE id = '60000000-0000-7000-8000-000000000025'; INSERT INTO storage_procedure_templates (id, organization_id, stored_asset_slot_id, stored_asset_model_id, stored_into_slot_id, stored_into_model_id) VALUES ('60000000-0000-7000-8000-000000000012', '20000000-0000-7000-8000-000000000001', '60000000-0000-7000-8000-000000000025', '60000000-0000-7000-8000-000000000001', '60000000-0000-7000-8000-000000000026', '60000000-0000-7000-8000-000000000003'); SET CONSTRAINTS ALL IMMEDIATE$statement$, '23503',
  'holder slot requires compatible models', 'storage requires directional compatibility');
SELECT pg_temp.create_ownable('60000000-0000-7000-8000-000000000070'); INSERT INTO container_compatibility_rules (id, organization_id, project_id, table_name_id, container_model_id, contained_asset_model_id) VALUES ('60000000-0000-7000-8000-000000000070', '20000000-0000-7000-8000-000000000001', '50000000-0000-7000-8000-000000000001', 'container_compatibility_rules', '60000000-0000-7000-8000-000000000003', '60000000-0000-7000-8000-000000000001'); UPDATE procedure_template_slots SET placed_into_slot_id = '60000000-0000-7000-8000-000000000026' WHERE id = '60000000-0000-7000-8000-000000000025'; INSERT INTO storage_procedure_templates (id, organization_id, stored_asset_slot_id, stored_asset_model_id, stored_into_slot_id, stored_into_model_id) VALUES ('60000000-0000-7000-8000-000000000012', '20000000-0000-7000-8000-000000000001', '60000000-0000-7000-8000-000000000025', '60000000-0000-7000-8000-000000000001', '60000000-0000-7000-8000-000000000026', '60000000-0000-7000-8000-000000000003');
SET CONSTRAINTS ALL IMMEDIATE;
SET CONSTRAINTS ALL DEFERRED;
SELECT isnt_empty($query$SELECT 1 FROM storage_procedure_templates WHERE id = '60000000-0000-7000-8000-000000000012'$query$, 'storage accepts its compatibility rule');
SELECT throws_ok($statement$UPDATE resources SET archived_at = now() WHERE id = '60000000-0000-7000-8000-000000000003'; UPDATE storage_procedure_templates SET temperature_k = 250 WHERE id = '60000000-0000-7000-8000-000000000012'; SELECT 1 / (count(*)::integer - 1) FROM storage_procedure_templates WHERE id = '60000000-0000-7000-8000-000000000012' AND temperature_k = 250$statement$, '22012', NULL,
  'archived referenced model leaves the template editable');
SELECT pg_temp.new_template('60000000-0000-7000-8000-000000000100', 'shared aliquoting', 'aliquoting_procedure_templates');
SELECT pg_temp.new_slot('60000000-0000-7000-8000-000000000101', '60000000-0000-7000-8000-000000000100', 'aliquoted_from', '60000000-0000-7000-8000-000000000003', 'input', 1, 1);
SELECT pg_temp.new_slot('60000000-0000-7000-8000-000000000102', '60000000-0000-7000-8000-000000000100', 'aliquoted_into', '60000000-0000-7000-8000-000000000003', 'input', 1, 1);
SELECT pg_temp.new_slot('60000000-0000-7000-8000-000000000103', '60000000-0000-7000-8000-000000000100', 'aliquoted_with', '60000000-0000-7000-8000-000000000004', 'instrument', 1, 1);
SELECT pg_temp.new_slot('60000000-0000-7000-8000-000000000104', '60000000-0000-7000-8000-000000000100', 'aliquot', '60000000-0000-7000-8000-000000000001', 'produced', 1, 1);
SELECT pg_temp.new_slot('60000000-0000-7000-8000-000000000105', '60000000-0000-7000-8000-000000000100', 'wrong_destination', '60000000-0000-7000-8000-000000000003', 'produced', 1, 1);
SELECT pg_temp.new_slot('60000000-0000-7000-8000-000000000106', '60000000-0000-7000-8000-000000000100', 'wrong_sample', '60000000-0000-7000-8000-000000000001', 'input', 1, 1);
SELECT pg_temp.new_slot('60000000-0000-7000-8000-000000000107', '60000000-0000-7000-8000-000000000100', 'source_sample', '60000000-0000-7000-8000-000000000001', 'input', 1, 1);

SELECT throws_ok($statement$UPDATE procedure_template_slots SET placed_into_slot_id = '60000000-0000-7000-8000-000000000105' WHERE id = '60000000-0000-7000-8000-000000000104'; INSERT INTO aliquoting_procedure_templates (
    id, organization_id, source_sample_slot_id, source_sample_model_id, aliquoted_from_slot_id, aliquoted_from_model_id, aliquoted_into_slot_id, aliquoted_into_model_id, aliquoted_with_slot_id, aliquoted_with_model_id, aliquot_slot_id, aliquot_model_id, volume_l)
    VALUES ('60000000-0000-7000-8000-000000000100', '20000000-0000-7000-8000-000000000001', '60000000-0000-7000-8000-000000000107', '60000000-0000-7000-8000-000000000001', '60000000-0000-7000-8000-000000000101', '60000000-0000-7000-8000-000000000003', '60000000-0000-7000-8000-000000000105', '60000000-0000-7000-8000-000000000003', '60000000-0000-7000-8000-000000000103', '60000000-0000-7000-8000-000000000004', '60000000-0000-7000-8000-000000000104', '60000000-0000-7000-8000-000000000001', 0.01); SET CONSTRAINTS ALL IMMEDIATE$statement$, '23503',
  NULL, 'aliquoting rejects a produced destination container');
SELECT throws_ok($statement$UPDATE procedure_template_slots SET placed_into_slot_id = '60000000-0000-7000-8000-000000000102' WHERE id = '60000000-0000-7000-8000-000000000106'; INSERT INTO aliquoting_procedure_templates (
    id, organization_id, source_sample_slot_id, source_sample_model_id, aliquoted_from_slot_id, aliquoted_from_model_id, aliquoted_into_slot_id, aliquoted_into_model_id, aliquoted_with_slot_id, aliquoted_with_model_id, aliquot_slot_id, aliquot_model_id, volume_l)
    VALUES ('60000000-0000-7000-8000-000000000100', '20000000-0000-7000-8000-000000000001', '60000000-0000-7000-8000-000000000107', '60000000-0000-7000-8000-000000000001', '60000000-0000-7000-8000-000000000101', '60000000-0000-7000-8000-000000000003', '60000000-0000-7000-8000-000000000102', '60000000-0000-7000-8000-000000000003', '60000000-0000-7000-8000-000000000103', '60000000-0000-7000-8000-000000000004', '60000000-0000-7000-8000-000000000106', '60000000-0000-7000-8000-000000000001', 0.01); SET CONSTRAINTS ALL IMMEDIATE$statement$, '23503',
  NULL, 'aliquoting rejects an input sample slot');
UPDATE procedure_template_slots SET placed_into_slot_id = '60000000-0000-7000-8000-000000000102' WHERE id = '60000000-0000-7000-8000-000000000104'; INSERT INTO aliquoting_procedure_templates (
    id, organization_id, source_sample_slot_id, source_sample_model_id, aliquoted_from_slot_id, aliquoted_from_model_id, aliquoted_into_slot_id, aliquoted_into_model_id, aliquoted_with_slot_id, aliquoted_with_model_id, aliquot_slot_id, aliquot_model_id, volume_l)
    VALUES ('60000000-0000-7000-8000-000000000100', '20000000-0000-7000-8000-000000000001', '60000000-0000-7000-8000-000000000107', '60000000-0000-7000-8000-000000000001', '60000000-0000-7000-8000-000000000101', '60000000-0000-7000-8000-000000000003', '60000000-0000-7000-8000-000000000102', '60000000-0000-7000-8000-000000000003', '60000000-0000-7000-8000-000000000103', '60000000-0000-7000-8000-000000000004', '60000000-0000-7000-8000-000000000104', '60000000-0000-7000-8000-000000000001', 0.01);
SET CONSTRAINTS ALL IMMEDIATE;
SET CONSTRAINTS ALL DEFERRED;
SELECT isnt_empty($query$SELECT 1 FROM aliquoting_procedure_templates WHERE id = '60000000-0000-7000-8000-000000000100'$query$, 'aliquoting accepts input containers and a produced sample');
SELECT pg_temp.new_template('60000000-0000-7000-8000-000000000110', 'shared pouring', 'pouring_procedure_templates');
SELECT pg_temp.new_slot('60000000-0000-7000-8000-000000000111', '60000000-0000-7000-8000-000000000110', 'poured_from', '60000000-0000-7000-8000-000000000003', 'input', 1, 1);
SELECT pg_temp.new_slot('60000000-0000-7000-8000-000000000112', '60000000-0000-7000-8000-000000000110', 'poured_into', '60000000-0000-7000-8000-000000000003', 'input', 1, 1);
SELECT pg_temp.new_slot('60000000-0000-7000-8000-000000000113', '60000000-0000-7000-8000-000000000110', 'measured_with', '60000000-0000-7000-8000-000000000004', 'instrument', 1, 1);
SELECT pg_temp.new_slot('60000000-0000-7000-8000-000000000114', '60000000-0000-7000-8000-000000000110', 'poured_sample', '60000000-0000-7000-8000-000000000001', 'produced', 1, 1);
SELECT pg_temp.new_slot('60000000-0000-7000-8000-000000000115', '60000000-0000-7000-8000-000000000110', 'wrong_destination', '60000000-0000-7000-8000-000000000003', 'produced', 1, 1);
SELECT pg_temp.new_slot('60000000-0000-7000-8000-000000000116', '60000000-0000-7000-8000-000000000110', 'wrong_sample', '60000000-0000-7000-8000-000000000001', 'input', 1, 1);
SELECT pg_temp.new_slot('60000000-0000-7000-8000-000000000117', '60000000-0000-7000-8000-000000000110', 'source_sample', '60000000-0000-7000-8000-000000000001', 'input', 1, 1);
SELECT throws_ok($statement$UPDATE procedure_template_slots SET placed_into_slot_id = '60000000-0000-7000-8000-000000000115' WHERE id = '60000000-0000-7000-8000-000000000114'; INSERT INTO pouring_procedure_templates (
    id, organization_id, source_sample_slot_id, source_sample_model_id, poured_from_slot_id, poured_from_model_id, poured_into_slot_id, poured_into_model_id, measured_with_slot_id, measured_with_model_id, poured_sample_slot_id, poured_sample_model_id, volume_l)
    VALUES ('60000000-0000-7000-8000-000000000110', '20000000-0000-7000-8000-000000000001', '60000000-0000-7000-8000-000000000117', '60000000-0000-7000-8000-000000000001', '60000000-0000-7000-8000-000000000111', '60000000-0000-7000-8000-000000000003', '60000000-0000-7000-8000-000000000115', '60000000-0000-7000-8000-000000000003', '60000000-0000-7000-8000-000000000113', '60000000-0000-7000-8000-000000000004', '60000000-0000-7000-8000-000000000114', '60000000-0000-7000-8000-000000000001', 0.01); SET CONSTRAINTS ALL IMMEDIATE$statement$, '23503',
  NULL, 'pouring rejects a produced destination container');
SELECT throws_ok($statement$UPDATE procedure_template_slots SET placed_into_slot_id = '60000000-0000-7000-8000-000000000112' WHERE id = '60000000-0000-7000-8000-000000000116'; INSERT INTO pouring_procedure_templates (
    id, organization_id, source_sample_slot_id, source_sample_model_id, poured_from_slot_id, poured_from_model_id, poured_into_slot_id, poured_into_model_id, measured_with_slot_id, measured_with_model_id, poured_sample_slot_id, poured_sample_model_id, volume_l)
    VALUES ('60000000-0000-7000-8000-000000000110', '20000000-0000-7000-8000-000000000001', '60000000-0000-7000-8000-000000000117', '60000000-0000-7000-8000-000000000001', '60000000-0000-7000-8000-000000000111', '60000000-0000-7000-8000-000000000003', '60000000-0000-7000-8000-000000000112', '60000000-0000-7000-8000-000000000003', '60000000-0000-7000-8000-000000000113', '60000000-0000-7000-8000-000000000004', '60000000-0000-7000-8000-000000000116', '60000000-0000-7000-8000-000000000001', 0.01); SET CONSTRAINTS ALL IMMEDIATE$statement$, '23503',
  NULL, 'pouring rejects an input sample slot');
UPDATE procedure_template_slots SET placed_into_slot_id = '60000000-0000-7000-8000-000000000112' WHERE id = '60000000-0000-7000-8000-000000000114'; INSERT INTO pouring_procedure_templates (
    id, organization_id, source_sample_slot_id, source_sample_model_id, poured_from_slot_id, poured_from_model_id, poured_into_slot_id, poured_into_model_id, measured_with_slot_id, measured_with_model_id, poured_sample_slot_id, poured_sample_model_id, volume_l)
    VALUES ('60000000-0000-7000-8000-000000000110', '20000000-0000-7000-8000-000000000001', '60000000-0000-7000-8000-000000000117', '60000000-0000-7000-8000-000000000001', '60000000-0000-7000-8000-000000000111', '60000000-0000-7000-8000-000000000003', '60000000-0000-7000-8000-000000000112', '60000000-0000-7000-8000-000000000003', '60000000-0000-7000-8000-000000000113', '60000000-0000-7000-8000-000000000004', '60000000-0000-7000-8000-000000000114', '60000000-0000-7000-8000-000000000001', 0.01);
SET CONSTRAINTS ALL IMMEDIATE;
SET CONSTRAINTS ALL DEFERRED;
SELECT isnt_empty($query$SELECT 1 FROM pouring_procedure_templates WHERE id = '60000000-0000-7000-8000-000000000110'$query$, 'pouring accepts input containers and a produced sample');
SELECT pg_temp.new_template('60000000-0000-7000-8000-000000000120', 'shared fractioning', 'fractioning_procedure_templates');
SELECT pg_temp.new_slot('60000000-0000-7000-8000-000000000121', '60000000-0000-7000-8000-000000000120', 'fragment_container', '60000000-0000-7000-8000-000000000003', 'input', 1, 1);
SELECT pg_temp.new_slot('60000000-0000-7000-8000-000000000122', '60000000-0000-7000-8000-000000000120', 'fragment_placed_into', '60000000-0000-7000-8000-000000000003', 'input', 1, 1);
SELECT pg_temp.new_slot('60000000-0000-7000-8000-000000000123', '60000000-0000-7000-8000-000000000120', 'weighed_with', '60000000-0000-7000-8000-000000000002', 'instrument', 1, 1);
SELECT pg_temp.new_slot('60000000-0000-7000-8000-000000000124', '60000000-0000-7000-8000-000000000120', 'fraction', '60000000-0000-7000-8000-000000000001', 'produced', 1, 1);
SELECT pg_temp.new_slot('60000000-0000-7000-8000-000000000125', '60000000-0000-7000-8000-000000000120', 'wrong_destination', '60000000-0000-7000-8000-000000000003', 'produced', 1, 1);
SELECT pg_temp.new_slot('60000000-0000-7000-8000-000000000126', '60000000-0000-7000-8000-000000000120', 'wrong_sample', '60000000-0000-7000-8000-000000000001', 'input', 1, 1);
SELECT pg_temp.new_slot('60000000-0000-7000-8000-000000000127', '60000000-0000-7000-8000-000000000120', 'source_sample', '60000000-0000-7000-8000-000000000001', 'input', 1, 1);
SELECT throws_ok($statement$UPDATE procedure_template_slots SET placed_into_slot_id = '60000000-0000-7000-8000-000000000125' WHERE id = '60000000-0000-7000-8000-000000000124'; INSERT INTO fractioning_procedure_templates (
    id, organization_id, source_sample_slot_id, source_sample_model_id, fragment_container_slot_id, fragment_container_model_id, fragment_placed_into_slot_id, fragment_placed_into_model_id, weighed_with_slot_id, weighed_with_model_id, fraction_slot_id, fraction_model_id, mass_kg, tolerance_percentage)
    VALUES ('60000000-0000-7000-8000-000000000120', '20000000-0000-7000-8000-000000000001', '60000000-0000-7000-8000-000000000127', '60000000-0000-7000-8000-000000000001', '60000000-0000-7000-8000-000000000121', '60000000-0000-7000-8000-000000000003', '60000000-0000-7000-8000-000000000125', '60000000-0000-7000-8000-000000000003', '60000000-0000-7000-8000-000000000123', '60000000-0000-7000-8000-000000000002', '60000000-0000-7000-8000-000000000124', '60000000-0000-7000-8000-000000000001', 0.01, 1.0); SET CONSTRAINTS ALL IMMEDIATE$statement$, '23503',
  NULL, 'fractioning rejects a produced destination container');
SELECT throws_ok($statement$UPDATE procedure_template_slots SET placed_into_slot_id = '60000000-0000-7000-8000-000000000122' WHERE id = '60000000-0000-7000-8000-000000000126'; INSERT INTO fractioning_procedure_templates (
    id, organization_id, source_sample_slot_id, source_sample_model_id, fragment_container_slot_id, fragment_container_model_id, fragment_placed_into_slot_id, fragment_placed_into_model_id, weighed_with_slot_id, weighed_with_model_id, fraction_slot_id, fraction_model_id, mass_kg, tolerance_percentage)
    VALUES ('60000000-0000-7000-8000-000000000120', '20000000-0000-7000-8000-000000000001', '60000000-0000-7000-8000-000000000127', '60000000-0000-7000-8000-000000000001', '60000000-0000-7000-8000-000000000121', '60000000-0000-7000-8000-000000000003', '60000000-0000-7000-8000-000000000122', '60000000-0000-7000-8000-000000000003', '60000000-0000-7000-8000-000000000123', '60000000-0000-7000-8000-000000000002', '60000000-0000-7000-8000-000000000126', '60000000-0000-7000-8000-000000000001', 0.01, 1.0); SET CONSTRAINTS ALL IMMEDIATE$statement$, '23503',
  NULL, 'fractioning rejects an input sample slot');
UPDATE procedure_template_slots SET placed_into_slot_id = '60000000-0000-7000-8000-000000000122' WHERE id = '60000000-0000-7000-8000-000000000124'; INSERT INTO fractioning_procedure_templates (
    id, organization_id, source_sample_slot_id, source_sample_model_id, fragment_container_slot_id, fragment_container_model_id, fragment_placed_into_slot_id, fragment_placed_into_model_id, weighed_with_slot_id, weighed_with_model_id, fraction_slot_id, fraction_model_id, mass_kg, tolerance_percentage)
    VALUES ('60000000-0000-7000-8000-000000000120', '20000000-0000-7000-8000-000000000001', '60000000-0000-7000-8000-000000000127', '60000000-0000-7000-8000-000000000001', '60000000-0000-7000-8000-000000000121', '60000000-0000-7000-8000-000000000003', '60000000-0000-7000-8000-000000000122', '60000000-0000-7000-8000-000000000003', '60000000-0000-7000-8000-000000000123', '60000000-0000-7000-8000-000000000002', '60000000-0000-7000-8000-000000000124', '60000000-0000-7000-8000-000000000001', 0.01, 1.0);
SET CONSTRAINTS ALL IMMEDIATE;
SET CONSTRAINTS ALL DEFERRED;
SELECT isnt_empty($query$SELECT 1 FROM fractioning_procedure_templates WHERE id = '60000000-0000-7000-8000-000000000120'$query$, 'fractioning accepts input containers and a produced sample');
SELECT pg_temp.new_template('60000000-0000-7000-8000-000000000130', 'shared supernatant', 'supernatant_procedure_templates');
SELECT pg_temp.new_slot('60000000-0000-7000-8000-000000000131', '60000000-0000-7000-8000-000000000130', 'stratified_source', '60000000-0000-7000-8000-000000000003', 'input', 1, 1);
SELECT pg_temp.new_slot('60000000-0000-7000-8000-000000000132', '60000000-0000-7000-8000-000000000130', 'supernatant_destination', '60000000-0000-7000-8000-000000000003', 'input', 1, 1);
SELECT pg_temp.new_slot('60000000-0000-7000-8000-000000000133', '60000000-0000-7000-8000-000000000130', 'transferred_with', '60000000-0000-7000-8000-000000000004', 'instrument', 1, 1);
SELECT pg_temp.new_slot('60000000-0000-7000-8000-000000000134', '60000000-0000-7000-8000-000000000130', 'supernatant', '60000000-0000-7000-8000-000000000001', 'produced', 1, 1);
SELECT pg_temp.new_slot('60000000-0000-7000-8000-000000000135', '60000000-0000-7000-8000-000000000130', 'wrong_destination', '60000000-0000-7000-8000-000000000003', 'produced', 1, 1);
SELECT pg_temp.new_slot('60000000-0000-7000-8000-000000000136', '60000000-0000-7000-8000-000000000130', 'wrong_sample', '60000000-0000-7000-8000-000000000001', 'input', 1, 1);
SELECT pg_temp.new_slot('60000000-0000-7000-8000-000000000137', '60000000-0000-7000-8000-000000000130', 'source_sample', '60000000-0000-7000-8000-000000000001', 'input', 1, 1);
SELECT throws_ok($statement$UPDATE procedure_template_slots SET placed_into_slot_id = '60000000-0000-7000-8000-000000000135' WHERE id = '60000000-0000-7000-8000-000000000134'; INSERT INTO supernatant_procedure_templates (
    id, organization_id, source_sample_slot_id, source_sample_model_id, stratified_source_slot_id, stratified_source_model_id, supernatant_destination_slot_id, supernatant_destination_model_id, transferred_with_slot_id, transferred_with_model_id, supernatant_slot_id, supernatant_model_id, volume_l)
    VALUES ('60000000-0000-7000-8000-000000000130', '20000000-0000-7000-8000-000000000001', '60000000-0000-7000-8000-000000000137', '60000000-0000-7000-8000-000000000001', '60000000-0000-7000-8000-000000000131', '60000000-0000-7000-8000-000000000003', '60000000-0000-7000-8000-000000000135', '60000000-0000-7000-8000-000000000003', '60000000-0000-7000-8000-000000000133', '60000000-0000-7000-8000-000000000004', '60000000-0000-7000-8000-000000000134', '60000000-0000-7000-8000-000000000001', 0.01); SET CONSTRAINTS ALL IMMEDIATE$statement$, '23503',
  NULL, 'supernatant rejects a produced destination container');
SELECT throws_ok($statement$UPDATE procedure_template_slots SET placed_into_slot_id = '60000000-0000-7000-8000-000000000132' WHERE id = '60000000-0000-7000-8000-000000000136'; INSERT INTO supernatant_procedure_templates (
    id, organization_id, source_sample_slot_id, source_sample_model_id, stratified_source_slot_id, stratified_source_model_id, supernatant_destination_slot_id, supernatant_destination_model_id, transferred_with_slot_id, transferred_with_model_id, supernatant_slot_id, supernatant_model_id, volume_l)
    VALUES ('60000000-0000-7000-8000-000000000130', '20000000-0000-7000-8000-000000000001', '60000000-0000-7000-8000-000000000137', '60000000-0000-7000-8000-000000000001', '60000000-0000-7000-8000-000000000131', '60000000-0000-7000-8000-000000000003', '60000000-0000-7000-8000-000000000132', '60000000-0000-7000-8000-000000000003', '60000000-0000-7000-8000-000000000133', '60000000-0000-7000-8000-000000000004', '60000000-0000-7000-8000-000000000136', '60000000-0000-7000-8000-000000000001', 0.01); SET CONSTRAINTS ALL IMMEDIATE$statement$, '23503',
  NULL, 'supernatant rejects an input sample slot');
UPDATE procedure_template_slots SET placed_into_slot_id = '60000000-0000-7000-8000-000000000132' WHERE id = '60000000-0000-7000-8000-000000000134'; INSERT INTO supernatant_procedure_templates (
    id, organization_id, source_sample_slot_id, source_sample_model_id, stratified_source_slot_id, stratified_source_model_id, supernatant_destination_slot_id, supernatant_destination_model_id, transferred_with_slot_id, transferred_with_model_id, supernatant_slot_id, supernatant_model_id, volume_l)
    VALUES ('60000000-0000-7000-8000-000000000130', '20000000-0000-7000-8000-000000000001', '60000000-0000-7000-8000-000000000137', '60000000-0000-7000-8000-000000000001', '60000000-0000-7000-8000-000000000131', '60000000-0000-7000-8000-000000000003', '60000000-0000-7000-8000-000000000132', '60000000-0000-7000-8000-000000000003', '60000000-0000-7000-8000-000000000133', '60000000-0000-7000-8000-000000000004', '60000000-0000-7000-8000-000000000134', '60000000-0000-7000-8000-000000000001', 0.01);
SET CONSTRAINTS ALL IMMEDIATE;
SET CONSTRAINTS ALL DEFERRED;
SELECT isnt_empty($query$SELECT 1 FROM supernatant_procedure_templates WHERE id = '60000000-0000-7000-8000-000000000130'$query$, 'supernatant accepts input containers and a produced sample');

SELECT throws_ok($statement$UPDATE pouring_procedure_templates SET volume_l = 2.0 WHERE id = '60000000-0000-7000-8000-000000000110'$statement$, '23514', NULL, 'transfer template respects destination capacity');
SELECT has_column('public', 'storage_procedures', 'temperature_k', 'storage records temperature');
SELECT has_column('public', 'freezing_procedures', 'temperature_k', 'freezing records temperature');
SELECT has_column('public', 'freeze_drying_procedures', 'pressure_pa', 'freeze drying records pressure');
SELECT has_column('public', 'centrifuge_procedures', 'speed_rpm', 'centrifuge records speed');
SELECT has_column('public', 'ball_mill_procedures', 'frequency_hz', 'ball mill records frequency');
SELECT has_column('public', 'pouring_procedures', 'volume_l', 'pouring records volume');
SELECT has_column('public', 'aliquoting_procedures', 'volume_l', 'aliquoting records volume');
SELECT has_column('public', 'supernatant_procedures', 'volume_l', 'supernatant records volume');
SELECT has_table('public', 'retrieval_procedures', 'retrieval has typed runs');
SELECT has_function('public', 'procedure_deviations', ARRAY['uuid'], 'UI can list deviations');

UPDATE procedure_templates SET state_id = 'published' WHERE id = '60000000-0000-7000-8000-000000000012';
SELECT pg_temp.new_asset('60000000-0000-7000-8000-000000000201', 'storage container', '60000000-0000-7000-8000-000000000003', 'volumetric_containers');
INSERT INTO physical_assets (id, organization_id, physical_asset_model_id) VALUES ('60000000-0000-7000-8000-000000000201', '20000000-0000-7000-8000-000000000001', '60000000-0000-7000-8000-000000000003');
INSERT INTO containers (id, organization_id, container_model_id) VALUES ('60000000-0000-7000-8000-000000000201', '20000000-0000-7000-8000-000000000001', '60000000-0000-7000-8000-000000000003');
INSERT INTO volumetric_containers (id, organization_id, volumetric_container_model_id) VALUES ('60000000-0000-7000-8000-000000000201', '20000000-0000-7000-8000-000000000001', '60000000-0000-7000-8000-000000000003');
SELECT pg_temp.new_asset('60000000-0000-7000-8000-000000000202', 'second stored sample', '60000000-0000-7000-8000-000000000001', 'samples');
INSERT INTO physical_assets (id, organization_id, physical_asset_model_id) VALUES ('60000000-0000-7000-8000-000000000202', '20000000-0000-7000-8000-000000000001', '60000000-0000-7000-8000-000000000001');
INSERT INTO samples (id, organization_id, sample_model_id, sample_source_model_id) VALUES ('60000000-0000-7000-8000-000000000202', '20000000-0000-7000-8000-000000000001', '60000000-0000-7000-8000-000000000001', '60000000-0000-7000-8000-000000000005');
SELECT pg_temp.new_procedure('60000000-0000-7000-8000-000000000210', '60000000-0000-7000-8000-000000000012', 'in_progress', NULL, NULL, NULL, 'storage_procedures');
UPDATE procedures SET started_at = '2026-01-02 00:00:00+00' WHERE id = '60000000-0000-7000-8000-000000000210';
INSERT INTO storage_procedures (id, organization_id, storage_procedure_template_id, status_id)
VALUES ('60000000-0000-7000-8000-000000000210', '20000000-0000-7000-8000-000000000001', '60000000-0000-7000-8000-000000000012', 'in_progress');
SELECT pg_temp.bind('60000000-0000-7000-8000-000000000211', '60000000-0000-7000-8000-000000000210', '60000000-0000-7000-8000-000000000025', '60000000-0000-7000-8000-000000000040');
SELECT pg_temp.bind('60000000-0000-7000-8000-000000000212', '60000000-0000-7000-8000-000000000210', '60000000-0000-7000-8000-000000000026', '60000000-0000-7000-8000-000000000201');
SELECT throws_ok($statement$UPDATE procedures SET status_id = 'completed', ended_at = '2026-01-02 01:00:00+00' WHERE id = '60000000-0000-7000-8000-000000000210'$statement$, '23514', NULL, 'storage completion requires temperature');
UPDATE storage_procedures SET temperature_k = 400 WHERE id = '60000000-0000-7000-8000-000000000210';
SELECT results_eq($query$SELECT * FROM procedure_deviations('60000000-0000-7000-8000-000000000210')$query$, $expected$VALUES ('temperature_k'::text)$expected$, 'UI lists measured temperature deviation');
SELECT throws_ok($statement$UPDATE procedures SET status_id = 'completed', ended_at = '2026-01-02 01:00:00+00' WHERE id = '60000000-0000-7000-8000-000000000210'; SET CONSTRAINTS ALL IMMEDIATE$statement$, '23514', NULL, 'deviation requires a reason');
UPDATE procedures SET note = 'Warm storage requested', status_id = 'completed', ended_at = '2026-01-02 01:00:00+00' WHERE id = '60000000-0000-7000-8000-000000000210';
SET CONSTRAINTS ALL IMMEDIATE;
SET CONSTRAINTS ALL DEFERRED;
SELECT is((SELECT temperature_k FROM storage_procedures WHERE id = '60000000-0000-7000-8000-000000000210'), 400::double precision, 'justified deviation retains measurement');
SELECT pg_temp.new_procedure('60000000-0000-7000-8000-000000000220', '60000000-0000-7000-8000-000000000012', 'in_progress', NULL, NULL, NULL, 'storage_procedures');
UPDATE procedures SET started_at = '2026-01-02 02:00:00+00' WHERE id = '60000000-0000-7000-8000-000000000220';
INSERT INTO storage_procedures (id, organization_id, storage_procedure_template_id, status_id, temperature_k)
VALUES ('60000000-0000-7000-8000-000000000220', '20000000-0000-7000-8000-000000000001', '60000000-0000-7000-8000-000000000012', 'in_progress', 293.15);
SELECT pg_temp.bind('60000000-0000-7000-8000-000000000221', '60000000-0000-7000-8000-000000000220', '60000000-0000-7000-8000-000000000025', '60000000-0000-7000-8000-000000000202');
SELECT pg_temp.bind('60000000-0000-7000-8000-000000000222', '60000000-0000-7000-8000-000000000220', '60000000-0000-7000-8000-000000000026', '60000000-0000-7000-8000-000000000201');
SELECT throws_ok($statement$UPDATE procedures SET status_id = 'completed', ended_at = '2026-01-02 03:00:00+00' WHERE id = '60000000-0000-7000-8000-000000000220'; SET CONSTRAINTS ALL IMMEDIATE$statement$, '23514', NULL, 'overlapping storage runs cannot exceed capacity');
SET CONSTRAINTS ALL DEFERRED;
SELECT pg_temp.new_template('60000000-0000-7000-8000-000000000230', 'shared retrieval', 'retrieval_procedure_templates');
SELECT pg_temp.new_slot('60000000-0000-7000-8000-000000000231', '60000000-0000-7000-8000-000000000230', 'retrieved_asset', '60000000-0000-7000-8000-000000000001', 'input', 1, 1);
SELECT pg_temp.new_slot('60000000-0000-7000-8000-000000000232', '60000000-0000-7000-8000-000000000230', 'retrieved_from', '60000000-0000-7000-8000-000000000003', 'input', 1, 1);
UPDATE procedure_template_slots SET retrieved_from_slot_id = '60000000-0000-7000-8000-000000000232' WHERE id = '60000000-0000-7000-8000-000000000231'; INSERT INTO retrieval_procedure_templates (id, organization_id, retrieved_asset_slot_id, retrieved_asset_model_id, retrieved_from_slot_id, retrieved_from_model_id)
VALUES ('60000000-0000-7000-8000-000000000230', '20000000-0000-7000-8000-000000000001', '60000000-0000-7000-8000-000000000231', '60000000-0000-7000-8000-000000000001', '60000000-0000-7000-8000-000000000232', '60000000-0000-7000-8000-000000000003');
UPDATE procedure_templates SET state_id = 'published' WHERE id = '60000000-0000-7000-8000-000000000230';
SELECT pg_temp.new_procedure('60000000-0000-7000-8000-000000000240', '60000000-0000-7000-8000-000000000230', 'in_progress', NULL, NULL, NULL, 'retrieval_procedures');
UPDATE procedures SET started_at = '2026-01-02 03:00:00+00' WHERE id = '60000000-0000-7000-8000-000000000240';
INSERT INTO retrieval_procedures (id, organization_id, retrieval_procedure_template_id, status_id)
VALUES ('60000000-0000-7000-8000-000000000240', '20000000-0000-7000-8000-000000000001', '60000000-0000-7000-8000-000000000230', 'in_progress');
SELECT pg_temp.bind('60000000-0000-7000-8000-000000000241', '60000000-0000-7000-8000-000000000240', '60000000-0000-7000-8000-000000000231', '60000000-0000-7000-8000-000000000040');
SELECT pg_temp.bind('60000000-0000-7000-8000-000000000242', '60000000-0000-7000-8000-000000000240', '60000000-0000-7000-8000-000000000232', '60000000-0000-7000-8000-000000000201');
SELECT lives_ok($statement$UPDATE procedures SET status_id = 'completed', ended_at = '2026-01-02 03:00:00+00' WHERE id = '60000000-0000-7000-8000-000000000220'; SET CONSTRAINTS ALL IMMEDIATE$statement$, 'retrieval frees capacity at its start');
SET CONSTRAINTS ALL DEFERRED;
SELECT throws_ok($statement$UPDATE procedures SET started_at = '2026-01-02 04:00:00+00' WHERE id = '60000000-0000-7000-8000-000000000240'; SET CONSTRAINTS ALL IMMEDIATE$statement$, '23514', NULL, 'retrieval time edits preserve historical capacity');
SELECT throws_ok($statement$DELETE FROM procedures WHERE id = '60000000-0000-7000-8000-000000000240'; SET CONSTRAINTS ALL IMMEDIATE$statement$, '23514', NULL, 'retrieval deletion cannot overfill a historical container');
SELECT lives_ok($statement$UPDATE procedures SET status_id = 'aborted', ended_at = '2026-01-02 03:30:00+00' WHERE id = '60000000-0000-7000-8000-000000000240'; SET CONSTRAINTS ALL IMMEDIATE$statement$, 'an aborted retrieval keeps the capacity it freed');
SET CONSTRAINTS ALL DEFERRED;

SELECT pg_temp.new_model('60000000-0000-7000-8000-000000000300', 'freezer model', NULL, 'freezer_models');
INSERT INTO physical_asset_models (id, organization_id) VALUES ('60000000-0000-7000-8000-000000000300', '20000000-0000-7000-8000-000000000001');
INSERT INTO freezer_models (id, organization_id) VALUES ('60000000-0000-7000-8000-000000000300', '20000000-0000-7000-8000-000000000001');
SELECT pg_temp.new_template('60000000-0000-7000-8000-000000000301', 'freezing treatment', 'freezing_procedure_templates');
SELECT pg_temp.new_slot('60000000-0000-7000-8000-000000000302', '60000000-0000-7000-8000-000000000301', 'frozen_with', '60000000-0000-7000-8000-000000000300', 'instrument', 1, 1);
SELECT pg_temp.new_slot('60000000-0000-7000-8000-000000000303', '60000000-0000-7000-8000-000000000301', 'frozen_container', '60000000-0000-7000-8000-000000000003', 'input', 1, 1);
SELECT throws_ok($statement$INSERT INTO freezing_procedure_templates (id, organization_id, frozen_with_slot_id, frozen_with_model_id, frozen_container_slot_id, frozen_container_model_id, duration_s)
VALUES ('60000000-0000-7000-8000-000000000301', '20000000-0000-7000-8000-000000000001', '60000000-0000-7000-8000-000000000302', '60000000-0000-7000-8000-000000000300', '60000000-0000-7000-8000-000000000303', '60000000-0000-7000-8000-000000000003', NULL)$statement$, '23502', NULL, 'freezing refuses missing duration');
INSERT INTO freezing_procedure_templates (id, organization_id, frozen_with_slot_id, frozen_with_model_id, frozen_container_slot_id, frozen_container_model_id, duration_s)
VALUES ('60000000-0000-7000-8000-000000000301', '20000000-0000-7000-8000-000000000001', '60000000-0000-7000-8000-000000000302', '60000000-0000-7000-8000-000000000300', '60000000-0000-7000-8000-000000000303', '60000000-0000-7000-8000-000000000003', 3600);
SELECT throws_ok($statement$UPDATE freezing_procedure_templates SET duration_s = 604801 WHERE id = '60000000-0000-7000-8000-000000000301'$statement$, '23514', NULL, 'freezing bounds duration');
SELECT throws_ok($statement$UPDATE freezing_procedure_templates SET duration_tolerance_percentage = 101 WHERE id = '60000000-0000-7000-8000-000000000301'$statement$, '23514', NULL, 'duration tolerance cannot exceed one hundred percent');
UPDATE procedure_templates SET state_id = 'published' WHERE id = '60000000-0000-7000-8000-000000000301';
SELECT pg_temp.new_procedure('60000000-0000-7000-8000-000000000310', '60000000-0000-7000-8000-000000000301', 'in_progress', NULL, NULL, NULL, 'freezing_procedures');
UPDATE procedures SET started_at = '2026-01-03 00:00:00+00' WHERE id = '60000000-0000-7000-8000-000000000310';
INSERT INTO freezing_procedures (id, organization_id, freezing_procedure_template_id, status_id, temperature_k)
VALUES ('60000000-0000-7000-8000-000000000310', '20000000-0000-7000-8000-000000000001', '60000000-0000-7000-8000-000000000301', 'in_progress', 203.15);
SELECT pg_temp.bind_model('60000000-0000-7000-8000-000000000311', '60000000-0000-7000-8000-000000000310', '60000000-0000-7000-8000-000000000302', NULL, '60000000-0000-7000-8000-000000000300');
SELECT pg_temp.bind('60000000-0000-7000-8000-000000000312', '60000000-0000-7000-8000-000000000310', '60000000-0000-7000-8000-000000000303', '60000000-0000-7000-8000-000000000201');
SELECT throws_ok($statement$UPDATE procedures SET status_id = 'completed', ended_at = '2026-01-03 02:00:00+00' WHERE id = '60000000-0000-7000-8000-000000000310'; SET CONSTRAINTS ALL IMMEDIATE$statement$, '23514', NULL, 'duration deviation requires a note');
SELECT lives_ok($statement$UPDATE procedures SET status_id = 'completed', ended_at = '2026-01-03 01:03:00+00' WHERE id = '60000000-0000-7000-8000-000000000310'; SET CONSTRAINTS ALL IMMEDIATE$statement$, 'duration tolerance includes its boundary');
SET CONSTRAINTS ALL DEFERRED;

UPDATE procedure_template_slots SET minimum_count = 0 WHERE procedure_template_id = '60000000-0000-7000-8000-000000000110' AND name IN ('wrong_destination', 'wrong_sample');
UPDATE procedure_templates SET state_id = 'published' WHERE id = '60000000-0000-7000-8000-000000000110';
SELECT pg_temp.new_procedure('60000000-0000-7000-8000-000000000400', '60000000-0000-7000-8000-000000000110', 'in_progress', NULL, NULL, NULL, 'pouring_procedures');
UPDATE procedures SET started_at = '2026-01-04 00:00:00+00' WHERE id = '60000000-0000-7000-8000-000000000400';
INSERT INTO pouring_procedures (id, organization_id, pouring_procedure_template_id, status_id, volume_l) VALUES ('60000000-0000-7000-8000-000000000400', '20000000-0000-7000-8000-000000000001', '60000000-0000-7000-8000-000000000110', 'in_progress', 2.0);
SELECT pg_temp.bind_model('60000000-0000-7000-8000-000000000401', '60000000-0000-7000-8000-000000000400', '60000000-0000-7000-8000-000000000111', NULL, '60000000-0000-7000-8000-000000000003');
SELECT pg_temp.bind_model('60000000-0000-7000-8000-000000000402', '60000000-0000-7000-8000-000000000400', '60000000-0000-7000-8000-000000000112', NULL, '60000000-0000-7000-8000-000000000003');
SELECT pg_temp.bind_model('60000000-0000-7000-8000-000000000403', '60000000-0000-7000-8000-000000000400', '60000000-0000-7000-8000-000000000113', NULL, '60000000-0000-7000-8000-000000000004');
SELECT pg_temp.bind_model('60000000-0000-7000-8000-000000000404', '60000000-0000-7000-8000-000000000400', '60000000-0000-7000-8000-000000000114', NULL, '60000000-0000-7000-8000-000000000001');
SELECT pg_temp.bind_model('60000000-0000-7000-8000-000000000405', '60000000-0000-7000-8000-000000000400', '60000000-0000-7000-8000-000000000117', NULL, '60000000-0000-7000-8000-000000000001');
SELECT results_eq($query$SELECT * FROM procedure_deviations('60000000-0000-7000-8000-000000000400')$query$, $expected$VALUES ('volume_l'::text)$expected$, 'pouring lists measured deviation');
SELECT throws_ok($statement$UPDATE procedures SET status_id = 'completed', ended_at = '2026-01-04 01:00:00+00' WHERE id = '60000000-0000-7000-8000-000000000400'; SET CONSTRAINTS ALL IMMEDIATE$statement$, '23514', NULL, 'pouring deviation requires a note');
SELECT lives_ok($statement$UPDATE procedures SET note = 'Recorded experimental deviation', status_id = 'completed', ended_at = '2026-01-04 01:00:00+00' WHERE id = '60000000-0000-7000-8000-000000000400'; SET CONSTRAINTS ALL IMMEDIATE$statement$, 'pouring preserves justified deviation');
SET CONSTRAINTS ALL DEFERRED;
UPDATE procedure_template_slots SET minimum_count = 0 WHERE procedure_template_id = '60000000-0000-7000-8000-000000000120' AND name IN ('wrong_destination', 'wrong_sample');
UPDATE procedure_templates SET state_id = 'published' WHERE id = '60000000-0000-7000-8000-000000000120';
SELECT pg_temp.new_procedure('60000000-0000-7000-8000-000000000420', '60000000-0000-7000-8000-000000000120', 'in_progress', NULL, NULL, NULL, 'fractioning_procedures');
UPDATE procedures SET started_at = '2026-01-04 00:00:00+00' WHERE id = '60000000-0000-7000-8000-000000000420';
INSERT INTO fractioning_procedures (id, organization_id, fractioning_procedure_template_id, status_id, mass_kg) VALUES ('60000000-0000-7000-8000-000000000420', '20000000-0000-7000-8000-000000000001', '60000000-0000-7000-8000-000000000120', 'in_progress', 0.02);
SELECT pg_temp.bind_model('60000000-0000-7000-8000-000000000421', '60000000-0000-7000-8000-000000000420', '60000000-0000-7000-8000-000000000121', NULL, '60000000-0000-7000-8000-000000000003');
SELECT pg_temp.bind_model('60000000-0000-7000-8000-000000000422', '60000000-0000-7000-8000-000000000420', '60000000-0000-7000-8000-000000000122', NULL, '60000000-0000-7000-8000-000000000003');
SELECT pg_temp.bind_model('60000000-0000-7000-8000-000000000423', '60000000-0000-7000-8000-000000000420', '60000000-0000-7000-8000-000000000123', NULL, '60000000-0000-7000-8000-000000000002');
SELECT pg_temp.bind_model('60000000-0000-7000-8000-000000000424', '60000000-0000-7000-8000-000000000420', '60000000-0000-7000-8000-000000000124', NULL, '60000000-0000-7000-8000-000000000001');
SELECT pg_temp.bind_model('60000000-0000-7000-8000-000000000425', '60000000-0000-7000-8000-000000000420', '60000000-0000-7000-8000-000000000127', NULL, '60000000-0000-7000-8000-000000000001');
SELECT results_eq($query$SELECT * FROM procedure_deviations('60000000-0000-7000-8000-000000000420')$query$, $expected$VALUES ('mass_kg'::text)$expected$, 'fractioning lists measured deviation');
SELECT throws_ok($statement$UPDATE procedures SET status_id = 'completed', ended_at = '2026-01-04 01:00:00+00' WHERE id = '60000000-0000-7000-8000-000000000420'; SET CONSTRAINTS ALL IMMEDIATE$statement$, '23514', NULL, 'fractioning deviation requires a note');
SELECT lives_ok($statement$UPDATE procedures SET note = 'Recorded experimental deviation', status_id = 'completed', ended_at = '2026-01-04 01:00:00+00' WHERE id = '60000000-0000-7000-8000-000000000420'; SET CONSTRAINTS ALL IMMEDIATE$statement$, 'fractioning preserves justified deviation');
SET CONSTRAINTS ALL DEFERRED;
SELECT pg_temp.new_model('60000000-0000-7000-8000-000000000500', 'bead model', NULL, 'bead_models');
INSERT INTO physical_asset_models (id, organization_id) VALUES ('60000000-0000-7000-8000-000000000500', '20000000-0000-7000-8000-000000000001');
INSERT INTO bead_models (id, organization_id, diameter_m) VALUES ('60000000-0000-7000-8000-000000000500', '20000000-0000-7000-8000-000000000001', 0.001);
SELECT pg_temp.new_model('60000000-0000-7000-8000-000000000501', 'mill model', NULL, 'ball_mill_machine_models');
INSERT INTO physical_asset_models (id, organization_id) VALUES ('60000000-0000-7000-8000-000000000501', '20000000-0000-7000-8000-000000000001');
INSERT INTO ball_mill_machine_models (id, organization_id) VALUES ('60000000-0000-7000-8000-000000000501', '20000000-0000-7000-8000-000000000001');
SELECT pg_temp.new_template('60000000-0000-7000-8000-000000000510', 'milling treatment', 'ball_mill_procedure_templates');
SELECT pg_temp.new_slot('60000000-0000-7000-8000-000000000511', '60000000-0000-7000-8000-000000000510', 'bead', '60000000-0000-7000-8000-000000000500', 'instrument', 1, 3);
SELECT pg_temp.new_slot('60000000-0000-7000-8000-000000000512', '60000000-0000-7000-8000-000000000510', 'milled_with', '60000000-0000-7000-8000-000000000501', 'instrument', 1, 1);
SELECT pg_temp.new_slot('60000000-0000-7000-8000-000000000513', '60000000-0000-7000-8000-000000000510', 'milled_container', '60000000-0000-7000-8000-000000000003', 'input', 1, 1);
INSERT INTO ball_mill_procedure_templates (id, organization_id, bead_slot_id, bead_model_id, milled_with_slot_id, milled_with_model_id, milled_container_slot_id, milled_container_model_id)
VALUES ('60000000-0000-7000-8000-000000000510', '20000000-0000-7000-8000-000000000001', '60000000-0000-7000-8000-000000000511', '60000000-0000-7000-8000-000000000500', '60000000-0000-7000-8000-000000000512', '60000000-0000-7000-8000-000000000501', '60000000-0000-7000-8000-000000000513', '60000000-0000-7000-8000-000000000003');
UPDATE procedure_templates SET state_id = 'published' WHERE id = '60000000-0000-7000-8000-000000000510';
SELECT pg_temp.new_procedure('60000000-0000-7000-8000-000000000520', '60000000-0000-7000-8000-000000000510', 'in_progress', NULL, NULL, NULL, 'ball_mill_procedures');
UPDATE procedures SET started_at = '2026-01-05 00:00:00+00' WHERE id = '60000000-0000-7000-8000-000000000520';
INSERT INTO ball_mill_procedures (id, organization_id, ball_mill_procedure_template_id, status_id, temperature_k, frequency_hz)
VALUES ('60000000-0000-7000-8000-000000000520', '20000000-0000-7000-8000-000000000001', '60000000-0000-7000-8000-000000000510', 'in_progress', 293.15, 25);
SELECT pg_temp.bind_model('60000000-0000-7000-8000-000000000521', '60000000-0000-7000-8000-000000000520', '60000000-0000-7000-8000-000000000511', NULL, '60000000-0000-7000-8000-000000000500');
SELECT pg_temp.bind_model('60000000-0000-7000-8000-000000000522', '60000000-0000-7000-8000-000000000520', '60000000-0000-7000-8000-000000000512', NULL, '60000000-0000-7000-8000-000000000501');
SELECT pg_temp.bind_model('60000000-0000-7000-8000-000000000523', '60000000-0000-7000-8000-000000000520', '60000000-0000-7000-8000-000000000513', NULL, '60000000-0000-7000-8000-000000000003');
SELECT results_eq($query$SELECT * FROM procedure_deviations('60000000-0000-7000-8000-000000000520')$query$, $expected$VALUES ('bead_count'::text)$expected$, 'milling lists bead count deviation');
SELECT throws_ok($statement$UPDATE procedures SET status_id = 'completed', ended_at = '2026-01-05 00:02:30+00' WHERE id = '60000000-0000-7000-8000-000000000520'; SET CONSTRAINTS ALL IMMEDIATE$statement$, '23514', NULL, 'bead count deviation requires a note');
SELECT lives_ok($statement$UPDATE procedures SET note = 'One bead used', status_id = 'completed', ended_at = '2026-01-05 00:02:30+00' WHERE id = '60000000-0000-7000-8000-000000000520'; SET CONSTRAINTS ALL IMMEDIATE$statement$, 'milling accepts justified bead count');
SET CONSTRAINTS ALL DEFERRED;
SELECT pg_temp.new_template('60000000-0000-7000-8000-000000000600', 'untyped placement');
SELECT pg_temp.new_slot('60000000-0000-7000-8000-000000000601', '60000000-0000-7000-8000-000000000600', 'sample', '60000000-0000-7000-8000-000000000001', 'input', 1, 1);
SELECT pg_temp.new_slot('60000000-0000-7000-8000-000000000602', '60000000-0000-7000-8000-000000000600', 'tube', '60000000-0000-7000-8000-000000000003', 'input', 1, 1);
UPDATE procedure_template_slots SET placed_into_slot_id = '60000000-0000-7000-8000-000000000602' WHERE id = '60000000-0000-7000-8000-000000000601';
UPDATE procedure_templates SET state_id = 'published' WHERE id = '60000000-0000-7000-8000-000000000600';
SELECT pg_temp.new_asset('60000000-0000-7000-8000-000000000603', 'untyped tube', '60000000-0000-7000-8000-000000000003', 'volumetric_containers');
INSERT INTO physical_assets (id, organization_id, physical_asset_model_id) VALUES ('60000000-0000-7000-8000-000000000603', '20000000-0000-7000-8000-000000000001', '60000000-0000-7000-8000-000000000003');
INSERT INTO containers (id, organization_id, container_model_id) VALUES ('60000000-0000-7000-8000-000000000603', '20000000-0000-7000-8000-000000000001', '60000000-0000-7000-8000-000000000003');
INSERT INTO volumetric_containers (id, organization_id, volumetric_container_model_id) VALUES ('60000000-0000-7000-8000-000000000603', '20000000-0000-7000-8000-000000000001', '60000000-0000-7000-8000-000000000003');
SELECT pg_temp.new_procedure('60000000-0000-7000-8000-000000000610', '60000000-0000-7000-8000-000000000600', 'in_progress', NULL, NULL);
UPDATE procedures SET started_at = '2026-01-06 00:00:00+00' WHERE id = '60000000-0000-7000-8000-000000000610';
SELECT pg_temp.bind('60000000-0000-7000-8000-000000000611', '60000000-0000-7000-8000-000000000610', '60000000-0000-7000-8000-000000000601', '60000000-0000-7000-8000-000000000040');
SELECT pg_temp.bind('60000000-0000-7000-8000-000000000612', '60000000-0000-7000-8000-000000000610', '60000000-0000-7000-8000-000000000602', '60000000-0000-7000-8000-000000000603');
UPDATE procedures SET status_id = 'completed', ended_at = '2026-01-06 01:00:00+00' WHERE id = '60000000-0000-7000-8000-000000000610';
SET CONSTRAINTS ALL IMMEDIATE;
SET CONSTRAINTS ALL DEFERRED;
RESET ROLE;
SELECT results_eq($query$SELECT asset_id, container_id FROM asset_moves WHERE kind_id = 'into' AND container_id = '60000000-0000-7000-8000-000000000603'$query$,
  $expected$VALUES ('60000000-0000-7000-8000-000000000040'::uuid, '60000000-0000-7000-8000-000000000603'::uuid)$expected$, 'untyped slot placement produces containment');
SELECT col_not_null('public', 'freezing_procedure_templates', 'duration_s', 'freezing duration is required');
SELECT col_default_is('public', 'freezing_procedure_templates', 'duration_tolerance_percentage', '5.0', 'freezing duration tolerance defaults to five percent');
SELECT col_default_is('public', 'freeze_drying_procedure_templates', 'duration_tolerance_percentage', '5.0', 'freeze drying duration tolerance defaults to five percent');
SELECT col_default_is('public', 'centrifuge_procedure_templates', 'duration_tolerance_percentage', '5.0', 'centrifuge duration tolerance defaults to five percent');
SELECT col_default_is('public', 'ball_mill_procedure_templates', 'duration_tolerance_percentage', '5.0', 'ball mill duration tolerance defaults to five percent');
SELECT is_empty($query$
    SELECT relation.relname FROM pg_class relation
    WHERE relation.relnamespace = 'public'::regnamespace AND relation.relkind = 'r'
      AND (relation.relname LIKE '%_procedure_templates' OR relation.relname LIKE '%_procedures')
      AND NOT EXISTS (SELECT 1 FROM pg_trigger trigger WHERE trigger.tgrelid = relation.oid AND trigger.tgname = relation.relname || '_audit')
$query$, 'typed tables audit inserts and deletes');
SELECT is_empty($query$
    SELECT relation.relname FROM pg_class relation
    WHERE relation.relnamespace = 'public'::regnamespace AND relation.relkind = 'r'
      AND (relation.relname LIKE '%_procedure_templates' OR relation.relname LIKE '%_procedures')
      AND NOT EXISTS (SELECT 1 FROM table_name_ancestors path WHERE path.descendant_table_name_id = relation.relname AND path.ancestor_table_name_id = relation.relname)
$query$, 'typed tables seed reflexive paths');
SELECT ok(EXISTS (SELECT 1 FROM pg_constraint constraint_row
    WHERE constraint_row.conrelid = 'packaging_procedure_templates'::regclass AND constraint_row.confrelid = 'sample_models'::regclass),
    'packaging requires a sample model');
SELECT ok(EXISTS (SELECT 1 FROM pg_constraint constraint_row
    WHERE constraint_row.conrelid = 'photograph_procedure_templates'::regclass AND constraint_row.confrelid = 'photograph_models'::regclass),
    'photography requires a photograph model');
SELECT is_empty($query$
    SELECT relation.relname, attribute.attname
    FROM pg_class relation JOIN pg_attribute attribute ON attribute.attrelid = relation.oid
    WHERE relation.relnamespace = 'public'::regnamespace
      AND relation.relname LIKE '%_procedure_templates' AND attribute.attname LIKE '%_model_id'
      AND NOT EXISTS (SELECT 1 FROM pg_policy policy WHERE policy.polrelid = relation.oid
          AND policy.polname = relation.relname || '_insert'
          AND pg_get_expr(policy.polwithcheck, policy.polrelid) LIKE '%' || attribute.attname || '%')
$query$, 'typed model references require reader access');
SELECT is_empty($query$
    SELECT relation.relname, attribute.attname
    FROM pg_class relation JOIN pg_attribute attribute ON attribute.attrelid = relation.oid
    WHERE relation.relnamespace = 'public'::regnamespace
      AND relation.relname LIKE '%_procedure_templates' AND attribute.attname LIKE '%_model_id'
      AND NOT EXISTS (SELECT 1 FROM pg_policy policy WHERE policy.polrelid = relation.oid
          AND policy.polname = relation.relname || '_archived_insert'
          AND pg_get_expr(policy.polwithcheck, policy.polrelid) LIKE '%is_archived(' || attribute.attname || ')%')
$query$, 'typed model references reject archived models');
SELECT * FROM finish();
ROLLBACK;
