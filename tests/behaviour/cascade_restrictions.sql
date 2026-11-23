\set ON_ERROR_STOP 1
SET search_path = public, test_tools;
BEGIN;
\ir ../fixture.sql
SELECT plan(15);
-- PostgreSQL 18 reports a refused `ON DELETE RESTRICT` with its own SQLSTATE, 23001, where 17 reports 23503.
SELECT CASE WHEN current_setting('server_version_num')::INTEGER >= 180000 THEN '23001' ELSE '23503' END AS restrict_violation \gset
SELECT pg_temp.create_ownable('61000000-0000-7000-8000-000000000001');
INSERT INTO brands (id, organization_id, project_id, table_name_id, name)
VALUES ('61000000-0000-7000-8000-000000000001', '20000000-0000-7000-8000-000000000001',
        '50000000-0000-7000-8000-000000000001', 'brands', 'Cascade test brand');
SELECT pg_temp.new_model('62000001-0000-7000-8000-000000000001', 'weighing_device base', NULL, 'weighing_device_models');
INSERT INTO physical_asset_models (id, organization_id)
VALUES ('62000001-0000-7000-8000-000000000001', '20000000-0000-7000-8000-000000000001');
INSERT INTO weighing_device_models (id, organization_id)
VALUES ('62000001-0000-7000-8000-000000000001', '20000000-0000-7000-8000-000000000001');
SELECT pg_temp.new_model('62000001-0000-7000-8000-000000000002', 'weighing_device commercial', '62000001-0000-7000-8000-000000000001', 'commercial_weighing_device_models');
INSERT INTO physical_asset_models (id, organization_id)
VALUES ('62000001-0000-7000-8000-000000000002', '20000000-0000-7000-8000-000000000001');
INSERT INTO weighing_device_models (id, organization_id)
VALUES ('62000001-0000-7000-8000-000000000002', '20000000-0000-7000-8000-000000000001');
INSERT INTO commercial_products (id, organization_id, brand_id)
VALUES ('62000001-0000-7000-8000-000000000002', '20000000-0000-7000-8000-000000000001', '61000000-0000-7000-8000-000000000001');
INSERT INTO commercial_weighing_device_models (id, organization_id, weighing_device_model_id)
VALUES ('62000001-0000-7000-8000-000000000002', '20000000-0000-7000-8000-000000000001', '62000001-0000-7000-8000-000000000001');
SET CONSTRAINTS ALL IMMEDIATE;
SET CONSTRAINTS ALL DEFERRED;
SELECT throws_ok($statement$DELETE FROM weighing_device_models WHERE id = '62000001-0000-7000-8000-000000000001'; SET CONSTRAINTS ALL IMMEDIATE$statement$, :'restrict_violation', NULL, 'weighing_device base model deletion preserves independent commercial model');
SELECT pg_temp.new_model('62000002-0000-7000-8000-000000000001', 'volume_measuring_device base', NULL, 'volume_measuring_device_models');
INSERT INTO physical_asset_models (id, organization_id)
VALUES ('62000002-0000-7000-8000-000000000001', '20000000-0000-7000-8000-000000000001');
INSERT INTO volume_measuring_device_models (id, organization_id)
VALUES ('62000002-0000-7000-8000-000000000001', '20000000-0000-7000-8000-000000000001');
SELECT pg_temp.new_model('62000002-0000-7000-8000-000000000002', 'volume_measuring_device commercial', '62000002-0000-7000-8000-000000000001', 'commercial_volume_measuring_device_models');
INSERT INTO physical_asset_models (id, organization_id)
VALUES ('62000002-0000-7000-8000-000000000002', '20000000-0000-7000-8000-000000000001');
INSERT INTO volume_measuring_device_models (id, organization_id)
VALUES ('62000002-0000-7000-8000-000000000002', '20000000-0000-7000-8000-000000000001');
INSERT INTO commercial_products (id, organization_id, brand_id)
VALUES ('62000002-0000-7000-8000-000000000002', '20000000-0000-7000-8000-000000000001', '61000000-0000-7000-8000-000000000001');
INSERT INTO commercial_volume_measuring_device_models (id, organization_id, volume_measuring_device_model_id)
VALUES ('62000002-0000-7000-8000-000000000002', '20000000-0000-7000-8000-000000000001', '62000002-0000-7000-8000-000000000001');
SET CONSTRAINTS ALL IMMEDIATE;
SET CONSTRAINTS ALL DEFERRED;
SELECT throws_ok($statement$DELETE FROM volume_measuring_device_models WHERE id = '62000002-0000-7000-8000-000000000001'; SET CONSTRAINTS ALL IMMEDIATE$statement$, :'restrict_violation', NULL, 'volume_measuring_device base model deletion preserves independent commercial model');
SELECT pg_temp.new_model('62000003-0000-7000-8000-000000000001', 'ball_mill_machine base', NULL, 'ball_mill_machine_models');
INSERT INTO physical_asset_models (id, organization_id)
VALUES ('62000003-0000-7000-8000-000000000001', '20000000-0000-7000-8000-000000000001');
INSERT INTO ball_mill_machine_models (id, organization_id)
VALUES ('62000003-0000-7000-8000-000000000001', '20000000-0000-7000-8000-000000000001');
SELECT pg_temp.new_model('62000003-0000-7000-8000-000000000002', 'ball_mill_machine commercial', '62000003-0000-7000-8000-000000000001', 'commercial_ball_mill_machine_models');
INSERT INTO physical_asset_models (id, organization_id)
VALUES ('62000003-0000-7000-8000-000000000002', '20000000-0000-7000-8000-000000000001');
INSERT INTO ball_mill_machine_models (id, organization_id)
VALUES ('62000003-0000-7000-8000-000000000002', '20000000-0000-7000-8000-000000000001');
INSERT INTO commercial_products (id, organization_id, brand_id)
VALUES ('62000003-0000-7000-8000-000000000002', '20000000-0000-7000-8000-000000000001', '61000000-0000-7000-8000-000000000001');
INSERT INTO commercial_ball_mill_machine_models (id, organization_id, ball_mill_machine_model_id)
VALUES ('62000003-0000-7000-8000-000000000002', '20000000-0000-7000-8000-000000000001', '62000003-0000-7000-8000-000000000001');
SET CONSTRAINTS ALL IMMEDIATE;
SET CONSTRAINTS ALL DEFERRED;
SELECT throws_ok($statement$DELETE FROM ball_mill_machine_models WHERE id = '62000003-0000-7000-8000-000000000001'; SET CONSTRAINTS ALL IMMEDIATE$statement$, :'restrict_violation', NULL, 'ball_mill_machine base model deletion preserves independent commercial model');
SELECT pg_temp.new_model('62000004-0000-7000-8000-000000000001', 'centrifuge base', NULL, 'centrifuge_models');
INSERT INTO physical_asset_models (id, organization_id)
VALUES ('62000004-0000-7000-8000-000000000001', '20000000-0000-7000-8000-000000000001');
INSERT INTO centrifuge_models (id, organization_id)
VALUES ('62000004-0000-7000-8000-000000000001', '20000000-0000-7000-8000-000000000001');
SELECT pg_temp.new_model('62000004-0000-7000-8000-000000000002', 'centrifuge commercial', '62000004-0000-7000-8000-000000000001', 'commercial_centrifuge_models');
INSERT INTO physical_asset_models (id, organization_id)
VALUES ('62000004-0000-7000-8000-000000000002', '20000000-0000-7000-8000-000000000001');
INSERT INTO centrifuge_models (id, organization_id)
VALUES ('62000004-0000-7000-8000-000000000002', '20000000-0000-7000-8000-000000000001');
INSERT INTO commercial_products (id, organization_id, brand_id)
VALUES ('62000004-0000-7000-8000-000000000002', '20000000-0000-7000-8000-000000000001', '61000000-0000-7000-8000-000000000001');
INSERT INTO commercial_centrifuge_models (id, organization_id, centrifuge_model_id)
VALUES ('62000004-0000-7000-8000-000000000002', '20000000-0000-7000-8000-000000000001', '62000004-0000-7000-8000-000000000001');
SET CONSTRAINTS ALL IMMEDIATE;
SET CONSTRAINTS ALL DEFERRED;
SELECT throws_ok($statement$DELETE FROM centrifuge_models WHERE id = '62000004-0000-7000-8000-000000000001'; SET CONSTRAINTS ALL IMMEDIATE$statement$, :'restrict_violation', NULL, 'centrifuge base model deletion preserves independent commercial model');
SELECT pg_temp.new_model('62000005-0000-7000-8000-000000000001', 'freeze_dryer base', NULL, 'freeze_dryer_models');
INSERT INTO physical_asset_models (id, organization_id)
VALUES ('62000005-0000-7000-8000-000000000001', '20000000-0000-7000-8000-000000000001');
INSERT INTO freeze_dryer_models (id, organization_id)
VALUES ('62000005-0000-7000-8000-000000000001', '20000000-0000-7000-8000-000000000001');
SELECT pg_temp.new_model('62000005-0000-7000-8000-000000000002', 'freeze_dryer commercial', '62000005-0000-7000-8000-000000000001', 'commercial_freeze_dryer_models');
INSERT INTO physical_asset_models (id, organization_id)
VALUES ('62000005-0000-7000-8000-000000000002', '20000000-0000-7000-8000-000000000001');
INSERT INTO freeze_dryer_models (id, organization_id)
VALUES ('62000005-0000-7000-8000-000000000002', '20000000-0000-7000-8000-000000000001');
INSERT INTO commercial_products (id, organization_id, brand_id)
VALUES ('62000005-0000-7000-8000-000000000002', '20000000-0000-7000-8000-000000000001', '61000000-0000-7000-8000-000000000001');
INSERT INTO commercial_freeze_dryer_models (id, organization_id, freeze_dryer_model_id)
VALUES ('62000005-0000-7000-8000-000000000002', '20000000-0000-7000-8000-000000000001', '62000005-0000-7000-8000-000000000001');
SET CONSTRAINTS ALL IMMEDIATE;
SET CONSTRAINTS ALL DEFERRED;
SELECT throws_ok($statement$DELETE FROM freeze_dryer_models WHERE id = '62000005-0000-7000-8000-000000000001'; SET CONSTRAINTS ALL IMMEDIATE$statement$, :'restrict_violation', NULL, 'freeze_dryer base model deletion preserves independent commercial model');
SELECT pg_temp.new_model('62000006-0000-7000-8000-000000000001', 'freezer base', NULL, 'freezer_models');
INSERT INTO physical_asset_models (id, organization_id)
VALUES ('62000006-0000-7000-8000-000000000001', '20000000-0000-7000-8000-000000000001');
INSERT INTO freezer_models (id, organization_id)
VALUES ('62000006-0000-7000-8000-000000000001', '20000000-0000-7000-8000-000000000001');
SELECT pg_temp.new_model('62000006-0000-7000-8000-000000000002', 'freezer commercial', '62000006-0000-7000-8000-000000000001', 'commercial_freezer_models');
INSERT INTO physical_asset_models (id, organization_id)
VALUES ('62000006-0000-7000-8000-000000000002', '20000000-0000-7000-8000-000000000001');
INSERT INTO freezer_models (id, organization_id)
VALUES ('62000006-0000-7000-8000-000000000002', '20000000-0000-7000-8000-000000000001');
INSERT INTO commercial_products (id, organization_id, brand_id)
VALUES ('62000006-0000-7000-8000-000000000002', '20000000-0000-7000-8000-000000000001', '61000000-0000-7000-8000-000000000001');
INSERT INTO commercial_freezer_models (id, organization_id, freezer_model_id)
VALUES ('62000006-0000-7000-8000-000000000002', '20000000-0000-7000-8000-000000000001', '62000006-0000-7000-8000-000000000001');
SET CONSTRAINTS ALL IMMEDIATE;
SET CONSTRAINTS ALL DEFERRED;
SELECT throws_ok($statement$DELETE FROM freezer_models WHERE id = '62000006-0000-7000-8000-000000000001'; SET CONSTRAINTS ALL IMMEDIATE$statement$, :'restrict_violation', NULL, 'freezer base model deletion preserves independent commercial model');
SELECT pg_temp.new_model('62000007-0000-7000-8000-000000000001', 'geopositioning_device base', NULL, 'geopositioning_device_models');
INSERT INTO physical_asset_models (id, organization_id)
VALUES ('62000007-0000-7000-8000-000000000001', '20000000-0000-7000-8000-000000000001');
INSERT INTO geopositioning_device_models (id, organization_id)
VALUES ('62000007-0000-7000-8000-000000000001', '20000000-0000-7000-8000-000000000001');
SELECT pg_temp.new_model('62000007-0000-7000-8000-000000000002', 'geopositioning_device commercial', '62000007-0000-7000-8000-000000000001', 'commercial_geopositioning_device_models');
INSERT INTO physical_asset_models (id, organization_id)
VALUES ('62000007-0000-7000-8000-000000000002', '20000000-0000-7000-8000-000000000001');
INSERT INTO geopositioning_device_models (id, organization_id)
VALUES ('62000007-0000-7000-8000-000000000002', '20000000-0000-7000-8000-000000000001');
INSERT INTO commercial_products (id, organization_id, brand_id)
VALUES ('62000007-0000-7000-8000-000000000002', '20000000-0000-7000-8000-000000000001', '61000000-0000-7000-8000-000000000001');
INSERT INTO commercial_geopositioning_device_models (id, organization_id, geopositioning_device_model_id)
VALUES ('62000007-0000-7000-8000-000000000002', '20000000-0000-7000-8000-000000000001', '62000007-0000-7000-8000-000000000001');
SET CONSTRAINTS ALL IMMEDIATE;
SET CONSTRAINTS ALL DEFERRED;
SELECT throws_ok($statement$DELETE FROM geopositioning_device_models WHERE id = '62000007-0000-7000-8000-000000000001'; SET CONSTRAINTS ALL IMMEDIATE$statement$, :'restrict_violation', NULL, 'geopositioning_device base model deletion preserves independent commercial model');
SELECT pg_temp.new_model('62000008-0000-7000-8000-000000000001', 'camera base', NULL, 'camera_models');
INSERT INTO physical_asset_models (id, organization_id)
VALUES ('62000008-0000-7000-8000-000000000001', '20000000-0000-7000-8000-000000000001');
INSERT INTO camera_models (id, organization_id)
VALUES ('62000008-0000-7000-8000-000000000001', '20000000-0000-7000-8000-000000000001');
SELECT pg_temp.new_model('62000008-0000-7000-8000-000000000002', 'camera commercial', '62000008-0000-7000-8000-000000000001', 'commercial_camera_models');
INSERT INTO physical_asset_models (id, organization_id)
VALUES ('62000008-0000-7000-8000-000000000002', '20000000-0000-7000-8000-000000000001');
INSERT INTO camera_models (id, organization_id)
VALUES ('62000008-0000-7000-8000-000000000002', '20000000-0000-7000-8000-000000000001');
INSERT INTO commercial_products (id, organization_id, brand_id)
VALUES ('62000008-0000-7000-8000-000000000002', '20000000-0000-7000-8000-000000000001', '61000000-0000-7000-8000-000000000001');
INSERT INTO commercial_camera_models (id, organization_id, camera_model_id)
VALUES ('62000008-0000-7000-8000-000000000002', '20000000-0000-7000-8000-000000000001', '62000008-0000-7000-8000-000000000001');
SET CONSTRAINTS ALL IMMEDIATE;
SET CONSTRAINTS ALL DEFERRED;
SELECT throws_ok($statement$DELETE FROM camera_models WHERE id = '62000008-0000-7000-8000-000000000001'; SET CONSTRAINTS ALL IMMEDIATE$statement$, :'restrict_violation', NULL, 'camera base model deletion preserves independent commercial model');
SELECT pg_temp.new_model('62000009-0000-7000-8000-000000000001', 'mass_spectrometer base', NULL, 'mass_spectrometer_models');
INSERT INTO physical_asset_models (id, organization_id)
VALUES ('62000009-0000-7000-8000-000000000001', '20000000-0000-7000-8000-000000000001');
INSERT INTO mass_spectrometer_models (id, organization_id)
VALUES ('62000009-0000-7000-8000-000000000001', '20000000-0000-7000-8000-000000000001');
SELECT pg_temp.new_model('62000009-0000-7000-8000-000000000002', 'mass_spectrometer commercial', '62000009-0000-7000-8000-000000000001', 'commercial_mass_spectrometer_models');
INSERT INTO physical_asset_models (id, organization_id)
VALUES ('62000009-0000-7000-8000-000000000002', '20000000-0000-7000-8000-000000000001');
INSERT INTO mass_spectrometer_models (id, organization_id)
VALUES ('62000009-0000-7000-8000-000000000002', '20000000-0000-7000-8000-000000000001');
INSERT INTO commercial_products (id, organization_id, brand_id)
VALUES ('62000009-0000-7000-8000-000000000002', '20000000-0000-7000-8000-000000000001', '61000000-0000-7000-8000-000000000001');
INSERT INTO commercial_mass_spectrometer_models (id, organization_id, mass_spectrometer_model_id)
VALUES ('62000009-0000-7000-8000-000000000002', '20000000-0000-7000-8000-000000000001', '62000009-0000-7000-8000-000000000001');
SET CONSTRAINTS ALL IMMEDIATE;
SET CONSTRAINTS ALL DEFERRED;
SELECT throws_ok($statement$DELETE FROM mass_spectrometer_models WHERE id = '62000009-0000-7000-8000-000000000001'; SET CONSTRAINTS ALL IMMEDIATE$statement$, :'restrict_violation', NULL, 'mass_spectrometer base model deletion preserves independent commercial model');
SELECT pg_temp.new_model('62000010-0000-7000-8000-000000000001', 'personal_protective_equipment base', NULL, 'personal_protective_equipment_models');
INSERT INTO physical_asset_models (id, organization_id)
VALUES ('62000010-0000-7000-8000-000000000001', '20000000-0000-7000-8000-000000000001');
INSERT INTO personal_protective_equipment_models (id, organization_id)
VALUES ('62000010-0000-7000-8000-000000000001', '20000000-0000-7000-8000-000000000001');
SELECT pg_temp.new_model('62000010-0000-7000-8000-000000000002', 'personal_protective_equipment commercial', '62000010-0000-7000-8000-000000000001', 'commercial_personal_protective_equipment_models');
INSERT INTO physical_asset_models (id, organization_id)
VALUES ('62000010-0000-7000-8000-000000000002', '20000000-0000-7000-8000-000000000001');
INSERT INTO personal_protective_equipment_models (id, organization_id)
VALUES ('62000010-0000-7000-8000-000000000002', '20000000-0000-7000-8000-000000000001');
INSERT INTO commercial_products (id, organization_id, brand_id)
VALUES ('62000010-0000-7000-8000-000000000002', '20000000-0000-7000-8000-000000000001', '61000000-0000-7000-8000-000000000001');
INSERT INTO commercial_personal_protective_equipment_models (id, organization_id, personal_protective_equipment_model_id)
VALUES ('62000010-0000-7000-8000-000000000002', '20000000-0000-7000-8000-000000000001', '62000010-0000-7000-8000-000000000001');
SET CONSTRAINTS ALL IMMEDIATE;
SET CONSTRAINTS ALL DEFERRED;
SELECT throws_ok($statement$DELETE FROM personal_protective_equipment_models WHERE id = '62000010-0000-7000-8000-000000000001'; SET CONSTRAINTS ALL IMMEDIATE$statement$, :'restrict_violation', NULL, 'personal_protective_equipment base model deletion preserves independent commercial model');
SELECT pg_temp.new_model('62000011-0000-7000-8000-000000000001', 'pipette_tip base', NULL, 'pipette_tip_models');
INSERT INTO physical_asset_models (id, organization_id)
VALUES ('62000011-0000-7000-8000-000000000001', '20000000-0000-7000-8000-000000000001');
INSERT INTO pipette_tip_models (id, organization_id)
VALUES ('62000011-0000-7000-8000-000000000001', '20000000-0000-7000-8000-000000000001');
SELECT pg_temp.new_model('62000011-0000-7000-8000-000000000002', 'pipette_tip commercial', '62000011-0000-7000-8000-000000000001', 'commercial_pipette_tip_models');
INSERT INTO physical_asset_models (id, organization_id)
VALUES ('62000011-0000-7000-8000-000000000002', '20000000-0000-7000-8000-000000000001');
INSERT INTO pipette_tip_models (id, organization_id)
VALUES ('62000011-0000-7000-8000-000000000002', '20000000-0000-7000-8000-000000000001');
INSERT INTO commercial_products (id, organization_id, brand_id)
VALUES ('62000011-0000-7000-8000-000000000002', '20000000-0000-7000-8000-000000000001', '61000000-0000-7000-8000-000000000001');
INSERT INTO commercial_pipette_tip_models (id, organization_id, pipette_tip_model_id)
VALUES ('62000011-0000-7000-8000-000000000002', '20000000-0000-7000-8000-000000000001', '62000011-0000-7000-8000-000000000001');
SET CONSTRAINTS ALL IMMEDIATE;
SET CONSTRAINTS ALL DEFERRED;
SELECT throws_ok($statement$DELETE FROM pipette_tip_models WHERE id = '62000011-0000-7000-8000-000000000001'; SET CONSTRAINTS ALL IMMEDIATE$statement$, :'restrict_violation', NULL, 'pipette_tip base model deletion preserves independent commercial model');
SELECT pg_temp.new_model('62000012-0000-7000-8000-000000000001', 'marker base', NULL, 'marker_models');
INSERT INTO physical_asset_models (id, organization_id)
VALUES ('62000012-0000-7000-8000-000000000001', '20000000-0000-7000-8000-000000000001');
INSERT INTO marker_models (id, organization_id)
VALUES ('62000012-0000-7000-8000-000000000001', '20000000-0000-7000-8000-000000000001');
SELECT pg_temp.new_model('62000012-0000-7000-8000-000000000002', 'marker commercial', '62000012-0000-7000-8000-000000000001', 'commercial_marker_models');
INSERT INTO physical_asset_models (id, organization_id)
VALUES ('62000012-0000-7000-8000-000000000002', '20000000-0000-7000-8000-000000000001');
INSERT INTO marker_models (id, organization_id)
VALUES ('62000012-0000-7000-8000-000000000002', '20000000-0000-7000-8000-000000000001');
INSERT INTO commercial_products (id, organization_id, brand_id)
VALUES ('62000012-0000-7000-8000-000000000002', '20000000-0000-7000-8000-000000000001', '61000000-0000-7000-8000-000000000001');
INSERT INTO commercial_marker_models (id, organization_id, marker_model_id)
VALUES ('62000012-0000-7000-8000-000000000002', '20000000-0000-7000-8000-000000000001', '62000012-0000-7000-8000-000000000001');
SET CONSTRAINTS ALL IMMEDIATE;
SET CONSTRAINTS ALL DEFERRED;
SELECT throws_ok($statement$DELETE FROM marker_models WHERE id = '62000012-0000-7000-8000-000000000001'; SET CONSTRAINTS ALL IMMEDIATE$statement$, :'restrict_violation', NULL, 'marker base model deletion preserves independent commercial model');
SELECT pg_temp.new_model('63000000-0000-7000-8000-000000000001', 'Product model', NULL, 'commercial_products');
INSERT INTO commercial_products (id, organization_id, brand_id)
VALUES ('63000000-0000-7000-8000-000000000001', '20000000-0000-7000-8000-000000000001', '61000000-0000-7000-8000-000000000001');
SELECT pg_temp.new_model('63000000-0000-7000-8000-000000000002', 'Lot model', '63000000-0000-7000-8000-000000000001', 'commercial_product_lots');
INSERT INTO physical_asset_models (id, organization_id)
VALUES ('63000000-0000-7000-8000-000000000002', '20000000-0000-7000-8000-000000000001');
INSERT INTO commercial_product_lots (id, organization_id, lot, product_model_id)
VALUES ('63000000-0000-7000-8000-000000000002', '20000000-0000-7000-8000-000000000001', 'Lot 1', '63000000-0000-7000-8000-000000000001');
SET CONSTRAINTS ALL IMMEDIATE;
SET CONSTRAINTS ALL DEFERRED;
SELECT throws_ok($statement$DELETE FROM commercial_products WHERE id = '63000000-0000-7000-8000-000000000001'; SET CONSTRAINTS ALL IMMEDIATE$statement$, :'restrict_violation', NULL, 'product deletion preserves independent lot');
SET CONSTRAINTS ALL DEFERRED;
SELECT pg_temp.new_model('64000000-0000-7000-8000-000000000001', 'Container model', NULL, 'container_models');
SELECT pg_temp.new_model('64000000-0000-7000-8000-000000000002', 'Contained model', NULL, 'physical_asset_models');
INSERT INTO physical_asset_models (id, organization_id) VALUES
    ('64000000-0000-7000-8000-000000000001', '20000000-0000-7000-8000-000000000001'),
    ('64000000-0000-7000-8000-000000000002', '20000000-0000-7000-8000-000000000001');
INSERT INTO container_models (id, organization_id)
VALUES ('64000000-0000-7000-8000-000000000001', '20000000-0000-7000-8000-000000000001');
SELECT pg_temp.create_ownable('64000000-0000-7000-8000-000000000003');
INSERT INTO container_compatibility_rules
    (id, organization_id, project_id, table_name_id, container_model_id, contained_asset_model_id)
VALUES ('64000000-0000-7000-8000-000000000003', '20000000-0000-7000-8000-000000000001',
    '50000000-0000-7000-8000-000000000001', 'container_compatibility_rules',
    '64000000-0000-7000-8000-000000000001', '64000000-0000-7000-8000-000000000002');
SET CONSTRAINTS ALL IMMEDIATE;
SET CONSTRAINTS ALL DEFERRED;
SELECT throws_ok($statement$DELETE FROM container_models WHERE id = '64000000-0000-7000-8000-000000000001'; SET CONSTRAINTS ALL IMMEDIATE$statement$, :'restrict_violation', NULL, 'container_models deletion preserves independent compatibility rule');
SELECT throws_ok($statement$DELETE FROM physical_asset_models WHERE id = '64000000-0000-7000-8000-000000000002'; SET CONSTRAINTS ALL IMMEDIATE$statement$, :'restrict_violation', NULL, 'physical_asset_models deletion preserves independent compatibility rule');
RESET ROLE;
SELECT * FROM finish();
ROLLBACK;
