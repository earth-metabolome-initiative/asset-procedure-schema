-- ============================================================================
-- Migration: Shared Assets / Commercial Products
-- Purpose:
--   Introduces commercial product models and lot-level model specializations so
--   APS can represent branded products and manufacturer lot groupings.
--
-- APS placement (inheritance and specialization):
--
--   entities
--     +-- ownables
--         +-- namespaced_ownables
--             +-- asset_models
--             |   +-- commercial_products
--             |   +-- physical_asset_models
--             |       +-- commercial_product_lots
--
-- Metadata registration:
--   All tables created by this migration are registered in `table_names`.
--
-- Security context (local, inherited through ancestor PK extension):
--   This migration defines no table-local RLS policies. Access is inherited
--   from ancestor ownables through PK/FK identity.
--
--   PK-extension chains used for inherited access checks:
--   - `commercial_products`:
--       id -> asset_models.id -> namespaced_ownables.id -> ownables.id
--   - `commercial_product_lots`:
--       id -> physical_asset_models.id -> asset_models.id
--          -> namespaced_ownables.id -> ownables.id
--
--   SQL/RLS semantics for these specific commercial product tables:
--   - `SELECT`: viewer-or-higher on ancestor owner (role >= 2).
--   - `INSERT`: editor-or-higher on ancestor owner (role >= 3).
--   - `UPDATE`: editor-or-higher on ancestor owner (role >= 3).
--   - `DELETE`: admin on ancestor owner (role >= 4).
--   - Roles are resolved through `get_owner_role(...)`.
--
--   Zanzibar semantics for these specific commercial product tables:
--   - Equivalent authorization can be modeled as relation checks on
--     `ownable:{id}` (`viewer`, `editor`, `admin`).
--   - Membership and grants are relation traversals
--     (user -> team -> owner, plus explicit grants).
--   - Zanzibar syntax captures relation intent; SQL/RLS enforces DB behavior.
-- ============================================================================

-- Catalog of branded product models that extend APS asset models.
CREATE TABLE commercial_products (
	-- Stable model identifier inherited from `asset_models`.
	id UUID PRIMARY KEY REFERENCES asset_models(id) ON DELETE CASCADE,
	-- Brand responsible for manufacturing or distributing this product model.
	brand_id UUID NOT NULL REFERENCES brands(id)
);
-- Registers the `commercial_products` table name for APS type lookup.
INSERT INTO table_names (id) VALUES ('commercial_products') ON CONFLICT DO NOTHING;

-- Catalog of lot-level model variants under a commercial product model.
CREATE TABLE commercial_product_lots (
	-- Stable lot-model identifier inherited from `physical_asset_models`.
	id UUID PRIMARY KEY REFERENCES physical_asset_models(id) ON DELETE CASCADE,
	-- Manufacturer or supplier lot label.
	lot TEXT NOT NULL CHECK (lot <> '' AND length(lot) <= 255),
	-- Commercial product model associated with this lot.
	product_model_id UUID NOT NULL REFERENCES commercial_products(id) ON DELETE CASCADE,
	-- A lot label must be unique within its product model scope.
	UNIQUE (lot, product_model_id),
	-- Ensures the lot model is a direct child of the product model in `asset_models`.
	FOREIGN KEY (id, product_model_id) REFERENCES asset_models(id, parent_model_id)
);
-- Registers the `commercial_product_lots` table name for APS type lookup.
INSERT INTO table_names (id) VALUES ('commercial_product_lots') ON CONFLICT DO NOTHING;
