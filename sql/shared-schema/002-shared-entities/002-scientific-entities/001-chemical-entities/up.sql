-- ============================================================================
-- Migration: Shared Entities / Scientific Entities / Chemical Entities
-- Purpose:
--   Adds a canonical scientific-identifier table for chemical substances so
--   APS can resolve the same substance across tools and datasets using stable,
--   globally recognized identifiers.
--
-- APS placement (inheritance and specialization):
--
--   entities
--     +-- chemical_entities
--
-- Metadata registration:
--   The created table is registered in `table_names` so APS can persist and
--   resolve `chemical_entities` as a concrete table type.
--
-- Commercial tables decision:
--   No `commercial_*` tables are introduced in this migration.
--   Rationale: this table models intrinsic chemical identity (InChI and
--   InChIKey), not vendor-specific commercial catalog models or lot/batch
--   traceability.
--
-- Security context (local, inherited through ancestor PK extension):
--   This migration defines no table-local RLS policies.
--   Access inheritance for these rows is through:
--   `chemical_entities.id -> entities.id`.
--   Because this table extends `entities` directly (not `ownables`), it does
--   not inherit owner-role checks via `get_owner_role(...)`.
--
-- SQL/RLS semantics for `chemical_entities`:
--   Entity-level access behavior is determined by core `entities` semantics and
--   any policies defined outside this migration.
--   This migration only defines data-shape and integrity constraints.
--
-- Zanzibar semantics for `chemical_entities`:
--   Zanzibar-style relation modeling for these rows is entity-scoped and
--   external to this migration; this DDL establishes identifiers and
--   consistency constraints, while authorization graph semantics are defined
--   elsewhere.
-- ============================================================================

-- Canonical registry of chemical entities keyed by InChI and InChIKey.
CREATE TABLE chemical_entities (
	-- Stable entity identifier inherited from the global `entities` root.
	id UUID PRIMARY KEY REFERENCES entities(id) ON DELETE CASCADE,
	-- Full International Chemical Identifier encoding molecular structure.
	inchi TEXT NOT NULL,
	-- Condensed 27-character hash identifier for fast indexing and lookup.
	inchi_key TEXT NOT NULL,
	-- Prevents empty InChI payloads and keeps text under APS document-size threshold.
	CHECK (inchi <> '' AND length(inchi) < 8192),
	-- Enforces non-empty 27-character InChIKey storage with explicit upper bound.
	CHECK (
		inchi_key <> ''
		AND length(inchi_key) = 27
		AND length(inchi_key) <= 27
	),
	-- Prevents duplicate full chemical-structure identifiers.
	UNIQUE (inchi),
	-- Prevents duplicate compact hash identifiers.
	UNIQUE (inchi_key)
);
-- Registers the `chemical_entities` table name for APS type lookup.
INSERT INTO table_names (id) VALUES ('chemical_entities') ON CONFLICT DO NOTHING;
