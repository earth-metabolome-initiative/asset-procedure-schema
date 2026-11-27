-- Migration: Shared Assets / Digital Assets / Photographs
-- Purpose: Photographs for project inventories.
-- APS placement
--   `resources` of kind `'ownable'`
--       +-- assets
--           +-- digital_assets
--               +-- photographs
-- Commercial tables decision: No vendor model or lot specialization is required.

-- Digital models describing image files.
CREATE TABLE photograph_models (
    -- Model inherited from the digital model catalog.
    id UUID PRIMARY KEY REFERENCES digital_asset_models (id) ON DELETE CASCADE,
    -- Organization containing this resource.
    organization_id UUID NOT NULL,
    -- Image media type pinned to the digital model.
    mime_type TEXT NOT NULL CHECK (mime_type LIKE 'image/%'),
    FOREIGN KEY (id, mime_type, organization_id) REFERENCES digital_asset_models (id, mime_type, organization_id) ON UPDATE CASCADE,
    -- Archive state inherited from this resource.
    archived BOOLEAN NOT NULL DEFAULT FALSE,
    UNIQUE (id, archived),
    FOREIGN KEY (organization_id, id, archived) REFERENCES resources (organization_id, id, archived) ON UPDATE CASCADE ON DELETE CASCADE
);
INSERT INTO table_names (id) VALUES ('photograph_models');
INSERT INTO table_name_ancestors (descendant_table_name_id, ancestor_table_name_id)
    SELECT 'photograph_models', ancestor_table_name_id FROM table_name_ancestors WHERE descendant_table_name_id IN ('digital_asset_models')
    UNION SELECT 'photograph_models', 'photograph_models';
CREATE TRIGGER photograph_models_path AFTER INSERT ON photograph_models
    FOR EACH ROW EXECUTE FUNCTION check_asset_model_subtype();

CREATE FUNCTION require_photograph_models_row() RETURNS TRIGGER
LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
DECLARE
    row_id UUID := CASE TG_OP WHEN 'DELETE' THEN OLD.id ELSE NEW.id END;
BEGIN
    IF EXISTS (
        SELECT 1 FROM public.asset_models root
        JOIN public.table_name_ancestors path ON path.descendant_table_name_id = root.table_name_id
        WHERE root.id = row_id AND path.ancestor_table_name_id = 'photograph_models'
    ) AND NOT EXISTS (SELECT 1 FROM public.photograph_models subtype WHERE subtype.id = row_id) THEN
        RAISE EXCEPTION '% names a table extending % but has no row there', row_id, 'photograph_models'
            USING ERRCODE = 'foreign_key_violation';
    END IF;
    RETURN NULL;
END;
$$;
ALTER FUNCTION require_photograph_models_row() OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION require_photograph_models_row() FROM PUBLIC;
REVOKE ALL ON photograph_models FROM PUBLIC, app, identity_writer;
GRANT SELECT ON photograph_models TO aps_writer;
CREATE CONSTRAINT TRIGGER photograph_models_row AFTER INSERT ON asset_models
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW WHEN (NEW.table_name_id = 'photograph_models')
    EXECUTE FUNCTION require_photograph_models_row();
CREATE CONSTRAINT TRIGGER photograph_models_kept AFTER DELETE ON photograph_models
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION require_photograph_models_row();

-- Digital assets containing photographs.
CREATE TABLE photographs (
    -- Stable asset identifier inherited from `digital_assets`.
    id UUID PRIMARY KEY REFERENCES digital_assets (id) ON DELETE CASCADE,
    -- Organization containing this resource.
    organization_id UUID NOT NULL,
    -- Image model instantiated by this photograph.
    photograph_model_id UUID NOT NULL REFERENCES photograph_models (id),
    FOREIGN KEY (id, photograph_model_id) REFERENCES assets (id, model_id) ON DELETE CASCADE,
    -- Archive state inherited from this resource.
    archived BOOLEAN NOT NULL DEFAULT FALSE,
    -- Archive state of the row named by `photograph_model_id`.
    photograph_model_archived BOOLEAN NOT NULL DEFAULT FALSE,
    FOREIGN KEY (photograph_model_id, photograph_model_archived) REFERENCES photograph_models (id, archived) ON UPDATE CASCADE,
    FOREIGN KEY (organization_id, id, archived) REFERENCES resources (organization_id, id, archived) ON UPDATE CASCADE ON DELETE CASCADE
);
CREATE INDEX ON photographs (photograph_model_id);
INSERT INTO table_names (id) VALUES ('photographs');
UPDATE table_names SET instance_table_name_id = 'photographs'
    WHERE id = 'photograph_models';
INSERT INTO table_name_ancestors (descendant_table_name_id, ancestor_table_name_id)
    SELECT 'photographs', ancestor_table_name_id FROM table_name_ancestors WHERE descendant_table_name_id IN ('digital_assets')
    UNION SELECT 'photographs', 'photographs';
CREATE TRIGGER photographs_path AFTER INSERT ON photographs
    FOR EACH ROW EXECUTE FUNCTION check_asset_subtype();

CREATE FUNCTION require_photographs_row() RETURNS TRIGGER
LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
DECLARE
    row_id UUID := CASE TG_OP WHEN 'DELETE' THEN OLD.id ELSE NEW.id END;
BEGIN
    IF EXISTS (
        SELECT 1 FROM public.assets root
        JOIN public.table_name_ancestors path ON path.descendant_table_name_id = root.table_name_id
        WHERE root.id = row_id AND path.ancestor_table_name_id = 'photographs'
    ) AND NOT EXISTS (SELECT 1 FROM public.photographs subtype WHERE subtype.id = row_id) THEN
        RAISE EXCEPTION '% names a table extending % but has no row there', row_id, 'photographs'
            USING ERRCODE = 'foreign_key_violation';
    END IF;
    RETURN NULL;
END;
$$;
ALTER FUNCTION require_photographs_row() OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION require_photographs_row() FROM PUBLIC;
REVOKE ALL ON photographs FROM PUBLIC, app, identity_writer;
GRANT SELECT ON photographs TO aps_writer;
CREATE CONSTRAINT TRIGGER photographs_row AFTER INSERT ON assets
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW WHEN (NEW.table_name_id = 'photographs')
    EXECUTE FUNCTION require_photographs_row();
CREATE CONSTRAINT TRIGGER photographs_kept AFTER DELETE ON photographs
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION require_photographs_row();

GRANT SELECT, INSERT, DELETE ON photographs TO app;
ALTER TABLE photographs ENABLE ROW LEVEL SECURITY;
ALTER TABLE photographs FORCE ROW LEVEL SECURITY;
CREATE POLICY photographs_select ON photographs FOR SELECT TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY photographs_insert ON photographs FOR INSERT TO app
    WITH CHECK (id = ANY (ARRAY(SELECT authz.resources_with('writer')))
        AND photograph_model_id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY photographs_delete ON photographs FOR DELETE TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('admin'))));
CREATE POLICY photographs_archived_insert ON photographs AS RESTRICTIVE FOR INSERT TO app
    WITH CHECK (NOT archived AND NOT photograph_model_archived);
CREATE POLICY photographs_archived_delete ON photographs AS RESTRICTIVE FOR DELETE TO app
    USING (NOT archived);
CREATE POLICY photographs_blocked ON photographs AS RESTRICTIVE FOR ALL TO app
    USING (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))))
    WITH CHECK (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))));

GRANT SELECT, INSERT, DELETE ON photograph_models TO app;
ALTER TABLE photograph_models ENABLE ROW LEVEL SECURITY;
ALTER TABLE photograph_models FORCE ROW LEVEL SECURITY;
CREATE POLICY photograph_models_select ON photograph_models FOR SELECT TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY photograph_models_insert ON photograph_models FOR INSERT TO app
    WITH CHECK (id = ANY (ARRAY(SELECT authz.resources_with('writer'))));
CREATE POLICY photograph_models_delete ON photograph_models FOR DELETE TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('admin'))));
CREATE POLICY photograph_models_archived_insert ON photograph_models AS RESTRICTIVE FOR INSERT TO app
    WITH CHECK (NOT archived);
CREATE POLICY photograph_models_archived_delete ON photograph_models AS RESTRICTIVE FOR DELETE TO app
    USING (NOT archived);
CREATE POLICY photograph_models_blocked ON photograph_models AS RESTRICTIVE FOR ALL TO app
    USING (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))))
    WITH CHECK (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))));

-- Records every change in connetto's `audit_events`.
CREATE TRIGGER photograph_models_audit AFTER INSERT OR DELETE ON photograph_models
    FOR EACH ROW EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER photograph_models_update_audit AFTER UPDATE ON photograph_models
    FOR EACH ROW WHEN (OLD IS DISTINCT FROM NEW) EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER photographs_audit AFTER INSERT OR DELETE ON photographs
    FOR EACH ROW EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER photographs_update_audit AFTER UPDATE ON photographs
    FOR EACH ROW WHEN (OLD IS DISTINCT FROM NEW) EXECUTE FUNCTION authz_write.record_audit_event();
