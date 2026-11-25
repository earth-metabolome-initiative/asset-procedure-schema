-- Migration: Shared Assets / Devices / Volume Measuring Devices
-- Purpose: Volume Measuring Devices for project inventories.
-- APS placement
--   ownables
--       +-- asset_models
--           +-- physical_asset_models
--               +-- volume_measuring_device_models
--                   +-- commercial_volume_measuring_device_models
--               +-- commercial_product_lots
--                   +-- commercial_volume_measuring_device_lots
--       +-- assets
--           +-- physical_assets
--               +-- volume_measuring_devices
-- Commercial tables decision: Vendor models and lots provide purchase traceability.

-- Catalog of volume-measuring device model definitions.
CREATE TABLE volume_measuring_device_models (
    -- Stable model identifier inherited from `physical_asset_models`.
    id UUID PRIMARY KEY REFERENCES physical_asset_models (id) ON DELETE CASCADE,
    -- Organization containing this resource.
    organization_id UUID NOT NULL,
    FOREIGN KEY (organization_id, id) REFERENCES resources (organization_id, id) ON DELETE CASCADE
);
INSERT INTO table_names (id) VALUES ('volume_measuring_device_models');
INSERT INTO table_name_ancestors (descendant_table_name_id, ancestor_table_name_id)
    SELECT 'volume_measuring_device_models', ancestor_table_name_id FROM table_name_ancestors WHERE descendant_table_name_id IN ('physical_asset_models')
    UNION SELECT 'volume_measuring_device_models', 'volume_measuring_device_models';
CREATE TRIGGER volume_measuring_device_models_path AFTER INSERT ON volume_measuring_device_models
    FOR EACH ROW EXECUTE FUNCTION check_asset_model_subtype();

CREATE FUNCTION require_volume_measuring_device_models_row() RETURNS TRIGGER
LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
DECLARE
    row_id UUID := CASE TG_OP WHEN 'DELETE' THEN OLD.id ELSE NEW.id END;
BEGIN
    IF EXISTS (
        SELECT 1 FROM public.asset_models root
        JOIN public.table_name_ancestors path ON path.descendant_table_name_id = root.table_name_id
        WHERE root.id = row_id AND path.ancestor_table_name_id = 'volume_measuring_device_models'
    ) AND NOT EXISTS (SELECT 1 FROM public.volume_measuring_device_models subtype WHERE subtype.id = row_id) THEN
        RAISE EXCEPTION '% names a table extending % but has no row there', row_id, 'volume_measuring_device_models'
            USING ERRCODE = 'foreign_key_violation';
    END IF;
    RETURN NULL;
END;
$$;
ALTER FUNCTION require_volume_measuring_device_models_row() OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION require_volume_measuring_device_models_row() FROM PUBLIC;
REVOKE ALL ON volume_measuring_device_models FROM PUBLIC, app, identity_writer;
GRANT SELECT ON volume_measuring_device_models TO aps_writer;
CREATE CONSTRAINT TRIGGER volume_measuring_device_models_row AFTER INSERT ON asset_models
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW WHEN (NEW.table_name_id = 'volume_measuring_device_models')
    EXECUTE FUNCTION require_volume_measuring_device_models_row();
CREATE CONSTRAINT TRIGGER volume_measuring_device_models_kept AFTER DELETE ON volume_measuring_device_models
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION require_volume_measuring_device_models_row();

-- Catalog of commercial volume-measuring device models.
CREATE TABLE commercial_volume_measuring_device_models (
    -- Stable commercial model identifier shared with parent model tables.
    id UUID PRIMARY KEY REFERENCES volume_measuring_device_models (id) ON DELETE CASCADE,
    -- Organization containing this resource.
    organization_id UUID NOT NULL,
    -- Base volume-measuring model represented by this commercial model.
    volume_measuring_device_model_id UUID NOT NULL REFERENCES volume_measuring_device_models (id) ON DELETE RESTRICT,
    FOREIGN KEY (id) REFERENCES commercial_products (id) ON DELETE CASCADE,
    FOREIGN KEY (id, volume_measuring_device_model_id) REFERENCES asset_models (id, parent_model_id) ON DELETE CASCADE,
    FOREIGN KEY (organization_id, id) REFERENCES resources (organization_id, id) ON DELETE CASCADE
);
CREATE INDEX ON commercial_volume_measuring_device_models (volume_measuring_device_model_id);
INSERT INTO table_names (id) VALUES ('commercial_volume_measuring_device_models');
INSERT INTO table_name_ancestors (descendant_table_name_id, ancestor_table_name_id)
    SELECT 'commercial_volume_measuring_device_models', ancestor_table_name_id FROM table_name_ancestors WHERE descendant_table_name_id IN ('volume_measuring_device_models', 'commercial_products')
    UNION SELECT 'commercial_volume_measuring_device_models', 'commercial_volume_measuring_device_models';
CREATE TRIGGER commercial_volume_measuring_device_models_path AFTER INSERT ON commercial_volume_measuring_device_models
    FOR EACH ROW EXECUTE FUNCTION check_asset_model_subtype();

CREATE FUNCTION require_commercial_volume_measuring_device_models_row() RETURNS TRIGGER
LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
DECLARE
    row_id UUID := CASE TG_OP WHEN 'DELETE' THEN OLD.id ELSE NEW.id END;
BEGIN
    IF EXISTS (
        SELECT 1 FROM public.asset_models root
        JOIN public.table_name_ancestors path ON path.descendant_table_name_id = root.table_name_id
        WHERE root.id = row_id AND path.ancestor_table_name_id = 'commercial_volume_measuring_device_models'
    ) AND NOT EXISTS (SELECT 1 FROM public.commercial_volume_measuring_device_models subtype WHERE subtype.id = row_id) THEN
        RAISE EXCEPTION '% names a table extending % but has no row there', row_id, 'commercial_volume_measuring_device_models'
            USING ERRCODE = 'foreign_key_violation';
    END IF;
    RETURN NULL;
END;
$$;
ALTER FUNCTION require_commercial_volume_measuring_device_models_row() OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION require_commercial_volume_measuring_device_models_row() FROM PUBLIC;
REVOKE ALL ON commercial_volume_measuring_device_models FROM PUBLIC, app, identity_writer;
GRANT SELECT ON commercial_volume_measuring_device_models TO aps_writer;
CREATE CONSTRAINT TRIGGER commercial_volume_measuring_device_models_row AFTER INSERT ON asset_models
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW WHEN (NEW.table_name_id = 'commercial_volume_measuring_device_models')
    EXECUTE FUNCTION require_commercial_volume_measuring_device_models_row();
CREATE CONSTRAINT TRIGGER commercial_volume_measuring_device_models_kept AFTER DELETE ON commercial_volume_measuring_device_models
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION require_commercial_volume_measuring_device_models_row();

-- Catalog of lot-specific commercial volume-measuring device models.
CREATE TABLE commercial_volume_measuring_device_lots (
    -- Stable lot-model identifier shared with parent lot/model tables.
    id UUID PRIMARY KEY REFERENCES commercial_product_lots (id) ON DELETE CASCADE,
    -- Organization containing this resource.
    organization_id UUID NOT NULL,
    FOREIGN KEY (id) REFERENCES volume_measuring_device_models (id) ON DELETE CASCADE,
    -- Commercial volume-measuring model from which this lot derives.
    commercial_volume_measuring_device_model_id UUID NOT NULL REFERENCES commercial_volume_measuring_device_models (id),
    FOREIGN KEY (id, commercial_volume_measuring_device_model_id) REFERENCES asset_models (id, parent_model_id) ON DELETE CASCADE,
    FOREIGN KEY (organization_id, id) REFERENCES resources (organization_id, id) ON DELETE CASCADE
);
CREATE INDEX ON commercial_volume_measuring_device_lots (commercial_volume_measuring_device_model_id);
INSERT INTO table_names (id) VALUES ('commercial_volume_measuring_device_lots');
INSERT INTO table_name_ancestors (descendant_table_name_id, ancestor_table_name_id)
    SELECT 'commercial_volume_measuring_device_lots', ancestor_table_name_id FROM table_name_ancestors WHERE descendant_table_name_id IN ('commercial_product_lots', 'volume_measuring_device_models')
    UNION SELECT 'commercial_volume_measuring_device_lots', 'commercial_volume_measuring_device_lots';
INSERT INTO table_name_admitted_parents (table_name_id, parent_table_name_id)
    VALUES ('commercial_volume_measuring_device_lots', 'commercial_volume_measuring_device_models');
CREATE TRIGGER commercial_volume_measuring_device_lots_path AFTER INSERT ON commercial_volume_measuring_device_lots
    FOR EACH ROW EXECUTE FUNCTION check_asset_model_subtype();

CREATE FUNCTION require_commercial_volume_measuring_device_lots_row() RETURNS TRIGGER
LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
DECLARE
    row_id UUID := CASE TG_OP WHEN 'DELETE' THEN OLD.id ELSE NEW.id END;
BEGIN
    IF EXISTS (
        SELECT 1 FROM public.asset_models root
        JOIN public.table_name_ancestors path ON path.descendant_table_name_id = root.table_name_id
        WHERE root.id = row_id AND path.ancestor_table_name_id = 'commercial_volume_measuring_device_lots'
    ) AND NOT EXISTS (SELECT 1 FROM public.commercial_volume_measuring_device_lots subtype WHERE subtype.id = row_id) THEN
        RAISE EXCEPTION '% names a table extending % but has no row there', row_id, 'commercial_volume_measuring_device_lots'
            USING ERRCODE = 'foreign_key_violation';
    END IF;
    RETURN NULL;
END;
$$;
ALTER FUNCTION require_commercial_volume_measuring_device_lots_row() OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION require_commercial_volume_measuring_device_lots_row() FROM PUBLIC;
REVOKE ALL ON commercial_volume_measuring_device_lots FROM PUBLIC, app, identity_writer;
GRANT SELECT ON commercial_volume_measuring_device_lots TO aps_writer;
CREATE CONSTRAINT TRIGGER commercial_volume_measuring_device_lots_row AFTER INSERT ON asset_models
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW WHEN (NEW.table_name_id = 'commercial_volume_measuring_device_lots')
    EXECUTE FUNCTION require_commercial_volume_measuring_device_lots_row();
CREATE CONSTRAINT TRIGGER commercial_volume_measuring_device_lots_kept AFTER DELETE ON commercial_volume_measuring_device_lots
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION require_commercial_volume_measuring_device_lots_row();

-- Physical volume-measuring devices tracked in APS inventory.
CREATE TABLE volume_measuring_devices (
    -- Stable asset identifier inherited from `physical_assets`.
    id UUID PRIMARY KEY REFERENCES physical_assets (id) ON DELETE CASCADE,
    -- Organization containing this resource.
    organization_id UUID NOT NULL,
    -- Volume-measuring model instantiated by this physical asset.
    volume_measuring_device_model_id UUID NOT NULL REFERENCES volume_measuring_device_models (id),
    FOREIGN KEY (id, volume_measuring_device_model_id) REFERENCES assets (id, model_id) ON DELETE CASCADE,
    FOREIGN KEY (organization_id, id) REFERENCES resources (organization_id, id) ON DELETE CASCADE
);
CREATE INDEX ON volume_measuring_devices (volume_measuring_device_model_id);
INSERT INTO table_names (id) VALUES ('volume_measuring_devices');
INSERT INTO table_name_instances (id, instance_table_name_id)
    VALUES ('volume_measuring_device_models', 'volume_measuring_devices');
INSERT INTO table_name_ancestors (descendant_table_name_id, ancestor_table_name_id)
    SELECT 'volume_measuring_devices', ancestor_table_name_id FROM table_name_ancestors WHERE descendant_table_name_id IN ('physical_assets')
    UNION SELECT 'volume_measuring_devices', 'volume_measuring_devices';
CREATE TRIGGER volume_measuring_devices_path AFTER INSERT ON volume_measuring_devices
    FOR EACH ROW EXECUTE FUNCTION check_asset_subtype();

CREATE FUNCTION require_volume_measuring_devices_row() RETURNS TRIGGER
LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
DECLARE
    row_id UUID := CASE TG_OP WHEN 'DELETE' THEN OLD.id ELSE NEW.id END;
BEGIN
    IF EXISTS (
        SELECT 1 FROM public.assets root
        JOIN public.table_name_ancestors path ON path.descendant_table_name_id = root.table_name_id
        WHERE root.id = row_id AND path.ancestor_table_name_id = 'volume_measuring_devices'
    ) AND NOT EXISTS (SELECT 1 FROM public.volume_measuring_devices subtype WHERE subtype.id = row_id) THEN
        RAISE EXCEPTION '% names a table extending % but has no row there', row_id, 'volume_measuring_devices'
            USING ERRCODE = 'foreign_key_violation';
    END IF;
    RETURN NULL;
END;
$$;
ALTER FUNCTION require_volume_measuring_devices_row() OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION require_volume_measuring_devices_row() FROM PUBLIC;
REVOKE ALL ON volume_measuring_devices FROM PUBLIC, app, identity_writer;
GRANT SELECT ON volume_measuring_devices TO aps_writer;
CREATE CONSTRAINT TRIGGER volume_measuring_devices_row AFTER INSERT ON assets
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW WHEN (NEW.table_name_id = 'volume_measuring_devices')
    EXECUTE FUNCTION require_volume_measuring_devices_row();
CREATE CONSTRAINT TRIGGER volume_measuring_devices_kept AFTER DELETE ON volume_measuring_devices
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION require_volume_measuring_devices_row();

GRANT SELECT, INSERT, DELETE ON volume_measuring_device_models TO app;
ALTER TABLE volume_measuring_device_models ENABLE ROW LEVEL SECURITY;
ALTER TABLE volume_measuring_device_models FORCE ROW LEVEL SECURITY;
CREATE POLICY volume_measuring_device_models_select ON volume_measuring_device_models FOR SELECT TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY volume_measuring_device_models_insert ON volume_measuring_device_models FOR INSERT TO app
    WITH CHECK (id = ANY (ARRAY(SELECT authz.resources_with('writer'))));
CREATE POLICY volume_measuring_device_models_delete ON volume_measuring_device_models FOR DELETE TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('admin'))));
CREATE POLICY volume_measuring_device_models_archived_insert ON volume_measuring_device_models AS RESTRICTIVE FOR INSERT TO app
    WITH CHECK (NOT authz.is_archived(id));
CREATE POLICY volume_measuring_device_models_archived_delete ON volume_measuring_device_models AS RESTRICTIVE FOR DELETE TO app
    USING (NOT authz.is_archived(id));
CREATE POLICY volume_measuring_device_models_blocked ON volume_measuring_device_models AS RESTRICTIVE FOR ALL TO app
    USING (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))))
    WITH CHECK (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))));

GRANT SELECT, INSERT, DELETE ON commercial_volume_measuring_device_models TO app;
ALTER TABLE commercial_volume_measuring_device_models ENABLE ROW LEVEL SECURITY;
ALTER TABLE commercial_volume_measuring_device_models FORCE ROW LEVEL SECURITY;
CREATE POLICY commercial_volume_measuring_device_models_select ON commercial_volume_measuring_device_models FOR SELECT TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY commercial_volume_measuring_device_models_insert ON commercial_volume_measuring_device_models FOR INSERT TO app
    WITH CHECK (id = ANY (ARRAY(SELECT authz.resources_with('writer')))
        AND volume_measuring_device_model_id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY commercial_volume_measuring_device_models_delete ON commercial_volume_measuring_device_models FOR DELETE TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('admin'))));
CREATE POLICY commercial_volume_measuring_device_models_archived_insert ON commercial_volume_measuring_device_models AS RESTRICTIVE FOR INSERT TO app
    WITH CHECK (NOT authz.is_archived(id) AND NOT authz.is_archived(volume_measuring_device_model_id));
CREATE POLICY commercial_volume_measuring_device_models_archived_delete ON commercial_volume_measuring_device_models AS RESTRICTIVE FOR DELETE TO app
    USING (NOT authz.is_archived(id));
CREATE POLICY commercial_volume_measuring_device_models_blocked ON commercial_volume_measuring_device_models AS RESTRICTIVE FOR ALL TO app
    USING (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))))
    WITH CHECK (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))));

GRANT SELECT, INSERT, DELETE ON commercial_volume_measuring_device_lots TO app;
ALTER TABLE commercial_volume_measuring_device_lots ENABLE ROW LEVEL SECURITY;
ALTER TABLE commercial_volume_measuring_device_lots FORCE ROW LEVEL SECURITY;
CREATE POLICY commercial_volume_measuring_device_lots_select ON commercial_volume_measuring_device_lots FOR SELECT TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY commercial_volume_measuring_device_lots_insert ON commercial_volume_measuring_device_lots FOR INSERT TO app
    WITH CHECK (id = ANY (ARRAY(SELECT authz.resources_with('writer')))
        AND commercial_volume_measuring_device_model_id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY commercial_volume_measuring_device_lots_delete ON commercial_volume_measuring_device_lots FOR DELETE TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('admin'))));
CREATE POLICY commercial_volume_measuring_device_lots_archived_insert ON commercial_volume_measuring_device_lots AS RESTRICTIVE FOR INSERT TO app
    WITH CHECK (NOT authz.is_archived(id) AND NOT authz.is_archived(commercial_volume_measuring_device_model_id));
CREATE POLICY commercial_volume_measuring_device_lots_archived_delete ON commercial_volume_measuring_device_lots AS RESTRICTIVE FOR DELETE TO app
    USING (NOT authz.is_archived(id));
CREATE POLICY commercial_volume_measuring_device_lots_blocked ON commercial_volume_measuring_device_lots AS RESTRICTIVE FOR ALL TO app
    USING (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))))
    WITH CHECK (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))));

GRANT SELECT, INSERT, DELETE ON volume_measuring_devices TO app;
ALTER TABLE volume_measuring_devices ENABLE ROW LEVEL SECURITY;
ALTER TABLE volume_measuring_devices FORCE ROW LEVEL SECURITY;
CREATE POLICY volume_measuring_devices_select ON volume_measuring_devices FOR SELECT TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY volume_measuring_devices_insert ON volume_measuring_devices FOR INSERT TO app
    WITH CHECK (id = ANY (ARRAY(SELECT authz.resources_with('writer')))
        AND volume_measuring_device_model_id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY volume_measuring_devices_delete ON volume_measuring_devices FOR DELETE TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('admin'))));
CREATE POLICY volume_measuring_devices_archived_insert ON volume_measuring_devices AS RESTRICTIVE FOR INSERT TO app
    WITH CHECK (NOT authz.is_archived(id) AND NOT authz.is_archived(volume_measuring_device_model_id));
CREATE POLICY volume_measuring_devices_archived_delete ON volume_measuring_devices AS RESTRICTIVE FOR DELETE TO app
    USING (NOT authz.is_archived(id));
CREATE POLICY volume_measuring_devices_blocked ON volume_measuring_devices AS RESTRICTIVE FOR ALL TO app
    USING (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))))
    WITH CHECK (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))));

-- Records every change in connetto's `audit_events`.
CREATE TRIGGER volume_measuring_device_models_audit AFTER INSERT OR DELETE ON volume_measuring_device_models
    FOR EACH ROW EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER volume_measuring_device_models_update_audit AFTER UPDATE ON volume_measuring_device_models
    FOR EACH ROW WHEN (OLD IS DISTINCT FROM NEW) EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER commercial_volume_measuring_device_models_audit AFTER INSERT OR DELETE ON commercial_volume_measuring_device_models
    FOR EACH ROW EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER commercial_volume_measuring_device_models_update_audit AFTER UPDATE ON commercial_volume_measuring_device_models
    FOR EACH ROW WHEN (OLD IS DISTINCT FROM NEW) EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER commercial_volume_measuring_device_lots_audit AFTER INSERT OR DELETE ON commercial_volume_measuring_device_lots
    FOR EACH ROW EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER commercial_volume_measuring_device_lots_update_audit AFTER UPDATE ON commercial_volume_measuring_device_lots
    FOR EACH ROW WHEN (OLD IS DISTINCT FROM NEW) EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER volume_measuring_devices_audit AFTER INSERT OR DELETE ON volume_measuring_devices
    FOR EACH ROW EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER volume_measuring_devices_update_audit AFTER UPDATE ON volume_measuring_devices
    FOR EACH ROW WHEN (OLD IS DISTINCT FROM NEW) EXECUTE FUNCTION authz_write.record_audit_event();
