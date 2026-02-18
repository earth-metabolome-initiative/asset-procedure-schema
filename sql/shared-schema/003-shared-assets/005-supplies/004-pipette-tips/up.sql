-- ============================================================================
-- Migration: Shared Assets / Supplies / Pipette Tips
-- Purpose:
--   Adds model, commercial model, commercial lot, and physical asset tables for
--   pipette tips used as single-use supplies in liquid-handling workflows.
--
-- APS placement (inheritance and specialization):
--
--   entities
--     +-- ownables
--         +-- namespaced_ownables
--             +-- asset_models
--             |   +-- physical_asset_models
--             |   |   +-- pipette_tip_models
--             |   +-- commercial_products
--             |   |   +-- commercial_pipette_tip_models
--             |   +-- commercial_product_lots
--             |       +-- commercial_pipette_tip_lots
--             +-- assets
--                 +-- physical_assets
--                     +-- pipette_tips
--
-- Integrity bridge to core APS tables:
--   pipette_tips (id, pipette_tip_model_id)
--   references assets (id, model_id) to enforce model/asset consistency.
--
-- Metadata registration:
--   Every created table is registered in table_names.
--
-- Commercial tables decision:
--   `commercial_pipette_tip_models` and `commercial_pipette_tip_lots` are
--   included to preserve vendor-model and lot-level traceability.
--
-- Lifecycle/reusability note:
--   Pipette tips are expected to be `single_use`, but lifecycle remains
--   authoritative on `physical_asset_models` lifecycle columns.
--   This migration is schema-only and does not override model lifecycle data.
--
-- Security context (local, inherited through ancestor PK extension):
--   No table-local RLS is defined in this migration. Access is inherited from
--   ancestor ownables through PK/FK identity chains.
--
--   PK-extension chains used for inherited access checks:
--   - pipette_tip_models:
--       id -> physical_asset_models.id -> asset_models.id
--          -> namespaced_ownables.id -> ownables.id
--   - commercial_pipette_tip_models:
--       id -> commercial_products.id -> asset_models.id
--          -> namespaced_ownables.id -> ownables.id
--   - commercial_pipette_tip_lots:
--       id -> commercial_product_lots.id -> physical_asset_models.id
--          -> asset_models.id -> namespaced_ownables.id -> ownables.id
--   - pipette_tips:
--       id -> physical_assets.id -> assets.id
--          -> namespaced_ownables.id -> ownables.id
--
--   SQL/RLS semantics for these specific pipette-tip tables:
--   - SELECT: viewer-or-higher on ancestor owner (role >= 2).
--   - INSERT: editor-or-higher on ancestor owner (role >= 3).
--   - UPDATE: editor-or-higher on ancestor owner (role >= 3).
--   - DELETE: admin on ancestor owner (role >= 4).
--   - Roles are resolved with get_owner_role(...).
--
--   Zanzibar semantics for these specific pipette-tip tables:
--   - Equivalent checks can be modeled on ownable:{id} relations.
--   - Team membership and grants are graph traversal
--     (user -> team -> owner, plus explicit grants).
--   - Zanzibar syntax expresses relation intent; SQL/RLS enforces row access.
-- ============================================================================

-- Catalog of pipette tip model definitions.
CREATE TABLE pipette_tip_models (
	-- Stable model identifier inherited from physical_asset_models.
	id UUID PRIMARY KEY REFERENCES physical_asset_models (id) ON DELETE CASCADE
);
-- Registers the pipette_tip_models table name for APS lookup.
INSERT INTO table_names (id) VALUES ('pipette_tip_models') ON CONFLICT DO NOTHING;

-- Catalog of commercial pipette tip models.
CREATE TABLE commercial_pipette_tip_models (
	-- Stable commercial model identifier shared with parent model tables.
	id UUID PRIMARY KEY,
	-- Base pipette tip model represented by this commercial model.
	pipette_tip_model_id UUID NOT NULL REFERENCES pipette_tip_models(id) ON DELETE CASCADE,
	-- Enforces inheritance from pipette_tip_models.
	FOREIGN KEY (id) REFERENCES pipette_tip_models(id) ON DELETE CASCADE,
	-- Enforces inheritance from commercial_products.
	FOREIGN KEY (id) REFERENCES commercial_products(id) ON DELETE CASCADE,
	-- Ensures parent linkage in asset_models matches the base pipette tip model.
	FOREIGN KEY (id, pipette_tip_model_id) REFERENCES asset_models(id, parent_model_id)
);
-- Registers the commercial_pipette_tip_models table name for APS lookup.
INSERT INTO table_names (id) VALUES ('commercial_pipette_tip_models') ON CONFLICT DO NOTHING;

-- Catalog of lot-specific commercial pipette tip models.
CREATE TABLE commercial_pipette_tip_lots (
	-- Stable lot-model identifier shared with parent lot/model tables.
	id UUID PRIMARY KEY,
	-- Enforces inheritance from commercial_product_lots.
	FOREIGN KEY (id) REFERENCES commercial_product_lots(id) ON DELETE CASCADE,
	-- Enforces inheritance from pipette_tip_models.
	FOREIGN KEY (id) REFERENCES pipette_tip_models(id) ON DELETE CASCADE,
	-- Commercial pipette tip model from which this lot derives.
	commercial_pipette_tip_model_id UUID NOT NULL REFERENCES commercial_pipette_tip_models(id),
	-- Ensures parent linkage in asset_models matches the commercial model.
	FOREIGN KEY (id, commercial_pipette_tip_model_id) REFERENCES asset_models(id, parent_model_id)
);
-- Registers the commercial_pipette_tip_lots table name for APS lookup.
INSERT INTO table_names (id) VALUES ('commercial_pipette_tip_lots') ON CONFLICT DO NOTHING;

-- Physical pipette tip assets tracked in APS inventory.
CREATE TABLE pipette_tips (
	-- Stable asset identifier inherited from physical_assets.
	id UUID PRIMARY KEY REFERENCES physical_assets (id) ON DELETE CASCADE,
	-- Pipette tip model instantiated by this physical asset.
	pipette_tip_model_id UUID NOT NULL REFERENCES pipette_tip_models (id),
	-- Ensures assets.model_id matches the declared pipette tip model.
	FOREIGN KEY (id, pipette_tip_model_id) REFERENCES assets (id, model_id)
);
-- Registers the pipette_tips table name for APS type lookup.
INSERT INTO table_names (id) VALUES ('pipette_tips') ON CONFLICT DO NOTHING;
