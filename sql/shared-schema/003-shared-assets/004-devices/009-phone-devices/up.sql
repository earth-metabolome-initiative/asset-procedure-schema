-- Migration: Shared Assets / Devices / Phone Devices
-- Purpose: Phone Devices for project inventories.
-- APS placement
--   `resources` of kind `'ownable'`
--       +-- asset_models
--           +-- physical_asset_models
--               +-- camera_models
--                   +-- phone_device_models
--                   +-- commercial_camera_models
--                       +-- commercial_phone_device_models
--               +-- commercial_product_lots
--                   +-- commercial_camera_lots
--                       +-- commercial_phone_device_lots
--       +-- assets
--           +-- physical_assets
--               +-- cameras
--                   +-- phone_devices
-- Commercial tables decision: Vendor models and lots provide purchase traceability.

-- Model catalog for phone-capable devices. A row exists only when the same model identity is valid as both a camera model and a geopositioning device model.
CREATE TABLE phone_device_models (
    -- Stable model identifier shared with parent camera/geopositioning models.
    id UUID PRIMARY KEY REFERENCES camera_models (id) ON DELETE CASCADE,
    -- Organization containing this resource.
    organization_id UUID NOT NULL,
    CONSTRAINT phone_device_models_geopositioning
    FOREIGN KEY (id) REFERENCES geopositioning_device_models (id) ON DELETE CASCADE,
    -- Archive state inherited from this resource.
    archived BOOLEAN NOT NULL DEFAULT FALSE,
    FOREIGN KEY (organization_id, id, archived) REFERENCES resources (organization_id, id, archived) ON UPDATE CASCADE ON DELETE CASCADE
);
INSERT INTO table_names (id) VALUES ('phone_device_models');
INSERT INTO table_name_ancestors (descendant_table_name_id, ancestor_table_name_id)
    SELECT 'phone_device_models', ancestor_table_name_id FROM table_name_ancestors WHERE descendant_table_name_id IN ('camera_models', 'geopositioning_device_models')
    UNION SELECT 'phone_device_models', 'phone_device_models';
CREATE TRIGGER phone_device_models_path AFTER INSERT ON phone_device_models
    FOR EACH ROW EXECUTE FUNCTION check_asset_model_subtype();

CREATE FUNCTION require_phone_device_models_row() RETURNS TRIGGER
LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
DECLARE
    row_id UUID := CASE TG_OP WHEN 'DELETE' THEN OLD.id ELSE NEW.id END;
BEGIN
    IF EXISTS (
        SELECT 1 FROM public.asset_models root
        JOIN public.table_name_ancestors path ON path.descendant_table_name_id = root.table_name_id
        WHERE root.id = row_id AND path.ancestor_table_name_id = 'phone_device_models'
    ) AND NOT EXISTS (SELECT 1 FROM public.phone_device_models subtype WHERE subtype.id = row_id) THEN
        RAISE EXCEPTION '% names a table extending % but has no row there', row_id, 'phone_device_models'
            USING ERRCODE = 'foreign_key_violation';
    END IF;
    RETURN NULL;
END;
$$;
ALTER FUNCTION require_phone_device_models_row() OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION require_phone_device_models_row() FROM PUBLIC;
REVOKE ALL ON phone_device_models FROM PUBLIC, app, identity_writer;
GRANT SELECT ON phone_device_models TO aps_writer;
CREATE CONSTRAINT TRIGGER phone_device_models_row AFTER INSERT ON asset_models
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW WHEN (NEW.table_name_id = 'phone_device_models')
    EXECUTE FUNCTION require_phone_device_models_row();
CREATE CONSTRAINT TRIGGER phone_device_models_kept AFTER DELETE ON phone_device_models
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION require_phone_device_models_row();

-- Commercial phone model catalog. This table is an id-only DAG bridge over commercial camera and commercial geopositioning models, so each row must satisfy both commercial ancestries.
CREATE TABLE commercial_phone_device_models (
    -- Stable commercial model identifier shared with both commercial parent models.
    id UUID PRIMARY KEY REFERENCES commercial_camera_models (id) ON DELETE CASCADE,
    -- Organization containing this resource.
    organization_id UUID NOT NULL,
    FOREIGN KEY (id) REFERENCES commercial_geopositioning_device_models (id) ON DELETE CASCADE,
    -- Archive state inherited from this resource.
    archived BOOLEAN NOT NULL DEFAULT FALSE,
    FOREIGN KEY (organization_id, id, archived) REFERENCES resources (organization_id, id, archived) ON UPDATE CASCADE ON DELETE CASCADE
);
INSERT INTO table_names (id) VALUES ('commercial_phone_device_models');
INSERT INTO table_name_ancestors (descendant_table_name_id, ancestor_table_name_id)
    SELECT 'commercial_phone_device_models', ancestor_table_name_id FROM table_name_ancestors WHERE descendant_table_name_id IN ('commercial_camera_models', 'commercial_geopositioning_device_models')
    UNION SELECT 'commercial_phone_device_models', 'commercial_phone_device_models';
CREATE TRIGGER commercial_phone_device_models_path AFTER INSERT ON commercial_phone_device_models
    FOR EACH ROW EXECUTE FUNCTION check_asset_model_subtype();

CREATE FUNCTION require_commercial_phone_device_models_row() RETURNS TRIGGER
LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
DECLARE
    row_id UUID := CASE TG_OP WHEN 'DELETE' THEN OLD.id ELSE NEW.id END;
BEGIN
    IF EXISTS (
        SELECT 1 FROM public.asset_models root
        JOIN public.table_name_ancestors path ON path.descendant_table_name_id = root.table_name_id
        WHERE root.id = row_id AND path.ancestor_table_name_id = 'commercial_phone_device_models'
    ) AND NOT EXISTS (SELECT 1 FROM public.commercial_phone_device_models subtype WHERE subtype.id = row_id) THEN
        RAISE EXCEPTION '% names a table extending % but has no row there', row_id, 'commercial_phone_device_models'
            USING ERRCODE = 'foreign_key_violation';
    END IF;
    RETURN NULL;
END;
$$;
ALTER FUNCTION require_commercial_phone_device_models_row() OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION require_commercial_phone_device_models_row() FROM PUBLIC;
REVOKE ALL ON commercial_phone_device_models FROM PUBLIC, app, identity_writer;
GRANT SELECT ON commercial_phone_device_models TO aps_writer;
CREATE CONSTRAINT TRIGGER commercial_phone_device_models_row AFTER INSERT ON asset_models
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW WHEN (NEW.table_name_id = 'commercial_phone_device_models')
    EXECUTE FUNCTION require_commercial_phone_device_models_row();
CREATE CONSTRAINT TRIGGER commercial_phone_device_models_kept AFTER DELETE ON commercial_phone_device_models
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION require_commercial_phone_device_models_row();

-- Commercial lot catalog for phone devices. A row exists only when the same lot identity is valid as both a commercial camera lot and a commercial geopositioning-device lot.
CREATE TABLE commercial_phone_device_lots (
    -- Stable lot identifier shared with both commercial parent lot tables.
    id UUID PRIMARY KEY REFERENCES commercial_camera_lots (id) ON DELETE CASCADE,
    -- Organization containing this resource.
    organization_id UUID NOT NULL,
    CONSTRAINT commercial_phone_device_lots_geopositioning
    FOREIGN KEY (id) REFERENCES commercial_geopositioning_device_lots (id) ON DELETE CASCADE,
    -- Archive state inherited from this resource.
    archived BOOLEAN NOT NULL DEFAULT FALSE,
    UNIQUE (id, archived),
    FOREIGN KEY (organization_id, id, archived) REFERENCES resources (organization_id, id, archived) ON UPDATE CASCADE ON DELETE CASCADE
);
INSERT INTO table_names (id) VALUES ('commercial_phone_device_lots');
INSERT INTO table_name_ancestors (descendant_table_name_id, ancestor_table_name_id)
    SELECT 'commercial_phone_device_lots', ancestor_table_name_id FROM table_name_ancestors WHERE descendant_table_name_id IN ('commercial_camera_lots', 'commercial_geopositioning_device_lots')
    UNION SELECT 'commercial_phone_device_lots', 'commercial_phone_device_lots';
INSERT INTO table_name_admitted_parents (table_name_id, parent_table_name_id)
    VALUES ('commercial_phone_device_lots', 'commercial_phone_device_models');
CREATE TRIGGER commercial_phone_device_lots_path AFTER INSERT ON commercial_phone_device_lots
    FOR EACH ROW EXECUTE FUNCTION check_asset_model_subtype();

CREATE FUNCTION require_commercial_phone_device_lots_row() RETURNS TRIGGER
LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
DECLARE
    row_id UUID := CASE TG_OP WHEN 'DELETE' THEN OLD.id ELSE NEW.id END;
BEGIN
    IF EXISTS (
        SELECT 1 FROM public.asset_models root
        JOIN public.table_name_ancestors path ON path.descendant_table_name_id = root.table_name_id
        WHERE root.id = row_id AND path.ancestor_table_name_id = 'commercial_phone_device_lots'
    ) AND NOT EXISTS (SELECT 1 FROM public.commercial_phone_device_lots subtype WHERE subtype.id = row_id) THEN
        RAISE EXCEPTION '% names a table extending % but has no row there', row_id, 'commercial_phone_device_lots'
            USING ERRCODE = 'foreign_key_violation';
    END IF;
    RETURN NULL;
END;
$$;
ALTER FUNCTION require_commercial_phone_device_lots_row() OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION require_commercial_phone_device_lots_row() FROM PUBLIC;
REVOKE ALL ON commercial_phone_device_lots FROM PUBLIC, app, identity_writer;
GRANT SELECT ON commercial_phone_device_lots TO aps_writer;
CREATE CONSTRAINT TRIGGER commercial_phone_device_lots_row AFTER INSERT ON asset_models
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW WHEN (NEW.table_name_id = 'commercial_phone_device_lots')
    EXECUTE FUNCTION require_commercial_phone_device_lots_row();
CREATE CONSTRAINT TRIGGER commercial_phone_device_lots_kept AFTER DELETE ON commercial_phone_device_lots
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION require_commercial_phone_device_lots_row();

-- Physical phone devices tracked in inventory and procedure execution. A row exists only when the same physical asset identity is valid as both a camera and a geopositioning device.
CREATE TABLE phone_devices (
    -- Stable device identifier shared with parent camera/geopositioning devices.
    id UUID PRIMARY KEY REFERENCES cameras (id) ON DELETE CASCADE,
    -- Organization containing this resource.
    organization_id UUID NOT NULL,
    -- Commercial lot model instantiated by this physical asset.
    commercial_phone_device_lot_id UUID NOT NULL REFERENCES commercial_phone_device_lots (id),
    CONSTRAINT phone_devices_geopositioning
    FOREIGN KEY (id) REFERENCES geopositioning_devices (id) ON DELETE CASCADE,
    FOREIGN KEY (id, commercial_phone_device_lot_id) REFERENCES assets (id, model_id) ON DELETE CASCADE,
    -- Archive state inherited from this resource.
    archived BOOLEAN NOT NULL DEFAULT FALSE,
    -- Archive state of the row named by `commercial_phone_device_lot_id`.
    commercial_phone_device_lot_archived BOOLEAN NOT NULL DEFAULT FALSE,
    FOREIGN KEY (commercial_phone_device_lot_id, commercial_phone_device_lot_archived) REFERENCES commercial_phone_device_lots (id, archived) ON UPDATE CASCADE,
    FOREIGN KEY (organization_id, id, archived) REFERENCES resources (organization_id, id, archived) ON UPDATE CASCADE ON DELETE CASCADE
);
CREATE INDEX ON phone_devices (commercial_phone_device_lot_id);
INSERT INTO table_names (id) VALUES ('phone_devices');
UPDATE table_names SET instance_table_name_id = 'phone_devices'
    WHERE id = 'commercial_phone_device_lots';
INSERT INTO table_name_ancestors (descendant_table_name_id, ancestor_table_name_id)
    SELECT 'phone_devices', ancestor_table_name_id FROM table_name_ancestors WHERE descendant_table_name_id IN ('cameras', 'geopositioning_devices')
    UNION SELECT 'phone_devices', 'phone_devices';
CREATE TRIGGER phone_devices_path AFTER INSERT ON phone_devices
    FOR EACH ROW EXECUTE FUNCTION check_asset_subtype();

CREATE FUNCTION require_phone_devices_row() RETURNS TRIGGER
LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
DECLARE
    row_id UUID := CASE TG_OP WHEN 'DELETE' THEN OLD.id ELSE NEW.id END;
BEGIN
    IF EXISTS (
        SELECT 1 FROM public.assets root
        JOIN public.table_name_ancestors path ON path.descendant_table_name_id = root.table_name_id
        WHERE root.id = row_id AND path.ancestor_table_name_id = 'phone_devices'
    ) AND NOT EXISTS (SELECT 1 FROM public.phone_devices subtype WHERE subtype.id = row_id) THEN
        RAISE EXCEPTION '% names a table extending % but has no row there', row_id, 'phone_devices'
            USING ERRCODE = 'foreign_key_violation';
    END IF;
    RETURN NULL;
END;
$$;
ALTER FUNCTION require_phone_devices_row() OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION require_phone_devices_row() FROM PUBLIC;
REVOKE ALL ON phone_devices FROM PUBLIC, app, identity_writer;
GRANT SELECT ON phone_devices TO aps_writer;
CREATE CONSTRAINT TRIGGER phone_devices_row AFTER INSERT ON assets
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW WHEN (NEW.table_name_id = 'phone_devices')
    EXECUTE FUNCTION require_phone_devices_row();
CREATE CONSTRAINT TRIGGER phone_devices_kept AFTER DELETE ON phone_devices
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION require_phone_devices_row();

GRANT SELECT, INSERT, DELETE ON phone_device_models TO app;
ALTER TABLE phone_device_models ENABLE ROW LEVEL SECURITY;
ALTER TABLE phone_device_models FORCE ROW LEVEL SECURITY;
CREATE POLICY phone_device_models_select ON phone_device_models FOR SELECT TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY phone_device_models_insert ON phone_device_models FOR INSERT TO app
    WITH CHECK (id = ANY (ARRAY(SELECT authz.resources_with('writer'))));
CREATE POLICY phone_device_models_delete ON phone_device_models FOR DELETE TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('admin'))));
CREATE POLICY phone_device_models_archived_insert ON phone_device_models AS RESTRICTIVE FOR INSERT TO app
    WITH CHECK (NOT archived);
CREATE POLICY phone_device_models_archived_delete ON phone_device_models AS RESTRICTIVE FOR DELETE TO app
    USING (NOT archived);
CREATE POLICY phone_device_models_blocked ON phone_device_models AS RESTRICTIVE FOR ALL TO app
    USING (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))))
    WITH CHECK (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))));

GRANT SELECT, INSERT, DELETE ON commercial_phone_device_models TO app;
ALTER TABLE commercial_phone_device_models ENABLE ROW LEVEL SECURITY;
ALTER TABLE commercial_phone_device_models FORCE ROW LEVEL SECURITY;
CREATE POLICY commercial_phone_device_models_select ON commercial_phone_device_models FOR SELECT TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY commercial_phone_device_models_insert ON commercial_phone_device_models FOR INSERT TO app
    WITH CHECK (id = ANY (ARRAY(SELECT authz.resources_with('writer'))));
CREATE POLICY commercial_phone_device_models_delete ON commercial_phone_device_models FOR DELETE TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('admin'))));
CREATE POLICY commercial_phone_device_models_archived_insert ON commercial_phone_device_models AS RESTRICTIVE FOR INSERT TO app
    WITH CHECK (NOT archived);
CREATE POLICY commercial_phone_device_models_archived_delete ON commercial_phone_device_models AS RESTRICTIVE FOR DELETE TO app
    USING (NOT archived);
CREATE POLICY commercial_phone_device_models_blocked ON commercial_phone_device_models AS RESTRICTIVE FOR ALL TO app
    USING (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))))
    WITH CHECK (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))));

GRANT SELECT, INSERT, DELETE ON commercial_phone_device_lots TO app;
ALTER TABLE commercial_phone_device_lots ENABLE ROW LEVEL SECURITY;
ALTER TABLE commercial_phone_device_lots FORCE ROW LEVEL SECURITY;
CREATE POLICY commercial_phone_device_lots_select ON commercial_phone_device_lots FOR SELECT TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY commercial_phone_device_lots_insert ON commercial_phone_device_lots FOR INSERT TO app
    WITH CHECK (id = ANY (ARRAY(SELECT authz.resources_with('writer'))));
CREATE POLICY commercial_phone_device_lots_delete ON commercial_phone_device_lots FOR DELETE TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('admin'))));
CREATE POLICY commercial_phone_device_lots_archived_insert ON commercial_phone_device_lots AS RESTRICTIVE FOR INSERT TO app
    WITH CHECK (NOT archived);
CREATE POLICY commercial_phone_device_lots_archived_delete ON commercial_phone_device_lots AS RESTRICTIVE FOR DELETE TO app
    USING (NOT archived);
CREATE POLICY commercial_phone_device_lots_blocked ON commercial_phone_device_lots AS RESTRICTIVE FOR ALL TO app
    USING (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))))
    WITH CHECK (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))));

GRANT SELECT, INSERT, DELETE ON phone_devices TO app;
ALTER TABLE phone_devices ENABLE ROW LEVEL SECURITY;
ALTER TABLE phone_devices FORCE ROW LEVEL SECURITY;
CREATE POLICY phone_devices_select ON phone_devices FOR SELECT TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY phone_devices_insert ON phone_devices FOR INSERT TO app
    WITH CHECK (id = ANY (ARRAY(SELECT authz.resources_with('writer')))
        AND commercial_phone_device_lot_id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY phone_devices_delete ON phone_devices FOR DELETE TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('admin'))));
CREATE POLICY phone_devices_archived_insert ON phone_devices AS RESTRICTIVE FOR INSERT TO app
    WITH CHECK (NOT archived AND NOT commercial_phone_device_lot_archived);
CREATE POLICY phone_devices_archived_delete ON phone_devices AS RESTRICTIVE FOR DELETE TO app
    USING (NOT archived);
CREATE POLICY phone_devices_blocked ON phone_devices AS RESTRICTIVE FOR ALL TO app
    USING (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))))
    WITH CHECK (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))));

-- Records every change in connetto's `audit_events`.
CREATE TRIGGER phone_device_models_audit AFTER INSERT OR DELETE ON phone_device_models
    FOR EACH ROW EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER phone_device_models_update_audit AFTER UPDATE ON phone_device_models
    FOR EACH ROW WHEN (OLD IS DISTINCT FROM NEW) EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER commercial_phone_device_models_audit AFTER INSERT OR DELETE ON commercial_phone_device_models
    FOR EACH ROW EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER commercial_phone_device_models_update_audit AFTER UPDATE ON commercial_phone_device_models
    FOR EACH ROW WHEN (OLD IS DISTINCT FROM NEW) EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER commercial_phone_device_lots_audit AFTER INSERT OR DELETE ON commercial_phone_device_lots
    FOR EACH ROW EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER commercial_phone_device_lots_update_audit AFTER UPDATE ON commercial_phone_device_lots
    FOR EACH ROW WHEN (OLD IS DISTINCT FROM NEW) EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER phone_devices_audit AFTER INSERT OR DELETE ON phone_devices
    FOR EACH ROW EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER phone_devices_update_audit AFTER UPDATE ON phone_devices
    FOR EACH ROW WHEN (OLD IS DISTINCT FROM NEW) EXECUTE FUNCTION authz_write.record_audit_event();
