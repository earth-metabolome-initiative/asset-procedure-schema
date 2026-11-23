-- Migration: Shared Assets / Devices / Centrifuges
-- Purpose: Centrifuges for project inventories.
-- APS placement
--   ownables
--       +-- asset_models
--           +-- physical_asset_models
--               +-- centrifuge_models
--                   +-- commercial_centrifuge_models
--               +-- commercial_product_lots
--                   +-- commercial_centrifuge_lots
--       +-- assets
--           +-- physical_assets
--               +-- centrifuges
-- Commercial tables decision: Vendor models and lots provide purchase traceability.

-- Catalog of centrifuge model definitions.
CREATE TABLE centrifuge_models (
    -- Stable model identifier inherited from physical_asset_models.
    id UUID PRIMARY KEY REFERENCES physical_asset_models (id) ON DELETE CASCADE,
    -- Organization containing this resource.
    organization_id UUID NOT NULL,
    FOREIGN KEY (organization_id, id) REFERENCES resources (organization_id, id) ON DELETE CASCADE
);
INSERT INTO table_names (id) VALUES ('centrifuge_models');
INSERT INTO table_name_ancestors (descendant_table_name_id, ancestor_table_name_id)
    SELECT 'centrifuge_models', ancestor_table_name_id FROM table_name_ancestors WHERE descendant_table_name_id IN ('physical_asset_models')
    UNION SELECT 'centrifuge_models', 'centrifuge_models';
CREATE TRIGGER centrifuge_models_path AFTER INSERT ON centrifuge_models
    FOR EACH ROW EXECUTE FUNCTION check_asset_model_subtype();

CREATE FUNCTION require_centrifuge_models_row() RETURNS TRIGGER
LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
DECLARE
    row_id UUID := CASE TG_OP WHEN 'DELETE' THEN OLD.id ELSE NEW.id END;
BEGIN
    IF EXISTS (
        SELECT 1 FROM public.asset_models root
        JOIN public.table_name_ancestors path ON path.descendant_table_name_id = root.table_name_id
        WHERE root.id = row_id AND path.ancestor_table_name_id = 'centrifuge_models'
    ) AND NOT EXISTS (SELECT 1 FROM public.centrifuge_models subtype WHERE subtype.id = row_id) THEN
        RAISE EXCEPTION '% names a table extending % but has no row there', row_id, 'centrifuge_models'
            USING ERRCODE = 'foreign_key_violation';
    END IF;
    RETURN NULL;
END;
$$;
ALTER FUNCTION require_centrifuge_models_row() OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION require_centrifuge_models_row() FROM PUBLIC;
GRANT SELECT ON centrifuge_models TO aps_writer;
CREATE CONSTRAINT TRIGGER centrifuge_models_row AFTER INSERT ON asset_models
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW WHEN (NEW.table_name_id = 'centrifuge_models')
    EXECUTE FUNCTION require_centrifuge_models_row();
CREATE CONSTRAINT TRIGGER centrifuge_models_kept AFTER DELETE ON centrifuge_models
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION require_centrifuge_models_row();

-- Catalog of commercial centrifuge models.
CREATE TABLE commercial_centrifuge_models (
    -- Stable commercial model identifier shared with parent model tables.
    id UUID PRIMARY KEY REFERENCES centrifuge_models (id) ON DELETE CASCADE,
    -- Organization containing this resource.
    organization_id UUID NOT NULL,
    -- Base centrifuge model represented by this commercial model.
    centrifuge_model_id UUID NOT NULL REFERENCES centrifuge_models (id) ON DELETE RESTRICT,
    FOREIGN KEY (id) REFERENCES commercial_products (id) ON DELETE CASCADE,
    FOREIGN KEY (id, centrifuge_model_id) REFERENCES asset_models (id, parent_model_id) ON DELETE CASCADE,
    FOREIGN KEY (organization_id, id) REFERENCES resources (organization_id, id) ON DELETE CASCADE
);
CREATE INDEX ON commercial_centrifuge_models (centrifuge_model_id);
INSERT INTO table_names (id) VALUES ('commercial_centrifuge_models');
INSERT INTO table_name_ancestors (descendant_table_name_id, ancestor_table_name_id)
    SELECT 'commercial_centrifuge_models', ancestor_table_name_id FROM table_name_ancestors WHERE descendant_table_name_id IN ('centrifuge_models', 'commercial_products')
    UNION SELECT 'commercial_centrifuge_models', 'commercial_centrifuge_models';
CREATE TRIGGER commercial_centrifuge_models_path AFTER INSERT ON commercial_centrifuge_models
    FOR EACH ROW EXECUTE FUNCTION check_asset_model_subtype();

CREATE FUNCTION require_commercial_centrifuge_models_row() RETURNS TRIGGER
LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
DECLARE
    row_id UUID := CASE TG_OP WHEN 'DELETE' THEN OLD.id ELSE NEW.id END;
BEGIN
    IF EXISTS (
        SELECT 1 FROM public.asset_models root
        JOIN public.table_name_ancestors path ON path.descendant_table_name_id = root.table_name_id
        WHERE root.id = row_id AND path.ancestor_table_name_id = 'commercial_centrifuge_models'
    ) AND NOT EXISTS (SELECT 1 FROM public.commercial_centrifuge_models subtype WHERE subtype.id = row_id) THEN
        RAISE EXCEPTION '% names a table extending % but has no row there', row_id, 'commercial_centrifuge_models'
            USING ERRCODE = 'foreign_key_violation';
    END IF;
    RETURN NULL;
END;
$$;
ALTER FUNCTION require_commercial_centrifuge_models_row() OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION require_commercial_centrifuge_models_row() FROM PUBLIC;
GRANT SELECT ON commercial_centrifuge_models TO aps_writer;
CREATE CONSTRAINT TRIGGER commercial_centrifuge_models_row AFTER INSERT ON asset_models
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW WHEN (NEW.table_name_id = 'commercial_centrifuge_models')
    EXECUTE FUNCTION require_commercial_centrifuge_models_row();
CREATE CONSTRAINT TRIGGER commercial_centrifuge_models_kept AFTER DELETE ON commercial_centrifuge_models
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION require_commercial_centrifuge_models_row();

-- Catalog of lot-specific commercial centrifuge models.
CREATE TABLE commercial_centrifuge_lots (
    -- Stable lot-model identifier shared with parent lot/model tables.
    id UUID PRIMARY KEY REFERENCES commercial_product_lots (id) ON DELETE CASCADE,
    -- Organization containing this resource.
    organization_id UUID NOT NULL,
    FOREIGN KEY (id) REFERENCES centrifuge_models (id) ON DELETE CASCADE,
    -- Commercial centrifuge model from which this lot derives.
    commercial_centrifuge_model_id UUID NOT NULL REFERENCES commercial_centrifuge_models (id),
    FOREIGN KEY (id, commercial_centrifuge_model_id) REFERENCES asset_models (id, parent_model_id) ON DELETE CASCADE,
    FOREIGN KEY (organization_id, id) REFERENCES resources (organization_id, id) ON DELETE CASCADE
);
CREATE INDEX ON commercial_centrifuge_lots (commercial_centrifuge_model_id);
INSERT INTO table_names (id) VALUES ('commercial_centrifuge_lots');
INSERT INTO table_name_ancestors (descendant_table_name_id, ancestor_table_name_id)
    SELECT 'commercial_centrifuge_lots', ancestor_table_name_id FROM table_name_ancestors WHERE descendant_table_name_id IN ('commercial_product_lots', 'centrifuge_models')
    UNION SELECT 'commercial_centrifuge_lots', 'commercial_centrifuge_lots';
INSERT INTO table_name_admitted_parents (table_name_id, parent_table_name_id)
    VALUES ('commercial_centrifuge_lots', 'commercial_centrifuge_models');
CREATE TRIGGER commercial_centrifuge_lots_path AFTER INSERT ON commercial_centrifuge_lots
    FOR EACH ROW EXECUTE FUNCTION check_asset_model_subtype();

CREATE FUNCTION require_commercial_centrifuge_lots_row() RETURNS TRIGGER
LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
DECLARE
    row_id UUID := CASE TG_OP WHEN 'DELETE' THEN OLD.id ELSE NEW.id END;
BEGIN
    IF EXISTS (
        SELECT 1 FROM public.asset_models root
        JOIN public.table_name_ancestors path ON path.descendant_table_name_id = root.table_name_id
        WHERE root.id = row_id AND path.ancestor_table_name_id = 'commercial_centrifuge_lots'
    ) AND NOT EXISTS (SELECT 1 FROM public.commercial_centrifuge_lots subtype WHERE subtype.id = row_id) THEN
        RAISE EXCEPTION '% names a table extending % but has no row there', row_id, 'commercial_centrifuge_lots'
            USING ERRCODE = 'foreign_key_violation';
    END IF;
    RETURN NULL;
END;
$$;
ALTER FUNCTION require_commercial_centrifuge_lots_row() OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION require_commercial_centrifuge_lots_row() FROM PUBLIC;
GRANT SELECT ON commercial_centrifuge_lots TO aps_writer;
CREATE CONSTRAINT TRIGGER commercial_centrifuge_lots_row AFTER INSERT ON asset_models
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW WHEN (NEW.table_name_id = 'commercial_centrifuge_lots')
    EXECUTE FUNCTION require_commercial_centrifuge_lots_row();
CREATE CONSTRAINT TRIGGER commercial_centrifuge_lots_kept AFTER DELETE ON commercial_centrifuge_lots
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION require_commercial_centrifuge_lots_row();

-- Physical centrifuges tracked in APS inventory.
CREATE TABLE centrifuges (
    -- Stable asset identifier inherited from physical_assets.
    id UUID PRIMARY KEY REFERENCES physical_assets (id) ON DELETE CASCADE,
    -- Organization containing this resource.
    organization_id UUID NOT NULL,
    -- Commercial lot model instantiated by this physical asset.
    commercial_centrifuge_lot_id UUID NOT NULL REFERENCES commercial_centrifuge_lots (id),
    FOREIGN KEY (id, commercial_centrifuge_lot_id) REFERENCES assets (id, model_id) ON DELETE CASCADE,
    FOREIGN KEY (organization_id, id) REFERENCES resources (organization_id, id) ON DELETE CASCADE
);
CREATE INDEX ON centrifuges (commercial_centrifuge_lot_id);
INSERT INTO table_names (id) VALUES ('centrifuges');
INSERT INTO table_name_instances (id, instance_table_name_id)
    VALUES ('commercial_centrifuge_lots', 'centrifuges');
INSERT INTO table_name_ancestors (descendant_table_name_id, ancestor_table_name_id)
    SELECT 'centrifuges', ancestor_table_name_id FROM table_name_ancestors WHERE descendant_table_name_id IN ('physical_assets')
    UNION SELECT 'centrifuges', 'centrifuges';
CREATE TRIGGER centrifuges_path AFTER INSERT ON centrifuges
    FOR EACH ROW EXECUTE FUNCTION check_asset_subtype();

CREATE FUNCTION require_centrifuges_row() RETURNS TRIGGER
LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
DECLARE
    row_id UUID := CASE TG_OP WHEN 'DELETE' THEN OLD.id ELSE NEW.id END;
BEGIN
    IF EXISTS (
        SELECT 1 FROM public.assets root
        JOIN public.table_name_ancestors path ON path.descendant_table_name_id = root.table_name_id
        WHERE root.id = row_id AND path.ancestor_table_name_id = 'centrifuges'
    ) AND NOT EXISTS (SELECT 1 FROM public.centrifuges subtype WHERE subtype.id = row_id) THEN
        RAISE EXCEPTION '% names a table extending % but has no row there', row_id, 'centrifuges'
            USING ERRCODE = 'foreign_key_violation';
    END IF;
    RETURN NULL;
END;
$$;
ALTER FUNCTION require_centrifuges_row() OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION require_centrifuges_row() FROM PUBLIC;
GRANT SELECT ON centrifuges TO aps_writer;
CREATE CONSTRAINT TRIGGER centrifuges_row AFTER INSERT ON assets
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW WHEN (NEW.table_name_id = 'centrifuges')
    EXECUTE FUNCTION require_centrifuges_row();
CREATE CONSTRAINT TRIGGER centrifuges_kept AFTER DELETE ON centrifuges
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION require_centrifuges_row();

GRANT SELECT, INSERT, DELETE ON centrifuge_models TO app;
ALTER TABLE centrifuge_models ENABLE ROW LEVEL SECURITY;
ALTER TABLE centrifuge_models FORCE ROW LEVEL SECURITY;
CREATE POLICY centrifuge_models_select ON centrifuge_models FOR SELECT TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY centrifuge_models_insert ON centrifuge_models FOR INSERT TO app
    WITH CHECK (id = ANY (ARRAY(SELECT authz.resources_with('writer'))));
CREATE POLICY centrifuge_models_delete ON centrifuge_models FOR DELETE TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('admin'))));
CREATE POLICY centrifuge_models_archived_insert ON centrifuge_models AS RESTRICTIVE FOR INSERT TO app
    WITH CHECK (NOT authz.is_archived(id));
CREATE POLICY centrifuge_models_archived_delete ON centrifuge_models AS RESTRICTIVE FOR DELETE TO app
    USING (NOT authz.is_archived(id));
CREATE POLICY centrifuge_models_blocked ON centrifuge_models AS RESTRICTIVE FOR ALL TO app
    USING (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))))
    WITH CHECK (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))));

GRANT SELECT, INSERT, DELETE ON commercial_centrifuge_models TO app;
ALTER TABLE commercial_centrifuge_models ENABLE ROW LEVEL SECURITY;
ALTER TABLE commercial_centrifuge_models FORCE ROW LEVEL SECURITY;
CREATE POLICY commercial_centrifuge_models_select ON commercial_centrifuge_models FOR SELECT TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY commercial_centrifuge_models_insert ON commercial_centrifuge_models FOR INSERT TO app
    WITH CHECK (id = ANY (ARRAY(SELECT authz.resources_with('writer')))
        AND centrifuge_model_id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY commercial_centrifuge_models_delete ON commercial_centrifuge_models FOR DELETE TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('admin'))));
CREATE POLICY commercial_centrifuge_models_archived_insert ON commercial_centrifuge_models AS RESTRICTIVE FOR INSERT TO app
    WITH CHECK (NOT authz.is_archived(id) AND NOT authz.is_archived(centrifuge_model_id));
CREATE POLICY commercial_centrifuge_models_archived_delete ON commercial_centrifuge_models AS RESTRICTIVE FOR DELETE TO app
    USING (NOT authz.is_archived(id));
CREATE POLICY commercial_centrifuge_models_blocked ON commercial_centrifuge_models AS RESTRICTIVE FOR ALL TO app
    USING (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))))
    WITH CHECK (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))));

GRANT SELECT, INSERT, DELETE ON commercial_centrifuge_lots TO app;
ALTER TABLE commercial_centrifuge_lots ENABLE ROW LEVEL SECURITY;
ALTER TABLE commercial_centrifuge_lots FORCE ROW LEVEL SECURITY;
CREATE POLICY commercial_centrifuge_lots_select ON commercial_centrifuge_lots FOR SELECT TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY commercial_centrifuge_lots_insert ON commercial_centrifuge_lots FOR INSERT TO app
    WITH CHECK (id = ANY (ARRAY(SELECT authz.resources_with('writer')))
        AND commercial_centrifuge_model_id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY commercial_centrifuge_lots_delete ON commercial_centrifuge_lots FOR DELETE TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('admin'))));
CREATE POLICY commercial_centrifuge_lots_archived_insert ON commercial_centrifuge_lots AS RESTRICTIVE FOR INSERT TO app
    WITH CHECK (NOT authz.is_archived(id) AND NOT authz.is_archived(commercial_centrifuge_model_id));
CREATE POLICY commercial_centrifuge_lots_archived_delete ON commercial_centrifuge_lots AS RESTRICTIVE FOR DELETE TO app
    USING (NOT authz.is_archived(id));
CREATE POLICY commercial_centrifuge_lots_blocked ON commercial_centrifuge_lots AS RESTRICTIVE FOR ALL TO app
    USING (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))))
    WITH CHECK (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))));

GRANT SELECT, INSERT, DELETE ON centrifuges TO app;
ALTER TABLE centrifuges ENABLE ROW LEVEL SECURITY;
ALTER TABLE centrifuges FORCE ROW LEVEL SECURITY;
CREATE POLICY centrifuges_select ON centrifuges FOR SELECT TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY centrifuges_insert ON centrifuges FOR INSERT TO app
    WITH CHECK (id = ANY (ARRAY(SELECT authz.resources_with('writer')))
        AND commercial_centrifuge_lot_id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY centrifuges_delete ON centrifuges FOR DELETE TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('admin'))));
CREATE POLICY centrifuges_archived_insert ON centrifuges AS RESTRICTIVE FOR INSERT TO app
    WITH CHECK (NOT authz.is_archived(id) AND NOT authz.is_archived(commercial_centrifuge_lot_id));
CREATE POLICY centrifuges_archived_delete ON centrifuges AS RESTRICTIVE FOR DELETE TO app
    USING (NOT authz.is_archived(id));
CREATE POLICY centrifuges_blocked ON centrifuges AS RESTRICTIVE FOR ALL TO app
    USING (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))))
    WITH CHECK (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))));

-- Records every change in connetto's `audit_events`.
CREATE TRIGGER centrifuge_models_audit AFTER INSERT OR DELETE ON centrifuge_models
    FOR EACH ROW EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER centrifuge_models_update_audit AFTER UPDATE ON centrifuge_models
    FOR EACH ROW WHEN (OLD IS DISTINCT FROM NEW) EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER commercial_centrifuge_models_audit AFTER INSERT OR DELETE ON commercial_centrifuge_models
    FOR EACH ROW EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER commercial_centrifuge_models_update_audit AFTER UPDATE ON commercial_centrifuge_models
    FOR EACH ROW WHEN (OLD IS DISTINCT FROM NEW) EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER commercial_centrifuge_lots_audit AFTER INSERT OR DELETE ON commercial_centrifuge_lots
    FOR EACH ROW EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER commercial_centrifuge_lots_update_audit AFTER UPDATE ON commercial_centrifuge_lots
    FOR EACH ROW WHEN (OLD IS DISTINCT FROM NEW) EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER centrifuges_audit AFTER INSERT OR DELETE ON centrifuges
    FOR EACH ROW EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER centrifuges_update_audit AFTER UPDATE ON centrifuges
    FOR EACH ROW WHEN (OLD IS DISTINCT FROM NEW) EXECUTE FUNCTION authz_write.record_audit_event();
