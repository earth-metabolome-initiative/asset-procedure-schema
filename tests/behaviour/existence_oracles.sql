\set ON_ERROR_STOP 1
SET search_path = public, test_tools;
BEGIN;
\ir ../fixture.sql
SELECT plan(12);
RESET ROLE;
-- The SQLSTATE a statement fails with, or `accepted`.
CREATE FUNCTION pg_temp.refusal(statement TEXT) RETURNS TEXT LANGUAGE plpgsql AS $$
BEGIN
  EXECUTE statement;
  RETURN 'accepted';
EXCEPTION WHEN OTHERS THEN
  RETURN SQLSTATE;
END
$$;
GRANT EXECUTE ON FUNCTION pg_temp.refusal(TEXT) TO app;
SET ROLE app;

-- Rows of a second organization the founder of the first cannot read.
SELECT set_config('app.user_id', '10000000-0000-7000-8000-000000000002', false);
INSERT INTO organizations (id, created_by_id) VALUES ('20000000-0000-7000-8000-000000000002', authz.caller_id());
INSERT INTO resources (id, kind, organization_id) VALUES ('20000000-0000-7000-8000-000000000002', 'organization', '20000000-0000-7000-8000-000000000002');
INSERT INTO memberships (organization_id, user_id, state) VALUES ('20000000-0000-7000-8000-000000000002', authz.caller_id(), 'active');
INSERT INTO grants (organization_id, resource_id, grantee_id, grantee_kind, role) VALUES ('20000000-0000-7000-8000-000000000002', '20000000-0000-7000-8000-000000000002', authz.caller_id(), 'user', 'admin');
SET CONSTRAINTS ALL IMMEDIATE;
SET CONSTRAINTS ALL DEFERRED;
INSERT INTO resources (id, kind, organization_id) VALUES ('50000000-0000-7000-8000-000000000002', 'project', '20000000-0000-7000-8000-000000000002');
INSERT INTO resource_parents (organization_id, id, resource_kind, parent_id, parent_kind) VALUES ('20000000-0000-7000-8000-000000000002', '50000000-0000-7000-8000-000000000002', 'project', '20000000-0000-7000-8000-000000000002', 'organization');
SET CONSTRAINTS ALL IMMEDIATE;
SET CONSTRAINTS ALL DEFERRED;
INSERT INTO ownables (id, organization_id, project_id, created_by_id)
SELECT identifier, '20000000-0000-7000-8000-000000000002', '50000000-0000-7000-8000-000000000002', authz.caller_id()
FROM unnest(ARRAY['0e000000-0000-7000-8000-000000000001'::uuid, '0e000000-0000-7000-8000-000000000002'::uuid]) identifier;
INSERT INTO resources (id, kind, organization_id)
SELECT identifier, 'ownable', '20000000-0000-7000-8000-000000000002'::uuid
FROM unnest(ARRAY['0e000000-0000-7000-8000-000000000001'::uuid, '0e000000-0000-7000-8000-000000000002'::uuid]) identifier;
INSERT INTO asset_models (id, organization_id, project_id, table_name_id, name)
VALUES ('0e000000-0000-7000-8000-000000000001', '20000000-0000-7000-8000-000000000002', '50000000-0000-7000-8000-000000000002', 'physical_asset_models', 'hidden model');
INSERT INTO physical_asset_models (id, organization_id) VALUES ('0e000000-0000-7000-8000-000000000001', '20000000-0000-7000-8000-000000000002');
INSERT INTO asset_model_ancestors (descendant_model_id, ancestor_model_id, organization_id, parent_model_id)
VALUES ('0e000000-0000-7000-8000-000000000001', '0e000000-0000-7000-8000-000000000001', '20000000-0000-7000-8000-000000000002', NULL);
INSERT INTO procedure_templates (id, organization_id, project_id, table_name_id, name)
VALUES ('0e000000-0000-7000-8000-000000000002', '20000000-0000-7000-8000-000000000002', '50000000-0000-7000-8000-000000000002', 'procedure_templates', 'hidden template');

-- The founder's own rows, each probe naming one hidden row where it names a visible one here.
SELECT set_config('app.user_id', '10000000-0000-7000-8000-000000000001', false);
SELECT pg_temp.new_model('0e000000-0000-7000-8000-000000000010', 'physical model', NULL, 'physical_asset_models');
INSERT INTO physical_asset_models (id, organization_id) VALUES ('0e000000-0000-7000-8000-000000000010', '20000000-0000-7000-8000-000000000001');
SELECT pg_temp.new_model('0e000000-0000-7000-8000-000000000011', 'digital model', NULL, 'digital_asset_models');
INSERT INTO digital_asset_models (id, organization_id, mime_type) VALUES ('0e000000-0000-7000-8000-000000000011', '20000000-0000-7000-8000-000000000001', 'image/png');
SELECT pg_temp.new_asset('0e000000-0000-7000-8000-000000000020', 'physical asset', '0e000000-0000-7000-8000-000000000010', 'physical_assets');
SELECT pg_temp.new_asset('0e000000-0000-7000-8000-000000000021', 'digital asset', '0e000000-0000-7000-8000-000000000011', 'digital_assets');
SELECT pg_temp.new_template('0e000000-0000-7000-8000-000000000030', 'child template');
SELECT pg_temp.new_slot('0e000000-0000-7000-8000-000000000031', '0e000000-0000-7000-8000-000000000030', 'input', '0e000000-0000-7000-8000-000000000010', 'input', 0, 1);
UPDATE procedure_templates SET state_id = 'published' WHERE id = '0e000000-0000-7000-8000-000000000030';
SELECT pg_temp.new_template('0e000000-0000-7000-8000-000000000032', 'parent template');
SELECT pg_temp.new_slot('0e000000-0000-7000-8000-000000000033', '0e000000-0000-7000-8000-000000000032', 'input', '0e000000-0000-7000-8000-000000000010', 'input', 0, 1);
SELECT pg_temp.new_step('0e000000-0000-7000-8000-000000000034', '0e000000-0000-7000-8000-000000000032', '0e000000-0000-7000-8000-000000000030', 'only', 0);
UPDATE procedure_templates SET state_id = 'published' WHERE id = '0e000000-0000-7000-8000-000000000032';
SELECT pg_temp.new_procedure('0e000000-0000-7000-8000-000000000040', '0e000000-0000-7000-8000-000000000032', 'planned', NULL, NULL);
SELECT pg_temp.create_ownable('0e000000-0000-7000-8000-000000000041');
SELECT pg_temp.create_ownable('0e000000-0000-7000-8000-000000000042');
SELECT pg_temp.create_ownable('0e000000-0000-7000-8000-000000000050');
INSERT INTO asset_models (id, organization_id, project_id, table_name_id, name, parent_model_id)
VALUES ('0e000000-0000-7000-8000-000000000050', '20000000-0000-7000-8000-000000000001', '50000000-0000-7000-8000-000000000001', 'physical_asset_models', 'specialised model', '0e000000-0000-7000-8000-000000000010');

SELECT is(pg_temp.refusal($$INSERT INTO physical_assets (id, organization_id, physical_asset_model_id)
  VALUES ('0e000000-0000-7000-8000-000000000020', '20000000-0000-7000-8000-000000000001', '0e000000-0000-7000-8000-000000000001')$$),
  '42501', 'G15 a physical asset cannot name a physical model its writer cannot read');
SELECT is(pg_temp.refusal($$INSERT INTO digital_assets (id, organization_id, digital_asset_model_id)
  VALUES ('0e000000-0000-7000-8000-000000000021', '20000000-0000-7000-8000-000000000001', '0e000000-0000-7000-8000-000000000001')$$),
  '42501', 'G15 a digital asset cannot name a digital model its writer cannot read');
SELECT is(pg_temp.refusal($$INSERT INTO procedure_parameter_values
  (procedure_id, parameter_id, procedure_template_id, status_id, value_kind_id, organization_id, number_value)
  VALUES ('0e000000-0000-7000-8000-000000000040', '0e000000-0000-7000-8000-000000000099', '0e000000-0000-7000-8000-000000000002',
    'planned', 'number', '20000000-0000-7000-8000-000000000001', 1)$$),
  '42501', 'G15 a parameter value cannot name a template its writer cannot read');
SELECT is(pg_temp.refusal($$INSERT INTO procedure_template_step_parameters
  (step_id, parameter_id, parent_template_id, child_template_id, value_kind_id, organization_id, number_value)
  VALUES ('0e000000-0000-7000-8000-000000000034', '0e000000-0000-7000-8000-000000000099', '0e000000-0000-7000-8000-000000000032',
    '0e000000-0000-7000-8000-000000000002', 'number', '20000000-0000-7000-8000-000000000001', 1)$$),
  '42501', 'G15 a step parameter cannot name a child template its writer cannot read');
SELECT is(pg_temp.refusal($$INSERT INTO procedure_template_slot_bindings
  (step_id, child_slot_id, parent_template_id, child_template_id, parent_slot_id, child_asset_model_id, parent_asset_model_id,
    child_role_id, parent_role_id, organization_id)
  VALUES ('0e000000-0000-7000-8000-000000000034', '0e000000-0000-7000-8000-000000000031', '0e000000-0000-7000-8000-000000000032',
    '0e000000-0000-7000-8000-000000000002', '0e000000-0000-7000-8000-000000000033', '0e000000-0000-7000-8000-000000000010',
    '0e000000-0000-7000-8000-000000000010', 'input', 'input', '20000000-0000-7000-8000-000000000001')$$),
  '42501', 'G15 a slot binding cannot name a child template its writer cannot read');
SELECT is(pg_temp.refusal($$INSERT INTO procedure_template_slot_bindings
  (step_id, child_slot_id, parent_template_id, child_template_id, parent_slot_id, child_asset_model_id, parent_asset_model_id,
    child_role_id, parent_role_id, organization_id)
  VALUES ('0e000000-0000-7000-8000-000000000034', '0e000000-0000-7000-8000-000000000031', '0e000000-0000-7000-8000-000000000032',
    '0e000000-0000-7000-8000-000000000030', '0e000000-0000-7000-8000-000000000033', '0e000000-0000-7000-8000-000000000010',
    '0e000000-0000-7000-8000-000000000001', 'input', 'input', '20000000-0000-7000-8000-000000000001')$$),
  '42501', 'G15 a slot binding cannot name a parent slot model its writer cannot read');
SELECT is(pg_temp.refusal($$INSERT INTO procedure_assets (id, organization_id, procedure_id, procedure_template_id, status_id, slot_id,
    slot_asset_model_id, slot_role_id, asset_model_id)
  VALUES ('0e000000-0000-7000-8000-000000000060', '20000000-0000-7000-8000-000000000001', '0e000000-0000-7000-8000-000000000040',
    '0e000000-0000-7000-8000-000000000002', 'planned', '0e000000-0000-7000-8000-000000000033', '0e000000-0000-7000-8000-000000000010',
    'input', '0e000000-0000-7000-8000-000000000010')$$),
  '42501', 'G15 a procedure asset cannot name a template its writer cannot read');
SELECT is(pg_temp.refusal($$INSERT INTO procedure_assets (id, organization_id, procedure_id, procedure_template_id, status_id, slot_id,
    slot_asset_model_id, slot_role_id, asset_model_id, parent_procedure_asset_id, parent_procedure_id)
  VALUES ('0e000000-0000-7000-8000-000000000061', '20000000-0000-7000-8000-000000000001', '0e000000-0000-7000-8000-000000000040',
    '0e000000-0000-7000-8000-000000000032', 'planned', '0e000000-0000-7000-8000-000000000033', '0e000000-0000-7000-8000-000000000010',
    'input', '0e000000-0000-7000-8000-000000000010', '0e000000-0000-7000-8000-000000000098', '0e000000-0000-7000-8000-000000000097')$$),
  '42501', 'G15 a procedure asset cannot name a parent procedure asset its writer cannot read');
SELECT is(pg_temp.refusal($$INSERT INTO procedures (id, organization_id, project_id, table_name_id, procedure_template_id, status_id,
    parent_procedure_id, parent_template_id, step_id, root_procedure_id)
  VALUES ('0e000000-0000-7000-8000-000000000041', '20000000-0000-7000-8000-000000000001', '50000000-0000-7000-8000-000000000001',
    'procedures', '0e000000-0000-7000-8000-000000000030', 'planned', '0e000000-0000-7000-8000-000000000040',
    '0e000000-0000-7000-8000-000000000002', '0e000000-0000-7000-8000-000000000034', '0e000000-0000-7000-8000-000000000040')$$),
  '42501', 'G15 a step run cannot name a parent template its writer cannot read');
SELECT is(pg_temp.refusal($$INSERT INTO procedures (id, organization_id, project_id, table_name_id, procedure_template_id, status_id,
    root_procedure_id, started_at, performed_by_id)
  VALUES ('0e000000-0000-7000-8000-000000000042', '20000000-0000-7000-8000-000000000001', '50000000-0000-7000-8000-000000000001',
    'procedures', '0e000000-0000-7000-8000-000000000032', 'in_progress', '0e000000-0000-7000-8000-000000000042', now(),
    '10000000-0000-7000-8000-000000000002')$$),
  '42501', 'G15 a procedure cannot name a performer its writer cannot read');
SELECT is(pg_temp.refusal($$INSERT INTO asset_model_ancestors (descendant_model_id, ancestor_model_id, organization_id, parent_model_id)
  VALUES ('0e000000-0000-7000-8000-000000000050', '0e000000-0000-7000-8000-000000000001', '20000000-0000-7000-8000-000000000001',
    '0e000000-0000-7000-8000-000000000001')$$),
  '42501', 'G15 an ancestor row cannot name a parent model its writer cannot read');
SELECT lives_ok($$INSERT INTO procedures (id, organization_id, project_id, table_name_id, procedure_template_id, status_id,
    root_procedure_id, started_at, performed_by_id)
  VALUES ('0e000000-0000-7000-8000-000000000042', '20000000-0000-7000-8000-000000000001', '50000000-0000-7000-8000-000000000001',
    'procedures', '0e000000-0000-7000-8000-000000000032', 'in_progress', '0e000000-0000-7000-8000-000000000042', now(),
    authz.caller_id())$$, 'G15 a procedure still names its writer as performer');
SELECT * FROM finish();
ROLLBACK;
