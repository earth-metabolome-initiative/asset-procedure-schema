-- ============================================================================
-- Migration: Shared Assets / Personal Protective Equipment (PPE)
-- Purpose:
--   Adds PPE model and PPE asset tables to APS so procedures can reference
--   protective equipment with strict model-to-asset consistency.
--
-- APS placement (inheritance and specialization):
--
--   entities
--     +-- ownables
--         +-- namespaced_ownables
--             +-- asset_models
--             |   +-- physical_asset_models
--             |       +-- personal_protective_equipment_models
--             +-- assets
--                 +-- physical_assets
--                     +-- personal_protective_equipments
--
-- Integrity bridge to core APS tables:
--   personal_protective_equipments (id, personal_protective_equipment_model_id)
--     -> assets (id, model_id)
--
-- Metadata registration:
--   Both PPE tables are inserted into `table_names` so APS entities can store
--   and resolve their concrete table type.
--
-- Security context (local, inherited through ancestor PK extension):
--
--   This migration does not define table-local RLS rules. Effective access to
--   PPE rows is inherited from the ancestor `ownables` row with the same `id`.
--
--   PK-extension chain used for policy inheritance:
--   - `personal_protective_equipment_models`:
--       personal_protective_equipment_models.id
--         -> physical_asset_models.id
--         -> asset_models.id
--         -> namespaced_ownables.id
--         -> ownables.id
--   - `personal_protective_equipments`:
--       personal_protective_equipments.id
--         -> physical_assets.id
--         -> assets.id
--         -> namespaced_ownables.id
--         -> ownables.id
--
--   SQL/RLS semantics for access to these specific PPE tables:
--   - `SELECT`: allowed when caller has at least viewer role on ancestor
--     `ownables.owner_id` (role >= 2).
--   - `INSERT`: allowed when caller has at least editor role on ancestor
--     `ownables.owner_id` (role >= 3).
--   - `UPDATE`: allowed when caller has at least editor role on ancestor
--     `ownables.owner_id` (role >= 3).
--   - `DELETE`: allowed when caller has admin role on ancestor
--     `ownables.owner_id` (role >= 4).
--   - Role resolution is SQL-native in APS through `get_owner_role(...)`,
--     which uses `team_members` and `owner_grants`.
--
--   Zanzibar semantics for access to these specific PPE tables:
--   - The same permissions can be described as relation checks on resource
--     `ownable:{id}` for each PPE row (`viewer`, `editor`, `admin` actions).
--   - Membership/grant inheritance is modeled as relationship traversal
--     (user -> team -> owner, plus explicit grants).
--   - Zanzibar-style syntax expresses relation graph intent; SQL/RLS enforces
--     row visibility and write checks inside PostgreSQL.
-- ============================================================================

-- Catalog of PPE model definitions that can be referenced by physical PPE assets.
CREATE TABLE personal_protective_equipment_models (
	-- Stable identifier inherited from `physical_asset_models`.
	id UUID PRIMARY KEY REFERENCES physical_asset_models (id) ON DELETE CASCADE
);
-- Registers this APS table identifier so entities can declare their concrete table type.
INSERT INTO table_names (id) VALUES ('personal_protective_equipment_models') ON CONFLICT DO NOTHING;

-- Physical PPE items tracked as individual assets in inventory and procedure execution.
CREATE TABLE personal_protective_equipments (
	-- Stable identifier inherited from `physical_assets`.
	id UUID PRIMARY KEY REFERENCES physical_assets (id) ON DELETE CASCADE,
	-- Declares which PPE model this physical item instantiates.
	personal_protective_equipment_model_id UUID NOT NULL REFERENCES personal_protective_equipment_models (id),
	-- Ensures the base asset model and PPE model are the same record.
	FOREIGN KEY (id, personal_protective_equipment_model_id) REFERENCES assets (id, model_id)
);
-- Registers this APS table identifier so entities can declare their concrete table type.
INSERT INTO table_names (id) VALUES ('personal_protective_equipments') ON CONFLICT DO NOTHING;
