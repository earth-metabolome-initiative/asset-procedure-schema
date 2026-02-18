-- ============================================================================
-- Migration: Shared Assets / Marking Assets / Panels
-- Purpose:
--   Adds panel model and asset specializations for durable signage and
--   identification artifacts used to communicate or anchor contextual metadata.
--
-- APS placement (inheritance and specialization):
--
--   entities
--     +-- ownables
--         +-- namespaced_ownables
--             +-- asset_models
--             |   +-- physical_asset_models
--             |   |   +-- panel_models
--             +-- assets
--                 +-- physical_assets
--                     +-- panels
--
-- Integrity bridge to core APS tables:
--   panels (id, panel_model_id)
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
--   - panel_models:
--       id -> physical_asset_models.id -> asset_models.id
--          -> namespaced_ownables.id -> ownables.id
--   - panels:
--       id -> physical_assets.id -> assets.id
--          -> namespaced_ownables.id -> ownables.id
--
--   SQL/RLS semantics for these specific panel tables:
--   - SELECT: viewer-or-higher on ancestor owner (role >= 2).
--   - INSERT: editor-or-higher on ancestor owner (role >= 3).
--   - UPDATE: editor-or-higher on ancestor owner (role >= 3).
--   - DELETE: admin on ancestor owner (role >= 4).
--   - Roles are resolved with get_owner_role(...).
--
--   Zanzibar semantics for these specific panel tables:
--   - Equivalent checks can be modeled on ownable:{id} relations.
--   - Team membership and grants are graph traversal
--     (user -> team -> owner, plus explicit grants).
--   - Zanzibar syntax expresses relation intent; SQL/RLS enforces row access.
-- ============================================================================

-- Catalog of panel model definitions.
CREATE TABLE panel_models (
	-- Stable model identifier inherited from physical_asset_models.
	id UUID PRIMARY KEY REFERENCES physical_asset_models (id) ON DELETE CASCADE
);
-- Registers the panel_models table name for APS lookup.
INSERT INTO table_names (id) VALUES ('panel_models') ON CONFLICT DO NOTHING;

-- Physical panels tracked in APS inventory.
CREATE TABLE panels (
	-- Stable asset identifier inherited from physical_assets.
	id UUID PRIMARY KEY REFERENCES physical_assets (id) ON DELETE CASCADE,
	-- Panel model instantiated by this physical asset.
	panel_model_id UUID NOT NULL REFERENCES panel_models (id),
	-- Ensures assets.model_id matches the declared panel model.
	FOREIGN KEY (id, panel_model_id) REFERENCES assets (id, model_id)
);
-- Registers the panels table name for APS type lookup.
INSERT INTO table_names (id) VALUES ('panels') ON CONFLICT DO NOTHING;
