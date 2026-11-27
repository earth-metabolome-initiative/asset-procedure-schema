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
  SELECT id::uuid, '20000000-0000-7000-8000-000000000001', '50000000-0000-7000-8000-000000000001', '10000000-0000-7000-8000-000000000001', 'ownable', 'project'
  FROM (VALUES ('a0000000-0000-7000-8000-000000000001'), ('c0000000-0000-7000-8000-000000000001')) ownable (id);
  INSERT INTO asset_models (id, organization_id, project_id, table_name_id, name)
  VALUES ('a0000000-0000-7000-8000-000000000001', '20000000-0000-7000-8000-000000000001', '50000000-0000-7000-8000-000000000001', 'asset_models', 'sample');
  INSERT INTO asset_model_ancestors (descendant_model_id, ancestor_model_id, organization_id, parent_model_id)
  VALUES ('a0000000-0000-7000-8000-000000000001', 'a0000000-0000-7000-8000-000000000001', '20000000-0000-7000-8000-000000000001', NULL);
  INSERT INTO procedure_templates (id, organization_id, project_id, table_name_id, name)
  VALUES ('c0000000-0000-7000-8000-000000000001', '20000000-0000-7000-8000-000000000001', '50000000-0000-7000-8000-000000000001', 'procedure_templates', 'weigh');
  INSERT INTO procedure_template_slots (id, organization_id, procedure_template_id, name, asset_model_id, role_id, minimum_count, maximum_count)
  VALUES ('d0000000-0000-7000-8000-000000000001', '20000000-0000-7000-8000-000000000001', 'c0000000-0000-7000-8000-000000000001', 'weighed', 'a0000000-0000-7000-8000-000000000001', 'input', 1, 1);
  COMMIT;
}

session first
setup { SET statement_timeout = '20s'; SET app.user_id = '10000000-0000-7000-8000-000000000001'; SET ROLE app; }
step first_begin { BEGIN ISOLATION LEVEL READ COMMITTED; }
step first_write { INSERT INTO procedure_template_slots (id, organization_id, procedure_template_id, name, asset_model_id, role_id, minimum_count, maximum_count) VALUES ('d0000000-0000-7000-8000-000000000002', '20000000-0000-7000-8000-000000000001', 'c0000000-0000-7000-8000-000000000001', 'intermediate sample', 'a0000000-0000-7000-8000-000000000001', 'intermediate', 1, 1); SET CONSTRAINTS ALL IMMEDIATE; }
step first_finish { COMMIT; }

session second
setup { SET statement_timeout = '20s'; SET app.user_id = '10000000-0000-7000-8000-000000000001'; SET ROLE app; }
step second_begin { BEGIN ISOLATION LEVEL READ COMMITTED; }
step second_write { UPDATE procedure_templates SET state_id = 'published' WHERE id = 'c0000000-0000-7000-8000-000000000001'; SET CONSTRAINTS ALL IMMEDIATE; }
step second_finish { COMMIT; }

session observer
setup { SET statement_timeout = '20s'; SET app.user_id = '10000000-0000-7000-8000-000000000001'; SET ROLE app; }
step invariant { SELECT template.state_id, count(slot.id) AS slots FROM procedure_templates template JOIN procedure_template_slots slot ON slot.procedure_template_id = template.id WHERE template.id = 'c0000000-0000-7000-8000-000000000001' GROUP BY template.state_id; }

permutation second_begin second_write first_begin first_write second_finish first_finish invariant
