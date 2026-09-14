-- ============================================================================
-- Migration: Shared Entities / Scientific Entities / Organism Taxonomies / Catalogue of Life Taxa
-- Purpose:
--   Adds Catalogue of Life taxonomy identifiers (Catalogue of Life:
--   https://www.wikidata.org/wiki/Q38840) as a source-specific specialization
--   of canonical `organism_taxa` nodes.
--
-- APS placement (inheritance and specialization):
--
--   entities
--     +-- organism_taxa
--         +-- col_taxa
--
-- Metadata registration:
--   The created table is registered in `table_names` so APS can persist and
--   resolve `col_taxa` as a concrete table type.
--
-- Commercial tables decision:
--   No `commercial_*` tables are introduced in this migration.
--   Rationale: Catalogue of Life taxonomy identifiers are scientific
--   classification references, not commercial product or lot-traceability
--   concepts.
--
-- Security context (local, inherited through ancestor PK extension):
--   This migration defines no table-local RLS policies.
--   Access inheritance for these rows is through:
--   `col_taxa.id -> organism_taxa.id -> entities.id`.
--   Because this specialization ultimately extends `entities` (not `ownables`),
--   it does not inherit owner-role checks via `get_owner_role(...)`.
--
-- SQL/RLS semantics for `col_taxa`:
--   Entity-level access behavior is determined by core `entities` semantics and
--   any policies defined outside this migration.
--   This migration only defines identifier linkage and integrity constraints.
--
-- Zanzibar semantics for `col_taxa`:
--   Zanzibar-style relation modeling for these rows is entity-scoped and
--   external to this migration; this DDL encodes source-specific taxonomy
--   identifiers while authorization graph semantics are defined elsewhere.
-- ============================================================================

-- Source-specific Catalogue of Life identifiers mapped to canonical organism taxa.
CREATE TABLE col_taxa (
	-- Stable taxon identifier inherited from canonical `organism_taxa`.
	id UUID PRIMARY KEY REFERENCES organism_taxa(id) ON DELETE CASCADE,
	-- Catalogue of Life taxon identifier from the source taxonomy dataset.
	col_id TEXT NOT NULL,
	-- Prevents empty Catalogue of Life identifiers.
	CHECK (col_id <> ''),
	-- Bounds identifier length for predictable indexing behavior.
	CHECK (length(col_id) <= 255),
	-- Prevents multiple canonical taxa from sharing the same source identifier.
	UNIQUE (col_id)
);
-- Registers the `col_taxa` table name for APS type lookup.
INSERT INTO table_names (id) VALUES ('col_taxa') ON CONFLICT DO NOTHING;
INSERT INTO organism_taxonomies (id) VALUES ('col_taxa') ON CONFLICT DO NOTHING;
