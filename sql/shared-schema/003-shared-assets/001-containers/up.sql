-- ============================================================================
-- Migration: Shared Assets / Containers
-- Purpose:
--   Introduces container model and asset specializations, plus compatibility
--   rules that declare which physical asset models are allowed inside a given
--   container model.
--
-- APS placement (inheritance and specialization):
--
--   entities
--     +-- ownables
--         +-- container_compatibility_rules
--         +-- namespaced_ownables
--             +-- asset_models
--             |   +-- physical_asset_models
--             |       +-- container_models
--             |           +-- weighed_container_models
--             |           +-- volumetric_container_models
--             |           +-- container_sealer_models
--             +-- assets
--                 +-- physical_assets
--                     +-- containers
--                         +-- volumetric_containers
--
-- Metadata registration:
--   Every table created by this migration is registered in `table_names` so APS
--   entities can persist and resolve concrete table types.
--
-- Security context (local, inherited through ancestor PK extension):
--   This migration defines no table-local RLS policies. Effective access flows
--   through ancestor ownable rows linked by PK/FK identity.
--
--   PK-extension chains used for inherited access checks:
--   - `container_models` / `weighed_container_models` /
--     `volumetric_container_models` / `container_sealer_models`:
--       id -> physical_asset_models.id -> asset_models.id
--          -> namespaced_ownables.id -> ownables.id
--   - `containers` / `volumetric_containers`:
--       id -> physical_assets.id -> assets.id
--          -> namespaced_ownables.id -> ownables.id
--   - `container_compatibility_rules`:
--       id -> ownables.id
--
--   SQL/RLS semantics for these specific container tables:
--   - `SELECT`: requires viewer-or-higher role on ancestor `ownables.owner_id`
--     (role >= 2).
--   - `INSERT`: requires editor-or-higher role on ancestor `ownables.owner_id`
--     (role >= 3).
--   - `UPDATE`: requires editor-or-higher role on ancestor `ownables.owner_id`
--     (role >= 3).
--   - `DELETE`: requires admin role on ancestor `ownables.owner_id` (role >= 4).
--   - Role evaluation is performed by `get_owner_role(...)` in core security.
--
--   Zanzibar semantics for these specific container tables:
--   - Equivalent checks can be expressed as relation checks against
--     `ownable:{id}` (`viewer`, `editor`, `admin`) for each row.
--   - Team membership and direct grants are modeled as graph traversal
--     (user -> team -> owner, plus explicit grants).
--   - Zanzibar syntax describes relation intent; SQL/RLS enforces row filtering
--     and write authorization in PostgreSQL.
-- ============================================================================

-- Catalog of reusable container models for physical container assets.
CREATE TABLE container_models (
    -- Stable model identifier inherited from `physical_asset_models`.
    id UUID PRIMARY KEY REFERENCES physical_asset_models(id) ON DELETE CASCADE
);
-- Registers the `container_models` table name for APS polymorphic resolution.
INSERT INTO table_names (id)
VALUES ('container_models') ON CONFLICT DO NOTHING;

-- Rule set that defines which physical asset models can be contained by a
-- specific container model and in what quantity.
CREATE TABLE container_compatibility_rules (
    -- Stable rule identifier inherited from `ownables`.
    id UUID PRIMARY KEY REFERENCES ownables(id) ON DELETE CASCADE,
    -- Container model that receives contained assets.
    container_model_id UUID NOT NULL REFERENCES container_models(id) ON DELETE CASCADE,
    -- Physical asset model allowed inside the container model.
    contained_asset_model_id UUID NOT NULL REFERENCES physical_asset_models(id) ON DELETE CASCADE,
    -- Maximum allowed count of `contained_asset_model_id` in the container.
    quantity SMALLINT NOT NULL DEFAULT 1 CHECK (quantity > 0),
    -- Prevents duplicate compatibility rules for the same model pair.
    UNIQUE(container_model_id, contained_asset_model_id),
    -- Prevents declaring a model as containing itself.
    CHECK (
        container_model_id <> contained_asset_model_id
    )
);
-- Registers the `container_compatibility_rules` table name for APS type lookup.
INSERT INTO table_names (id)
VALUES ('container_compatibility_rules') ON CONFLICT DO NOTHING;

-- Specialization of container models with known empty mass.
CREATE TABLE weighed_container_models (
    -- Stable model identifier inherited from `container_models`.
    id UUID PRIMARY KEY REFERENCES container_models(id) ON DELETE CASCADE,
    -- Empty container mass in kilograms.
    mass REAL NOT NULL CHECK (mass > 0.0)
);
-- Registers the `weighed_container_models` table name for APS type lookup.
INSERT INTO table_names (id)
VALUES ('weighed_container_models') ON CONFLICT DO NOTHING;

-- Specialization of container models with known maximum volume.
CREATE TABLE volumetric_container_models (
    -- Stable model identifier inherited from `container_models`.
    id UUID PRIMARY KEY REFERENCES container_models(id) ON DELETE CASCADE,
    -- Maximum internal volume in liters.
    volume REAL NOT NULL CHECK (volume > 0.0)
);
-- Registers the `volumetric_container_models` table name for APS type lookup.
INSERT INTO table_names (id)
VALUES ('volumetric_container_models') ON CONFLICT DO NOTHING;

-- Physical containers tracked in inventory and execution contexts.
CREATE TABLE containers (
    -- Stable asset identifier inherited from `physical_assets`.
    id UUID PRIMARY KEY REFERENCES physical_assets(id) ON DELETE CASCADE,
    -- Container model instantiated by this physical container.
    container_model_id UUID NOT NULL REFERENCES container_models(id),
    -- Ensures the base `assets.model_id` matches `container_model_id`.
    FOREIGN KEY (id, container_model_id) REFERENCES assets(id, model_id)
);
-- Registers the `containers` table name for APS polymorphic resolution.
INSERT INTO table_names (id)
VALUES ('containers') ON CONFLICT DO NOTHING;

-- Physical containers whose model explicitly carries volumetric metadata.
CREATE TABLE volumetric_containers (
    -- Stable asset identifier inherited from `containers`.
    id UUID PRIMARY KEY REFERENCES containers(id) ON DELETE CASCADE,
    -- Volumetric model instantiated by this container.
    volumetric_container_model_id UUID NOT NULL REFERENCES volumetric_container_models(id),
    -- Ensures the parent asset model aligns with the volumetric model.
    FOREIGN KEY (id, volumetric_container_model_id) REFERENCES assets(id, model_id)
);
-- Registers the `volumetric_containers` table name for APS type lookup.
INSERT INTO table_names (id)
VALUES ('volumetric_containers') ON CONFLICT DO NOTHING;

-- Catalog of models whose primary role is sealing container openings.
CREATE TABLE container_sealer_models (
    -- Stable model identifier inherited from `physical_asset_models`.
    id UUID PRIMARY KEY REFERENCES physical_asset_models(id) ON DELETE CASCADE
);
-- Registers the `container_sealer_models` table name for APS type lookup.
INSERT INTO table_names (id)
VALUES ('container_sealer_models') ON CONFLICT DO NOTHING;
