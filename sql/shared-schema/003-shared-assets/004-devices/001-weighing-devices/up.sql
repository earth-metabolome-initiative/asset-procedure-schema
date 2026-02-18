-- ============================================================================
-- Migration: Shared Assets / Devices / Weighing Devices
-- Purpose:
--   Adds model, commercial model, commercial lot, and physical asset tables for
--   weighing devices used to measure mass during APS procedures.
--
-- APS placement (inheritance and specialization):
--
--   entities
--     +-- ownables
--         +-- namespaced_ownables
--             +-- asset_models
--             |   +-- physical_asset_models
--             |   |   +-- weighing_device_models
--             |   +-- commercial_products
--             |   |   +-- commercial_weighing_device_models
--             |   +-- commercial_product_lots
--             |       +-- commercial_weighing_device_lots
--             +-- assets
--                 +-- physical_assets
--                     +-- weighing_devices
--
-- Integrity bridge to core APS tables:
--   `weighing_devices (id, weighing_device_model_id)` references
--   `assets (id, model_id)` to guarantee model consistency.
--
-- Metadata registration:
--   Every created table is registered in `table_names`.
--
-- Security context (local, inherited through ancestor PK extension):
--   No table-local RLS is defined in this migration. Access is inherited from
--   ancestor ownables through PK/FK identity chains.
--
--   PK-extension chains used for inherited access checks:
--   - `weighing_device_models`:
--       id -> physical_asset_models.id -> asset_models.id
--          -> namespaced_ownables.id -> ownables.id
--   - `commercial_weighing_device_models`:
--       id -> commercial_products.id -> asset_models.id
--          -> namespaced_ownables.id -> ownables.id
--   - `commercial_weighing_device_lots`:
--       id -> commercial_product_lots.id -> physical_asset_models.id
--          -> asset_models.id -> namespaced_ownables.id -> ownables.id
--   - `weighing_devices`:
--       id -> physical_assets.id -> assets.id
--          -> namespaced_ownables.id -> ownables.id
--
--   SQL/RLS semantics for these specific weighing-device tables:
--   - `SELECT`: viewer-or-higher on ancestor owner (role >= 2).
--   - `INSERT`: editor-or-higher on ancestor owner (role >= 3).
--   - `UPDATE`: editor-or-higher on ancestor owner (role >= 3).
--   - `DELETE`: admin on ancestor owner (role >= 4).
--   - Roles are resolved with `get_owner_role(...)`.
--
--   Zanzibar semantics for these specific weighing-device tables:
--   - Equivalent access can be modeled as relation checks on `ownable:{id}`.
--   - Grants and team membership are graph traversal
--     (user -> team -> owner, plus explicit grants).
--   - Zanzibar syntax expresses relation intent; SQL/RLS enforces row access.
-- ============================================================================

-- Catalog of weighing-device model definitions.
CREATE TABLE weighing_device_models (
	-- Stable model identifier inherited from `physical_asset_models`.
	id UUID PRIMARY KEY REFERENCES physical_asset_models (id) ON DELETE CASCADE
);
-- Registers the `weighing_device_models` table name for APS type lookup.
INSERT INTO table_names (id) VALUES ('weighing_device_models') ON CONFLICT DO NOTHING;

-- Catalog of commercial weighing-device models.
CREATE TABLE commercial_weighing_device_models (
	-- Stable commercial model identifier shared with parent model tables.
	id UUID PRIMARY KEY,
	-- Base weighing-device model represented by this commercial model.
	weighing_device_model_id UUID NOT NULL REFERENCES weighing_device_models(id) ON DELETE CASCADE,
	-- Enforces inheritance from `weighing_device_models`.
	FOREIGN KEY (id) REFERENCES weighing_device_models(id) ON DELETE CASCADE,
	-- Enforces inheritance from `commercial_products`.
	FOREIGN KEY (id) REFERENCES commercial_products(id) ON DELETE CASCADE,
	-- Ensures parent linkage in `asset_models` matches `weighing_device_model_id`.
	FOREIGN KEY (id, weighing_device_model_id) REFERENCES asset_models(id, parent_model_id)
);
-- Registers the `commercial_weighing_device_models` table name for APS lookup.
INSERT INTO table_names (id) VALUES ('commercial_weighing_device_models') ON CONFLICT DO NOTHING;

-- Catalog of commercial lot-specific weighing-device models.
CREATE TABLE commercial_weighing_device_lots (
	-- Stable lot-model identifier shared with parent lot/model tables.
	id UUID PRIMARY KEY,
	-- Enforces inheritance from `commercial_product_lots`.
	FOREIGN KEY (id) REFERENCES commercial_product_lots(id) ON DELETE CASCADE,
	-- Enforces inheritance from `weighing_device_models`.
	FOREIGN KEY (id) REFERENCES weighing_device_models(id) ON DELETE CASCADE,
	-- Commercial weighing-device model from which this lot derives.
	commercial_weighing_device_model_id UUID NOT NULL REFERENCES commercial_weighing_device_models(id),
	-- Ensures parent linkage in `asset_models` matches the commercial model.
	FOREIGN KEY (id, commercial_weighing_device_model_id) REFERENCES asset_models(id, parent_model_id)
);
-- Registers the `commercial_weighing_device_lots` table name for APS lookup.
INSERT INTO table_names (id) VALUES ('commercial_weighing_device_lots') ON CONFLICT DO NOTHING;

-- Physical weighing devices tracked in APS inventory.
CREATE TABLE weighing_devices (
	-- Stable asset identifier inherited from `physical_assets`.
	id UUID PRIMARY KEY REFERENCES physical_assets (id) ON DELETE CASCADE,
	-- Weighing-device model instantiated by this physical asset.
	weighing_device_model_id UUID NOT NULL REFERENCES weighing_device_models (id),
	-- Ensures `assets.model_id` is exactly the declared weighing-device model.
	FOREIGN KEY (id, weighing_device_model_id) REFERENCES assets (id, model_id)
);
-- Registers the `weighing_devices` table name for APS type lookup.
INSERT INTO table_names (id) VALUES ('weighing_devices') ON CONFLICT DO NOTHING;
