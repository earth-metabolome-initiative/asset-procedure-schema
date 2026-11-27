-- Migration: Shared Assets / Supplies / Packaging Models
-- Purpose: Packaging Models for project inventories.
-- APS placement
--   `resources` of kind `'ownable'`
--       +-- asset_models
--           +-- physical_asset_models
--               +-- packaging_models
-- Commercial tables decision: No vendor model or lot specialization is required.

-- Catalog of physical asset models used as packaging.
CREATE TABLE packaging_models (
    -- Stable model identifier inherited from `physical_asset_models`.
    id UUID PRIMARY KEY REFERENCES physical_asset_models (id) ON DELETE CASCADE,
    -- Organization containing this resource.
    organization_id UUID NOT NULL,
    -- Archive state inherited from this resource.
    archived BOOLEAN NOT NULL DEFAULT FALSE,
    UNIQUE (id, archived),
    FOREIGN KEY (organization_id, id, archived) REFERENCES resources (organization_id, id, archived) ON UPDATE CASCADE ON DELETE CASCADE
);
INSERT INTO table_names (id) VALUES ('packaging_models');
INSERT INTO table_name_ancestors (descendant_table_name_id, ancestor_table_name_id)
    SELECT 'packaging_models', ancestor_table_name_id FROM table_name_ancestors WHERE descendant_table_name_id IN ('physical_asset_models')
    UNION SELECT 'packaging_models', 'packaging_models';
CREATE TRIGGER packaging_models_path AFTER INSERT ON packaging_models
    FOR EACH ROW EXECUTE FUNCTION check_asset_model_subtype();

CREATE FUNCTION require_packaging_models_row() RETURNS TRIGGER
LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
DECLARE
    row_id UUID := CASE TG_OP WHEN 'DELETE' THEN OLD.id ELSE NEW.id END;
BEGIN
    IF EXISTS (
        SELECT 1 FROM public.asset_models root
        JOIN public.table_name_ancestors path ON path.descendant_table_name_id = root.table_name_id
        WHERE root.id = row_id AND path.ancestor_table_name_id = 'packaging_models'
    ) AND NOT EXISTS (SELECT 1 FROM public.packaging_models subtype WHERE subtype.id = row_id) THEN
        RAISE EXCEPTION '% names a table extending % but has no row there', row_id, 'packaging_models'
            USING ERRCODE = 'foreign_key_violation';
    END IF;
    RETURN NULL;
END;
$$;
ALTER FUNCTION require_packaging_models_row() OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION require_packaging_models_row() FROM PUBLIC;
REVOKE ALL ON packaging_models FROM PUBLIC, app, identity_writer;
GRANT SELECT ON packaging_models TO aps_writer;
CREATE CONSTRAINT TRIGGER packaging_models_row AFTER INSERT ON asset_models
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW WHEN (NEW.table_name_id = 'packaging_models')
    EXECUTE FUNCTION require_packaging_models_row();
CREATE CONSTRAINT TRIGGER packaging_models_kept AFTER DELETE ON packaging_models
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION require_packaging_models_row();

GRANT SELECT, INSERT, DELETE ON packaging_models TO app;
ALTER TABLE packaging_models ENABLE ROW LEVEL SECURITY;
ALTER TABLE packaging_models FORCE ROW LEVEL SECURITY;
CREATE POLICY packaging_models_select ON packaging_models FOR SELECT TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY packaging_models_insert ON packaging_models FOR INSERT TO app
    WITH CHECK (id = ANY (ARRAY(SELECT authz.resources_with('writer'))));
CREATE POLICY packaging_models_delete ON packaging_models FOR DELETE TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('admin'))));
CREATE POLICY packaging_models_archived_insert ON packaging_models AS RESTRICTIVE FOR INSERT TO app
    WITH CHECK (NOT archived);
CREATE POLICY packaging_models_archived_delete ON packaging_models AS RESTRICTIVE FOR DELETE TO app
    USING (NOT archived);
CREATE POLICY packaging_models_blocked ON packaging_models AS RESTRICTIVE FOR ALL TO app
    USING (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))))
    WITH CHECK (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))));

-- Records every change in connetto's `audit_events`.
CREATE TRIGGER packaging_models_audit AFTER INSERT OR DELETE ON packaging_models
    FOR EACH ROW EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER packaging_models_update_audit AFTER UPDATE ON packaging_models
    FOR EACH ROW WHEN (OLD IS DISTINCT FROM NEW) EXECUTE FUNCTION authz_write.record_audit_event();
