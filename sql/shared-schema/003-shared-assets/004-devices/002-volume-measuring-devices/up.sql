-- ============================================================================
-- Migration: Shared Assets / Devices / Volume Measuring Devices
-- Purpose:
--   Adds model, commercial model, commercial lot, and physical asset tables for
--   devices used to measure liquid or gas volume in APS procedures.
--
-- APS placement (inheritance and specialization):
--
--   entities
--     +-- ownables
--         +-- namespaced_ownables
--             +-- asset_models
--             |   +-- physical_asset_models
--             |   |   +-- volume_measuring_device_models
--             |   +-- commercial_products
--             |   |   +-- commercial_volume_measuring_device_models
--             |   +-- commercial_product_lots
--             |       +-- commercial_volume_measuring_device_lots
--             +-- assets
--                 +-- physical_assets
--                     +-- volume_measuring_devices
--
-- Integrity bridge to core APS tables:
--   `volume_measuring_devices (id, volume_measuring_device_model_id)` references
--   `assets (id, model_id)` to prevent model/asset mismatches.
--
-- Metadata registration:
--   Every created table is registered in `table_names`.
--
-- Security context (local, inherited through ancestor PK extension):
--   No table-local RLS is defined here. Access is inherited from ancestor
--   ownables through PK/FK identity chains.
--
--   PK-extension chains used for inherited access checks:
--   - `volume_measuring_device_models`:
--       id -> physical_asset_models.id -> asset_models.id
--          -> namespaced_ownables.id -> ownables.id
--   - `commercial_volume_measuring_device_models`:
--       id -> commercial_products.id -> asset_models.id
--          -> namespaced_ownables.id -> ownables.id
--   - `commercial_volume_measuring_device_lots`:
--       id -> commercial_product_lots.id -> physical_asset_models.id
--          -> asset_models.id -> namespaced_ownables.id -> ownables.id
--   - `volume_measuring_devices`:
--       id -> physical_assets.id -> assets.id
--          -> namespaced_ownables.id -> ownables.id
--
--   SQL/RLS semantics for these specific volume-measuring tables:
--   - `SELECT`: viewer-or-higher on ancestor owner (role >= 2).
--   - `INSERT`: editor-or-higher on ancestor owner (role >= 3).
--   - `UPDATE`: editor-or-higher on ancestor owner (role >= 3).
--   - `DELETE`: admin on ancestor owner (role >= 4).
--   - Roles are resolved with `get_owner_role(...)`.
--
--   Zanzibar semantics for these specific volume-measuring tables:
--   - Equivalent access can be modeled as relation checks on `ownable:{id}`.
--   - Team membership and grants are relation traversal
--     (user -> team -> owner, plus explicit grants).
--   - Zanzibar syntax expresses relation intent; SQL/RLS enforces row access.
-- ============================================================================

-- Catalog of volume-measuring device model definitions.
CREATE TABLE volume_measuring_device_models (
	-- Stable model identifier inherited from `physical_asset_models`.
	id UUID PRIMARY KEY REFERENCES physical_asset_models (id) ON DELETE CASCADE
);
-- Registers the `volume_measuring_device_models` table name for APS lookup.
INSERT INTO table_names (id) VALUES ('volume_measuring_device_models') ON CONFLICT DO NOTHING;

-- Catalog of commercial volume-measuring device models.
CREATE TABLE commercial_volume_measuring_device_models (
	-- Stable commercial model identifier shared with parent model tables.
	id UUID PRIMARY KEY,
	-- Base volume-measuring model represented by this commercial model.
	volume_measuring_device_model_id UUID NOT NULL REFERENCES volume_measuring_device_models(id) ON DELETE CASCADE,
	-- Enforces inheritance from `volume_measuring_device_models`.
	FOREIGN KEY (id) REFERENCES volume_measuring_device_models(id) ON DELETE CASCADE,
	-- Enforces inheritance from `commercial_products`.
	FOREIGN KEY (id) REFERENCES commercial_products(id) ON DELETE CASCADE,
	-- Ensures parent linkage in `asset_models` matches the base model.
	FOREIGN KEY (id, volume_measuring_device_model_id) REFERENCES asset_models(id, parent_model_id)
);
-- Registers the `commercial_volume_measuring_device_models` table name for APS lookup.
INSERT INTO table_names (id) VALUES ('commercial_volume_measuring_device_models') ON CONFLICT DO NOTHING;

-- Catalog of lot-specific commercial volume-measuring device models.
CREATE TABLE commercial_volume_measuring_device_lots (
	-- Stable lot-model identifier shared with parent lot/model tables.
	id UUID PRIMARY KEY,
	-- Enforces inheritance from `commercial_product_lots`.
	FOREIGN KEY (id) REFERENCES commercial_product_lots(id) ON DELETE CASCADE,
	-- Enforces inheritance from `volume_measuring_device_models`.
	FOREIGN KEY (id) REFERENCES volume_measuring_device_models(id) ON DELETE CASCADE,
	-- Commercial volume-measuring model from which this lot derives.
	commercial_volume_measuring_device_model_id UUID NOT NULL REFERENCES commercial_volume_measuring_device_models(id),
	-- Ensures parent linkage in `asset_models` matches the commercial model.
	FOREIGN KEY (id, commercial_volume_measuring_device_model_id) REFERENCES asset_models(id, parent_model_id)
);
-- Registers the `commercial_volume_measuring_device_lots` table name for APS lookup.
INSERT INTO table_names (id) VALUES ('commercial_volume_measuring_device_lots') ON CONFLICT DO NOTHING;

-- Physical volume-measuring devices tracked in APS inventory.
CREATE TABLE volume_measuring_devices (
	-- Stable asset identifier inherited from `physical_assets`.
	id UUID PRIMARY KEY REFERENCES physical_assets (id) ON DELETE CASCADE,
	-- Volume-measuring model instantiated by this physical asset.
	volume_measuring_device_model_id UUID NOT NULL REFERENCES volume_measuring_device_models (id),
	-- Ensures `assets.model_id` matches the declared volume-measuring model.
	FOREIGN KEY (id, volume_measuring_device_model_id) REFERENCES assets (id, model_id)
);
-- Registers the `volume_measuring_devices` table name for APS type lookup.
INSERT INTO table_names (id) VALUES ('volume_measuring_devices') ON CONFLICT DO NOTHING;
