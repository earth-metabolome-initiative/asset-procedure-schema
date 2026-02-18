-- ============================================================================
-- Migration: Shared Assets / Supplies / Packaging Models
-- Purpose:
--   Adds a packaging model specialization for physical asset models used as
--   packaging in storage, transport, and procedure logistics.
--
-- APS placement (inheritance and specialization):
--
--   entities
--     +-- ownables
--         +-- namespaced_ownables
--             +-- asset_models
--                 +-- physical_asset_models
--                     +-- packaging_models
--
-- Metadata registration:
--   The created table is registered in `table_names`.
--
-- Security context (local, inherited through ancestor PK extension):
--   No table-local RLS is defined here. Access is inherited through:
--   `packaging_models.id -> physical_asset_models.id -> asset_models.id
--   -> namespaced_ownables.id -> ownables.id`.
--
--   SQL/RLS semantics for `packaging_models`:
--   - `SELECT`: viewer-or-higher (role >= 2).
--   - `INSERT`: editor-or-higher (role >= 3).
--   - `UPDATE`: editor-or-higher (role >= 3).
--   - `DELETE`: admin (role >= 4).
--
--   Zanzibar semantics for `packaging_models`:
--   - Equivalent checks are relation checks on `ownable:{id}`.
--   - Zanzibar describes graph permissions; SQL/RLS enforces row behavior.
-- ============================================================================

-- Catalog of physical asset models used as packaging.
CREATE TABLE packaging_models (
    -- Stable model identifier inherited from `physical_asset_models`.
    id UUID PRIMARY KEY REFERENCES physical_asset_models(id) ON DELETE CASCADE
);
-- Registers the `packaging_models` table name for APS type lookup.
INSERT INTO table_names (id) VALUES ('packaging_models') ON CONFLICT DO NOTHING;
