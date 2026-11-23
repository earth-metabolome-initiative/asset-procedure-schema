-- Migration: Shared Assets / Devices / Cameras
-- Purpose: Cameras for project inventories.
-- APS placement
--   ownables
--       +-- asset_models
--           +-- physical_asset_models
--               +-- camera_models
--                   +-- commercial_camera_models
--               +-- commercial_product_lots
--                   +-- commercial_camera_lots
--       +-- assets
--           +-- physical_assets
--               +-- cameras
-- Commercial tables decision: Vendor models and lots provide purchase traceability.

-- Catalog of camera model definitions.
CREATE TABLE camera_models (
    -- Stable model identifier inherited from physical_asset_models.
    id UUID PRIMARY KEY REFERENCES physical_asset_models (id) ON DELETE CASCADE,
    -- Organization containing this resource.
    organization_id UUID NOT NULL,
    FOREIGN KEY (organization_id, id) REFERENCES resources (organization_id, id) ON DELETE CASCADE
);
INSERT INTO table_names (id) VALUES ('camera_models');
INSERT INTO table_name_ancestors (descendant_table_name_id, ancestor_table_name_id)
    SELECT 'camera_models', ancestor_table_name_id FROM table_name_ancestors WHERE descendant_table_name_id IN ('physical_asset_models')
    UNION SELECT 'camera_models', 'camera_models';
CREATE TRIGGER camera_models_path AFTER INSERT ON camera_models
    FOR EACH ROW EXECUTE FUNCTION check_asset_model_subtype();

CREATE FUNCTION require_camera_models_row() RETURNS TRIGGER
LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
DECLARE
    row_id UUID := CASE TG_OP WHEN 'DELETE' THEN OLD.id ELSE NEW.id END;
BEGIN
    IF EXISTS (
        SELECT 1 FROM public.asset_models root
        JOIN public.table_name_ancestors path ON path.descendant_table_name_id = root.table_name_id
        WHERE root.id = row_id AND path.ancestor_table_name_id = 'camera_models'
    ) AND NOT EXISTS (SELECT 1 FROM public.camera_models subtype WHERE subtype.id = row_id) THEN
        RAISE EXCEPTION '% names a table extending % but has no row there', row_id, 'camera_models'
            USING ERRCODE = 'foreign_key_violation';
    END IF;
    RETURN NULL;
END;
$$;
ALTER FUNCTION require_camera_models_row() OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION require_camera_models_row() FROM PUBLIC;
GRANT SELECT ON camera_models TO aps_writer;
CREATE CONSTRAINT TRIGGER camera_models_row AFTER INSERT ON asset_models
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW WHEN (NEW.table_name_id = 'camera_models')
    EXECUTE FUNCTION require_camera_models_row();
CREATE CONSTRAINT TRIGGER camera_models_kept AFTER DELETE ON camera_models
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION require_camera_models_row();

-- Catalog of commercial camera models.
CREATE TABLE commercial_camera_models (
    -- Stable commercial model identifier shared with parent model tables.
    id UUID PRIMARY KEY REFERENCES camera_models (id) ON DELETE CASCADE,
    -- Organization containing this resource.
    organization_id UUID NOT NULL,
    -- Base camera model represented by this commercial model.
    camera_model_id UUID NOT NULL REFERENCES camera_models (id) ON DELETE RESTRICT,
    FOREIGN KEY (id) REFERENCES commercial_products (id) ON DELETE CASCADE,
    FOREIGN KEY (id, camera_model_id) REFERENCES asset_models (id, parent_model_id) ON DELETE CASCADE,
    FOREIGN KEY (organization_id, id) REFERENCES resources (organization_id, id) ON DELETE CASCADE
);
CREATE INDEX ON commercial_camera_models (camera_model_id);
INSERT INTO table_names (id) VALUES ('commercial_camera_models');
INSERT INTO table_name_ancestors (descendant_table_name_id, ancestor_table_name_id)
    SELECT 'commercial_camera_models', ancestor_table_name_id FROM table_name_ancestors WHERE descendant_table_name_id IN ('camera_models', 'commercial_products')
    UNION SELECT 'commercial_camera_models', 'commercial_camera_models';
CREATE TRIGGER commercial_camera_models_path AFTER INSERT ON commercial_camera_models
    FOR EACH ROW EXECUTE FUNCTION check_asset_model_subtype();

CREATE FUNCTION require_commercial_camera_models_row() RETURNS TRIGGER
LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
DECLARE
    row_id UUID := CASE TG_OP WHEN 'DELETE' THEN OLD.id ELSE NEW.id END;
BEGIN
    IF EXISTS (
        SELECT 1 FROM public.asset_models root
        JOIN public.table_name_ancestors path ON path.descendant_table_name_id = root.table_name_id
        WHERE root.id = row_id AND path.ancestor_table_name_id = 'commercial_camera_models'
    ) AND NOT EXISTS (SELECT 1 FROM public.commercial_camera_models subtype WHERE subtype.id = row_id) THEN
        RAISE EXCEPTION '% names a table extending % but has no row there', row_id, 'commercial_camera_models'
            USING ERRCODE = 'foreign_key_violation';
    END IF;
    RETURN NULL;
END;
$$;
ALTER FUNCTION require_commercial_camera_models_row() OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION require_commercial_camera_models_row() FROM PUBLIC;
GRANT SELECT ON commercial_camera_models TO aps_writer;
CREATE CONSTRAINT TRIGGER commercial_camera_models_row AFTER INSERT ON asset_models
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW WHEN (NEW.table_name_id = 'commercial_camera_models')
    EXECUTE FUNCTION require_commercial_camera_models_row();
CREATE CONSTRAINT TRIGGER commercial_camera_models_kept AFTER DELETE ON commercial_camera_models
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION require_commercial_camera_models_row();

-- Catalog of lot-specific commercial camera models.
CREATE TABLE commercial_camera_lots (
    -- Stable lot-model identifier shared with parent lot/model tables.
    id UUID PRIMARY KEY REFERENCES commercial_product_lots (id) ON DELETE CASCADE,
    -- Organization containing this resource.
    organization_id UUID NOT NULL,
    FOREIGN KEY (id) REFERENCES camera_models (id) ON DELETE CASCADE,
    -- Commercial camera model from which this lot derives.
    commercial_camera_model_id UUID NOT NULL REFERENCES commercial_camera_models (id),
    FOREIGN KEY (id, commercial_camera_model_id) REFERENCES asset_models (id, parent_model_id) ON DELETE CASCADE,
    FOREIGN KEY (organization_id, id) REFERENCES resources (organization_id, id) ON DELETE CASCADE
);
CREATE INDEX ON commercial_camera_lots (commercial_camera_model_id);
INSERT INTO table_names (id) VALUES ('commercial_camera_lots');
INSERT INTO table_name_ancestors (descendant_table_name_id, ancestor_table_name_id)
    SELECT 'commercial_camera_lots', ancestor_table_name_id FROM table_name_ancestors WHERE descendant_table_name_id IN ('commercial_product_lots', 'camera_models')
    UNION SELECT 'commercial_camera_lots', 'commercial_camera_lots';
INSERT INTO table_name_admitted_parents (table_name_id, parent_table_name_id)
    VALUES ('commercial_camera_lots', 'commercial_camera_models');
CREATE TRIGGER commercial_camera_lots_path AFTER INSERT ON commercial_camera_lots
    FOR EACH ROW EXECUTE FUNCTION check_asset_model_subtype();

CREATE FUNCTION require_commercial_camera_lots_row() RETURNS TRIGGER
LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
DECLARE
    row_id UUID := CASE TG_OP WHEN 'DELETE' THEN OLD.id ELSE NEW.id END;
BEGIN
    IF EXISTS (
        SELECT 1 FROM public.asset_models root
        JOIN public.table_name_ancestors path ON path.descendant_table_name_id = root.table_name_id
        WHERE root.id = row_id AND path.ancestor_table_name_id = 'commercial_camera_lots'
    ) AND NOT EXISTS (SELECT 1 FROM public.commercial_camera_lots subtype WHERE subtype.id = row_id) THEN
        RAISE EXCEPTION '% names a table extending % but has no row there', row_id, 'commercial_camera_lots'
            USING ERRCODE = 'foreign_key_violation';
    END IF;
    RETURN NULL;
END;
$$;
ALTER FUNCTION require_commercial_camera_lots_row() OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION require_commercial_camera_lots_row() FROM PUBLIC;
GRANT SELECT ON commercial_camera_lots TO aps_writer;
CREATE CONSTRAINT TRIGGER commercial_camera_lots_row AFTER INSERT ON asset_models
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW WHEN (NEW.table_name_id = 'commercial_camera_lots')
    EXECUTE FUNCTION require_commercial_camera_lots_row();
CREATE CONSTRAINT TRIGGER commercial_camera_lots_kept AFTER DELETE ON commercial_camera_lots
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION require_commercial_camera_lots_row();

-- Physical cameras tracked in APS inventory.
CREATE TABLE cameras (
    -- Stable asset identifier inherited from physical_assets.
    id UUID PRIMARY KEY REFERENCES physical_assets (id) ON DELETE CASCADE,
    -- Organization containing this resource.
    organization_id UUID NOT NULL,
    -- Commercial lot model instantiated by this physical asset.
    commercial_camera_lot_id UUID NOT NULL REFERENCES commercial_camera_lots (id),
    FOREIGN KEY (id, commercial_camera_lot_id) REFERENCES assets (id, model_id) ON DELETE CASCADE,
    FOREIGN KEY (organization_id, id) REFERENCES resources (organization_id, id) ON DELETE CASCADE
);
CREATE INDEX ON cameras (commercial_camera_lot_id);
INSERT INTO table_names (id) VALUES ('cameras');
INSERT INTO table_name_instances (id, instance_table_name_id)
    VALUES ('commercial_camera_lots', 'cameras');
INSERT INTO table_name_ancestors (descendant_table_name_id, ancestor_table_name_id)
    SELECT 'cameras', ancestor_table_name_id FROM table_name_ancestors WHERE descendant_table_name_id IN ('physical_assets')
    UNION SELECT 'cameras', 'cameras';
CREATE TRIGGER cameras_path AFTER INSERT ON cameras
    FOR EACH ROW EXECUTE FUNCTION check_asset_subtype();

CREATE FUNCTION require_cameras_row() RETURNS TRIGGER
LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
DECLARE
    row_id UUID := CASE TG_OP WHEN 'DELETE' THEN OLD.id ELSE NEW.id END;
BEGIN
    IF EXISTS (
        SELECT 1 FROM public.assets root
        JOIN public.table_name_ancestors path ON path.descendant_table_name_id = root.table_name_id
        WHERE root.id = row_id AND path.ancestor_table_name_id = 'cameras'
    ) AND NOT EXISTS (SELECT 1 FROM public.cameras subtype WHERE subtype.id = row_id) THEN
        RAISE EXCEPTION '% names a table extending % but has no row there', row_id, 'cameras'
            USING ERRCODE = 'foreign_key_violation';
    END IF;
    RETURN NULL;
END;
$$;
ALTER FUNCTION require_cameras_row() OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION require_cameras_row() FROM PUBLIC;
GRANT SELECT ON cameras TO aps_writer;
CREATE CONSTRAINT TRIGGER cameras_row AFTER INSERT ON assets
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW WHEN (NEW.table_name_id = 'cameras')
    EXECUTE FUNCTION require_cameras_row();
CREATE CONSTRAINT TRIGGER cameras_kept AFTER DELETE ON cameras
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION require_cameras_row();

GRANT SELECT, INSERT, DELETE ON camera_models TO app;
ALTER TABLE camera_models ENABLE ROW LEVEL SECURITY;
ALTER TABLE camera_models FORCE ROW LEVEL SECURITY;
CREATE POLICY camera_models_select ON camera_models FOR SELECT TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY camera_models_insert ON camera_models FOR INSERT TO app
    WITH CHECK (id = ANY (ARRAY(SELECT authz.resources_with('writer'))));
CREATE POLICY camera_models_delete ON camera_models FOR DELETE TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('admin'))));
CREATE POLICY camera_models_archived_insert ON camera_models AS RESTRICTIVE FOR INSERT TO app
    WITH CHECK (NOT authz.is_archived(id));
CREATE POLICY camera_models_archived_delete ON camera_models AS RESTRICTIVE FOR DELETE TO app
    USING (NOT authz.is_archived(id));
CREATE POLICY camera_models_blocked ON camera_models AS RESTRICTIVE FOR ALL TO app
    USING (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))))
    WITH CHECK (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))));

GRANT SELECT, INSERT, DELETE ON commercial_camera_models TO app;
ALTER TABLE commercial_camera_models ENABLE ROW LEVEL SECURITY;
ALTER TABLE commercial_camera_models FORCE ROW LEVEL SECURITY;
CREATE POLICY commercial_camera_models_select ON commercial_camera_models FOR SELECT TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY commercial_camera_models_insert ON commercial_camera_models FOR INSERT TO app
    WITH CHECK (id = ANY (ARRAY(SELECT authz.resources_with('writer')))
        AND camera_model_id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY commercial_camera_models_delete ON commercial_camera_models FOR DELETE TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('admin'))));
CREATE POLICY commercial_camera_models_archived_insert ON commercial_camera_models AS RESTRICTIVE FOR INSERT TO app
    WITH CHECK (NOT authz.is_archived(id) AND NOT authz.is_archived(camera_model_id));
CREATE POLICY commercial_camera_models_archived_delete ON commercial_camera_models AS RESTRICTIVE FOR DELETE TO app
    USING (NOT authz.is_archived(id));
CREATE POLICY commercial_camera_models_blocked ON commercial_camera_models AS RESTRICTIVE FOR ALL TO app
    USING (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))))
    WITH CHECK (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))));

GRANT SELECT, INSERT, DELETE ON commercial_camera_lots TO app;
ALTER TABLE commercial_camera_lots ENABLE ROW LEVEL SECURITY;
ALTER TABLE commercial_camera_lots FORCE ROW LEVEL SECURITY;
CREATE POLICY commercial_camera_lots_select ON commercial_camera_lots FOR SELECT TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY commercial_camera_lots_insert ON commercial_camera_lots FOR INSERT TO app
    WITH CHECK (id = ANY (ARRAY(SELECT authz.resources_with('writer')))
        AND commercial_camera_model_id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY commercial_camera_lots_delete ON commercial_camera_lots FOR DELETE TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('admin'))));
CREATE POLICY commercial_camera_lots_archived_insert ON commercial_camera_lots AS RESTRICTIVE FOR INSERT TO app
    WITH CHECK (NOT authz.is_archived(id) AND NOT authz.is_archived(commercial_camera_model_id));
CREATE POLICY commercial_camera_lots_archived_delete ON commercial_camera_lots AS RESTRICTIVE FOR DELETE TO app
    USING (NOT authz.is_archived(id));
CREATE POLICY commercial_camera_lots_blocked ON commercial_camera_lots AS RESTRICTIVE FOR ALL TO app
    USING (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))))
    WITH CHECK (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))));

GRANT SELECT, INSERT, DELETE ON cameras TO app;
ALTER TABLE cameras ENABLE ROW LEVEL SECURITY;
ALTER TABLE cameras FORCE ROW LEVEL SECURITY;
CREATE POLICY cameras_select ON cameras FOR SELECT TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY cameras_insert ON cameras FOR INSERT TO app
    WITH CHECK (id = ANY (ARRAY(SELECT authz.resources_with('writer')))
        AND commercial_camera_lot_id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY cameras_delete ON cameras FOR DELETE TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('admin'))));
CREATE POLICY cameras_archived_insert ON cameras AS RESTRICTIVE FOR INSERT TO app
    WITH CHECK (NOT authz.is_archived(id) AND NOT authz.is_archived(commercial_camera_lot_id));
CREATE POLICY cameras_archived_delete ON cameras AS RESTRICTIVE FOR DELETE TO app
    USING (NOT authz.is_archived(id));
CREATE POLICY cameras_blocked ON cameras AS RESTRICTIVE FOR ALL TO app
    USING (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))))
    WITH CHECK (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))));

-- Records every change in connetto's `audit_events`.
CREATE TRIGGER camera_models_audit AFTER INSERT OR DELETE ON camera_models
    FOR EACH ROW EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER camera_models_update_audit AFTER UPDATE ON camera_models
    FOR EACH ROW WHEN (OLD IS DISTINCT FROM NEW) EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER commercial_camera_models_audit AFTER INSERT OR DELETE ON commercial_camera_models
    FOR EACH ROW EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER commercial_camera_models_update_audit AFTER UPDATE ON commercial_camera_models
    FOR EACH ROW WHEN (OLD IS DISTINCT FROM NEW) EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER commercial_camera_lots_audit AFTER INSERT OR DELETE ON commercial_camera_lots
    FOR EACH ROW EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER commercial_camera_lots_update_audit AFTER UPDATE ON commercial_camera_lots
    FOR EACH ROW WHEN (OLD IS DISTINCT FROM NEW) EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER cameras_audit AFTER INSERT OR DELETE ON cameras
    FOR EACH ROW EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER cameras_update_audit AFTER UPDATE ON cameras
    FOR EACH ROW WHEN (OLD IS DISTINCT FROM NEW) EXECUTE FUNCTION authz_write.record_audit_event();
