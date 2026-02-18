-- ============================================================================
-- Migration: Shared Assets / Devices / Cameras
-- Purpose:
--   Adds model, commercial model, commercial lot, and physical asset tables for
--   cameras used for visual capture and inspection.
--
-- APS placement (inheritance and specialization):
--
--   entities
--     +-- ownables
--         +-- namespaced_ownables
--             +-- asset_models
--             |   +-- physical_asset_models
--             |   |   +-- camera_models
--             |   +-- commercial_products
--             |   |   +-- commercial_camera_models
--             |   +-- commercial_product_lots
--             |       +-- commercial_camera_lots
--             +-- assets
--                 +-- physical_assets
--                     +-- cameras
--
-- Integrity bridge to core APS tables:
--   cameras (id, commercial_camera_lot_id) references assets (id, model_id) to enforce
--   model/asset consistency.
--
-- Metadata registration:
--   Every created table is registered in table_names.
--
-- Security context (local, inherited through ancestor PK extension):
--   No table-local RLS is defined here. Access is inherited from ancestor
--   ownables through PK/FK identity chains.
--
--   PK-extension chains used for inherited access checks:
--   - camera_models:
--       id -> physical_asset_models.id -> asset_models.id
--          -> namespaced_ownables.id -> ownables.id
--   - commercial_camera_models:
--       id -> commercial_products.id -> asset_models.id
--          -> namespaced_ownables.id -> ownables.id
--   - commercial_camera_lots:
--       id -> commercial_product_lots.id -> physical_asset_models.id
--          -> asset_models.id -> namespaced_ownables.id -> ownables.id
--   - cameras:
--       id -> physical_assets.id -> assets.id
--          -> namespaced_ownables.id -> ownables.id
--
--   SQL/RLS semantics for these specific cameras tables:
--   - SELECT: viewer-or-higher on ancestor owner (role >= 2).
--   - INSERT: editor-or-higher on ancestor owner (role >= 3).
--   - UPDATE: editor-or-higher on ancestor owner (role >= 3).
--   - DELETE: admin on ancestor owner (role >= 4).
--   - Roles are resolved with get_owner_role(...).
--
--   Zanzibar semantics for these specific cameras tables:
--   - Equivalent checks can be modeled on ownable:{id} relations.
--   - Team membership and grants are graph traversal
--     (user -> team -> owner, plus explicit grants).
--   - Zanzibar syntax expresses relation intent; SQL/RLS enforces row access.
-- ============================================================================

-- Catalog of camera model definitions.
CREATE TABLE camera_models (
	-- Stable model identifier inherited from physical_asset_models.
	id UUID PRIMARY KEY REFERENCES physical_asset_models (id) ON DELETE CASCADE
);
-- Registers the camera_models table name for APS type lookup.
INSERT INTO table_names (id) VALUES ('camera_models') ON CONFLICT DO NOTHING;

-- Catalog of commercial camera models.
CREATE TABLE commercial_camera_models (
	-- Stable commercial model identifier shared with parent model tables.
	id UUID PRIMARY KEY,
	-- Base camera model represented by this commercial model.
	camera_model_id UUID NOT NULL REFERENCES camera_models(id) ON DELETE CASCADE,
	-- Enforces inheritance from camera_models.
	FOREIGN KEY (id) REFERENCES camera_models(id) ON DELETE CASCADE,
	-- Enforces inheritance from commercial_products.
	FOREIGN KEY (id) REFERENCES commercial_products(id) ON DELETE CASCADE,
	-- Ensures parent linkage in asset_models matches the base model.
	FOREIGN KEY (id, camera_model_id) REFERENCES asset_models(id, parent_model_id)
);
-- Registers the commercial_camera_models table name for APS lookup.
INSERT INTO table_names (id) VALUES ('commercial_camera_models') ON CONFLICT DO NOTHING;

-- Catalog of lot-specific commercial camera models.
CREATE TABLE commercial_camera_lots (
	-- Stable lot-model identifier shared with parent lot/model tables.
	id UUID PRIMARY KEY,
	-- Enforces inheritance from commercial_product_lots.
	FOREIGN KEY (id) REFERENCES commercial_product_lots(id) ON DELETE CASCADE,
	-- Enforces inheritance from camera_models.
	FOREIGN KEY (id) REFERENCES camera_models(id) ON DELETE CASCADE,
	-- Commercial camera model from which this lot derives.
	commercial_camera_model_id UUID NOT NULL REFERENCES commercial_camera_models(id),
	-- Ensures parent linkage in asset_models matches the commercial model.
	FOREIGN KEY (id, commercial_camera_model_id) REFERENCES asset_models(id, parent_model_id)
);
-- Registers the commercial_camera_lots table name for APS lookup.
INSERT INTO table_names (id) VALUES ('commercial_camera_lots') ON CONFLICT DO NOTHING;

-- Physical cameras tracked in APS inventory.
CREATE TABLE cameras (
	-- Stable asset identifier inherited from physical_assets.
	id UUID PRIMARY KEY REFERENCES physical_assets (id) ON DELETE CASCADE,
	-- Commercial lot model instantiated by this physical asset.
	commercial_camera_lot_id UUID NOT NULL REFERENCES commercial_camera_lots (id),
	-- Ensures assets.model_id matches the declared commercial lot model.
	FOREIGN KEY (id, commercial_camera_lot_id) REFERENCES assets (id, model_id)
);
-- Registers the cameras table name for APS type lookup.
INSERT INTO table_names (id) VALUES ('cameras') ON CONFLICT DO NOTHING;
