-- ============================================================================
-- Migration: Shared Assets / Devices / Ball Mill Machines
-- Purpose:
--   Adds model, commercial model, commercial lot, and physical asset tables for
--   ball mill machines used in material grinding workflows.
--
-- APS placement (inheritance and specialization):
--
--   entities
--     +-- ownables
--         +-- namespaced_ownables
--             +-- asset_models
--             |   +-- physical_asset_models
--             |   |   +-- ball_mill_machine_models
--             |   +-- commercial_products
--             |   |   +-- commercial_ball_mill_machine_models
--             |   +-- commercial_product_lots
--             |       +-- commercial_ball_mill_machine_lots
--             +-- assets
--                 +-- physical_assets
--                     +-- ball_mill_machines
--
-- Integrity bridge to core APS tables:
--   `ball_mill_machines (id, commercial_ball_mill_machine_lot_id)` references
--   `assets (id, model_id)` to enforce model/asset consistency.
--
-- Metadata registration:
--   Every created table is registered in `table_names`.
--
-- Security context (local, inherited through ancestor PK extension):
--   No table-local RLS is defined here. Access is inherited from ancestor
--   ownables through PK/FK identity chains.
--
--   PK-extension chains used for inherited access checks:
--   - `ball_mill_machine_models`:
--       id -> physical_asset_models.id -> asset_models.id
--          -> namespaced_ownables.id -> ownables.id
--   - `commercial_ball_mill_machine_models`:
--       id -> commercial_products.id -> asset_models.id
--          -> namespaced_ownables.id -> ownables.id
--   - `commercial_ball_mill_machine_lots`:
--       id -> commercial_product_lots.id -> physical_asset_models.id
--          -> asset_models.id -> namespaced_ownables.id -> ownables.id
--   - `ball_mill_machines`:
--       id -> physical_assets.id -> assets.id
--          -> namespaced_ownables.id -> ownables.id
--
--   SQL/RLS semantics for these specific ball-mill tables:
--   - `SELECT`: viewer-or-higher on ancestor owner (role >= 2).
--   - `INSERT`: editor-or-higher on ancestor owner (role >= 3).
--   - `UPDATE`: editor-or-higher on ancestor owner (role >= 3).
--   - `DELETE`: admin on ancestor owner (role >= 4).
--   - Roles are resolved with `get_owner_role(...)`.
--
--   Zanzibar semantics for these specific ball-mill tables:
--   - Equivalent checks can be modeled on `ownable:{id}` relations.
--   - Team and grant inheritance is graph traversal
--     (user -> team -> owner, plus explicit grants).
--   - Zanzibar syntax expresses relation intent; SQL/RLS enforces row access.
-- ============================================================================

-- Catalog of ball mill machine model definitions.
CREATE TABLE ball_mill_machine_models (
	-- Stable model identifier inherited from `physical_asset_models`.
	id UUID PRIMARY KEY REFERENCES physical_asset_models (id) ON DELETE CASCADE
);
-- Registers the `ball_mill_machine_models` table name for APS type lookup.
INSERT INTO table_names (id) VALUES ('ball_mill_machine_models') ON CONFLICT DO NOTHING;

-- Catalog of commercial ball mill machine models.
CREATE TABLE commercial_ball_mill_machine_models (
	-- Stable commercial model identifier shared with parent model tables.
	id UUID PRIMARY KEY,
	-- Base ball-mill model represented by this commercial model.
	ball_mill_machine_model_id UUID NOT NULL REFERENCES ball_mill_machine_models(id) ON DELETE CASCADE,
	-- Enforces inheritance from `ball_mill_machine_models`.
	FOREIGN KEY (id) REFERENCES ball_mill_machine_models(id) ON DELETE CASCADE,
	-- Enforces inheritance from `commercial_products`.
	FOREIGN KEY (id) REFERENCES commercial_products(id) ON DELETE CASCADE,
	-- Ensures parent linkage in `asset_models` matches the base model.
	FOREIGN KEY (id, ball_mill_machine_model_id) REFERENCES asset_models(id, parent_model_id)
);
-- Registers the `commercial_ball_mill_machine_models` table name for APS lookup.
INSERT INTO table_names (id) VALUES ('commercial_ball_mill_machine_models') ON CONFLICT DO NOTHING;

-- Catalog of lot-specific commercial ball mill machine models.
CREATE TABLE commercial_ball_mill_machine_lots (
	-- Stable lot-model identifier shared with parent lot/model tables.
	id UUID PRIMARY KEY,
	-- Enforces inheritance from `commercial_product_lots`.
	FOREIGN KEY (id) REFERENCES commercial_product_lots(id) ON DELETE CASCADE,
	-- Enforces inheritance from `ball_mill_machine_models`.
	FOREIGN KEY (id) REFERENCES ball_mill_machine_models(id) ON DELETE CASCADE,
	-- Commercial ball-mill model from which this lot derives.
	commercial_ball_mill_machine_model_id UUID NOT NULL REFERENCES commercial_ball_mill_machine_models(id),
	-- Ensures parent linkage in `asset_models` matches the commercial model.
	FOREIGN KEY (id, commercial_ball_mill_machine_model_id) REFERENCES asset_models(id, parent_model_id)
);
-- Registers the `commercial_ball_mill_machine_lots` table name for APS lookup.
INSERT INTO table_names (id) VALUES ('commercial_ball_mill_machine_lots') ON CONFLICT DO NOTHING;

-- Physical ball mill machines tracked in APS inventory.
CREATE TABLE ball_mill_machines (
	-- Stable asset identifier inherited from `physical_assets`.
	id UUID PRIMARY KEY REFERENCES physical_assets (id) ON DELETE CASCADE,
	-- Commercial lot model instantiated by this physical machine.
	commercial_ball_mill_machine_lot_id UUID NOT NULL REFERENCES commercial_ball_mill_machine_lots (id),
	-- Ensures `assets.model_id` matches the declared commercial lot model.
	FOREIGN KEY (id, commercial_ball_mill_machine_lot_id) REFERENCES assets (id, model_id)
);
-- Registers the `ball_mill_machines` table name for APS type lookup.
INSERT INTO table_names (id) VALUES ('ball_mill_machines') ON CONFLICT DO NOTHING;
