-- Migration: Shared Assets / Marking Assets / Panels
-- Purpose: Panels for project inventories.
-- APS placement
--   `resources` of kind `'ownable'`
--       +-- asset_models
--           +-- physical_asset_models
--               +-- panel_models
--       +-- assets
--           +-- physical_assets
--               +-- panels
-- Commercial tables decision: No vendor model or lot specialization is required.

-- Catalog of panel model definitions.
CREATE TABLE panel_models (
    -- Stable model identifier inherited from physical_asset_models.
    id UUID PRIMARY KEY REFERENCES physical_asset_models (id) ON DELETE CASCADE,
    -- Organization containing this resource.
    organization_id UUID NOT NULL,
    -- Archive state inherited from this resource.
    archived BOOLEAN NOT NULL DEFAULT FALSE,
    UNIQUE (id, archived),
    FOREIGN KEY (organization_id, id, archived) REFERENCES resources (organization_id, id, archived) ON UPDATE CASCADE ON DELETE CASCADE
);
INSERT INTO table_names (id) VALUES ('panel_models');
INSERT INTO table_name_ancestors (descendant_table_name_id, ancestor_table_name_id)
    SELECT 'panel_models', ancestor_table_name_id FROM table_name_ancestors WHERE descendant_table_name_id IN ('physical_asset_models')
    UNION SELECT 'panel_models', 'panel_models';
CREATE TRIGGER panel_models_path AFTER INSERT ON panel_models
    FOR EACH ROW EXECUTE FUNCTION check_asset_model_subtype();

CREATE FUNCTION require_panel_models_row() RETURNS TRIGGER
LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
DECLARE
    row_id UUID := CASE TG_OP WHEN 'DELETE' THEN OLD.id ELSE NEW.id END;
BEGIN
    IF EXISTS (
        SELECT 1 FROM public.asset_models root
        JOIN public.table_name_ancestors path ON path.descendant_table_name_id = root.table_name_id
        WHERE root.id = row_id AND path.ancestor_table_name_id = 'panel_models'
    ) AND NOT EXISTS (SELECT 1 FROM public.panel_models subtype WHERE subtype.id = row_id) THEN
        RAISE EXCEPTION '% names a table extending % but has no row there', row_id, 'panel_models'
            USING ERRCODE = 'foreign_key_violation';
    END IF;
    RETURN NULL;
END;
$$;
ALTER FUNCTION require_panel_models_row() OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION require_panel_models_row() FROM PUBLIC;
REVOKE ALL ON panel_models FROM PUBLIC, app, identity_writer;
GRANT SELECT ON panel_models TO aps_writer;
CREATE CONSTRAINT TRIGGER panel_models_row AFTER INSERT ON asset_models
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW WHEN (NEW.table_name_id = 'panel_models')
    EXECUTE FUNCTION require_panel_models_row();
CREATE CONSTRAINT TRIGGER panel_models_kept AFTER DELETE ON panel_models
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION require_panel_models_row();

-- Physical panels tracked in APS inventory.
CREATE TABLE panels (
    -- Stable asset identifier inherited from physical_assets.
    id UUID PRIMARY KEY REFERENCES physical_assets (id) ON DELETE CASCADE,
    -- Organization containing this resource.
    organization_id UUID NOT NULL,
    -- Panel model instantiated by this physical asset.
    panel_model_id UUID NOT NULL REFERENCES panel_models (id),
    FOREIGN KEY (id, panel_model_id) REFERENCES assets (id, model_id) ON DELETE CASCADE,
    -- Archive state inherited from this resource.
    archived BOOLEAN NOT NULL DEFAULT FALSE,
    -- Archive state of the row named by `panel_model_id`.
    panel_model_archived BOOLEAN NOT NULL DEFAULT FALSE,
    FOREIGN KEY (panel_model_id, panel_model_archived) REFERENCES panel_models (id, archived) ON UPDATE CASCADE,
    FOREIGN KEY (organization_id, id, archived) REFERENCES resources (organization_id, id, archived) ON UPDATE CASCADE ON DELETE CASCADE
);
CREATE INDEX ON panels (panel_model_id);
INSERT INTO table_names (id) VALUES ('panels');
UPDATE table_names SET instance_table_name_id = 'panels'
    WHERE id = 'panel_models';
INSERT INTO table_name_ancestors (descendant_table_name_id, ancestor_table_name_id)
    SELECT 'panels', ancestor_table_name_id FROM table_name_ancestors WHERE descendant_table_name_id IN ('physical_assets')
    UNION SELECT 'panels', 'panels';
CREATE TRIGGER panels_path AFTER INSERT ON panels
    FOR EACH ROW EXECUTE FUNCTION check_asset_subtype();

CREATE FUNCTION require_panels_row() RETURNS TRIGGER
LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
DECLARE
    row_id UUID := CASE TG_OP WHEN 'DELETE' THEN OLD.id ELSE NEW.id END;
BEGIN
    IF EXISTS (
        SELECT 1 FROM public.assets root
        JOIN public.table_name_ancestors path ON path.descendant_table_name_id = root.table_name_id
        WHERE root.id = row_id AND path.ancestor_table_name_id = 'panels'
    ) AND NOT EXISTS (SELECT 1 FROM public.panels subtype WHERE subtype.id = row_id) THEN
        RAISE EXCEPTION '% names a table extending % but has no row there', row_id, 'panels'
            USING ERRCODE = 'foreign_key_violation';
    END IF;
    RETURN NULL;
END;
$$;
ALTER FUNCTION require_panels_row() OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION require_panels_row() FROM PUBLIC;
REVOKE ALL ON panels FROM PUBLIC, app, identity_writer;
GRANT SELECT ON panels TO aps_writer;
CREATE CONSTRAINT TRIGGER panels_row AFTER INSERT ON assets
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW WHEN (NEW.table_name_id = 'panels')
    EXECUTE FUNCTION require_panels_row();
CREATE CONSTRAINT TRIGGER panels_kept AFTER DELETE ON panels
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION require_panels_row();

GRANT SELECT, INSERT, DELETE ON panel_models TO app;
ALTER TABLE panel_models ENABLE ROW LEVEL SECURITY;
ALTER TABLE panel_models FORCE ROW LEVEL SECURITY;
CREATE POLICY panel_models_select ON panel_models FOR SELECT TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY panel_models_insert ON panel_models FOR INSERT TO app
    WITH CHECK (id = ANY (ARRAY(SELECT authz.resources_with('writer'))));
CREATE POLICY panel_models_delete ON panel_models FOR DELETE TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('admin'))));
CREATE POLICY panel_models_archived_insert ON panel_models AS RESTRICTIVE FOR INSERT TO app
    WITH CHECK (NOT archived);
CREATE POLICY panel_models_archived_delete ON panel_models AS RESTRICTIVE FOR DELETE TO app
    USING (NOT archived);
CREATE POLICY panel_models_blocked ON panel_models AS RESTRICTIVE FOR ALL TO app
    USING (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))))
    WITH CHECK (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))));

GRANT SELECT, INSERT, DELETE ON panels TO app;
ALTER TABLE panels ENABLE ROW LEVEL SECURITY;
ALTER TABLE panels FORCE ROW LEVEL SECURITY;
CREATE POLICY panels_select ON panels FOR SELECT TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY panels_insert ON panels FOR INSERT TO app
    WITH CHECK (id = ANY (ARRAY(SELECT authz.resources_with('writer')))
        AND panel_model_id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY panels_delete ON panels FOR DELETE TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('admin'))));
CREATE POLICY panels_archived_insert ON panels AS RESTRICTIVE FOR INSERT TO app
    WITH CHECK (NOT archived AND NOT panel_model_archived);
CREATE POLICY panels_archived_delete ON panels AS RESTRICTIVE FOR DELETE TO app
    USING (NOT archived);
CREATE POLICY panels_blocked ON panels AS RESTRICTIVE FOR ALL TO app
    USING (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))))
    WITH CHECK (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))));

-- Records every change in connetto's `audit_events`.
CREATE TRIGGER panel_models_audit AFTER INSERT OR DELETE ON panel_models
    FOR EACH ROW EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER panel_models_update_audit AFTER UPDATE ON panel_models
    FOR EACH ROW WHEN (OLD IS DISTINCT FROM NEW) EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER panels_audit AFTER INSERT OR DELETE ON panels
    FOR EACH ROW EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER panels_update_audit AFTER UPDATE ON panels
    FOR EACH ROW WHEN (OLD IS DISTINCT FROM NEW) EXECUTE FUNCTION authz_write.record_audit_event();
