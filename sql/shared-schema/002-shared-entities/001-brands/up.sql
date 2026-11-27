-- Migration: Shared Catalog / Brands
-- Purpose: Brands for project inventories.
-- APS placement
--   `resources` of kind `'ownable'`
--       +-- brands
-- Commercial tables decision: No vendor model or lot specialization is required.

-- Project-scoped brand catalog.
CREATE TABLE brands (
    -- Resource identity.
    id UUID PRIMARY KEY REFERENCES resources (id) ON DELETE CASCADE,
    -- Organization containing this resource.
    organization_id UUID NOT NULL,
    -- Project containing this ownable.
    project_id UUID NOT NULL,
    -- Concrete APS table holding this row.
    table_name_id TEXT NOT NULL REFERENCES table_names (id) CHECK (table_name_id = 'brands'),
    -- Brand display name.
    name TEXT NOT NULL CHECK (
    name <> ''
    AND length(name) <= 255
    ),
    UNIQUE (project_id, name),
    FOREIGN KEY (id, project_id) REFERENCES resources (id, parent_id) ON UPDATE CASCADE ON DELETE CASCADE,
    archived BOOLEAN NOT NULL DEFAULT FALSE,
    kind authz.resource_kind NOT NULL DEFAULT 'ownable' CHECK (kind = 'ownable'),
    UNIQUE (id, archived),
    FOREIGN KEY (organization_id, id, kind, archived) REFERENCES resources (organization_id, id, kind, archived) ON UPDATE CASCADE ON DELETE CASCADE
);
CREATE INDEX ON brands (table_name_id);
INSERT INTO table_names (id) VALUES ('brands');

REVOKE ALL ON brands FROM PUBLIC, app, identity_writer;
GRANT SELECT, INSERT, UPDATE (name), DELETE ON brands TO app;
ALTER TABLE brands ENABLE ROW LEVEL SECURITY;
ALTER TABLE brands FORCE ROW LEVEL SECURITY;
CREATE POLICY brands_select ON brands FOR SELECT TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('reader')))
        OR project_id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY brands_insert ON brands FOR INSERT TO app
    WITH CHECK (id = ANY (ARRAY(SELECT authz.resources_with('writer')))
        AND project_id = ANY (ARRAY(SELECT authz.resources_with('writer'))));
CREATE POLICY brands_update ON brands FOR UPDATE TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('writer'))))
    WITH CHECK (id = ANY (ARRAY(SELECT authz.resources_with('writer')))
        AND project_id = ANY (ARRAY(SELECT authz.resources_with('writer'))));
CREATE POLICY brands_delete ON brands FOR DELETE TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('admin'))));
CREATE POLICY brands_archived_insert ON brands AS RESTRICTIVE FOR INSERT TO app
    WITH CHECK (NOT archived);
CREATE POLICY brands_archived_update ON brands AS RESTRICTIVE FOR UPDATE TO app
    USING (NOT archived) WITH CHECK (NOT archived);
CREATE POLICY brands_archived_delete ON brands AS RESTRICTIVE FOR DELETE TO app
    USING (NOT archived);
CREATE POLICY brands_blocked ON brands AS RESTRICTIVE FOR ALL TO app
    USING (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))))
    WITH CHECK (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))));
-- Records every change in connetto's `audit_events`.
CREATE TRIGGER brands_audit AFTER INSERT OR DELETE ON brands
    FOR EACH ROW EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER brands_update_audit AFTER UPDATE ON brands
    FOR EACH ROW WHEN (OLD IS DISTINCT FROM NEW) EXECUTE FUNCTION authz_write.record_audit_event();