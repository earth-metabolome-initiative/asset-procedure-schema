\set ON_ERROR_STOP 1
SET search_path = public, test_tools;
BEGIN;
\ir ../fixture.sql
SELECT no_plan();

INSERT INTO resources (organization_id, id, kind, parent_id, parent_kind)
VALUES ('20000000-0000-7000-8000-000000000001', '50000000-0000-7000-8000-000000000004', 'project', '20000000-0000-7000-8000-000000000001', 'organization');
SELECT pg_temp.new_model('ac000000-0000-7000-8000-000000000001', 'item', NULL, 'physical_asset_models');
SELECT pg_temp.new_model('ac000000-0000-7000-8000-000000000002', 'box', NULL, 'container_models');
INSERT INTO physical_asset_models (id, organization_id)
VALUES ('ac000000-0000-7000-8000-000000000001', '20000000-0000-7000-8000-000000000001'),
  ('ac000000-0000-7000-8000-000000000002', '20000000-0000-7000-8000-000000000001');
INSERT INTO container_models (id, organization_id)
VALUES ('ac000000-0000-7000-8000-000000000002', '20000000-0000-7000-8000-000000000001');
SELECT pg_temp.create_ownable('8c000000-0000-7000-8000-000000000001');
INSERT INTO container_compatibility_rules (id, organization_id, project_id, table_name_id, container_model_id, contained_asset_model_id, quantity)
VALUES ('8c000000-0000-7000-8000-000000000001', '20000000-0000-7000-8000-000000000001', '50000000-0000-7000-8000-000000000001', 'container_compatibility_rules', 'ac000000-0000-7000-8000-000000000002', 'ac000000-0000-7000-8000-000000000001', 1);
SELECT pg_temp.new_template('cc000000-0000-7000-8000-000000000001', 'placement');
SELECT pg_temp.new_slot('dc000000-0000-7000-8000-000000000001', 'cc000000-0000-7000-8000-000000000001', 'item', 'ac000000-0000-7000-8000-000000000001', 'input', 1, 1);
SELECT pg_temp.new_slot('dc000000-0000-7000-8000-000000000002', 'cc000000-0000-7000-8000-000000000001', 'box', 'ac000000-0000-7000-8000-000000000002', 'input', 1, 1);
UPDATE procedure_template_slots SET placed_into_slot_id = 'dc000000-0000-7000-8000-000000000002' WHERE id = 'dc000000-0000-7000-8000-000000000001';
UPDATE procedure_templates SET state_id = 'published' WHERE id = 'cc000000-0000-7000-8000-000000000001';
SELECT pg_temp.new_asset('bc000000-0000-7000-8000-000000000001', 'item', 'ac000000-0000-7000-8000-000000000001', 'physical_assets');
SELECT pg_temp.new_asset('bc000000-0000-7000-8000-000000000002', 'box', 'ac000000-0000-7000-8000-000000000002', 'containers');
INSERT INTO physical_assets (id, organization_id, physical_asset_model_id)
VALUES ('bc000000-0000-7000-8000-000000000001', '20000000-0000-7000-8000-000000000001', 'ac000000-0000-7000-8000-000000000001'),
  ('bc000000-0000-7000-8000-000000000002', '20000000-0000-7000-8000-000000000001', 'ac000000-0000-7000-8000-000000000002');
INSERT INTO containers (id, organization_id, container_model_id)
VALUES ('bc000000-0000-7000-8000-000000000002', '20000000-0000-7000-8000-000000000001', 'ac000000-0000-7000-8000-000000000002');
SELECT pg_temp.new_procedure('fc000000-0000-7000-8000-000000000001', 'cc000000-0000-7000-8000-000000000001', 'in_progress', NULL, NULL);
UPDATE procedures SET started_at = now() - interval '1 hour' WHERE id = 'fc000000-0000-7000-8000-000000000001';
SELECT pg_temp.bind('9c000000-0000-7000-8000-000000000001', 'fc000000-0000-7000-8000-000000000001', 'dc000000-0000-7000-8000-000000000001', 'bc000000-0000-7000-8000-000000000001');
SELECT pg_temp.bind('9c000000-0000-7000-8000-000000000002', 'fc000000-0000-7000-8000-000000000001', 'dc000000-0000-7000-8000-000000000002', 'bc000000-0000-7000-8000-000000000002');
UPDATE procedures SET status_id = 'completed', ended_at = now() WHERE id = 'fc000000-0000-7000-8000-000000000001';
SET CONSTRAINTS ALL IMMEDIATE;
SET CONSTRAINTS ALL DEFERRED;
SELECT ok((SELECT in_use FROM container_compatibility_rules WHERE id = '8c000000-0000-7000-8000-000000000001'), 'placement freezes the compatibility rule');

SELECT throws_ok($$UPDATE procedures SET organization_id = organization_id WHERE id = 'fc000000-0000-7000-8000-000000000001'$$, '42501', NULL, 'app cannot update organization metadata');
SELECT throws_ok($$UPDATE procedures SET project_id = project_id WHERE id = 'fc000000-0000-7000-8000-000000000001'$$, '42501', NULL, 'app cannot update project metadata');
SELECT throws_ok($$UPDATE procedure_templates SET archived = TRUE WHERE id = 'cc000000-0000-7000-8000-000000000001'$$, '42501', NULL, 'app cannot update archive metadata');
SELECT throws_ok($$UPDATE container_compatibility_rules SET contained_asset_model_archived = TRUE WHERE id = '8c000000-0000-7000-8000-000000000001'$$, '42501', NULL, 'app cannot update reference archive metadata');
SELECT throws_ok($$UPDATE procedure_templates SET kind = DEFAULT WHERE id = 'cc000000-0000-7000-8000-000000000001'$$, '42501', NULL, 'app cannot update resource kind metadata');

SAVEPOINT archive_project;
SELECT lives_ok($$UPDATE resources SET archived_at = now() WHERE id = '50000000-0000-7000-8000-000000000001'; SET CONSTRAINTS ALL IMMEDIATE$$, 'project archival cascades through frozen business rows');
SELECT is((SELECT archived FROM procedures WHERE id = 'fc000000-0000-7000-8000-000000000001'), TRUE, 'completed procedure inherits project archival');
SELECT is((SELECT archived FROM procedure_templates WHERE id = 'cc000000-0000-7000-8000-000000000001'), TRUE, 'published template inherits project archival');
SELECT results_eq($$SELECT archived FROM procedure_template_slots WHERE procedure_template_id = 'cc000000-0000-7000-8000-000000000001' ORDER BY id$$, $$VALUES (TRUE), (TRUE)$$, 'published slots inherit project archival');
SELECT is((SELECT archived FROM container_compatibility_rules WHERE id = '8c000000-0000-7000-8000-000000000001'), TRUE, 'used compatibility rule inherits project archival');
SELECT is_empty($$UPDATE procedures SET started_at = started_at - interval '1 hour' WHERE id = 'fc000000-0000-7000-8000-000000000001' RETURNING id$$, 'archived completed procedure refuses business updates');
SELECT is_empty($$UPDATE procedure_templates SET name = 'changed' WHERE id = 'cc000000-0000-7000-8000-000000000001' RETURNING id$$, 'archived published template refuses business updates');
SELECT is_empty($$UPDATE procedure_template_slots SET name = 'changed' WHERE id = 'dc000000-0000-7000-8000-000000000001' RETURNING id$$, 'archived published slot refuses business updates');
SELECT is_empty($$UPDATE container_compatibility_rules SET quantity = 2 WHERE id = '8c000000-0000-7000-8000-000000000001' RETURNING id$$, 'archived used rule refuses business updates');
ROLLBACK TO archive_project;

SELECT lives_ok($$UPDATE resource_moves SET parent_id = '50000000-0000-7000-8000-000000000004'
WHERE id IN ('ac000000-0000-7000-8000-000000000002', '8c000000-0000-7000-8000-000000000001', 'cc000000-0000-7000-8000-000000000001', 'fc000000-0000-7000-8000-000000000001');
SET CONSTRAINTS ALL IMMEDIATE$$, 'moving frozen resources cascades their project metadata');
SET CONSTRAINTS ALL DEFERRED;
SELECT results_eq($$SELECT project_id FROM procedures WHERE id = 'fc000000-0000-7000-8000-000000000001'
UNION ALL SELECT project_id FROM procedure_templates WHERE id = 'cc000000-0000-7000-8000-000000000001'
UNION ALL SELECT project_id FROM container_compatibility_rules WHERE id = '8c000000-0000-7000-8000-000000000001'$$,
$$VALUES ('50000000-0000-7000-8000-000000000004'::uuid), ('50000000-0000-7000-8000-000000000004'::uuid), ('50000000-0000-7000-8000-000000000004'::uuid)$$, 'frozen rows retain the destination project');
SELECT throws_ok($$UPDATE procedures SET started_at = started_at - interval '1 hour' WHERE id = 'fc000000-0000-7000-8000-000000000001'$$, '23514', NULL, 'moved completed procedure keeps its business fields frozen');
SELECT throws_ok($$UPDATE procedure_templates SET name = 'changed' WHERE id = 'cc000000-0000-7000-8000-000000000001'$$, '23514', NULL, 'moved published template stays frozen');
SELECT throws_ok($$UPDATE procedure_template_slots SET name = 'changed' WHERE id = 'dc000000-0000-7000-8000-000000000001'$$, '23514', NULL, 'moved published template keeps its slots frozen');
SELECT throws_ok($$UPDATE container_compatibility_rules SET quantity = 2 WHERE id = '8c000000-0000-7000-8000-000000000001'$$, '23514', NULL, 'moved used rule stays frozen');
SELECT lives_ok($$UPDATE procedures SET note = 'completion note' WHERE id = 'fc000000-0000-7000-8000-000000000001'$$, 'moved completed procedure permits its note update');
SELECT is((SELECT note FROM procedures WHERE id = 'fc000000-0000-7000-8000-000000000001'), 'completion note', 'completed procedure stores its note after moving');
SELECT * FROM finish();
ROLLBACK;
