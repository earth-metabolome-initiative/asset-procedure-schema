-- ============================================================================
-- Migration: Shared Assets / Supplies / Beads
-- Purpose:
--   Adds bead model specialization with physical diameter metadata used by
--   procedures that depend on bead size.
--
-- APS placement (inheritance and specialization):
--
--   entities
--     +-- ownables
--         +-- namespaced_ownables
--             +-- asset_models
--                 +-- physical_asset_models
--                     +-- bead_models
--
-- Metadata registration:
--   The created table is registered in `table_names`.
--
-- Security context (local, inherited through ancestor PK extension):
--   No table-local RLS policies are defined here. Access is inherited through:
--   `bead_models.id -> physical_asset_models.id -> asset_models.id
--   -> namespaced_ownables.id -> ownables.id`.
--
--   SQL/RLS semantics for `bead_models`:
--   - `SELECT`: viewer-or-higher (role >= 2).
--   - `INSERT`: editor-or-higher (role >= 3).
--   - `UPDATE`: editor-or-higher (role >= 3).
--   - `DELETE`: admin (role >= 4).
--
--   Zanzibar semantics for `bead_models`:
--   - Equivalent authorization is a relation check on `ownable:{id}`.
--   - Zanzibar describes relation intent; SQL/RLS enforces row permissions.
-- ============================================================================

-- Catalog of bead models with diameter metadata.
CREATE TABLE bead_models (
	-- Stable model identifier inherited from `physical_asset_models`.
	id UUID PRIMARY KEY REFERENCES physical_asset_models(id) ON DELETE CASCADE,
	-- Bead diameter in millimeters.
	diameter REAL NOT NULL CHECK (diameter > 0.0)
);
-- Registers the `bead_models` table name for APS type lookup.
INSERT INTO table_names (id) VALUES ('bead_models') ON CONFLICT DO NOTHING;
