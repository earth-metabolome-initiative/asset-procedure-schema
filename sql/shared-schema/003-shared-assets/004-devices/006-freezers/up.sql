-- Migration: Shared Assets / Devices / Freezers
-- Purpose: Freezers for project inventories.
-- APS placement
--   `resources` of kind `'ownable'`
--       +-- asset_models
--           +-- physical_asset_models
--               +-- freezer_models
--                   +-- commercial_freezer_models
--               +-- commercial_product_lots
--                   +-- commercial_freezer_lots
--       +-- assets
--           +-- physical_assets
--               +-- freezers
-- Commercial tables decision: Vendor models and lots provide purchase traceability.

-- Catalog of freezer model definitions.
CREATE TABLE freezer_models (
    -- Stable model identifier inherited from physical_asset_models.
    id UUID PRIMARY KEY REFERENCES physical_asset_models (id) ON DELETE CASCADE,
    -- Organization containing this resource.
    organization_id UUID NOT NULL,
    -- Archive state inherited from this resource.
    archived BOOLEAN NOT NULL DEFAULT FALSE,
    UNIQUE (id, archived),
    FOREIGN KEY (organization_id, id, archived) REFERENCES resources (organization_id, id, archived) ON UPDATE CASCADE ON DELETE CASCADE
);
INSERT INTO table_names (id) VALUES ('freezer_models');
INSERT INTO table_name_ancestors (descendant_table_name_id, ancestor_table_name_id)
    SELECT 'freezer_models', ancestor_table_name_id FROM table_name_ancestors WHERE descendant_table_name_id IN ('physical_asset_models')
    UNION SELECT 'freezer_models', 'freezer_models';
CREATE TRIGGER freezer_models_path AFTER INSERT ON freezer_models
    FOR EACH ROW EXECUTE FUNCTION check_asset_model_subtype();

CREATE FUNCTION require_freezer_models_row() RETURNS TRIGGER
LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
DECLARE
    row_id UUID := CASE TG_OP WHEN 'DELETE' THEN OLD.id ELSE NEW.id END;
BEGIN
    IF EXISTS (
        SELECT 1 FROM public.asset_models root
        JOIN public.table_name_ancestors path ON path.descendant_table_name_id = root.table_name_id
        WHERE root.id = row_id AND path.ancestor_table_name_id = 'freezer_models'
    ) AND NOT EXISTS (SELECT 1 FROM public.freezer_models subtype WHERE subtype.id = row_id) THEN
        RAISE EXCEPTION '% names a table extending % but has no row there', row_id, 'freezer_models'
            USING ERRCODE = 'foreign_key_violation';
    END IF;
    RETURN NULL;
END;
$$;
ALTER FUNCTION require_freezer_models_row() OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION require_freezer_models_row() FROM PUBLIC;
REVOKE ALL ON freezer_models FROM PUBLIC, app, identity_writer;
GRANT SELECT ON freezer_models TO aps_writer;
CREATE CONSTRAINT TRIGGER freezer_models_row AFTER INSERT ON asset_models
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW WHEN (NEW.table_name_id = 'freezer_models')
    EXECUTE FUNCTION require_freezer_models_row();
CREATE CONSTRAINT TRIGGER freezer_models_kept AFTER DELETE ON freezer_models
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION require_freezer_models_row();

-- Catalog of commercial freezer models.
CREATE TABLE commercial_freezer_models (
    -- Stable commercial model identifier shared with parent model tables.
    id UUID PRIMARY KEY REFERENCES freezer_models (id) ON DELETE CASCADE,
    -- Organization containing this resource.
    organization_id UUID NOT NULL,
    -- Base freezer model represented by this commercial model.
    freezer_model_id UUID NOT NULL REFERENCES freezer_models (id) ON DELETE RESTRICT,
    FOREIGN KEY (id) REFERENCES commercial_products (id) ON DELETE CASCADE,
    FOREIGN KEY (id, freezer_model_id) REFERENCES asset_models (id, parent_model_id) ON DELETE CASCADE,
    -- Archive state inherited from this resource.
    archived BOOLEAN NOT NULL DEFAULT FALSE,
    -- Archive state of the row named by `freezer_model_id`.
    freezer_model_archived BOOLEAN NOT NULL DEFAULT FALSE,
    UNIQUE (id, archived),
    FOREIGN KEY (freezer_model_id, freezer_model_archived) REFERENCES freezer_models (id, archived) ON UPDATE CASCADE,
    FOREIGN KEY (organization_id, id, archived) REFERENCES resources (organization_id, id, archived) ON UPDATE CASCADE ON DELETE CASCADE
);
CREATE INDEX ON commercial_freezer_models (freezer_model_id);
INSERT INTO table_names (id) VALUES ('commercial_freezer_models');
INSERT INTO table_name_ancestors (descendant_table_name_id, ancestor_table_name_id)
    SELECT 'commercial_freezer_models', ancestor_table_name_id FROM table_name_ancestors WHERE descendant_table_name_id IN ('freezer_models', 'commercial_products')
    UNION SELECT 'commercial_freezer_models', 'commercial_freezer_models';
CREATE TRIGGER commercial_freezer_models_path AFTER INSERT ON commercial_freezer_models
    FOR EACH ROW EXECUTE FUNCTION check_asset_model_subtype();

CREATE FUNCTION require_commercial_freezer_models_row() RETURNS TRIGGER
LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
DECLARE
    row_id UUID := CASE TG_OP WHEN 'DELETE' THEN OLD.id ELSE NEW.id END;
BEGIN
    IF EXISTS (
        SELECT 1 FROM public.asset_models root
        JOIN public.table_name_ancestors path ON path.descendant_table_name_id = root.table_name_id
        WHERE root.id = row_id AND path.ancestor_table_name_id = 'commercial_freezer_models'
    ) AND NOT EXISTS (SELECT 1 FROM public.commercial_freezer_models subtype WHERE subtype.id = row_id) THEN
        RAISE EXCEPTION '% names a table extending % but has no row there', row_id, 'commercial_freezer_models'
            USING ERRCODE = 'foreign_key_violation';
    END IF;
    RETURN NULL;
END;
$$;
ALTER FUNCTION require_commercial_freezer_models_row() OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION require_commercial_freezer_models_row() FROM PUBLIC;
REVOKE ALL ON commercial_freezer_models FROM PUBLIC, app, identity_writer;
GRANT SELECT ON commercial_freezer_models TO aps_writer;
CREATE CONSTRAINT TRIGGER commercial_freezer_models_row AFTER INSERT ON asset_models
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW WHEN (NEW.table_name_id = 'commercial_freezer_models')
    EXECUTE FUNCTION require_commercial_freezer_models_row();
CREATE CONSTRAINT TRIGGER commercial_freezer_models_kept AFTER DELETE ON commercial_freezer_models
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION require_commercial_freezer_models_row();

-- Catalog of lot-specific commercial freezer models.
CREATE TABLE commercial_freezer_lots (
    -- Stable lot-model identifier shared with parent lot/model tables.
    id UUID PRIMARY KEY REFERENCES commercial_product_lots (id) ON DELETE CASCADE,
    -- Organization containing this resource.
    organization_id UUID NOT NULL,
    FOREIGN KEY (id) REFERENCES freezer_models (id) ON DELETE CASCADE,
    -- Commercial freezer model from which this lot derives.
    commercial_freezer_model_id UUID NOT NULL REFERENCES commercial_freezer_models (id),
    FOREIGN KEY (id, commercial_freezer_model_id) REFERENCES asset_models (id, parent_model_id) ON DELETE CASCADE,
    -- Archive state inherited from this resource.
    archived BOOLEAN NOT NULL DEFAULT FALSE,
    -- Archive state of the row named by `commercial_freezer_model_id`.
    commercial_freezer_model_archived BOOLEAN NOT NULL DEFAULT FALSE,
    UNIQUE (id, archived),
    FOREIGN KEY (commercial_freezer_model_id, commercial_freezer_model_archived) REFERENCES commercial_freezer_models (id, archived) ON UPDATE CASCADE,
    FOREIGN KEY (organization_id, id, archived) REFERENCES resources (organization_id, id, archived) ON UPDATE CASCADE ON DELETE CASCADE
);
CREATE INDEX ON commercial_freezer_lots (commercial_freezer_model_id);
INSERT INTO table_names (id) VALUES ('commercial_freezer_lots');
INSERT INTO table_name_ancestors (descendant_table_name_id, ancestor_table_name_id)
    SELECT 'commercial_freezer_lots', ancestor_table_name_id FROM table_name_ancestors WHERE descendant_table_name_id IN ('commercial_product_lots', 'freezer_models')
    UNION SELECT 'commercial_freezer_lots', 'commercial_freezer_lots';
INSERT INTO table_name_admitted_parents (table_name_id, parent_table_name_id)
    VALUES ('commercial_freezer_lots', 'commercial_freezer_models');
CREATE TRIGGER commercial_freezer_lots_path AFTER INSERT ON commercial_freezer_lots
    FOR EACH ROW EXECUTE FUNCTION check_asset_model_subtype();

CREATE FUNCTION require_commercial_freezer_lots_row() RETURNS TRIGGER
LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
DECLARE
    row_id UUID := CASE TG_OP WHEN 'DELETE' THEN OLD.id ELSE NEW.id END;
BEGIN
    IF EXISTS (
        SELECT 1 FROM public.asset_models root
        JOIN public.table_name_ancestors path ON path.descendant_table_name_id = root.table_name_id
        WHERE root.id = row_id AND path.ancestor_table_name_id = 'commercial_freezer_lots'
    ) AND NOT EXISTS (SELECT 1 FROM public.commercial_freezer_lots subtype WHERE subtype.id = row_id) THEN
        RAISE EXCEPTION '% names a table extending % but has no row there', row_id, 'commercial_freezer_lots'
            USING ERRCODE = 'foreign_key_violation';
    END IF;
    RETURN NULL;
END;
$$;
ALTER FUNCTION require_commercial_freezer_lots_row() OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION require_commercial_freezer_lots_row() FROM PUBLIC;
REVOKE ALL ON commercial_freezer_lots FROM PUBLIC, app, identity_writer;
GRANT SELECT ON commercial_freezer_lots TO aps_writer;
CREATE CONSTRAINT TRIGGER commercial_freezer_lots_row AFTER INSERT ON asset_models
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW WHEN (NEW.table_name_id = 'commercial_freezer_lots')
    EXECUTE FUNCTION require_commercial_freezer_lots_row();
CREATE CONSTRAINT TRIGGER commercial_freezer_lots_kept AFTER DELETE ON commercial_freezer_lots
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION require_commercial_freezer_lots_row();

-- Physical freezers tracked in APS inventory.
CREATE TABLE freezers (
    -- Stable asset identifier inherited from physical_assets.
    id UUID PRIMARY KEY REFERENCES physical_assets (id) ON DELETE CASCADE,
    -- Organization containing this resource.
    organization_id UUID NOT NULL,
    -- Commercial lot model instantiated by this physical asset.
    commercial_freezer_lot_id UUID NOT NULL REFERENCES commercial_freezer_lots (id),
    FOREIGN KEY (id, commercial_freezer_lot_id) REFERENCES assets (id, model_id) ON DELETE CASCADE,
    -- Archive state inherited from this resource.
    archived BOOLEAN NOT NULL DEFAULT FALSE,
    -- Archive state of the row named by `commercial_freezer_lot_id`.
    commercial_freezer_lot_archived BOOLEAN NOT NULL DEFAULT FALSE,
    FOREIGN KEY (commercial_freezer_lot_id, commercial_freezer_lot_archived) REFERENCES commercial_freezer_lots (id, archived) ON UPDATE CASCADE,
    FOREIGN KEY (organization_id, id, archived) REFERENCES resources (organization_id, id, archived) ON UPDATE CASCADE ON DELETE CASCADE
);
CREATE INDEX ON freezers (commercial_freezer_lot_id);
INSERT INTO table_names (id) VALUES ('freezers');
UPDATE table_names SET instance_table_name_id = 'freezers'
    WHERE id = 'commercial_freezer_lots';
INSERT INTO table_name_ancestors (descendant_table_name_id, ancestor_table_name_id)
    SELECT 'freezers', ancestor_table_name_id FROM table_name_ancestors WHERE descendant_table_name_id IN ('physical_assets')
    UNION SELECT 'freezers', 'freezers';
CREATE TRIGGER freezers_path AFTER INSERT ON freezers
    FOR EACH ROW EXECUTE FUNCTION check_asset_subtype();

CREATE FUNCTION require_freezers_row() RETURNS TRIGGER
LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
DECLARE
    row_id UUID := CASE TG_OP WHEN 'DELETE' THEN OLD.id ELSE NEW.id END;
BEGIN
    IF EXISTS (
        SELECT 1 FROM public.assets root
        JOIN public.table_name_ancestors path ON path.descendant_table_name_id = root.table_name_id
        WHERE root.id = row_id AND path.ancestor_table_name_id = 'freezers'
    ) AND NOT EXISTS (SELECT 1 FROM public.freezers subtype WHERE subtype.id = row_id) THEN
        RAISE EXCEPTION '% names a table extending % but has no row there', row_id, 'freezers'
            USING ERRCODE = 'foreign_key_violation';
    END IF;
    RETURN NULL;
END;
$$;
ALTER FUNCTION require_freezers_row() OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION require_freezers_row() FROM PUBLIC;
REVOKE ALL ON freezers FROM PUBLIC, app, identity_writer;
GRANT SELECT ON freezers TO aps_writer;
CREATE CONSTRAINT TRIGGER freezers_row AFTER INSERT ON assets
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW WHEN (NEW.table_name_id = 'freezers')
    EXECUTE FUNCTION require_freezers_row();
CREATE CONSTRAINT TRIGGER freezers_kept AFTER DELETE ON freezers
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION require_freezers_row();

GRANT SELECT, INSERT, DELETE ON freezer_models TO app;
ALTER TABLE freezer_models ENABLE ROW LEVEL SECURITY;
ALTER TABLE freezer_models FORCE ROW LEVEL SECURITY;
CREATE POLICY freezer_models_select ON freezer_models FOR SELECT TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY freezer_models_insert ON freezer_models FOR INSERT TO app
    WITH CHECK (id = ANY (ARRAY(SELECT authz.resources_with('writer'))));
CREATE POLICY freezer_models_delete ON freezer_models FOR DELETE TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('admin'))));
CREATE POLICY freezer_models_archived_insert ON freezer_models AS RESTRICTIVE FOR INSERT TO app
    WITH CHECK (NOT archived);
CREATE POLICY freezer_models_archived_delete ON freezer_models AS RESTRICTIVE FOR DELETE TO app
    USING (NOT archived);
CREATE POLICY freezer_models_blocked ON freezer_models AS RESTRICTIVE FOR ALL TO app
    USING (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))))
    WITH CHECK (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))));

GRANT SELECT, INSERT, DELETE ON commercial_freezer_models TO app;
ALTER TABLE commercial_freezer_models ENABLE ROW LEVEL SECURITY;
ALTER TABLE commercial_freezer_models FORCE ROW LEVEL SECURITY;
CREATE POLICY commercial_freezer_models_select ON commercial_freezer_models FOR SELECT TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY commercial_freezer_models_insert ON commercial_freezer_models FOR INSERT TO app
    WITH CHECK (id = ANY (ARRAY(SELECT authz.resources_with('writer')))
        AND freezer_model_id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY commercial_freezer_models_delete ON commercial_freezer_models FOR DELETE TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('admin'))));
CREATE POLICY commercial_freezer_models_archived_insert ON commercial_freezer_models AS RESTRICTIVE FOR INSERT TO app
    WITH CHECK (NOT archived AND NOT freezer_model_archived);
CREATE POLICY commercial_freezer_models_archived_delete ON commercial_freezer_models AS RESTRICTIVE FOR DELETE TO app
    USING (NOT archived);
CREATE POLICY commercial_freezer_models_blocked ON commercial_freezer_models AS RESTRICTIVE FOR ALL TO app
    USING (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))))
    WITH CHECK (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))));

GRANT SELECT, INSERT, DELETE ON commercial_freezer_lots TO app;
ALTER TABLE commercial_freezer_lots ENABLE ROW LEVEL SECURITY;
ALTER TABLE commercial_freezer_lots FORCE ROW LEVEL SECURITY;
CREATE POLICY commercial_freezer_lots_select ON commercial_freezer_lots FOR SELECT TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY commercial_freezer_lots_insert ON commercial_freezer_lots FOR INSERT TO app
    WITH CHECK (id = ANY (ARRAY(SELECT authz.resources_with('writer')))
        AND commercial_freezer_model_id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY commercial_freezer_lots_delete ON commercial_freezer_lots FOR DELETE TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('admin'))));
CREATE POLICY commercial_freezer_lots_archived_insert ON commercial_freezer_lots AS RESTRICTIVE FOR INSERT TO app
    WITH CHECK (NOT archived AND NOT commercial_freezer_model_archived);
CREATE POLICY commercial_freezer_lots_archived_delete ON commercial_freezer_lots AS RESTRICTIVE FOR DELETE TO app
    USING (NOT archived);
CREATE POLICY commercial_freezer_lots_blocked ON commercial_freezer_lots AS RESTRICTIVE FOR ALL TO app
    USING (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))))
    WITH CHECK (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))));

GRANT SELECT, INSERT, DELETE ON freezers TO app;
ALTER TABLE freezers ENABLE ROW LEVEL SECURITY;
ALTER TABLE freezers FORCE ROW LEVEL SECURITY;
CREATE POLICY freezers_select ON freezers FOR SELECT TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY freezers_insert ON freezers FOR INSERT TO app
    WITH CHECK (id = ANY (ARRAY(SELECT authz.resources_with('writer')))
        AND commercial_freezer_lot_id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY freezers_delete ON freezers FOR DELETE TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('admin'))));
CREATE POLICY freezers_archived_insert ON freezers AS RESTRICTIVE FOR INSERT TO app
    WITH CHECK (NOT archived AND NOT commercial_freezer_lot_archived);
CREATE POLICY freezers_archived_delete ON freezers AS RESTRICTIVE FOR DELETE TO app
    USING (NOT archived);
CREATE POLICY freezers_blocked ON freezers AS RESTRICTIVE FOR ALL TO app
    USING (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))))
    WITH CHECK (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))));

-- Records every change in connetto's `audit_events`.
CREATE TRIGGER freezer_models_audit AFTER INSERT OR DELETE ON freezer_models
    FOR EACH ROW EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER freezer_models_update_audit AFTER UPDATE ON freezer_models
    FOR EACH ROW WHEN (OLD IS DISTINCT FROM NEW) EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER commercial_freezer_models_audit AFTER INSERT OR DELETE ON commercial_freezer_models
    FOR EACH ROW EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER commercial_freezer_models_update_audit AFTER UPDATE ON commercial_freezer_models
    FOR EACH ROW WHEN (OLD IS DISTINCT FROM NEW) EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER commercial_freezer_lots_audit AFTER INSERT OR DELETE ON commercial_freezer_lots
    FOR EACH ROW EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER commercial_freezer_lots_update_audit AFTER UPDATE ON commercial_freezer_lots
    FOR EACH ROW WHEN (OLD IS DISTINCT FROM NEW) EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER freezers_audit AFTER INSERT OR DELETE ON freezers
    FOR EACH ROW EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER freezers_update_audit AFTER UPDATE ON freezers
    FOR EACH ROW WHEN (OLD IS DISTINCT FROM NEW) EXECUTE FUNCTION authz_write.record_audit_event();
