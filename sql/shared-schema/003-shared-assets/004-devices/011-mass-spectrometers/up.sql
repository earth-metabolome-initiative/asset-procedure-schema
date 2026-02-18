-- ============================================================================
-- Migration: Shared Assets / Mass Spectrometers
-- Purpose:
--   Adds model, commercial model, commercial lot, and physical asset tables for
--   mass spectrometers used to identify and quantify compounds in APS workflows.
--
-- APS placement (inheritance and specialization):
--
--   entities
--     +-- ownables
--         +-- namespaced_ownables
--             +-- asset_models
--             |   +-- physical_asset_models
--             |   |   +-- mass_spectrometer_models
--             |   +-- commercial_products
--             |   |   +-- commercial_mass_spectrometer_models
--             |   +-- commercial_product_lots
--             |       +-- commercial_mass_spectrometer_lots
--             +-- assets
--                 +-- physical_assets
--                     +-- mass_spectrometers
--
-- Metadata registration:
--   Every table created by this migration is registered in table_names so APS
--   entities can resolve concrete table types at runtime.
--
-- Security context (local, inherited through ancestor PK extension):
--   This migration defines no table-local RLS policies. Access is inherited
--   from ancestor ownables through PK/FK identity chains.
--
--   PK-extension chains used for inherited access checks:
--   - mass_spectrometer_models:
--       id -> physical_asset_models.id -> asset_models.id
--          -> namespaced_ownables.id -> ownables.id
--   - commercial_mass_spectrometer_models:
--       id -> commercial_products.id -> asset_models.id
--          -> namespaced_ownables.id -> ownables.id
--   - commercial_mass_spectrometer_lots:
--       id -> commercial_product_lots.id -> physical_asset_models.id
--          -> asset_models.id -> namespaced_ownables.id -> ownables.id
--   - mass_spectrometers:
--       id -> physical_assets.id -> assets.id
--          -> namespaced_ownables.id -> ownables.id
--
--   SQL/RLS semantics for these specific mass-spectrometer tables:
--   - SELECT: viewer-or-higher on ancestor owner (role >= 2).
--   - INSERT: editor-or-higher on ancestor owner (role >= 3).
--   - UPDATE: editor-or-higher on ancestor owner (role >= 3).
--   - DELETE: admin on ancestor owner (role >= 4).
--   - Role resolution is delegated to get_owner_role(...).
--
--   Zanzibar semantics for these specific mass-spectrometer tables:
--   - Equivalent authorization can be represented as relation checks on
--     ownable:{id} resources (viewer/editor/admin actions).
--   - Team membership and direct grants are modeled as graph traversal
--     (user -> team -> owner, plus explicit grants).
--   - Zanzibar expresses relation intent; SQL/RLS enforces DB row behavior.
-- ============================================================================

-- Catalog of mass spectrometer model definitions.
CREATE TABLE mass_spectrometer_models (
	-- Stable model identifier inherited from physical_asset_models.
	id UUID PRIMARY KEY REFERENCES physical_asset_models(id) ON DELETE CASCADE
);
-- Registers the mass_spectrometer_models table name for APS type lookup.
INSERT INTO table_names (id) VALUES ('mass_spectrometer_models') ON CONFLICT DO NOTHING;

-- Catalog of commercial mass spectrometer models sold by brands.
CREATE TABLE commercial_mass_spectrometer_models (
	-- Stable commercial model identifier shared with parent model tables.
	id UUID PRIMARY KEY,
	-- Base mass spectrometer model represented by this commercial model.
	mass_spectrometer_model_id UUID NOT NULL REFERENCES mass_spectrometer_models(id) ON DELETE CASCADE,
	-- Enforces inheritance from mass_spectrometer_models.
	FOREIGN KEY (id) REFERENCES mass_spectrometer_models(id) ON DELETE CASCADE,
	-- Enforces inheritance from commercial_products.
	FOREIGN KEY (id) REFERENCES commercial_products(id) ON DELETE CASCADE,
	-- Ensures parent linkage in asset_models matches the base model.
	FOREIGN KEY (id, mass_spectrometer_model_id) REFERENCES asset_models(id, parent_model_id)
);
-- Registers the commercial_mass_spectrometer_models table name for APS lookup.
INSERT INTO table_names (id) VALUES ('commercial_mass_spectrometer_models') ON CONFLICT DO NOTHING;

-- Catalog of lot-specific commercial mass spectrometer models.
CREATE TABLE commercial_mass_spectrometer_lots (
	-- Stable lot-model identifier shared with parent lot/model tables.
	id UUID PRIMARY KEY,
	-- Enforces inheritance from commercial_product_lots.
	FOREIGN KEY (id) REFERENCES commercial_product_lots(id) ON DELETE CASCADE,
	-- Enforces inheritance from mass_spectrometer_models.
	FOREIGN KEY (id) REFERENCES mass_spectrometer_models(id) ON DELETE CASCADE,
	-- Commercial model from which this lot-level model descends.
	commercial_mass_spectrometer_model_id UUID NOT NULL REFERENCES commercial_mass_spectrometer_models(id),
	-- Ensures parent linkage in asset_models matches the commercial model.
	FOREIGN KEY (id, commercial_mass_spectrometer_model_id) REFERENCES asset_models(id, parent_model_id)
);
-- Registers the commercial_mass_spectrometer_lots table name for APS lookup.
INSERT INTO table_names (id) VALUES ('commercial_mass_spectrometer_lots') ON CONFLICT DO NOTHING;

-- Physical mass spectrometers tracked in APS inventory and procedures.
CREATE TABLE mass_spectrometers (
	-- Stable asset identifier inherited from physical_assets.
	id UUID PRIMARY KEY REFERENCES physical_assets(id) ON DELETE CASCADE,
	-- Commercial lot model instantiated by this physical instrument.
	commercial_mass_spectrometer_lot_id UUID NOT NULL REFERENCES commercial_mass_spectrometer_lots(id),
	-- Ensures assets.model_id matches the declared commercial lot model.
	FOREIGN KEY (id, commercial_mass_spectrometer_lot_id) REFERENCES assets(id, model_id)
);
-- Registers the mass_spectrometers table name for APS type lookup.
INSERT INTO table_names (id) VALUES ('mass_spectrometers') ON CONFLICT DO NOTHING;
