-- ============================================================================
-- Migration: Shared Assets / Supplies / Personal Protective Equipment
-- Purpose:
--   Adds model, commercial model, commercial lot, and physical asset tables for
--   personal protective equipment used to protect operators during procedures.
--
-- APS placement (inheritance and specialization):
--
--   entities
--     +-- ownables
--         +-- namespaced_ownables
--             +-- asset_models
--             |   +-- physical_asset_models
--             |   |   +-- personal_protective_equipment_models
--             |   +-- commercial_products
--             |   |   +-- commercial_personal_protective_equipment_models
--             |   +-- commercial_product_lots
--             |       +-- commercial_personal_protective_equipment_lots
--             +-- assets
--                 +-- physical_assets
--                     +-- personal_protective_equipments
--
-- Integrity bridge to core APS tables:
--   personal_protective_equipments (id, personal_protective_equipment_model_id)
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
--   - personal_protective_equipment_models:
--       id -> physical_asset_models.id -> asset_models.id
--          -> namespaced_ownables.id -> ownables.id
--   - commercial_personal_protective_equipment_models:
--       id -> commercial_products.id -> asset_models.id
--          -> namespaced_ownables.id -> ownables.id
--   - commercial_personal_protective_equipment_lots:
--       id -> commercial_product_lots.id -> physical_asset_models.id
--          -> asset_models.id -> namespaced_ownables.id -> ownables.id
--   - personal_protective_equipments:
--       id -> physical_assets.id -> assets.id
--          -> namespaced_ownables.id -> ownables.id
--
--   SQL/RLS semantics for these specific personal-protective-equipment tables:
--   - SELECT: viewer-or-higher on ancestor owner (role >= 2).
--   - INSERT: editor-or-higher on ancestor owner (role >= 3).
--   - UPDATE: editor-or-higher on ancestor owner (role >= 3).
--   - DELETE: admin on ancestor owner (role >= 4).
--   - Roles are resolved with get_owner_role(...).
--
--   Zanzibar semantics for these specific personal-protective-equipment tables:
--   - Equivalent checks can be modeled on ownable:{id} relations.
--   - Team membership and grants are graph traversal
--     (user -> team -> owner, plus explicit grants).
--   - Zanzibar syntax expresses relation intent; SQL/RLS enforces row access.
-- ============================================================================

-- Catalog of personal protective equipment model definitions.
CREATE TABLE personal_protective_equipment_models (
	-- Stable model identifier inherited from physical_asset_models.
	id UUID PRIMARY KEY REFERENCES physical_asset_models (id) ON DELETE CASCADE
);
-- Registers the personal_protective_equipment_models table name for APS lookup.
INSERT INTO table_names (id) VALUES ('personal_protective_equipment_models') ON CONFLICT DO NOTHING;

-- Catalog of commercial personal protective equipment models.
CREATE TABLE commercial_personal_protective_equipment_models (
	-- Stable commercial model identifier shared with parent model tables.
	id UUID PRIMARY KEY,
	-- Base PPE model represented by this commercial model.
	personal_protective_equipment_model_id UUID NOT NULL REFERENCES personal_protective_equipment_models(id) ON DELETE CASCADE,
	-- Enforces inheritance from personal_protective_equipment_models.
	FOREIGN KEY (id) REFERENCES personal_protective_equipment_models(id) ON DELETE CASCADE,
	-- Enforces inheritance from commercial_products.
	FOREIGN KEY (id) REFERENCES commercial_products(id) ON DELETE CASCADE,
	-- Ensures parent linkage in asset_models matches the base PPE model.
	FOREIGN KEY (id, personal_protective_equipment_model_id) REFERENCES asset_models(id, parent_model_id)
);
-- Registers the commercial_personal_protective_equipment_models table name for APS lookup.
INSERT INTO table_names (id) VALUES ('commercial_personal_protective_equipment_models') ON CONFLICT DO NOTHING;

-- Catalog of lot-specific commercial personal protective equipment models.
CREATE TABLE commercial_personal_protective_equipment_lots (
	-- Stable lot-model identifier shared with parent lot/model tables.
	id UUID PRIMARY KEY,
	-- Enforces inheritance from commercial_product_lots.
	FOREIGN KEY (id) REFERENCES commercial_product_lots(id) ON DELETE CASCADE,
	-- Enforces inheritance from personal_protective_equipment_models.
	FOREIGN KEY (id) REFERENCES personal_protective_equipment_models(id) ON DELETE CASCADE,
	-- Commercial PPE model from which this lot derives.
	commercial_personal_protective_equipment_model_id UUID NOT NULL REFERENCES commercial_personal_protective_equipment_models(id),
	-- Ensures parent linkage in asset_models matches the commercial model.
	FOREIGN KEY (id, commercial_personal_protective_equipment_model_id) REFERENCES asset_models(id, parent_model_id)
);
-- Registers the commercial_personal_protective_equipment_lots table name for APS lookup.
INSERT INTO table_names (id) VALUES ('commercial_personal_protective_equipment_lots') ON CONFLICT DO NOTHING;

-- Physical personal protective equipment assets tracked in APS inventory.
CREATE TABLE personal_protective_equipments (
	-- Stable asset identifier inherited from physical_assets.
	id UUID PRIMARY KEY REFERENCES physical_assets (id) ON DELETE CASCADE,
	-- PPE model instantiated by this physical asset.
	personal_protective_equipment_model_id UUID NOT NULL REFERENCES personal_protective_equipment_models (id),
	-- Ensures assets.model_id matches the declared PPE model.
	FOREIGN KEY (id, personal_protective_equipment_model_id) REFERENCES assets (id, model_id)
);
-- Registers the personal_protective_equipments table name for APS type lookup.
INSERT INTO table_names (id) VALUES ('personal_protective_equipments') ON CONFLICT DO NOTHING;
