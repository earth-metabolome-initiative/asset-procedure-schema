-- ============================================================================
-- Migration: Shared Assets / Marking Assets / Markers
-- Purpose:
--   Adds marker model and asset specializations for disposable or reusable
--   identification artifacts used to mark assets, organisms, or locations.
--
-- APS placement (inheritance and specialization):
--
--   entities
--     +-- ownables
--         +-- namespaced_ownables
--             +-- asset_models
--             |   +-- physical_asset_models
--             |   |   +-- marker_models
--             |   +-- commercial_products
--             |   |   +-- commercial_marker_models
--             |   +-- commercial_product_lots
--             |       +-- commercial_marker_lots
--             +-- assets
--                 +-- physical_assets
--                     +-- markers
--
-- Integrity bridge to core APS tables:
--   markers (id, marker_model_id)
--   references assets (id, model_id) to enforce model/asset consistency.
--
-- Metadata registration:
--   Every created table is registered in table_names.
--
-- Security context (local, inherited through ancestor PK extension):
--   No table-local RLS is defined in this migration. Access is inherited from
--   ancestor ownables through PK/FK identity chains.
--
--   PK-extension chains used for inherited access checks:
--   - marker_models:
--       id -> physical_asset_models.id -> asset_models.id
--          -> namespaced_ownables.id -> ownables.id
--   - commercial_marker_models:
--       id -> commercial_products.id -> asset_models.id
--          -> namespaced_ownables.id -> ownables.id
--   - commercial_marker_lots:
--       id -> commercial_product_lots.id -> physical_asset_models.id
--          -> asset_models.id -> namespaced_ownables.id -> ownables.id
--   - markers:
--       id -> physical_assets.id -> assets.id
--          -> namespaced_ownables.id -> ownables.id
--
--   SQL/RLS semantics for these specific marker tables:
--   - SELECT: viewer-or-higher on ancestor owner (role >= 2).
--   - INSERT: editor-or-higher on ancestor owner (role >= 3).
--   - UPDATE: editor-or-higher on ancestor owner (role >= 3).
--   - DELETE: admin on ancestor owner (role >= 4).
--   - Roles are resolved with get_owner_role(...).
--
--   Zanzibar semantics for these specific marker tables:
--   - Equivalent checks can be modeled on ownable:{id} relations.
--   - Team membership and grants are graph traversal
--     (user -> team -> owner, plus explicit grants).
--   - Zanzibar syntax expresses relation intent; SQL/RLS enforces row access.
-- ============================================================================

-- Catalog of marker model definitions.
CREATE TABLE marker_models (
	-- Stable model identifier inherited from physical_asset_models.
	id UUID PRIMARY KEY REFERENCES physical_asset_models (id) ON DELETE CASCADE
);
-- Registers the marker_models table name for APS lookup.
INSERT INTO table_names (id) VALUES ('marker_models') ON CONFLICT DO NOTHING;

-- Catalog of commercial marker models.
CREATE TABLE commercial_marker_models (
	-- Stable commercial model identifier shared with parent model tables.
	id UUID PRIMARY KEY,
	-- Base marker model represented by this commercial model.
	marker_model_id UUID NOT NULL REFERENCES marker_models(id) ON DELETE CASCADE,
	-- Enforces inheritance from marker_models.
	FOREIGN KEY (id) REFERENCES marker_models(id) ON DELETE CASCADE,
	-- Enforces inheritance from commercial_products.
	FOREIGN KEY (id) REFERENCES commercial_products(id) ON DELETE CASCADE,
	-- Ensures parent linkage in asset_models matches the base marker model.
	FOREIGN KEY (id, marker_model_id) REFERENCES asset_models(id, parent_model_id)
);
-- Registers the commercial_marker_models table name for APS lookup.
INSERT INTO table_names (id) VALUES ('commercial_marker_models') ON CONFLICT DO NOTHING;

-- Catalog of lot-specific commercial marker models.
CREATE TABLE commercial_marker_lots (
	-- Stable lot-model identifier shared with parent lot/model tables.
	id UUID PRIMARY KEY,
	-- Enforces inheritance from commercial_product_lots.
	FOREIGN KEY (id) REFERENCES commercial_product_lots(id) ON DELETE CASCADE,
	-- Enforces inheritance from marker_models.
	FOREIGN KEY (id) REFERENCES marker_models(id) ON DELETE CASCADE,
	-- Commercial marker model from which this lot derives.
	commercial_marker_model_id UUID NOT NULL REFERENCES commercial_marker_models(id),
	-- Ensures parent linkage in asset_models matches the commercial model.
	FOREIGN KEY (id, commercial_marker_model_id) REFERENCES asset_models(id, parent_model_id)
);
-- Registers the commercial_marker_lots table name for APS lookup.
INSERT INTO table_names (id) VALUES ('commercial_marker_lots') ON CONFLICT DO NOTHING;

-- Physical markers tracked in APS inventory.
CREATE TABLE markers (
	-- Stable asset identifier inherited from physical_assets.
	id UUID PRIMARY KEY REFERENCES physical_assets (id) ON DELETE CASCADE,
	-- Marker model instantiated by this physical asset.
	marker_model_id UUID NOT NULL REFERENCES marker_models (id),
	-- Ensures assets.model_id matches the declared marker model.
	FOREIGN KEY (id, marker_model_id) REFERENCES assets (id, model_id)
);
-- Registers the markers table name for APS type lookup.
INSERT INTO table_names (id) VALUES ('markers') ON CONFLICT DO NOTHING;
