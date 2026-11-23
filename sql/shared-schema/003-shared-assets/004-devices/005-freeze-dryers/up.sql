-- Migration: Shared Assets / Devices / Freeze Dryers
-- Purpose: Freeze Dryers for project inventories.
-- APS placement
--   ownables
--       +-- asset_models
--           +-- physical_asset_models
--               +-- freeze_dryer_models
--                   +-- commercial_freeze_dryer_models
--               +-- commercial_product_lots
--                   +-- commercial_freeze_dryer_lots
--       +-- assets
--           +-- physical_assets
--               +-- freeze_dryers
-- Commercial tables decision: Vendor models and lots provide purchase traceability.

-- Catalog of freeze dryer model definitions.
CREATE TABLE freeze_dryer_models (
    -- Stable model identifier inherited from physical_asset_models.
    id UUID PRIMARY KEY REFERENCES physical_asset_models (id) ON DELETE CASCADE,
    -- Organization containing this resource.
    organization_id UUID NOT NULL,
    FOREIGN KEY (organization_id, id) REFERENCES resources (organization_id, id) ON DELETE CASCADE
);
INSERT INTO table_names (id) VALUES ('freeze_dryer_models');
INSERT INTO table_name_ancestors (descendant_table_name_id, ancestor_table_name_id)
    SELECT 'freeze_dryer_models', ancestor_table_name_id FROM table_name_ancestors WHERE descendant_table_name_id IN ('physical_asset_models')
    UNION SELECT 'freeze_dryer_models', 'freeze_dryer_models';
CREATE TRIGGER freeze_dryer_models_path AFTER INSERT ON freeze_dryer_models
    FOR EACH ROW EXECUTE FUNCTION check_asset_model_subtype();

CREATE FUNCTION require_freeze_dryer_models_row() RETURNS TRIGGER
LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
DECLARE
    row_id UUID := CASE TG_OP WHEN 'DELETE' THEN OLD.id ELSE NEW.id END;
BEGIN
    IF EXISTS (
        SELECT 1 FROM public.asset_models root
        JOIN public.table_name_ancestors path ON path.descendant_table_name_id = root.table_name_id
        WHERE root.id = row_id AND path.ancestor_table_name_id = 'freeze_dryer_models'
    ) AND NOT EXISTS (SELECT 1 FROM public.freeze_dryer_models subtype WHERE subtype.id = row_id) THEN
        RAISE EXCEPTION '% names a table extending % but has no row there', row_id, 'freeze_dryer_models'
            USING ERRCODE = 'foreign_key_violation';
    END IF;
    RETURN NULL;
END;
$$;
ALTER FUNCTION require_freeze_dryer_models_row() OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION require_freeze_dryer_models_row() FROM PUBLIC;
GRANT SELECT ON freeze_dryer_models TO aps_writer;
CREATE CONSTRAINT TRIGGER freeze_dryer_models_row AFTER INSERT ON asset_models
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW WHEN (NEW.table_name_id = 'freeze_dryer_models')
    EXECUTE FUNCTION require_freeze_dryer_models_row();
CREATE CONSTRAINT TRIGGER freeze_dryer_models_kept AFTER DELETE ON freeze_dryer_models
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION require_freeze_dryer_models_row();

-- Catalog of commercial freeze dryer models.
CREATE TABLE commercial_freeze_dryer_models (
    -- Stable commercial model identifier shared with parent model tables.
    id UUID PRIMARY KEY REFERENCES freeze_dryer_models (id) ON DELETE CASCADE,
    -- Organization containing this resource.
    organization_id UUID NOT NULL,
    -- Base freeze dryer model represented by this commercial model.
    freeze_dryer_model_id UUID NOT NULL REFERENCES freeze_dryer_models (id) ON DELETE RESTRICT,
    FOREIGN KEY (id) REFERENCES commercial_products (id) ON DELETE CASCADE,
    FOREIGN KEY (id, freeze_dryer_model_id) REFERENCES asset_models (id, parent_model_id) ON DELETE CASCADE,
    FOREIGN KEY (organization_id, id) REFERENCES resources (organization_id, id) ON DELETE CASCADE
);
CREATE INDEX ON commercial_freeze_dryer_models (freeze_dryer_model_id);
INSERT INTO table_names (id) VALUES ('commercial_freeze_dryer_models');
INSERT INTO table_name_ancestors (descendant_table_name_id, ancestor_table_name_id)
    SELECT 'commercial_freeze_dryer_models', ancestor_table_name_id FROM table_name_ancestors WHERE descendant_table_name_id IN ('freeze_dryer_models', 'commercial_products')
    UNION SELECT 'commercial_freeze_dryer_models', 'commercial_freeze_dryer_models';
CREATE TRIGGER commercial_freeze_dryer_models_path AFTER INSERT ON commercial_freeze_dryer_models
    FOR EACH ROW EXECUTE FUNCTION check_asset_model_subtype();

CREATE FUNCTION require_commercial_freeze_dryer_models_row() RETURNS TRIGGER
LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
DECLARE
    row_id UUID := CASE TG_OP WHEN 'DELETE' THEN OLD.id ELSE NEW.id END;
BEGIN
    IF EXISTS (
        SELECT 1 FROM public.asset_models root
        JOIN public.table_name_ancestors path ON path.descendant_table_name_id = root.table_name_id
        WHERE root.id = row_id AND path.ancestor_table_name_id = 'commercial_freeze_dryer_models'
    ) AND NOT EXISTS (SELECT 1 FROM public.commercial_freeze_dryer_models subtype WHERE subtype.id = row_id) THEN
        RAISE EXCEPTION '% names a table extending % but has no row there', row_id, 'commercial_freeze_dryer_models'
            USING ERRCODE = 'foreign_key_violation';
    END IF;
    RETURN NULL;
END;
$$;
ALTER FUNCTION require_commercial_freeze_dryer_models_row() OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION require_commercial_freeze_dryer_models_row() FROM PUBLIC;
GRANT SELECT ON commercial_freeze_dryer_models TO aps_writer;
CREATE CONSTRAINT TRIGGER commercial_freeze_dryer_models_row AFTER INSERT ON asset_models
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW WHEN (NEW.table_name_id = 'commercial_freeze_dryer_models')
    EXECUTE FUNCTION require_commercial_freeze_dryer_models_row();
CREATE CONSTRAINT TRIGGER commercial_freeze_dryer_models_kept AFTER DELETE ON commercial_freeze_dryer_models
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION require_commercial_freeze_dryer_models_row();

-- Catalog of lot-specific commercial freeze dryer models.
CREATE TABLE commercial_freeze_dryer_lots (
    -- Stable lot-model identifier shared with parent lot/model tables.
    id UUID PRIMARY KEY REFERENCES commercial_product_lots (id) ON DELETE CASCADE,
    -- Organization containing this resource.
    organization_id UUID NOT NULL,
    FOREIGN KEY (id) REFERENCES freeze_dryer_models (id) ON DELETE CASCADE,
    -- Commercial freeze dryer model from which this lot derives.
    commercial_freeze_dryer_model_id UUID NOT NULL REFERENCES commercial_freeze_dryer_models (id),
    FOREIGN KEY (id, commercial_freeze_dryer_model_id) REFERENCES asset_models (id, parent_model_id) ON DELETE CASCADE,
    FOREIGN KEY (organization_id, id) REFERENCES resources (organization_id, id) ON DELETE CASCADE
);
CREATE INDEX ON commercial_freeze_dryer_lots (commercial_freeze_dryer_model_id);
INSERT INTO table_names (id) VALUES ('commercial_freeze_dryer_lots');
INSERT INTO table_name_ancestors (descendant_table_name_id, ancestor_table_name_id)
    SELECT 'commercial_freeze_dryer_lots', ancestor_table_name_id FROM table_name_ancestors WHERE descendant_table_name_id IN ('commercial_product_lots', 'freeze_dryer_models')
    UNION SELECT 'commercial_freeze_dryer_lots', 'commercial_freeze_dryer_lots';
INSERT INTO table_name_admitted_parents (table_name_id, parent_table_name_id)
    VALUES ('commercial_freeze_dryer_lots', 'commercial_freeze_dryer_models');
CREATE TRIGGER commercial_freeze_dryer_lots_path AFTER INSERT ON commercial_freeze_dryer_lots
    FOR EACH ROW EXECUTE FUNCTION check_asset_model_subtype();

CREATE FUNCTION require_commercial_freeze_dryer_lots_row() RETURNS TRIGGER
LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
DECLARE
    row_id UUID := CASE TG_OP WHEN 'DELETE' THEN OLD.id ELSE NEW.id END;
BEGIN
    IF EXISTS (
        SELECT 1 FROM public.asset_models root
        JOIN public.table_name_ancestors path ON path.descendant_table_name_id = root.table_name_id
        WHERE root.id = row_id AND path.ancestor_table_name_id = 'commercial_freeze_dryer_lots'
    ) AND NOT EXISTS (SELECT 1 FROM public.commercial_freeze_dryer_lots subtype WHERE subtype.id = row_id) THEN
        RAISE EXCEPTION '% names a table extending % but has no row there', row_id, 'commercial_freeze_dryer_lots'
            USING ERRCODE = 'foreign_key_violation';
    END IF;
    RETURN NULL;
END;
$$;
ALTER FUNCTION require_commercial_freeze_dryer_lots_row() OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION require_commercial_freeze_dryer_lots_row() FROM PUBLIC;
GRANT SELECT ON commercial_freeze_dryer_lots TO aps_writer;
CREATE CONSTRAINT TRIGGER commercial_freeze_dryer_lots_row AFTER INSERT ON asset_models
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW WHEN (NEW.table_name_id = 'commercial_freeze_dryer_lots')
    EXECUTE FUNCTION require_commercial_freeze_dryer_lots_row();
CREATE CONSTRAINT TRIGGER commercial_freeze_dryer_lots_kept AFTER DELETE ON commercial_freeze_dryer_lots
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION require_commercial_freeze_dryer_lots_row();

-- Physical freeze dryers tracked in APS inventory.
CREATE TABLE freeze_dryers (
    -- Stable asset identifier inherited from physical_assets.
    id UUID PRIMARY KEY REFERENCES physical_assets (id) ON DELETE CASCADE,
    -- Organization containing this resource.
    organization_id UUID NOT NULL,
    -- Commercial lot model instantiated by this physical asset.
    commercial_freeze_dryer_lot_id UUID NOT NULL REFERENCES commercial_freeze_dryer_lots (id),
    FOREIGN KEY (id, commercial_freeze_dryer_lot_id) REFERENCES assets (id, model_id) ON DELETE CASCADE,
    FOREIGN KEY (organization_id, id) REFERENCES resources (organization_id, id) ON DELETE CASCADE
);
CREATE INDEX ON freeze_dryers (commercial_freeze_dryer_lot_id);
INSERT INTO table_names (id) VALUES ('freeze_dryers');
INSERT INTO table_name_instances (id, instance_table_name_id)
    VALUES ('commercial_freeze_dryer_lots', 'freeze_dryers');
INSERT INTO table_name_ancestors (descendant_table_name_id, ancestor_table_name_id)
    SELECT 'freeze_dryers', ancestor_table_name_id FROM table_name_ancestors WHERE descendant_table_name_id IN ('physical_assets')
    UNION SELECT 'freeze_dryers', 'freeze_dryers';
CREATE TRIGGER freeze_dryers_path AFTER INSERT ON freeze_dryers
    FOR EACH ROW EXECUTE FUNCTION check_asset_subtype();

CREATE FUNCTION require_freeze_dryers_row() RETURNS TRIGGER
LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
DECLARE
    row_id UUID := CASE TG_OP WHEN 'DELETE' THEN OLD.id ELSE NEW.id END;
BEGIN
    IF EXISTS (
        SELECT 1 FROM public.assets root
        JOIN public.table_name_ancestors path ON path.descendant_table_name_id = root.table_name_id
        WHERE root.id = row_id AND path.ancestor_table_name_id = 'freeze_dryers'
    ) AND NOT EXISTS (SELECT 1 FROM public.freeze_dryers subtype WHERE subtype.id = row_id) THEN
        RAISE EXCEPTION '% names a table extending % but has no row there', row_id, 'freeze_dryers'
            USING ERRCODE = 'foreign_key_violation';
    END IF;
    RETURN NULL;
END;
$$;
ALTER FUNCTION require_freeze_dryers_row() OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION require_freeze_dryers_row() FROM PUBLIC;
GRANT SELECT ON freeze_dryers TO aps_writer;
CREATE CONSTRAINT TRIGGER freeze_dryers_row AFTER INSERT ON assets
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW WHEN (NEW.table_name_id = 'freeze_dryers')
    EXECUTE FUNCTION require_freeze_dryers_row();
CREATE CONSTRAINT TRIGGER freeze_dryers_kept AFTER DELETE ON freeze_dryers
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION require_freeze_dryers_row();

GRANT SELECT, INSERT, DELETE ON freeze_dryer_models TO app;
ALTER TABLE freeze_dryer_models ENABLE ROW LEVEL SECURITY;
ALTER TABLE freeze_dryer_models FORCE ROW LEVEL SECURITY;
CREATE POLICY freeze_dryer_models_select ON freeze_dryer_models FOR SELECT TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY freeze_dryer_models_insert ON freeze_dryer_models FOR INSERT TO app
    WITH CHECK (id = ANY (ARRAY(SELECT authz.resources_with('writer'))));
CREATE POLICY freeze_dryer_models_delete ON freeze_dryer_models FOR DELETE TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('admin'))));
CREATE POLICY freeze_dryer_models_archived_insert ON freeze_dryer_models AS RESTRICTIVE FOR INSERT TO app
    WITH CHECK (NOT authz.is_archived(id));
CREATE POLICY freeze_dryer_models_archived_delete ON freeze_dryer_models AS RESTRICTIVE FOR DELETE TO app
    USING (NOT authz.is_archived(id));
CREATE POLICY freeze_dryer_models_blocked ON freeze_dryer_models AS RESTRICTIVE FOR ALL TO app
    USING (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))))
    WITH CHECK (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))));

GRANT SELECT, INSERT, DELETE ON commercial_freeze_dryer_models TO app;
ALTER TABLE commercial_freeze_dryer_models ENABLE ROW LEVEL SECURITY;
ALTER TABLE commercial_freeze_dryer_models FORCE ROW LEVEL SECURITY;
CREATE POLICY commercial_freeze_dryer_models_select ON commercial_freeze_dryer_models FOR SELECT TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY commercial_freeze_dryer_models_insert ON commercial_freeze_dryer_models FOR INSERT TO app
    WITH CHECK (id = ANY (ARRAY(SELECT authz.resources_with('writer')))
        AND freeze_dryer_model_id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY commercial_freeze_dryer_models_delete ON commercial_freeze_dryer_models FOR DELETE TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('admin'))));
CREATE POLICY commercial_freeze_dryer_models_archived_insert ON commercial_freeze_dryer_models AS RESTRICTIVE FOR INSERT TO app
    WITH CHECK (NOT authz.is_archived(id) AND NOT authz.is_archived(freeze_dryer_model_id));
CREATE POLICY commercial_freeze_dryer_models_archived_delete ON commercial_freeze_dryer_models AS RESTRICTIVE FOR DELETE TO app
    USING (NOT authz.is_archived(id));
CREATE POLICY commercial_freeze_dryer_models_blocked ON commercial_freeze_dryer_models AS RESTRICTIVE FOR ALL TO app
    USING (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))))
    WITH CHECK (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))));

GRANT SELECT, INSERT, DELETE ON commercial_freeze_dryer_lots TO app;
ALTER TABLE commercial_freeze_dryer_lots ENABLE ROW LEVEL SECURITY;
ALTER TABLE commercial_freeze_dryer_lots FORCE ROW LEVEL SECURITY;
CREATE POLICY commercial_freeze_dryer_lots_select ON commercial_freeze_dryer_lots FOR SELECT TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY commercial_freeze_dryer_lots_insert ON commercial_freeze_dryer_lots FOR INSERT TO app
    WITH CHECK (id = ANY (ARRAY(SELECT authz.resources_with('writer')))
        AND commercial_freeze_dryer_model_id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY commercial_freeze_dryer_lots_delete ON commercial_freeze_dryer_lots FOR DELETE TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('admin'))));
CREATE POLICY commercial_freeze_dryer_lots_archived_insert ON commercial_freeze_dryer_lots AS RESTRICTIVE FOR INSERT TO app
    WITH CHECK (NOT authz.is_archived(id) AND NOT authz.is_archived(commercial_freeze_dryer_model_id));
CREATE POLICY commercial_freeze_dryer_lots_archived_delete ON commercial_freeze_dryer_lots AS RESTRICTIVE FOR DELETE TO app
    USING (NOT authz.is_archived(id));
CREATE POLICY commercial_freeze_dryer_lots_blocked ON commercial_freeze_dryer_lots AS RESTRICTIVE FOR ALL TO app
    USING (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))))
    WITH CHECK (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))));

GRANT SELECT, INSERT, DELETE ON freeze_dryers TO app;
ALTER TABLE freeze_dryers ENABLE ROW LEVEL SECURITY;
ALTER TABLE freeze_dryers FORCE ROW LEVEL SECURITY;
CREATE POLICY freeze_dryers_select ON freeze_dryers FOR SELECT TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY freeze_dryers_insert ON freeze_dryers FOR INSERT TO app
    WITH CHECK (id = ANY (ARRAY(SELECT authz.resources_with('writer')))
        AND commercial_freeze_dryer_lot_id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY freeze_dryers_delete ON freeze_dryers FOR DELETE TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('admin'))));
CREATE POLICY freeze_dryers_archived_insert ON freeze_dryers AS RESTRICTIVE FOR INSERT TO app
    WITH CHECK (NOT authz.is_archived(id) AND NOT authz.is_archived(commercial_freeze_dryer_lot_id));
CREATE POLICY freeze_dryers_archived_delete ON freeze_dryers AS RESTRICTIVE FOR DELETE TO app
    USING (NOT authz.is_archived(id));
CREATE POLICY freeze_dryers_blocked ON freeze_dryers AS RESTRICTIVE FOR ALL TO app
    USING (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))))
    WITH CHECK (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))));

-- Records every change in connetto's `audit_events`.
CREATE TRIGGER freeze_dryer_models_audit AFTER INSERT OR DELETE ON freeze_dryer_models
    FOR EACH ROW EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER freeze_dryer_models_update_audit AFTER UPDATE ON freeze_dryer_models
    FOR EACH ROW WHEN (OLD IS DISTINCT FROM NEW) EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER commercial_freeze_dryer_models_audit AFTER INSERT OR DELETE ON commercial_freeze_dryer_models
    FOR EACH ROW EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER commercial_freeze_dryer_models_update_audit AFTER UPDATE ON commercial_freeze_dryer_models
    FOR EACH ROW WHEN (OLD IS DISTINCT FROM NEW) EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER commercial_freeze_dryer_lots_audit AFTER INSERT OR DELETE ON commercial_freeze_dryer_lots
    FOR EACH ROW EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER commercial_freeze_dryer_lots_update_audit AFTER UPDATE ON commercial_freeze_dryer_lots
    FOR EACH ROW WHEN (OLD IS DISTINCT FROM NEW) EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER freeze_dryers_audit AFTER INSERT OR DELETE ON freeze_dryers
    FOR EACH ROW EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER freeze_dryers_update_audit AFTER UPDATE ON freeze_dryers
    FOR EACH ROW WHEN (OLD IS DISTINCT FROM NEW) EXECUTE FUNCTION authz_write.record_audit_event();
