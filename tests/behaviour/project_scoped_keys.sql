\set ON_ERROR_STOP 1
SET search_path = public, test_tools;
BEGIN;
\ir ../fixture.sql
SELECT plan(6);

-- A second project of the organization.
INSERT INTO resources (organization_id, id, kind, parent_id, parent_kind)
  VALUES ('20000000-0000-7000-8000-000000000001', '50000000-0000-7000-8000-000000000003', 'project',
    '20000000-0000-7000-8000-000000000001', 'organization');
SET CONSTRAINTS ALL IMMEDIATE;
SET CONSTRAINTS ALL DEFERRED;

-- A commercial product with one lot moved to the second project.
SELECT pg_temp.create_ownable('5c000000-0000-7000-8000-000000000001');
INSERT INTO brands (id, organization_id, project_id, table_name_id, name)
  VALUES ('5c000000-0000-7000-8000-000000000001', '20000000-0000-7000-8000-000000000001', '50000000-0000-7000-8000-000000000001', 'brands', 'brand');
SELECT pg_temp.new_model('5c000000-0000-7000-8000-000000000002', 'product', NULL, 'commercial_products');
INSERT INTO commercial_products (id, organization_id, brand_id)
  VALUES ('5c000000-0000-7000-8000-000000000002', '20000000-0000-7000-8000-000000000001', '5c000000-0000-7000-8000-000000000001');
SELECT pg_temp.new_model('5c000000-0000-7000-8000-000000000003', 'lot in the second project', '5c000000-0000-7000-8000-000000000002', 'commercial_product_lots');
INSERT INTO physical_asset_models (id, organization_id) VALUES ('5c000000-0000-7000-8000-000000000003', '20000000-0000-7000-8000-000000000001');
INSERT INTO commercial_product_lots (id, organization_id, project_id, lot, product_model_id)
  VALUES ('5c000000-0000-7000-8000-000000000003', '20000000-0000-7000-8000-000000000001', '50000000-0000-7000-8000-000000000001', 'L1', '5c000000-0000-7000-8000-000000000002');
UPDATE resource_moves SET parent_id = '50000000-0000-7000-8000-000000000003' WHERE id = '5c000000-0000-7000-8000-000000000003';
SET CONSTRAINTS ALL IMMEDIATE;
SET CONSTRAINTS ALL DEFERRED;
SELECT is((SELECT project_id FROM commercial_product_lots WHERE id = '5c000000-0000-7000-8000-000000000003'),
  '50000000-0000-7000-8000-000000000003'::uuid, 'moving a resource cascades its project through APS rows');

SELECT lives_ok($statement$SELECT pg_temp.new_model('5c000000-0000-7000-8000-000000000004', 'lot in the first project', '5c000000-0000-7000-8000-000000000002', 'commercial_product_lots');
    INSERT INTO physical_asset_models (id, organization_id) VALUES ('5c000000-0000-7000-8000-000000000004', '20000000-0000-7000-8000-000000000001');
    INSERT INTO commercial_product_lots (id, organization_id, project_id, lot, product_model_id)
      VALUES ('5c000000-0000-7000-8000-000000000004', '20000000-0000-7000-8000-000000000001', '50000000-0000-7000-8000-000000000001', 'L1', '5c000000-0000-7000-8000-000000000002');
    SET CONSTRAINTS ALL IMMEDIATE$statement$,
  'two projects each register the same lot of one product');
SET CONSTRAINTS ALL DEFERRED;
SELECT throws_ok($statement$SELECT pg_temp.new_model('5c000000-0000-7000-8000-000000000005', 'repeated lot', '5c000000-0000-7000-8000-000000000002', 'commercial_product_lots');
    INSERT INTO physical_asset_models (id, organization_id) VALUES ('5c000000-0000-7000-8000-000000000005', '20000000-0000-7000-8000-000000000001');
    INSERT INTO commercial_product_lots (id, organization_id, project_id, lot, product_model_id)
      VALUES ('5c000000-0000-7000-8000-000000000005', '20000000-0000-7000-8000-000000000001', '50000000-0000-7000-8000-000000000001', 'L1', '5c000000-0000-7000-8000-000000000002')$statement$,
  '23505', NULL, 'one project registers a lot of a product once');
SET CONSTRAINTS ALL DEFERRED;

-- A container model and a model it may hold, both in the first project.
SELECT pg_temp.new_model('5c000000-0000-7000-8000-000000000010', 'rack', NULL, 'container_models');
INSERT INTO physical_asset_models (id, organization_id) VALUES ('5c000000-0000-7000-8000-000000000010', '20000000-0000-7000-8000-000000000001');
INSERT INTO container_models (id, organization_id) VALUES ('5c000000-0000-7000-8000-000000000010', '20000000-0000-7000-8000-000000000001');
SELECT pg_temp.new_model('5c000000-0000-7000-8000-000000000011', 'vial', NULL, 'physical_asset_models');
INSERT INTO physical_asset_models (id, organization_id) VALUES ('5c000000-0000-7000-8000-000000000011', '20000000-0000-7000-8000-000000000001');
SET CONSTRAINTS ALL IMMEDIATE;
SET CONSTRAINTS ALL DEFERRED;

SELECT throws_ok($statement$SELECT pg_temp.create_ownable('5c000000-0000-7000-8000-000000000012');
    UPDATE resource_moves SET parent_id = '50000000-0000-7000-8000-000000000003' WHERE id = '5c000000-0000-7000-8000-000000000012';
    INSERT INTO container_compatibility_rules (id, organization_id, project_id, table_name_id, container_model_id, contained_asset_model_id, quantity)
      VALUES ('5c000000-0000-7000-8000-000000000012', '20000000-0000-7000-8000-000000000001', '50000000-0000-7000-8000-000000000003',
        'container_compatibility_rules', '5c000000-0000-7000-8000-000000000010', '5c000000-0000-7000-8000-000000000011', 1);
    SET CONSTRAINTS ALL IMMEDIATE$statement$,
  '23503', NULL, 'a compatibility rule sits in its container model''s project');
SET CONSTRAINTS ALL DEFERRED;
SELECT lives_ok($statement$SELECT pg_temp.create_ownable('5c000000-0000-7000-8000-000000000013');
    INSERT INTO container_compatibility_rules (id, organization_id, project_id, table_name_id, container_model_id, contained_asset_model_id, quantity)
      VALUES ('5c000000-0000-7000-8000-000000000013', '20000000-0000-7000-8000-000000000001', '50000000-0000-7000-8000-000000000001',
        'container_compatibility_rules', '5c000000-0000-7000-8000-000000000010', '5c000000-0000-7000-8000-000000000011', 1);
    SET CONSTRAINTS ALL IMMEDIATE$statement$,
  'a compatibility rule is written in its container model''s project');
SET CONSTRAINTS ALL DEFERRED;

-- A template running one child template as its only step.
SELECT pg_temp.new_template('5c000000-0000-7000-8000-000000000020', 'child');
SELECT pg_temp.new_slot('5c000000-0000-7000-8000-000000000025', '5c000000-0000-7000-8000-000000000020', 'vial', '5c000000-0000-7000-8000-000000000011', 'input', 0, 1);
UPDATE procedure_templates SET state_id = 'published' WHERE id = '5c000000-0000-7000-8000-000000000020';
SELECT pg_temp.new_template('5c000000-0000-7000-8000-000000000021', 'parent');
SELECT pg_temp.new_step('5c000000-0000-7000-8000-000000000022', '5c000000-0000-7000-8000-000000000021', '5c000000-0000-7000-8000-000000000020', 'only', 0);
UPDATE procedure_templates SET state_id = 'published' WHERE id = '5c000000-0000-7000-8000-000000000021';
SELECT pg_temp.new_procedure('5c000000-0000-7000-8000-000000000023', '5c000000-0000-7000-8000-000000000021', 'in_progress', NULL, NULL);
SET CONSTRAINTS ALL IMMEDIATE;
SET CONSTRAINTS ALL DEFERRED;

SELECT throws_ok($statement$SELECT pg_temp.create_ownable('5c000000-0000-7000-8000-000000000024');
    UPDATE resource_moves SET parent_id = '50000000-0000-7000-8000-000000000003' WHERE id = '5c000000-0000-7000-8000-000000000024';
    INSERT INTO procedures (id, organization_id, project_id, table_name_id, procedure_template_id, status_id,
        parent_procedure_id, parent_template_id, step_id, root_procedure_id, started_at, performed_by_id)
      VALUES ('5c000000-0000-7000-8000-000000000024', '20000000-0000-7000-8000-000000000001', '50000000-0000-7000-8000-000000000003',
        'procedures', '5c000000-0000-7000-8000-000000000020', 'in_progress', '5c000000-0000-7000-8000-000000000023',
        '5c000000-0000-7000-8000-000000000021', '5c000000-0000-7000-8000-000000000022', '5c000000-0000-7000-8000-000000000023',
        now(), authz.caller_id());
    SET CONSTRAINTS ALL IMMEDIATE$statement$,
  '23503', NULL, 'a child procedure sits in its parent''s project');

SELECT * FROM finish();
ROLLBACK;
