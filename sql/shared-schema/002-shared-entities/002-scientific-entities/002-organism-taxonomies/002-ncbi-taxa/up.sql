-- ============================================================================
-- Migration: Shared Entities / Scientific Entities / Organism Taxonomies / NCBI Taxa
-- Purpose:
--   Adds NCBI taxonomy identifiers (NCBI taxonomy database:
--   https://www.wikidata.org/wiki/Q82494) as a source-specific specialization
--   of canonical `organism_taxa` nodes.
--
-- APS placement (inheritance and specialization):
--
--   entities
--     +-- organism_taxa
--         +-- ncbi_taxa
--
-- Metadata registration:
--   The created table is registered in `table_names` so APS can persist and
--   resolve `ncbi_taxa` as a concrete table type.
--
-- Commercial tables decision:
--   No `commercial_*` tables are introduced in this migration.
--   Rationale: NCBI taxonomy identifiers are scientific classification
--   references, not commercial product or lot-traceability concepts.
--
-- Security context (local, inherited through ancestor PK extension):
--   This migration defines no table-local RLS policies.
--   Access inheritance for these rows is through:
--   `ncbi_taxa.id -> organism_taxa.id -> entities.id`.
--   Because this specialization ultimately extends `entities` (not `ownables`),
--   it does not inherit owner-role checks via `get_owner_role(...)`.
--
-- SQL/RLS semantics for `ncbi_taxa`:
--   Entity-level access behavior is determined by core `entities` semantics and
--   any policies defined outside this migration.
--   This migration only defines identifier linkage and integrity constraints.
--
-- Zanzibar semantics for `ncbi_taxa`:
--   Zanzibar-style relation modeling for these rows is entity-scoped and
--   external to this migration; this DDL encodes source-specific taxonomy
--   identifiers while authorization graph semantics are defined elsewhere.
-- ============================================================================

-- Source-specific NCBI taxonomy identifiers mapped to canonical organism taxa.
CREATE TABLE ncbi_taxa (
	-- Stable taxon identifier inherited from canonical `organism_taxa`.
	id UUID PRIMARY KEY REFERENCES organism_taxa(id) ON DELETE CASCADE,
	-- Positive NCBI taxonomy identifier assigned by the NCBI taxonomy dataset.
	ncbi_id INTEGER NOT NULL,
	-- Prevents non-positive identifiers that cannot map to valid NCBI taxa.
	CHECK (ncbi_id > 0),
	-- Prevents multiple canonical taxa from sharing the same NCBI identifier.
	UNIQUE (ncbi_id)
);
-- Registers the `ncbi_taxa` table name for APS type lookup.
INSERT INTO table_names (id) VALUES ('ncbi_taxa') ON CONFLICT DO NOTHING;
INSERT INTO organism_taxonomies (id) VALUES ('ncbi_taxa') ON CONFLICT DO NOTHING;
