-- ============================================================================
-- Migration: Shared Assets / Digital Assets / Photographs
-- Purpose:
--   Adds photograph assets as a specialization of digital assets for procedure
--   evidence, inspection records, and contextual documentation.
--
-- APS placement (inheritance and specialization):
--
--   entities
--     +-- ownables
--         +-- namespaced_ownables
--             +-- assets
--                 +-- digital_assets
--                     +-- photographs
--
-- Metadata registration:
--   The created table is registered in `table_names`.
--
-- Security context (local, inherited through ancestor PK extension):
--   No table-local RLS policies are defined here. Access is inherited through:
--   `photographs.id -> digital_assets.id -> assets.id
--   -> namespaced_ownables.id -> ownables.id`.
--
--   SQL/RLS semantics for `photographs`:
--   - `SELECT`: viewer-or-higher (role >= 2).
--   - `INSERT`: editor-or-higher (role >= 3).
--   - `UPDATE`: editor-or-higher (role >= 3).
--   - `DELETE`: admin (role >= 4).
--
--   Zanzibar semantics for `photographs`:
--   - Equivalent checks are modeled as relation checks on `ownable:{id}`.
--   - Zanzibar describes relation graph intent; SQL/RLS enforces DB behavior.
-- ============================================================================

-- Physical record of a digital asset that is specifically a photograph.
CREATE TABLE photographs (
	-- Stable asset identifier inherited from `digital_assets`.
	id UUID PRIMARY KEY REFERENCES digital_assets(id) ON DELETE CASCADE
);
-- Registers the `photographs` table name for APS type lookup.
INSERT INTO table_names (id) VALUES ('photographs') ON CONFLICT DO NOTHING;
