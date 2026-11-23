\set ON_ERROR_STOP 1
SET search_path = public, test_tools;
BEGIN;
\ir ../fixture.sql
SELECT plan(19);
SELECT lives_ok($$SELECT 'SRID=4326;POINT(11 46)'::geography(Point, 4326)$$, 'app casts a geographic point literal');
SELECT ok(NOT has_function_privilege('app', 'st_astext(geography)', 'EXECUTE'), 'unneeded PostGIS functions remain private');
SELECT pg_temp.new_model('a0000000-0000-7000-8000-000000000001', 'Disposable model', NULL);
SET CONSTRAINTS ALL IMMEDIATE;
SET CONSTRAINTS ALL DEFERRED;
SELECT throws_ok($$DELETE FROM asset_model_ancestors WHERE descendant_model_id = 'a0000000-0000-7000-8000-000000000001'$$, '42501', 'permission denied for table asset_model_ancestors', 'app cannot delete model closure rows');
SELECT lives_ok($$DELETE FROM asset_models WHERE id = 'a0000000-0000-7000-8000-000000000001'$$, 'model deletion cascades through closure');
SELECT is_empty($$SELECT 1 FROM asset_model_ancestors WHERE descendant_model_id = 'a0000000-0000-7000-8000-000000000001'$$, 'deleted model has no closure rows');
SELECT pg_temp.create_ownable('a0000000-0000-7000-8000-000000000002');
SELECT throws_ok($$INSERT INTO brands (id, organization_id, project_id, table_name_id, name) VALUES ('a0000000-0000-7000-8000-000000000002', '20000000-0000-7000-8000-000000000001', '50000000-0000-7000-8000-000000000001', 'assets', 'Wrong brand table')$$, '23514', 'new row for relation "brands" violates check constraint "brands_table_name_id_check"', 'brand fixes its table identity');
SELECT pg_temp.new_model('a0000000-0000-7000-8000-000000000003', 'Container', NULL, 'container_models');
INSERT INTO physical_asset_models VALUES ('a0000000-0000-7000-8000-000000000003', '20000000-0000-7000-8000-000000000001');
INSERT INTO container_models VALUES ('a0000000-0000-7000-8000-000000000003', '20000000-0000-7000-8000-000000000001');
SELECT pg_temp.new_model('a0000000-0000-7000-8000-000000000004', 'Source family', NULL, 'sample_source_models');
INSERT INTO physical_asset_models VALUES ('a0000000-0000-7000-8000-000000000004', '20000000-0000-7000-8000-000000000001');
INSERT INTO sample_source_models VALUES ('a0000000-0000-7000-8000-000000000004', '20000000-0000-7000-8000-000000000001');
SELECT pg_temp.new_model('a0000000-0000-7000-8000-000000000005', 'Other family', NULL, 'sample_source_models');
INSERT INTO physical_asset_models VALUES ('a0000000-0000-7000-8000-000000000005', '20000000-0000-7000-8000-000000000001');
INSERT INTO sample_source_models VALUES ('a0000000-0000-7000-8000-000000000005', '20000000-0000-7000-8000-000000000001');
SELECT pg_temp.new_model('a0000000-0000-7000-8000-000000000006', 'Parent sample', NULL, 'sample_models');
INSERT INTO physical_asset_models VALUES ('a0000000-0000-7000-8000-000000000006', '20000000-0000-7000-8000-000000000001');
INSERT INTO sample_models VALUES ('a0000000-0000-7000-8000-000000000006', '20000000-0000-7000-8000-000000000001', 'a0000000-0000-7000-8000-000000000004');
SELECT pg_temp.new_model('a0000000-0000-7000-8000-000000000007', 'Child sample', 'a0000000-0000-7000-8000-000000000006', 'sample_models');
INSERT INTO physical_asset_models VALUES ('a0000000-0000-7000-8000-000000000007', '20000000-0000-7000-8000-000000000001');
SELECT throws_ok($$INSERT INTO sample_models VALUES ('a0000000-0000-7000-8000-000000000007', '20000000-0000-7000-8000-000000000001', 'a0000000-0000-7000-8000-000000000005'); SET CONSTRAINTS ALL IMMEDIATE$$, '23514', 'sample model source family must descend from its parent family', 'child sample refuses unrelated source family');
DELETE FROM sample_models WHERE id = 'a0000000-0000-7000-8000-000000000007';
SELECT lives_ok($$INSERT INTO sample_models VALUES ('a0000000-0000-7000-8000-000000000007', '20000000-0000-7000-8000-000000000001', 'a0000000-0000-7000-8000-000000000004'); SET CONSTRAINTS ALL IMMEDIATE$$, 'child sample accepts parent source family');
SET CONSTRAINTS ALL DEFERRED;
SELECT pg_temp.new_model('a0000000-0000-7000-8000-000000000009', 'Specialised source', 'a0000000-0000-7000-8000-000000000004', 'sample_source_models');
INSERT INTO physical_asset_models VALUES ('a0000000-0000-7000-8000-000000000009', '20000000-0000-7000-8000-000000000001');
INSERT INTO sample_source_models VALUES ('a0000000-0000-7000-8000-000000000009', '20000000-0000-7000-8000-000000000001');
DELETE FROM sample_models WHERE id = 'a0000000-0000-7000-8000-000000000007';
SELECT lives_ok($$INSERT INTO sample_models VALUES ('a0000000-0000-7000-8000-000000000007', '20000000-0000-7000-8000-000000000001', 'a0000000-0000-7000-8000-000000000009'); SET CONSTRAINTS ALL IMMEDIATE$$, 'child sample accepts descendant source family');
SET CONSTRAINTS ALL DEFERRED;
SELECT pg_temp.create_ownable('a0000000-0000-7000-8000-000000000008');
SELECT throws_ok($$INSERT INTO container_compatibility_rules (id, organization_id, project_id, table_name_id, container_model_id, contained_asset_model_id) VALUES ('a0000000-0000-7000-8000-000000000008', '20000000-0000-7000-8000-000000000001', '50000000-0000-7000-8000-000000000001', 'assets', 'a0000000-0000-7000-8000-000000000003', 'a0000000-0000-7000-8000-000000000004')$$, '23514', 'new row for relation "container_compatibility_rules" violates check constraint "container_compatibility_rules_table_name_id_check"', 'compatibility rule fixes its table identity');
DELETE FROM container_compatibility_rules WHERE id = 'a0000000-0000-7000-8000-000000000008';
INSERT INTO memberships (organization_id, user_id, state) VALUES ('20000000-0000-7000-8000-000000000001', '10000000-0000-7000-8000-000000000002', 'active');
INSERT INTO grants (organization_id, resource_id, grantee_id, grantee_kind, role) VALUES
('20000000-0000-7000-8000-000000000001', 'a0000000-0000-7000-8000-000000000003', '10000000-0000-7000-8000-000000000002', 'user', 'reader'),
('20000000-0000-7000-8000-000000000001', 'a0000000-0000-7000-8000-000000000004', '10000000-0000-7000-8000-000000000002', 'user', 'reader'),
('20000000-0000-7000-8000-000000000001', 'a0000000-0000-7000-8000-000000000008', '10000000-0000-7000-8000-000000000002', 'user', 'admin');
SET CONSTRAINTS ALL IMMEDIATE;
SET CONSTRAINTS ALL DEFERRED;
SELECT set_config('app.user_id', '10000000-0000-7000-8000-000000000002', false);
SELECT throws_ok($$INSERT INTO container_compatibility_rules (id, organization_id, project_id, table_name_id, container_model_id, contained_asset_model_id) VALUES ('a0000000-0000-7000-8000-000000000008', '20000000-0000-7000-8000-000000000001', '50000000-0000-7000-8000-000000000001', 'container_compatibility_rules', 'a0000000-0000-7000-8000-000000000003', 'a0000000-0000-7000-8000-000000000004')$$, '42501', 'new row violates row-level security policy for table "container_compatibility_rules"', 'container reader cannot insert capacity rule');
SELECT set_config('app.user_id', '10000000-0000-7000-8000-000000000001', false);
DELETE FROM container_compatibility_rules WHERE id = 'a0000000-0000-7000-8000-000000000008';
INSERT INTO container_compatibility_rules (id, organization_id, project_id, table_name_id, container_model_id, contained_asset_model_id) VALUES ('a0000000-0000-7000-8000-000000000008', '20000000-0000-7000-8000-000000000001', '50000000-0000-7000-8000-000000000001', 'container_compatibility_rules', 'a0000000-0000-7000-8000-000000000003', 'a0000000-0000-7000-8000-000000000004');
SELECT set_config('app.user_id', '10000000-0000-7000-8000-000000000002', false);
SELECT is_empty($$UPDATE container_compatibility_rules SET quantity = 2 RETURNING id$$, 'container reader cannot update capacity rule');
SELECT is_empty($$DELETE FROM container_compatibility_rules RETURNING id$$, 'container reader cannot delete capacity rule');
SELECT set_config('app.user_id', '10000000-0000-7000-8000-000000000001', false);
SELECT lives_ok($$UPDATE container_compatibility_rules SET quantity = 2 WHERE id = 'a0000000-0000-7000-8000-000000000008'$$, 'container writer updates capacity rule');
SELECT is((SELECT quantity FROM container_compatibility_rules WHERE id = 'a0000000-0000-7000-8000-000000000008'), 2::smallint, 'writer capacity update is stored');
SELECT isnt_empty($$DELETE FROM container_compatibility_rules WHERE id = 'a0000000-0000-7000-8000-000000000008' RETURNING id$$, 'container writer with rule admin deletes rule');
SELECT throws_ok($$SELECT sample_source_id FROM samples$$, '42703', 'column "sample_source_id" does not exist', 'samples have no duplicate concrete source');
SELECT throws_ok($$SELECT source_model_id FROM samples$$, '42703', 'column "source_model_id" does not exist', 'samples have no duplicate concrete source model');
UPDATE grants SET role = 'writer' WHERE resource_id = 'a0000000-0000-7000-8000-000000000003'
    AND grantee_id = '10000000-0000-7000-8000-000000000002';
SELECT set_config('app.user_id', '10000000-0000-7000-8000-000000000002', false);
INSERT INTO organizations (id, created_by_id) VALUES ('20000000-0000-7000-8000-000000000002', authz.caller_id());
INSERT INTO resources (id, kind, organization_id)
VALUES ('20000000-0000-7000-8000-000000000002', 'organization', '20000000-0000-7000-8000-000000000002');
INSERT INTO memberships (organization_id, user_id, state)
VALUES ('20000000-0000-7000-8000-000000000002', authz.caller_id(), 'active');
INSERT INTO grants (organization_id, resource_id, grantee_id, grantee_kind, role)
VALUES ('20000000-0000-7000-8000-000000000002', '20000000-0000-7000-8000-000000000002', authz.caller_id(), 'user', 'admin');
SET CONSTRAINTS ALL IMMEDIATE;
SET CONSTRAINTS ALL DEFERRED;
INSERT INTO resources (id, kind, organization_id)
VALUES ('50000000-0000-7000-8000-000000000002', 'project', '20000000-0000-7000-8000-000000000002');
INSERT INTO resource_parents (organization_id, id, resource_kind, parent_id, parent_kind)
VALUES ('20000000-0000-7000-8000-000000000002', '50000000-0000-7000-8000-000000000002', 'project', '20000000-0000-7000-8000-000000000002', 'organization');
SET CONSTRAINTS ALL IMMEDIATE;
SET CONSTRAINTS ALL DEFERRED;
INSERT INTO ownables (id, organization_id, project_id, created_by_id)
VALUES ('a0000000-0000-7000-8000-000000000010', '20000000-0000-7000-8000-000000000002', '50000000-0000-7000-8000-000000000002', authz.caller_id());
INSERT INTO resources (id, kind, organization_id)
VALUES ('a0000000-0000-7000-8000-000000000010', 'ownable', '20000000-0000-7000-8000-000000000002');
SELECT throws_ok($$INSERT INTO container_compatibility_rules (id, organization_id, project_id, table_name_id, container_model_id, contained_asset_model_id)
VALUES ('a0000000-0000-7000-8000-000000000010', '20000000-0000-7000-8000-000000000002', '50000000-0000-7000-8000-000000000002', 'container_compatibility_rules', 'a0000000-0000-7000-8000-000000000003', 'a0000000-0000-7000-8000-000000000004')$$,
'23503', NULL, 'capacity rule belongs to the container model organization');
SET CONSTRAINTS ALL IMMEDIATE;
RESET ROLE;
SELECT * FROM finish();
ROLLBACK;
