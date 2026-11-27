setup
{
  INSERT INTO users (id) VALUES ('10000000-0000-7000-8000-000000000001');
  SET app.user_id = '10000000-0000-7000-8000-000000000001';
  BEGIN;
  INSERT INTO organizations (id, created_by_id) VALUES ('20000000-0000-7000-8000-000000000001', '10000000-0000-7000-8000-000000000001');
  INSERT INTO resources (id, kind, organization_id) VALUES ('20000000-0000-7000-8000-000000000001', 'organization', '20000000-0000-7000-8000-000000000001');
  INSERT INTO memberships (organization_id, user_id, state) VALUES ('20000000-0000-7000-8000-000000000001', '10000000-0000-7000-8000-000000000001', 'active');
  COMMIT;
  BEGIN;
  INSERT INTO resources (organization_id, id, kind, parent_id, parent_kind)
  VALUES ('20000000-0000-7000-8000-000000000001', '50000000-0000-7000-8000-000000000001', 'project', '20000000-0000-7000-8000-000000000001', 'organization');
  COMMIT;
  BEGIN;
  INSERT INTO resources (id, organization_id, parent_id, created_by_id, kind, parent_kind)
  SELECT id::uuid, '20000000-0000-7000-8000-000000000001', '50000000-0000-7000-8000-000000000001', '10000000-0000-7000-8000-000000000001', 'ownable', 'project' FROM (VALUES ('a0000000-0000-7000-8000-000000000001'), ('c0000000-0000-7000-8000-000000000001'), ('c0000000-0000-7000-8000-000000000002'), ('f0000000-0000-7000-8000-000000000001'), ('f0000000-0000-7000-8000-000000000002'), ('b0000000-0000-7000-8000-000000000001')) ownable (id);
  INSERT INTO asset_models (id, organization_id, project_id, table_name_id, name)
  VALUES ('a0000000-0000-7000-8000-000000000001', '20000000-0000-7000-8000-000000000001', '50000000-0000-7000-8000-000000000001', 'asset_models', 'sample');
  INSERT INTO asset_model_ancestors (descendant_model_id, ancestor_model_id, organization_id, parent_model_id)
  VALUES ('a0000000-0000-7000-8000-000000000001', 'a0000000-0000-7000-8000-000000000001', '20000000-0000-7000-8000-000000000001', NULL);
  INSERT INTO assets (id, organization_id, project_id, table_name_id, model_id, name)
  VALUES ('b0000000-0000-7000-8000-000000000001', '20000000-0000-7000-8000-000000000001', '50000000-0000-7000-8000-000000000001', 'assets', 'a0000000-0000-7000-8000-000000000001', 'sample one');
  INSERT INTO procedure_templates (id, organization_id, project_id, table_name_id, name) VALUES ('c0000000-0000-7000-8000-000000000001', '20000000-0000-7000-8000-000000000001', '50000000-0000-7000-8000-000000000001', 'procedure_templates', 'parent');
  INSERT INTO procedure_templates (id, organization_id, project_id, table_name_id, name) VALUES ('c0000000-0000-7000-8000-000000000002', '20000000-0000-7000-8000-000000000001', '50000000-0000-7000-8000-000000000001', 'procedure_templates', 'child');
  INSERT INTO procedure_template_slots (id, organization_id, procedure_template_id, name, asset_model_id, role_id, minimum_count, maximum_count) VALUES ('d0000000-0000-7000-8000-000000000002', '20000000-0000-7000-8000-000000000001', 'c0000000-0000-7000-8000-000000000002', 'sample', 'a0000000-0000-7000-8000-000000000001', 'input', 0, NULL);
  INSERT INTO procedure_template_slots (id, organization_id, procedure_template_id, name, asset_model_id, role_id, minimum_count, maximum_count) VALUES ('d0000000-0000-7000-8000-000000000001', '20000000-0000-7000-8000-000000000001', 'c0000000-0000-7000-8000-000000000001', 'sample', 'a0000000-0000-7000-8000-000000000001', 'input', 0, NULL);
  UPDATE procedure_templates SET state_id = 'published' WHERE id = 'c0000000-0000-7000-8000-000000000002';
  INSERT INTO procedure_template_steps (id, organization_id, parent_template_id, child_template_id, name, position, repeat_slot_id, optional) VALUES ('90000000-0000-7000-8000-000000000001', '20000000-0000-7000-8000-000000000001', 'c0000000-0000-7000-8000-000000000001', 'c0000000-0000-7000-8000-000000000002', 'first step', 1, 'd0000000-0000-7000-8000-000000000001', FALSE);
  INSERT INTO procedure_template_slot_bindings (step_id, child_slot_id, parent_template_id, child_template_id, parent_slot_id, child_asset_model_id, parent_asset_model_id, child_role_id, parent_role_id, organization_id)
  VALUES ('90000000-0000-7000-8000-000000000001', 'd0000000-0000-7000-8000-000000000002', 'c0000000-0000-7000-8000-000000000001', 'c0000000-0000-7000-8000-000000000002', 'd0000000-0000-7000-8000-000000000001', 'a0000000-0000-7000-8000-000000000001', 'a0000000-0000-7000-8000-000000000001', 'input', 'input', '20000000-0000-7000-8000-000000000001');
  UPDATE procedure_templates SET state_id = 'published' WHERE id = 'c0000000-0000-7000-8000-000000000001';
  INSERT INTO procedures (id, organization_id, project_id, table_name_id, procedure_template_id, status_id, root_procedure_id, parent_procedure_id, parent_template_id, step_id, repeat_procedure_asset_id, started_at, performed_by_id) VALUES ('f0000000-0000-7000-8000-000000000001', '20000000-0000-7000-8000-000000000001', '50000000-0000-7000-8000-000000000001', 'procedures', 'c0000000-0000-7000-8000-000000000001', 'in_progress', 'f0000000-0000-7000-8000-000000000001', NULL, NULL, NULL, NULL, '2026-01-01 10:00+00', '10000000-0000-7000-8000-000000000001');
  INSERT INTO procedure_assets (id, organization_id, procedure_id, procedure_template_id, slot_id, slot_asset_model_id, slot_role_id, asset_model_id, asset_id, parent_procedure_asset_id) VALUES ('e0000000-0000-7000-8000-000000000001', '20000000-0000-7000-8000-000000000001', 'f0000000-0000-7000-8000-000000000001', 'c0000000-0000-7000-8000-000000000001', 'd0000000-0000-7000-8000-000000000001', 'a0000000-0000-7000-8000-000000000001', 'input', 'a0000000-0000-7000-8000-000000000001', 'b0000000-0000-7000-8000-000000000001', NULL);
  COMMIT;
}

session first
setup { SET statement_timeout = '20s'; SET app.user_id = '10000000-0000-7000-8000-000000000001'; SET ROLE app; }
step first_begin { BEGIN ISOLATION LEVEL READ COMMITTED; }
step first_write { DELETE FROM procedure_assets WHERE id = 'e0000000-0000-7000-8000-000000000001'; SET CONSTRAINTS ALL IMMEDIATE; }
step first_finish { COMMIT; }

session second
setup { SET statement_timeout = '20s'; SET app.user_id = '10000000-0000-7000-8000-000000000001'; SET ROLE app; }
step second_begin { BEGIN ISOLATION LEVEL READ COMMITTED; }
step second_write { INSERT INTO procedures (id, organization_id, project_id, table_name_id, procedure_template_id, status_id, root_procedure_id, parent_procedure_id, parent_template_id, step_id, repeat_procedure_asset_id, started_at, performed_by_id) VALUES ('f0000000-0000-7000-8000-000000000002', '20000000-0000-7000-8000-000000000001', '50000000-0000-7000-8000-000000000001', 'procedures', 'c0000000-0000-7000-8000-000000000002', 'in_progress', 'f0000000-0000-7000-8000-000000000001', 'f0000000-0000-7000-8000-000000000001', 'c0000000-0000-7000-8000-000000000001', '90000000-0000-7000-8000-000000000001', 'e0000000-0000-7000-8000-000000000001', '2026-01-01 10:00+00', '10000000-0000-7000-8000-000000000001'); SET CONSTRAINTS ALL IMMEDIATE; }
step second_finish { COMMIT; }

session observer
setup { SET statement_timeout = '20s'; SET app.user_id = '10000000-0000-7000-8000-000000000001'; SET ROLE app; }
step invariant { SELECT run.status_id, count(binding.id) AS repeat_bindings FROM procedures run LEFT JOIN procedure_assets binding ON binding.id = run.repeat_procedure_asset_id WHERE run.id = 'f0000000-0000-7000-8000-000000000002' GROUP BY run.status_id; }

permutation first_begin first_write second_begin second_write first_finish second_finish invariant
