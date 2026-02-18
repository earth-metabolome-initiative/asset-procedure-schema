-- ============================================================================
-- Migration: Shared Assets / Samples
-- Purpose:
--   Adds sample-source and sample entities so APS can track sample provenance
--   and enforce model-level compatibility between sample, source, and instance.
--
-- APS placement (inheritance and specialization):
--
--   entities
--     +-- ownables
--         +-- namespaced_ownables
--             +-- asset_models
--             |   +-- physical_asset_models
--             |       +-- sample_source_models
--             |       +-- sample_models
--             +-- assets
--                 +-- physical_assets
--                     +-- sample_sources
--                     +-- samples
--
-- Metadata registration:
--   This historical migration currently creates sample tables without inserting
--   corresponding entries into `table_names`.
--
-- Security context (local, inherited through ancestor PK extension):
--   No table-local RLS policies are defined here. Access is inherited from
--   ancestor ownables through PK/FK identity chains.
--
--   PK-extension chains used for inherited access checks:
--   - `sample_source_models` / `sample_models`:
--       id -> physical_asset_models.id -> asset_models.id
--          -> namespaced_ownables.id -> ownables.id
--   - `sample_sources` / `samples`:
--       id -> physical_assets.id -> assets.id
--          -> namespaced_ownables.id -> ownables.id
--
--   SQL/RLS semantics for these specific sample tables:
--   - `SELECT`: viewer-or-higher on ancestor owner (role >= 2).
--   - `INSERT`: editor-or-higher on ancestor owner (role >= 3).
--   - `UPDATE`: editor-or-higher on ancestor owner (role >= 3).
--   - `DELETE`: admin on ancestor owner (role >= 4).
--   - Roles are resolved through `get_owner_role(...)`.
--
--   Zanzibar semantics for these specific sample tables:
--   - Equivalent checks can be expressed as relation evaluation on
--     `ownable:{id}` (`viewer`, `editor`, `admin`).
--   - Membership and grants are relation traversals
--     (user -> team -> owner, plus explicit grants).
--   - Zanzibar syntax describes graph intent; SQL/RLS enforces row behavior.
-- ============================================================================

-- Catalog of model definitions for assets that can be sampled from.
CREATE TABLE sample_source_models (
  -- Stable model identifier inherited from `physical_asset_models`.
  id UUID PRIMARY KEY REFERENCES physical_asset_models(id) ON DELETE CASCADE
);

-- Physical assets that can act as concrete sample sources.
CREATE TABLE sample_sources (
  -- Stable asset identifier inherited from `physical_assets`.
  id UUID PRIMARY KEY REFERENCES physical_assets(id) ON DELETE CASCADE,
  -- Source model instantiated by this physical source asset.
  sample_source_model_id UUID NOT NULL REFERENCES sample_source_models(id),
  -- Ensures the parent asset model matches `sample_source_model_id`.
  FOREIGN KEY (id, sample_source_model_id) REFERENCES assets(id, model_id)
);

-- Catalog of sample models and the source-model family they require.
CREATE TABLE sample_models (
	-- Stable model identifier inherited from `physical_asset_models`.
	id UUID PRIMARY KEY REFERENCES physical_asset_models(id) ON DELETE CASCADE,
	-- Source-model family that sample instances of this model must reference.
	sample_source_model_id UUID NOT NULL REFERENCES sample_source_models(id),
	-- Enables composite FK checks tying a sample model to its source-model family.
	UNIQUE (id, sample_source_model_id)
);

-- Physical sample assets linked to sample model, optional source instance, and
-- mandatory source-model compatibility.
CREATE TABLE samples (
	-- Stable asset identifier inherited from `physical_assets`.
	id UUID PRIMARY KEY REFERENCES physical_assets(id) ON DELETE CASCADE,
	-- Sample model instantiated by this physical sample.
	sample_model_id UUID NOT NULL REFERENCES sample_models(id),
	-- Optional concrete source asset the sample was taken from.
	sample_source_id UUID REFERENCES sample_sources(id),
	-- Required source-model family associated with the sample and source relation.
	sample_source_model_id UUID NOT NULL REFERENCES sample_source_models(id),
	-- Ensures `assets.model_id` matches the selected sample model.
	FOREIGN KEY (id, sample_model_id) REFERENCES assets(id, model_id),
	-- Ensures the sample model belongs to the declared source-model family.
	FOREIGN KEY (sample_model_id, sample_source_model_id) REFERENCES sample_models(id, sample_source_model_id),
	-- Ensures the optional source asset matches the declared source-model family.
	FOREIGN KEY (sample_source_id, sample_source_model_id) REFERENCES assets(id, model_id)
);
