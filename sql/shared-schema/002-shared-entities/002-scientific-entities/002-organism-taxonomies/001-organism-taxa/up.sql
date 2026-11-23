-- ============================================================================
-- Migration: Shared Entities / Scientific Entities / Organism Taxonomies / Organism Taxa
-- Purpose:
--   Adds canonical organism taxon entities (organism taxon:
--   https://www.wikidata.org/wiki/Q16521) with taxonomy-tree structure so APS
--   can model taxonomy itself, while source-specific taxon attributes are
--   handled by descendant tables.
--   This root table intentionally excludes source identifiers (for example
--   NCBI or Catalogue of Life IDs), which are modeled in specialized child
--   tables such as `ncbi_taxa` and `col_taxa`.
--
-- APS placement (inheritance and specialization):
--
--   entities
--     +-- organism_taxa
--
-- Metadata registration:
--   The created table is registered in `table_names` so APS can persist and
--   resolve `organism_taxa` as a concrete table type.
--
-- Commercial tables decision:
--   No `commercial_*` tables are introduced in this migration.
--   Rationale: organism taxonomy records represent scientific classification,
--   not vendor product catalogs or lot/batch inventory concepts.
--
-- Security context (local, inherited through ancestor PK extension):
--   This migration defines no table-local RLS policies.
--   Access inheritance for these rows is through:
--   `organism_taxa.id -> entities.id`.
--   Because this table extends `entities` directly (not `ownables`), it does
--   not inherit owner-role checks via `get_owner_role(...)`.
--
-- SQL/RLS semantics for `organism_taxa`:
--   Entity-level access behavior is determined by core `entities` semantics and
--   any policies defined outside this migration.
--   This migration only defines taxonomy data shape and integrity constraints.
--
-- Zanzibar semantics for `organism_taxa`:
--   Zanzibar-style relation modeling for these rows is entity-scoped and
--   external to this migration; this DDL establishes canonical taxonomy
--   structure while authorization graph semantics are defined elsewhere.
-- ============================================================================

CREATE TABLE organism_taxonomies (
    id TEXT NOT NULL check (id <> '' AND length(id) < 255) PRIMARY KEY REFERENCES table_names(id) ON DELETE CASCADE
    -- TODO Handle versionskn
);
INSERT INTO table_names (id) VALUES ('organism_taxonomies') ON CONFLICT DO NOTHING;



CREATE TABLE taxon_ranks (
    id UUID PRIMARY KEY REFERENCES entities(id) ON DELETE CASCADE,
    name TEXT NOT NULL check (name <> '' AND length(name) < 255),
    organism_taxonomy_id TEXT check (organism_taxonomy_id <> '' AND length(organism_taxonomy_id) < 255) REFERENCES organism_taxonomies(id) ON DELETE CASCADE,
    UNIQUE (id, organism_taxonomy_id)
);
INSERT INTO table_names (id) VALUES ('taxon_ranks') ON CONFLICT DO NOTHING;

CREATE TABLE name_classes (
    id UUID PRIMARY KEY REFERENCES entities(id) ON DELETE CASCADE,
    name TEXT NOT NULL check (name <> '' AND length(name) < 255),
    organism_taxonomy_id TEXT check (organism_taxonomy_id <> '' AND length(organism_taxonomy_id) < 255) REFERENCES organism_taxonomies(id) ON DELETE CASCADE,
    UNIQUE (id, organism_taxonomy_id)
);
INSERT INTO table_names (id) VALUES ('name_classes') ON CONFLICT DO NOTHING;


-- Canonical registry of organism taxa with parent-child taxonomy links.
CREATE TABLE organism_taxa (
	-- Stable entity identifier inherited from the global `entities` root.
	id UUID PRIMARY KEY REFERENCES entities(id) ON DELETE CASCADE,
    rank_id UUID REFERENCES taxon_ranks(id) ON DELETE CASCADE,
	-- Optional immediate parent taxon in the classification hierarchy.
	parent_organism_taxon_id UUID REFERENCES organism_taxa(id) ON DELETE CASCADE,
	-- Prevents a taxon from directly referencing itself as its parent.
	CHECK (id <> parent_organism_taxon_id),
    organism_taxonomy_id TEXT check (organism_taxonomy_id <> '' AND length(organism_taxonomy_id) < 255) REFERENCES organism_taxonomies(id) ON DELETE CASCADE,
    FOREIGN KEY (rank_id, organism_taxonomy_id) REFERENCES taxon_ranks(id, organism_taxonomy_id)
);
INSERT INTO table_names (id) VALUES ('organism_taxa') ON CONFLICT DO NOTHING;

CREATE TABLE taxon_names (
    id UUID PRIMARY KEY REFERENCES entities(id) ON DELETE CASCADE,
    organism_taxon_id UUID REFERENCES organism_taxa(id) ON DELETE CASCADE,
    name TEXT NOT NULL check (name <> '' AND length(name) < 255),
    UNIQUE (organism_taxon_id, name),
    class_id UUID REFERENCES name_classes(id) ON DELETE CASCADE,
    organism_taxonomy_id TEXT check (organism_taxonomy_id <> '' AND length(organism_taxonomy_id) < 255) REFERENCES organism_taxonomies(id) ON DELETE CASCADE,
    FOREIGN KEY (class_id, organism_taxonomy_id) REFERENCES name_classes(id, organism_taxonomy_id)
);
-- Registers the `organism_taxa` table name for APS type lookup.
INSERT INTO table_names (id) VALUES ('taxon_names') ON CONFLICT DO NOTHING;
