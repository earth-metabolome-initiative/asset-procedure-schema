-- Migration: Shared Assets / Devices / Weighing Devices
-- Purpose: Weighing Devices for project inventories.
-- APS placement
--   `resources` of kind `'ownable'`
--       +-- asset_models
--           +-- physical_asset_models
--               +-- weighing_device_models
--                   +-- commercial_weighing_device_models
--               +-- commercial_product_lots
--                   +-- commercial_weighing_device_lots
--       +-- assets
--           +-- physical_assets
--               +-- weighing_devices
-- Commercial tables decision: Vendor models and lots provide purchase traceability.

-- Catalog of weighing-device model definitions.
CREATE TABLE weighing_device_models (
    -- Stable model identifier inherited from `physical_asset_models`.
    id UUID PRIMARY KEY REFERENCES physical_asset_models (id) ON DELETE CASCADE,
    -- Organization containing this resource.
    organization_id UUID NOT NULL,
    archived BOOLEAN NOT NULL DEFAULT FALSE,
    UNIQUE (id, archived),
    FOREIGN KEY (organization_id, id, archived) REFERENCES resources (organization_id, id, archived) ON UPDATE CASCADE ON DELETE CASCADE
);
INSERT INTO table_names (id) VALUES ('weighing_device_models');
INSERT INTO table_name_ancestors (descendant_table_name_id, ancestor_table_name_id)
    SELECT 'weighing_device_models', ancestor_table_name_id FROM table_name_ancestors WHERE descendant_table_name_id IN ('physical_asset_models')
    UNION SELECT 'weighing_device_models', 'weighing_device_models';
CREATE TRIGGER weighing_device_models_path AFTER INSERT ON weighing_device_models
    FOR EACH ROW EXECUTE FUNCTION check_asset_model_subtype();

CREATE FUNCTION require_weighing_device_models_row() RETURNS TRIGGER
LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
DECLARE
    row_id UUID := CASE TG_OP WHEN 'DELETE' THEN OLD.id ELSE NEW.id END;
BEGIN
    IF EXISTS (
        SELECT 1 FROM public.asset_models root
        JOIN public.table_name_ancestors path ON path.descendant_table_name_id = root.table_name_id
        WHERE root.id = row_id AND path.ancestor_table_name_id = 'weighing_device_models'
    ) AND NOT EXISTS (SELECT 1 FROM public.weighing_device_models subtype WHERE subtype.id = row_id) THEN
        RAISE EXCEPTION '% names a table extending % but has no row there', row_id, 'weighing_device_models'
            USING ERRCODE = 'foreign_key_violation';
    END IF;
    RETURN NULL;
END;
$$;
ALTER FUNCTION require_weighing_device_models_row() OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION require_weighing_device_models_row() FROM PUBLIC;
REVOKE ALL ON weighing_device_models FROM PUBLIC, app, identity_writer;
GRANT SELECT ON weighing_device_models TO aps_writer;
CREATE CONSTRAINT TRIGGER weighing_device_models_row AFTER INSERT ON asset_models
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW WHEN (NEW.table_name_id = 'weighing_device_models')
    EXECUTE FUNCTION require_weighing_device_models_row();
CREATE CONSTRAINT TRIGGER weighing_device_models_kept AFTER DELETE ON weighing_device_models
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION require_weighing_device_models_row();

-- Catalog of commercial weighing-device models.
CREATE TABLE commercial_weighing_device_models (
    -- Stable commercial model identifier shared with parent model tables.
    id UUID PRIMARY KEY REFERENCES weighing_device_models (id) ON DELETE CASCADE,
    -- Organization containing this resource.
    organization_id UUID NOT NULL,
    -- Base weighing-device model represented by this commercial model.
    weighing_device_model_id UUID NOT NULL REFERENCES weighing_device_models (id) ON DELETE RESTRICT,
    FOREIGN KEY (id) REFERENCES commercial_products (id) ON DELETE CASCADE,
    FOREIGN KEY (id, weighing_device_model_id) REFERENCES asset_models (id, parent_model_id) ON DELETE CASCADE,
    archived BOOLEAN NOT NULL DEFAULT FALSE,
    weighing_device_model_archived BOOLEAN NOT NULL DEFAULT FALSE,
    UNIQUE (id, archived),
    FOREIGN KEY (weighing_device_model_id, weighing_device_model_archived) REFERENCES weighing_device_models (id, archived) ON UPDATE CASCADE,
    FOREIGN KEY (organization_id, id, archived) REFERENCES resources (organization_id, id, archived) ON UPDATE CASCADE ON DELETE CASCADE
);
CREATE INDEX ON commercial_weighing_device_models (weighing_device_model_id);
INSERT INTO table_names (id) VALUES ('commercial_weighing_device_models');
INSERT INTO table_name_ancestors (descendant_table_name_id, ancestor_table_name_id)
    SELECT 'commercial_weighing_device_models', ancestor_table_name_id FROM table_name_ancestors WHERE descendant_table_name_id IN ('weighing_device_models', 'commercial_products')
    UNION SELECT 'commercial_weighing_device_models', 'commercial_weighing_device_models';
CREATE TRIGGER commercial_weighing_device_models_path AFTER INSERT ON commercial_weighing_device_models
    FOR EACH ROW EXECUTE FUNCTION check_asset_model_subtype();

CREATE FUNCTION require_commercial_weighing_device_models_row() RETURNS TRIGGER
LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
DECLARE
    row_id UUID := CASE TG_OP WHEN 'DELETE' THEN OLD.id ELSE NEW.id END;
BEGIN
    IF EXISTS (
        SELECT 1 FROM public.asset_models root
        JOIN public.table_name_ancestors path ON path.descendant_table_name_id = root.table_name_id
        WHERE root.id = row_id AND path.ancestor_table_name_id = 'commercial_weighing_device_models'
    ) AND NOT EXISTS (SELECT 1 FROM public.commercial_weighing_device_models subtype WHERE subtype.id = row_id) THEN
        RAISE EXCEPTION '% names a table extending % but has no row there', row_id, 'commercial_weighing_device_models'
            USING ERRCODE = 'foreign_key_violation';
    END IF;
    RETURN NULL;
END;
$$;
ALTER FUNCTION require_commercial_weighing_device_models_row() OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION require_commercial_weighing_device_models_row() FROM PUBLIC;
REVOKE ALL ON commercial_weighing_device_models FROM PUBLIC, app, identity_writer;
GRANT SELECT ON commercial_weighing_device_models TO aps_writer;
CREATE CONSTRAINT TRIGGER commercial_weighing_device_models_row AFTER INSERT ON asset_models
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW WHEN (NEW.table_name_id = 'commercial_weighing_device_models')
    EXECUTE FUNCTION require_commercial_weighing_device_models_row();
CREATE CONSTRAINT TRIGGER commercial_weighing_device_models_kept AFTER DELETE ON commercial_weighing_device_models
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION require_commercial_weighing_device_models_row();

-- Catalog of commercial lot-specific weighing-device models.
CREATE TABLE commercial_weighing_device_lots (
    -- Stable lot-model identifier shared with parent lot/model tables.
    id UUID PRIMARY KEY REFERENCES commercial_product_lots (id) ON DELETE CASCADE,
    -- Organization containing this resource.
    organization_id UUID NOT NULL,
    FOREIGN KEY (id) REFERENCES weighing_device_models (id) ON DELETE CASCADE,
    -- Commercial weighing-device model from which this lot derives.
    commercial_weighing_device_model_id UUID NOT NULL REFERENCES commercial_weighing_device_models (id),
    FOREIGN KEY (id, commercial_weighing_device_model_id) REFERENCES asset_models (id, parent_model_id) ON DELETE CASCADE,
    archived BOOLEAN NOT NULL DEFAULT FALSE,
    commercial_weighing_device_model_archived BOOLEAN NOT NULL DEFAULT FALSE,
    FOREIGN KEY (commercial_weighing_device_model_id, commercial_weighing_device_model_archived) REFERENCES commercial_weighing_device_models (id, archived) ON UPDATE CASCADE,
    FOREIGN KEY (organization_id, id, archived) REFERENCES resources (organization_id, id, archived) ON UPDATE CASCADE ON DELETE CASCADE
);
CREATE INDEX ON commercial_weighing_device_lots (commercial_weighing_device_model_id);
INSERT INTO table_names (id) VALUES ('commercial_weighing_device_lots');
INSERT INTO table_name_ancestors (descendant_table_name_id, ancestor_table_name_id)
    SELECT 'commercial_weighing_device_lots', ancestor_table_name_id FROM table_name_ancestors WHERE descendant_table_name_id IN ('commercial_product_lots', 'weighing_device_models')
    UNION SELECT 'commercial_weighing_device_lots', 'commercial_weighing_device_lots';
INSERT INTO table_name_admitted_parents (table_name_id, parent_table_name_id)
    VALUES ('commercial_weighing_device_lots', 'commercial_weighing_device_models');
CREATE TRIGGER commercial_weighing_device_lots_path AFTER INSERT ON commercial_weighing_device_lots
    FOR EACH ROW EXECUTE FUNCTION check_asset_model_subtype();

CREATE FUNCTION require_commercial_weighing_device_lots_row() RETURNS TRIGGER
LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
DECLARE
    row_id UUID := CASE TG_OP WHEN 'DELETE' THEN OLD.id ELSE NEW.id END;
BEGIN
    IF EXISTS (
        SELECT 1 FROM public.asset_models root
        JOIN public.table_name_ancestors path ON path.descendant_table_name_id = root.table_name_id
        WHERE root.id = row_id AND path.ancestor_table_name_id = 'commercial_weighing_device_lots'
    ) AND NOT EXISTS (SELECT 1 FROM public.commercial_weighing_device_lots subtype WHERE subtype.id = row_id) THEN
        RAISE EXCEPTION '% names a table extending % but has no row there', row_id, 'commercial_weighing_device_lots'
            USING ERRCODE = 'foreign_key_violation';
    END IF;
    RETURN NULL;
END;
$$;
ALTER FUNCTION require_commercial_weighing_device_lots_row() OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION require_commercial_weighing_device_lots_row() FROM PUBLIC;
REVOKE ALL ON commercial_weighing_device_lots FROM PUBLIC, app, identity_writer;
GRANT SELECT ON commercial_weighing_device_lots TO aps_writer;
CREATE CONSTRAINT TRIGGER commercial_weighing_device_lots_row AFTER INSERT ON asset_models
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW WHEN (NEW.table_name_id = 'commercial_weighing_device_lots')
    EXECUTE FUNCTION require_commercial_weighing_device_lots_row();
CREATE CONSTRAINT TRIGGER commercial_weighing_device_lots_kept AFTER DELETE ON commercial_weighing_device_lots
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION require_commercial_weighing_device_lots_row();

-- Physical weighing devices tracked in APS inventory.
CREATE TABLE weighing_devices (
    -- Stable asset identifier inherited from `physical_assets`.
    id UUID PRIMARY KEY REFERENCES physical_assets (id) ON DELETE CASCADE,
    -- Organization containing this resource.
    organization_id UUID NOT NULL,
    -- Weighing-device model instantiated by this physical asset.
    weighing_device_model_id UUID NOT NULL REFERENCES weighing_device_models (id),
    FOREIGN KEY (id, weighing_device_model_id) REFERENCES assets (id, model_id) ON DELETE CASCADE,
    archived BOOLEAN NOT NULL DEFAULT FALSE,
    weighing_device_model_archived BOOLEAN NOT NULL DEFAULT FALSE,
    FOREIGN KEY (weighing_device_model_id, weighing_device_model_archived) REFERENCES weighing_device_models (id, archived) ON UPDATE CASCADE,
    FOREIGN KEY (organization_id, id, archived) REFERENCES resources (organization_id, id, archived) ON UPDATE CASCADE ON DELETE CASCADE
);
CREATE INDEX ON weighing_devices (weighing_device_model_id);
INSERT INTO table_names (id) VALUES ('weighing_devices');
INSERT INTO table_name_instances (id, instance_table_name_id)
    VALUES ('weighing_device_models', 'weighing_devices');
INSERT INTO table_name_ancestors (descendant_table_name_id, ancestor_table_name_id)
    SELECT 'weighing_devices', ancestor_table_name_id FROM table_name_ancestors WHERE descendant_table_name_id IN ('physical_assets')
    UNION SELECT 'weighing_devices', 'weighing_devices';
CREATE TRIGGER weighing_devices_path AFTER INSERT ON weighing_devices
    FOR EACH ROW EXECUTE FUNCTION check_asset_subtype();

CREATE FUNCTION require_weighing_devices_row() RETURNS TRIGGER
LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
DECLARE
    row_id UUID := CASE TG_OP WHEN 'DELETE' THEN OLD.id ELSE NEW.id END;
BEGIN
    IF EXISTS (
        SELECT 1 FROM public.assets root
        JOIN public.table_name_ancestors path ON path.descendant_table_name_id = root.table_name_id
        WHERE root.id = row_id AND path.ancestor_table_name_id = 'weighing_devices'
    ) AND NOT EXISTS (SELECT 1 FROM public.weighing_devices subtype WHERE subtype.id = row_id) THEN
        RAISE EXCEPTION '% names a table extending % but has no row there', row_id, 'weighing_devices'
            USING ERRCODE = 'foreign_key_violation';
    END IF;
    RETURN NULL;
END;
$$;
ALTER FUNCTION require_weighing_devices_row() OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION require_weighing_devices_row() FROM PUBLIC;
REVOKE ALL ON weighing_devices FROM PUBLIC, app, identity_writer;
GRANT SELECT ON weighing_devices TO aps_writer;
CREATE CONSTRAINT TRIGGER weighing_devices_row AFTER INSERT ON assets
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW WHEN (NEW.table_name_id = 'weighing_devices')
    EXECUTE FUNCTION require_weighing_devices_row();
CREATE CONSTRAINT TRIGGER weighing_devices_kept AFTER DELETE ON weighing_devices
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION require_weighing_devices_row();

GRANT SELECT, INSERT, DELETE ON weighing_device_models TO app;
ALTER TABLE weighing_device_models ENABLE ROW LEVEL SECURITY;
ALTER TABLE weighing_device_models FORCE ROW LEVEL SECURITY;
CREATE POLICY weighing_device_models_select ON weighing_device_models FOR SELECT TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY weighing_device_models_insert ON weighing_device_models FOR INSERT TO app
    WITH CHECK (id = ANY (ARRAY(SELECT authz.resources_with('writer'))));
CREATE POLICY weighing_device_models_delete ON weighing_device_models FOR DELETE TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('admin'))));
CREATE POLICY weighing_device_models_archived_insert ON weighing_device_models AS RESTRICTIVE FOR INSERT TO app
    WITH CHECK (NOT archived);
CREATE POLICY weighing_device_models_archived_delete ON weighing_device_models AS RESTRICTIVE FOR DELETE TO app
    USING (NOT archived);
CREATE POLICY weighing_device_models_blocked ON weighing_device_models AS RESTRICTIVE FOR ALL TO app
    USING (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))))
    WITH CHECK (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))));

GRANT SELECT, INSERT, DELETE ON commercial_weighing_device_models TO app;
ALTER TABLE commercial_weighing_device_models ENABLE ROW LEVEL SECURITY;
ALTER TABLE commercial_weighing_device_models FORCE ROW LEVEL SECURITY;
CREATE POLICY commercial_weighing_device_models_select ON commercial_weighing_device_models FOR SELECT TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY commercial_weighing_device_models_insert ON commercial_weighing_device_models FOR INSERT TO app
    WITH CHECK (id = ANY (ARRAY(SELECT authz.resources_with('writer')))
        AND weighing_device_model_id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY commercial_weighing_device_models_delete ON commercial_weighing_device_models FOR DELETE TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('admin'))));
CREATE POLICY commercial_weighing_device_models_archived_insert ON commercial_weighing_device_models AS RESTRICTIVE FOR INSERT TO app
    WITH CHECK (NOT archived AND NOT weighing_device_model_archived);
CREATE POLICY commercial_weighing_device_models_archived_delete ON commercial_weighing_device_models AS RESTRICTIVE FOR DELETE TO app
    USING (NOT archived);
CREATE POLICY commercial_weighing_device_models_blocked ON commercial_weighing_device_models AS RESTRICTIVE FOR ALL TO app
    USING (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))))
    WITH CHECK (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))));

GRANT SELECT, INSERT, DELETE ON commercial_weighing_device_lots TO app;
ALTER TABLE commercial_weighing_device_lots ENABLE ROW LEVEL SECURITY;
ALTER TABLE commercial_weighing_device_lots FORCE ROW LEVEL SECURITY;
CREATE POLICY commercial_weighing_device_lots_select ON commercial_weighing_device_lots FOR SELECT TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY commercial_weighing_device_lots_insert ON commercial_weighing_device_lots FOR INSERT TO app
    WITH CHECK (id = ANY (ARRAY(SELECT authz.resources_with('writer')))
        AND commercial_weighing_device_model_id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY commercial_weighing_device_lots_delete ON commercial_weighing_device_lots FOR DELETE TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('admin'))));
CREATE POLICY commercial_weighing_device_lots_archived_insert ON commercial_weighing_device_lots AS RESTRICTIVE FOR INSERT TO app
    WITH CHECK (NOT archived AND NOT commercial_weighing_device_model_archived);
CREATE POLICY commercial_weighing_device_lots_archived_delete ON commercial_weighing_device_lots AS RESTRICTIVE FOR DELETE TO app
    USING (NOT archived);
CREATE POLICY commercial_weighing_device_lots_blocked ON commercial_weighing_device_lots AS RESTRICTIVE FOR ALL TO app
    USING (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))))
    WITH CHECK (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))));

GRANT SELECT, INSERT, DELETE ON weighing_devices TO app;
ALTER TABLE weighing_devices ENABLE ROW LEVEL SECURITY;
ALTER TABLE weighing_devices FORCE ROW LEVEL SECURITY;
CREATE POLICY weighing_devices_select ON weighing_devices FOR SELECT TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY weighing_devices_insert ON weighing_devices FOR INSERT TO app
    WITH CHECK (id = ANY (ARRAY(SELECT authz.resources_with('writer')))
        AND weighing_device_model_id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY weighing_devices_delete ON weighing_devices FOR DELETE TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('admin'))));
CREATE POLICY weighing_devices_archived_insert ON weighing_devices AS RESTRICTIVE FOR INSERT TO app
    WITH CHECK (NOT archived AND NOT weighing_device_model_archived);
CREATE POLICY weighing_devices_archived_delete ON weighing_devices AS RESTRICTIVE FOR DELETE TO app
    USING (NOT archived);
CREATE POLICY weighing_devices_blocked ON weighing_devices AS RESTRICTIVE FOR ALL TO app
    USING (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))))
    WITH CHECK (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))));

-- Records every change in connetto's `audit_events`.
CREATE TRIGGER weighing_device_models_audit AFTER INSERT OR DELETE ON weighing_device_models
    FOR EACH ROW EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER weighing_device_models_update_audit AFTER UPDATE ON weighing_device_models
    FOR EACH ROW WHEN (OLD IS DISTINCT FROM NEW) EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER commercial_weighing_device_models_audit AFTER INSERT OR DELETE ON commercial_weighing_device_models
    FOR EACH ROW EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER commercial_weighing_device_models_update_audit AFTER UPDATE ON commercial_weighing_device_models
    FOR EACH ROW WHEN (OLD IS DISTINCT FROM NEW) EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER commercial_weighing_device_lots_audit AFTER INSERT OR DELETE ON commercial_weighing_device_lots
    FOR EACH ROW EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER commercial_weighing_device_lots_update_audit AFTER UPDATE ON commercial_weighing_device_lots
    FOR EACH ROW WHEN (OLD IS DISTINCT FROM NEW) EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER weighing_devices_audit AFTER INSERT OR DELETE ON weighing_devices
    FOR EACH ROW EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER weighing_devices_update_audit AFTER UPDATE ON weighing_devices
    FOR EACH ROW WHEN (OLD IS DISTINCT FROM NEW) EXECUTE FUNCTION authz_write.record_audit_event();
