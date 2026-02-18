-- ============================================================================
-- Migration: Shared Assets / Devices / Freeze Dryers
-- Purpose:
--   Adds model, commercial model, commercial lot, and physical asset tables for
--   freeze dryers used for lyophilization and drying.
--
-- APS placement (inheritance and specialization):
--
--   entities
--     +-- ownables
--         +-- namespaced_ownables
--             +-- asset_models
--             |   +-- physical_asset_models
--             |   |   +-- freeze_dryer_models
--             |   +-- commercial_products
--             |   |   +-- commercial_freeze_dryer_models
--             |   +-- commercial_product_lots
--             |       +-- commercial_freeze_dryer_lots
--             +-- assets
--                 +-- physical_assets
--                     +-- freeze_dryers
--
-- Integrity bridge to core APS tables:
--   freeze_dryers (id, commercial_freeze_dryer_lot_id) references assets (id, model_id) to enforce
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
--   - freeze_dryer_models:
--       id -> physical_asset_models.id -> asset_models.id
--          -> namespaced_ownables.id -> ownables.id
--   - commercial_freeze_dryer_models:
--       id -> commercial_products.id -> asset_models.id
--          -> namespaced_ownables.id -> ownables.id
--   - commercial_freeze_dryer_lots:
--       id -> commercial_product_lots.id -> physical_asset_models.id
--          -> asset_models.id -> namespaced_ownables.id -> ownables.id
--   - freeze_dryers:
--       id -> physical_assets.id -> assets.id
--          -> namespaced_ownables.id -> ownables.id
--
--   SQL/RLS semantics for these specific freeze dryers tables:
--   - SELECT: viewer-or-higher on ancestor owner (role >= 2).
--   - INSERT: editor-or-higher on ancestor owner (role >= 3).
--   - UPDATE: editor-or-higher on ancestor owner (role >= 3).
--   - DELETE: admin on ancestor owner (role >= 4).
--   - Roles are resolved with get_owner_role(...).
--
--   Zanzibar semantics for these specific freeze dryers tables:
--   - Equivalent checks can be modeled on ownable:{id} relations.
--   - Team membership and grants are graph traversal
--     (user -> team -> owner, plus explicit grants).
--   - Zanzibar syntax expresses relation intent; SQL/RLS enforces row access.
-- ============================================================================

-- Catalog of freeze dryer model definitions.
CREATE TABLE freeze_dryer_models (
	-- Stable model identifier inherited from physical_asset_models.
	id UUID PRIMARY KEY REFERENCES physical_asset_models (id) ON DELETE CASCADE
);
-- Registers the freeze_dryer_models table name for APS type lookup.
INSERT INTO table_names (id) VALUES ('freeze_dryer_models') ON CONFLICT DO NOTHING;

-- Catalog of commercial freeze dryer models.
CREATE TABLE commercial_freeze_dryer_models (
	-- Stable commercial model identifier shared with parent model tables.
	id UUID PRIMARY KEY,
	-- Base freeze dryer model represented by this commercial model.
	freeze_dryer_model_id UUID NOT NULL REFERENCES freeze_dryer_models(id) ON DELETE CASCADE,
	-- Enforces inheritance from freeze_dryer_models.
	FOREIGN KEY (id) REFERENCES freeze_dryer_models(id) ON DELETE CASCADE,
	-- Enforces inheritance from commercial_products.
	FOREIGN KEY (id) REFERENCES commercial_products(id) ON DELETE CASCADE,
	-- Ensures parent linkage in asset_models matches the base model.
	FOREIGN KEY (id, freeze_dryer_model_id) REFERENCES asset_models(id, parent_model_id)
);
-- Registers the commercial_freeze_dryer_models table name for APS lookup.
INSERT INTO table_names (id) VALUES ('commercial_freeze_dryer_models') ON CONFLICT DO NOTHING;

-- Catalog of lot-specific commercial freeze dryer models.
CREATE TABLE commercial_freeze_dryer_lots (
	-- Stable lot-model identifier shared with parent lot/model tables.
	id UUID PRIMARY KEY,
	-- Enforces inheritance from commercial_product_lots.
	FOREIGN KEY (id) REFERENCES commercial_product_lots(id) ON DELETE CASCADE,
	-- Enforces inheritance from freeze_dryer_models.
	FOREIGN KEY (id) REFERENCES freeze_dryer_models(id) ON DELETE CASCADE,
	-- Commercial freeze dryer model from which this lot derives.
	commercial_freeze_dryer_model_id UUID NOT NULL REFERENCES commercial_freeze_dryer_models(id),
	-- Ensures parent linkage in asset_models matches the commercial model.
	FOREIGN KEY (id, commercial_freeze_dryer_model_id) REFERENCES asset_models(id, parent_model_id)
);
-- Registers the commercial_freeze_dryer_lots table name for APS lookup.
INSERT INTO table_names (id) VALUES ('commercial_freeze_dryer_lots') ON CONFLICT DO NOTHING;

-- Physical freeze dryers tracked in APS inventory.
CREATE TABLE freeze_dryers (
	-- Stable asset identifier inherited from physical_assets.
	id UUID PRIMARY KEY REFERENCES physical_assets (id) ON DELETE CASCADE,
	-- Commercial lot model instantiated by this physical asset.
	commercial_freeze_dryer_lot_id UUID NOT NULL REFERENCES commercial_freeze_dryer_lots (id),
	-- Ensures assets.model_id matches the declared commercial lot model.
	FOREIGN KEY (id, commercial_freeze_dryer_lot_id) REFERENCES assets (id, model_id)
);
-- Registers the freeze_dryers table name for APS type lookup.
INSERT INTO table_names (id) VALUES ('freeze_dryers') ON CONFLICT DO NOTHING;
