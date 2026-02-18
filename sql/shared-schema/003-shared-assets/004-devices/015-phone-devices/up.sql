-- ============================================================================
-- Migration: Shared Assets / Devices / Phone Devices
-- Purpose:
--   Adds phone-specific device tables to APS with a dual-capability model DAG:
--   every phone device model is both a camera model and a geopositioning
--   device model, and every commercial phone model is both a commercial camera
--   model and a commercial geopositioning device model.
--
-- APS placement (inheritance and specialization):
--
--   entities
--     +-- ownables
--         +-- namespaced_ownables
--             +-- asset_models
--             |   +-- physical_asset_models
--             |   |   +-- camera_models
--             |   |   |   +-- phone_device_models
--             |   |   +-- geopositioning_device_models
--             |   |       +-- phone_device_models
--             |   +-- commercial_products
--             |   |   +-- commercial_camera_models
--             |   |   |   +-- commercial_phone_device_models
--             |   |   +-- commercial_geopositioning_device_models
--             |   |       +-- commercial_phone_device_models
--             |   +-- commercial_product_lots
--             |       +-- commercial_camera_lots
--             |       |   +-- commercial_phone_device_lots
--             |       +-- commercial_geopositioning_device_lots
--             |           +-- commercial_phone_device_lots
--             +-- assets
--                 +-- physical_assets
--                     +-- cameras
--                     |   +-- phone_devices
--                     +-- geopositioning_devices
--                         +-- phone_devices
--
-- Integrity bridge to core APS tables:
--   phone_devices.id
--     -> cameras.id
--     -> geopositioning_devices.id
--
-- Metadata registration:
--   Every new table in this migration is inserted into `table_names` so APS
--   entities can resolve their concrete table type.
--
-- Security context (local, inherited through ancestor PK extension):
--   This migration does not define table-local RLS policies. Effective access
--   to rows is inherited from ancestor `ownables` rows through PK/FK identity
--   chains in the model and asset hierarchies.
--
--   PK-extension chains used for inherited access checks:
--   - `phone_device_models`:
--       phone_device_models.id
--         -> camera_models.id / geopositioning_device_models.id
--         -> physical_asset_models.id
--         -> asset_models.id
--         -> namespaced_ownables.id
--         -> ownables.id
--   - `commercial_phone_device_models`:
--       commercial_phone_device_models.id
--         -> commercial_camera_models.id / commercial_geopositioning_device_models.id
--         -> commercial_products.id
--         -> asset_models.id
--         -> namespaced_ownables.id
--         -> ownables.id
--   - `commercial_phone_device_lots`:
--       commercial_phone_device_lots.id
--         -> commercial_camera_lots.id / commercial_geopositioning_device_lots.id
--         -> commercial_product_lots.id
--         -> physical_asset_models.id
--         -> asset_models.id
--         -> namespaced_ownables.id
--         -> ownables.id
--   - `phone_devices`:
--       phone_devices.id
--         -> cameras.id / geopositioning_devices.id
--         -> physical_assets.id
--         -> assets.id
--         -> namespaced_ownables.id
--         -> ownables.id
--
--   SQL/RLS semantics for these specific phone tables:
--   - `SELECT`: allowed when caller has at least viewer role on ancestor
--     `ownables.owner_id` (role >= 2).
--   - `INSERT`: allowed when caller has at least editor role on ancestor
--     `ownables.owner_id` (role >= 3).
--   - `UPDATE`: allowed when caller has at least editor role on ancestor
--     `ownables.owner_id` (role >= 3).
--   - `DELETE`: allowed when caller has admin role on ancestor
--     `ownables.owner_id` (role >= 4).
--   - Role resolution is SQL-native in APS via `get_owner_role(...)`, which
--     composes `team_members` membership and `owner_grants`.
--
--   Zanzibar semantics for these specific phone tables:
--   - Equivalent permissions are modeled as relation checks on `ownable:{id}`
--     (`viewer`, `editor`, `admin`) for each table row.
--   - Membership and grants are represented as graph traversal
--     (user -> team -> owner, plus explicit owner grants).
--   - Zanzibar-style syntax expresses relationship intent, while SQL/RLS
--     enforces row filtering and write authorization inside PostgreSQL.
-- ============================================================================

-- Model catalog for phone-capable devices.
-- A row exists only when the same model identity is valid as both a camera
-- model and a geopositioning device model.
CREATE TABLE phone_device_models (
	-- Stable model identifier shared with parent camera/geopositioning models.
	id UUID PRIMARY KEY,
	-- Enforces camera-model ancestry for every phone device model.
	CONSTRAINT phone_device_models_camera
		FOREIGN KEY (id) REFERENCES camera_models (id) ON DELETE CASCADE,
	-- Enforces geopositioning-model ancestry for every phone device model.
	CONSTRAINT phone_device_models_geopositioning
		FOREIGN KEY (id) REFERENCES geopositioning_device_models (id) ON DELETE CASCADE
);
-- Registers this table name so APS entities can reference this concrete model type.
INSERT INTO table_names (id) VALUES ('phone_device_models') ON CONFLICT DO NOTHING;

-- Commercial phone model catalog.
-- This table is an id-only DAG bridge over commercial camera and commercial
-- geopositioning models, so each row must satisfy both commercial ancestries.
CREATE TABLE commercial_phone_device_models (
	-- Stable commercial model identifier shared with both commercial parent models.
	id UUID PRIMARY KEY,
	-- Enforces commercial camera-model ancestry for every commercial phone model.
	FOREIGN KEY (id) REFERENCES commercial_camera_models (id) ON DELETE CASCADE,
	-- Enforces commercial geopositioning-model ancestry for every commercial phone model.
	FOREIGN KEY (id) REFERENCES commercial_geopositioning_device_models (id) ON DELETE CASCADE
);
-- Registers this table name so APS entities can reference this concrete commercial model type.
INSERT INTO table_names (id) VALUES ('commercial_phone_device_models') ON CONFLICT DO NOTHING;

-- Commercial lot catalog for phone devices.
-- A row exists only when the same lot identity is valid as both a commercial
-- camera lot and a commercial geopositioning-device lot.
CREATE TABLE commercial_phone_device_lots (
	-- Stable lot identifier shared with both commercial parent lot tables.
	id UUID PRIMARY KEY,
	-- Enforces commercial camera-lot ancestry for every commercial phone lot.
	CONSTRAINT commercial_phone_device_lots_camera
		FOREIGN KEY (id) REFERENCES commercial_camera_lots (id) ON DELETE CASCADE,
	-- Enforces commercial geopositioning-lot ancestry for every commercial phone lot.
	CONSTRAINT commercial_phone_device_lots_geopositioning
		FOREIGN KEY (id) REFERENCES commercial_geopositioning_device_lots (id) ON DELETE CASCADE
);
-- Registers this table name so APS entities can reference this concrete lot type.
INSERT INTO table_names (id) VALUES ('commercial_phone_device_lots') ON CONFLICT DO NOTHING;

-- Physical phone devices tracked in inventory and procedure execution.
-- A row exists only when the same physical asset identity is valid as both a
-- camera and a geopositioning device.
CREATE TABLE phone_devices (
	-- Stable device identifier shared with parent camera/geopositioning devices.
	id UUID PRIMARY KEY,
	-- Enforces camera-device ancestry for every phone device.
	CONSTRAINT phone_devices_camera
		FOREIGN KEY (id) REFERENCES cameras (id) ON DELETE CASCADE,
	-- Enforces geopositioning-device ancestry for every phone device.
	CONSTRAINT phone_devices_geopositioning
		FOREIGN KEY (id) REFERENCES geopositioning_devices (id) ON DELETE CASCADE
);
-- Registers this table name so APS entities can reference this concrete physical asset type.
INSERT INTO table_names (id) VALUES ('phone_devices') ON CONFLICT DO NOTHING;
