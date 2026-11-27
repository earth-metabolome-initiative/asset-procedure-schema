-- Migration: Shared Assets / Supplies / Pipette Tips
-- Purpose: Pipette Tips for project inventories.
-- APS placement
--   `resources` of kind `'ownable'`
--       +-- asset_models
--           +-- physical_asset_models
--               +-- pipette_tip_models
--                   +-- commercial_pipette_tip_models
--               +-- commercial_product_lots
--                   +-- commercial_pipette_tip_lots
--       +-- assets
--           +-- physical_assets
--               +-- pipette_tips
-- Commercial tables decision: Vendor models and lots provide purchase traceability.

-- Catalog of pipette tip model definitions.
CREATE TABLE pipette_tip_models (
    -- Stable model identifier inherited from physical_asset_models.
    id UUID PRIMARY KEY REFERENCES physical_asset_models (id) ON DELETE CASCADE,
    -- Organization containing this resource.
    organization_id UUID NOT NULL,
    -- Archive state inherited from this resource.
    archived BOOLEAN NOT NULL DEFAULT FALSE,
    UNIQUE (id, archived),
    FOREIGN KEY (organization_id, id, archived) REFERENCES resources (organization_id, id, archived) ON UPDATE CASCADE ON DELETE CASCADE
);
INSERT INTO table_names (id) VALUES ('pipette_tip_models');
INSERT INTO table_name_ancestors (descendant_table_name_id, ancestor_table_name_id)
    SELECT 'pipette_tip_models', ancestor_table_name_id FROM table_name_ancestors WHERE descendant_table_name_id IN ('physical_asset_models')
    UNION SELECT 'pipette_tip_models', 'pipette_tip_models';
CREATE TRIGGER pipette_tip_models_path AFTER INSERT ON pipette_tip_models
    FOR EACH ROW EXECUTE FUNCTION check_asset_model_subtype();

CREATE FUNCTION require_pipette_tip_models_row() RETURNS TRIGGER
LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
DECLARE
    row_id UUID := CASE TG_OP WHEN 'DELETE' THEN OLD.id ELSE NEW.id END;
BEGIN
    IF EXISTS (
        SELECT 1 FROM public.asset_models root
        JOIN public.table_name_ancestors path ON path.descendant_table_name_id = root.table_name_id
        WHERE root.id = row_id AND path.ancestor_table_name_id = 'pipette_tip_models'
    ) AND NOT EXISTS (SELECT 1 FROM public.pipette_tip_models subtype WHERE subtype.id = row_id) THEN
        RAISE EXCEPTION '% names a table extending % but has no row there', row_id, 'pipette_tip_models'
            USING ERRCODE = 'foreign_key_violation';
    END IF;
    RETURN NULL;
END;
$$;
ALTER FUNCTION require_pipette_tip_models_row() OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION require_pipette_tip_models_row() FROM PUBLIC;
REVOKE ALL ON pipette_tip_models FROM PUBLIC, app, identity_writer;
GRANT SELECT ON pipette_tip_models TO aps_writer;
CREATE CONSTRAINT TRIGGER pipette_tip_models_row AFTER INSERT ON asset_models
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW WHEN (NEW.table_name_id = 'pipette_tip_models')
    EXECUTE FUNCTION require_pipette_tip_models_row();
CREATE CONSTRAINT TRIGGER pipette_tip_models_kept AFTER DELETE ON pipette_tip_models
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION require_pipette_tip_models_row();

-- Catalog of commercial pipette tip models.
CREATE TABLE commercial_pipette_tip_models (
    -- Stable commercial model identifier shared with parent model tables.
    id UUID PRIMARY KEY REFERENCES pipette_tip_models (id) ON DELETE CASCADE,
    -- Organization containing this resource.
    organization_id UUID NOT NULL,
    -- Base pipette tip model represented by this commercial model.
    pipette_tip_model_id UUID NOT NULL REFERENCES pipette_tip_models (id) ON DELETE RESTRICT,
    FOREIGN KEY (id) REFERENCES commercial_products (id) ON DELETE CASCADE,
    FOREIGN KEY (id, pipette_tip_model_id) REFERENCES asset_models (id, parent_model_id) ON DELETE CASCADE,
    -- Archive state inherited from this resource.
    archived BOOLEAN NOT NULL DEFAULT FALSE,
    -- Archive state of the row named by `pipette_tip_model_id`.
    pipette_tip_model_archived BOOLEAN NOT NULL DEFAULT FALSE,
    UNIQUE (id, archived),
    FOREIGN KEY (pipette_tip_model_id, pipette_tip_model_archived) REFERENCES pipette_tip_models (id, archived) ON UPDATE CASCADE,
    FOREIGN KEY (organization_id, id, archived) REFERENCES resources (organization_id, id, archived) ON UPDATE CASCADE ON DELETE CASCADE
);
CREATE INDEX ON commercial_pipette_tip_models (pipette_tip_model_id);
INSERT INTO table_names (id) VALUES ('commercial_pipette_tip_models');
INSERT INTO table_name_ancestors (descendant_table_name_id, ancestor_table_name_id)
    SELECT 'commercial_pipette_tip_models', ancestor_table_name_id FROM table_name_ancestors WHERE descendant_table_name_id IN ('pipette_tip_models', 'commercial_products')
    UNION SELECT 'commercial_pipette_tip_models', 'commercial_pipette_tip_models';
CREATE TRIGGER commercial_pipette_tip_models_path AFTER INSERT ON commercial_pipette_tip_models
    FOR EACH ROW EXECUTE FUNCTION check_asset_model_subtype();

CREATE FUNCTION require_commercial_pipette_tip_models_row() RETURNS TRIGGER
LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
DECLARE
    row_id UUID := CASE TG_OP WHEN 'DELETE' THEN OLD.id ELSE NEW.id END;
BEGIN
    IF EXISTS (
        SELECT 1 FROM public.asset_models root
        JOIN public.table_name_ancestors path ON path.descendant_table_name_id = root.table_name_id
        WHERE root.id = row_id AND path.ancestor_table_name_id = 'commercial_pipette_tip_models'
    ) AND NOT EXISTS (SELECT 1 FROM public.commercial_pipette_tip_models subtype WHERE subtype.id = row_id) THEN
        RAISE EXCEPTION '% names a table extending % but has no row there', row_id, 'commercial_pipette_tip_models'
            USING ERRCODE = 'foreign_key_violation';
    END IF;
    RETURN NULL;
END;
$$;
ALTER FUNCTION require_commercial_pipette_tip_models_row() OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION require_commercial_pipette_tip_models_row() FROM PUBLIC;
REVOKE ALL ON commercial_pipette_tip_models FROM PUBLIC, app, identity_writer;
GRANT SELECT ON commercial_pipette_tip_models TO aps_writer;
CREATE CONSTRAINT TRIGGER commercial_pipette_tip_models_row AFTER INSERT ON asset_models
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW WHEN (NEW.table_name_id = 'commercial_pipette_tip_models')
    EXECUTE FUNCTION require_commercial_pipette_tip_models_row();
CREATE CONSTRAINT TRIGGER commercial_pipette_tip_models_kept AFTER DELETE ON commercial_pipette_tip_models
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION require_commercial_pipette_tip_models_row();

-- Catalog of lot-specific commercial pipette tip models.
CREATE TABLE commercial_pipette_tip_lots (
    -- Stable lot-model identifier shared with parent lot/model tables.
    id UUID PRIMARY KEY REFERENCES commercial_product_lots (id) ON DELETE CASCADE,
    -- Organization containing this resource.
    organization_id UUID NOT NULL,
    FOREIGN KEY (id) REFERENCES pipette_tip_models (id) ON DELETE CASCADE,
    -- Commercial pipette tip model from which this lot derives.
    commercial_pipette_tip_model_id UUID NOT NULL REFERENCES commercial_pipette_tip_models (id),
    FOREIGN KEY (id, commercial_pipette_tip_model_id) REFERENCES asset_models (id, parent_model_id) ON DELETE CASCADE,
    -- Archive state inherited from this resource.
    archived BOOLEAN NOT NULL DEFAULT FALSE,
    -- Archive state of the row named by `commercial_pipette_tip_model_id`.
    commercial_pipette_tip_model_archived BOOLEAN NOT NULL DEFAULT FALSE,
    FOREIGN KEY (commercial_pipette_tip_model_id, commercial_pipette_tip_model_archived) REFERENCES commercial_pipette_tip_models (id, archived) ON UPDATE CASCADE,
    FOREIGN KEY (organization_id, id, archived) REFERENCES resources (organization_id, id, archived) ON UPDATE CASCADE ON DELETE CASCADE
);
CREATE INDEX ON commercial_pipette_tip_lots (commercial_pipette_tip_model_id);
INSERT INTO table_names (id) VALUES ('commercial_pipette_tip_lots');
INSERT INTO table_name_ancestors (descendant_table_name_id, ancestor_table_name_id)
    SELECT 'commercial_pipette_tip_lots', ancestor_table_name_id FROM table_name_ancestors WHERE descendant_table_name_id IN ('commercial_product_lots', 'pipette_tip_models')
    UNION SELECT 'commercial_pipette_tip_lots', 'commercial_pipette_tip_lots';
INSERT INTO table_name_admitted_parents (table_name_id, parent_table_name_id)
    VALUES ('commercial_pipette_tip_lots', 'commercial_pipette_tip_models');
CREATE TRIGGER commercial_pipette_tip_lots_path AFTER INSERT ON commercial_pipette_tip_lots
    FOR EACH ROW EXECUTE FUNCTION check_asset_model_subtype();

CREATE FUNCTION require_commercial_pipette_tip_lots_row() RETURNS TRIGGER
LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
DECLARE
    row_id UUID := CASE TG_OP WHEN 'DELETE' THEN OLD.id ELSE NEW.id END;
BEGIN
    IF EXISTS (
        SELECT 1 FROM public.asset_models root
        JOIN public.table_name_ancestors path ON path.descendant_table_name_id = root.table_name_id
        WHERE root.id = row_id AND path.ancestor_table_name_id = 'commercial_pipette_tip_lots'
    ) AND NOT EXISTS (SELECT 1 FROM public.commercial_pipette_tip_lots subtype WHERE subtype.id = row_id) THEN
        RAISE EXCEPTION '% names a table extending % but has no row there', row_id, 'commercial_pipette_tip_lots'
            USING ERRCODE = 'foreign_key_violation';
    END IF;
    RETURN NULL;
END;
$$;
ALTER FUNCTION require_commercial_pipette_tip_lots_row() OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION require_commercial_pipette_tip_lots_row() FROM PUBLIC;
REVOKE ALL ON commercial_pipette_tip_lots FROM PUBLIC, app, identity_writer;
GRANT SELECT ON commercial_pipette_tip_lots TO aps_writer;
CREATE CONSTRAINT TRIGGER commercial_pipette_tip_lots_row AFTER INSERT ON asset_models
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW WHEN (NEW.table_name_id = 'commercial_pipette_tip_lots')
    EXECUTE FUNCTION require_commercial_pipette_tip_lots_row();
CREATE CONSTRAINT TRIGGER commercial_pipette_tip_lots_kept AFTER DELETE ON commercial_pipette_tip_lots
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION require_commercial_pipette_tip_lots_row();

-- Physical pipette tip assets tracked in APS inventory.
CREATE TABLE pipette_tips (
    -- Stable asset identifier inherited from physical_assets.
    id UUID PRIMARY KEY REFERENCES physical_assets (id) ON DELETE CASCADE,
    -- Organization containing this resource.
    organization_id UUID NOT NULL,
    -- Pipette tip model instantiated by this physical asset.
    pipette_tip_model_id UUID NOT NULL REFERENCES pipette_tip_models (id),
    FOREIGN KEY (id, pipette_tip_model_id) REFERENCES assets (id, model_id) ON DELETE CASCADE,
    -- Archive state inherited from this resource.
    archived BOOLEAN NOT NULL DEFAULT FALSE,
    -- Archive state of the row named by `pipette_tip_model_id`.
    pipette_tip_model_archived BOOLEAN NOT NULL DEFAULT FALSE,
    FOREIGN KEY (pipette_tip_model_id, pipette_tip_model_archived) REFERENCES pipette_tip_models (id, archived) ON UPDATE CASCADE,
    FOREIGN KEY (organization_id, id, archived) REFERENCES resources (organization_id, id, archived) ON UPDATE CASCADE ON DELETE CASCADE
);
CREATE INDEX ON pipette_tips (pipette_tip_model_id);
INSERT INTO table_names (id) VALUES ('pipette_tips');
UPDATE table_names SET instance_table_name_id = 'pipette_tips'
    WHERE id = 'pipette_tip_models';
INSERT INTO table_name_ancestors (descendant_table_name_id, ancestor_table_name_id)
    SELECT 'pipette_tips', ancestor_table_name_id FROM table_name_ancestors WHERE descendant_table_name_id IN ('physical_assets')
    UNION SELECT 'pipette_tips', 'pipette_tips';
CREATE TRIGGER pipette_tips_path AFTER INSERT ON pipette_tips
    FOR EACH ROW EXECUTE FUNCTION check_asset_subtype();

CREATE FUNCTION require_pipette_tips_row() RETURNS TRIGGER
LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
DECLARE
    row_id UUID := CASE TG_OP WHEN 'DELETE' THEN OLD.id ELSE NEW.id END;
BEGIN
    IF EXISTS (
        SELECT 1 FROM public.assets root
        JOIN public.table_name_ancestors path ON path.descendant_table_name_id = root.table_name_id
        WHERE root.id = row_id AND path.ancestor_table_name_id = 'pipette_tips'
    ) AND NOT EXISTS (SELECT 1 FROM public.pipette_tips subtype WHERE subtype.id = row_id) THEN
        RAISE EXCEPTION '% names a table extending % but has no row there', row_id, 'pipette_tips'
            USING ERRCODE = 'foreign_key_violation';
    END IF;
    RETURN NULL;
END;
$$;
ALTER FUNCTION require_pipette_tips_row() OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION require_pipette_tips_row() FROM PUBLIC;
REVOKE ALL ON pipette_tips FROM PUBLIC, app, identity_writer;
GRANT SELECT ON pipette_tips TO aps_writer;
CREATE CONSTRAINT TRIGGER pipette_tips_row AFTER INSERT ON assets
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW WHEN (NEW.table_name_id = 'pipette_tips')
    EXECUTE FUNCTION require_pipette_tips_row();
CREATE CONSTRAINT TRIGGER pipette_tips_kept AFTER DELETE ON pipette_tips
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION require_pipette_tips_row();

GRANT SELECT, INSERT, DELETE ON pipette_tip_models TO app;
ALTER TABLE pipette_tip_models ENABLE ROW LEVEL SECURITY;
ALTER TABLE pipette_tip_models FORCE ROW LEVEL SECURITY;
CREATE POLICY pipette_tip_models_select ON pipette_tip_models FOR SELECT TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY pipette_tip_models_insert ON pipette_tip_models FOR INSERT TO app
    WITH CHECK (id = ANY (ARRAY(SELECT authz.resources_with('writer'))));
CREATE POLICY pipette_tip_models_delete ON pipette_tip_models FOR DELETE TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('admin'))));
CREATE POLICY pipette_tip_models_archived_insert ON pipette_tip_models AS RESTRICTIVE FOR INSERT TO app
    WITH CHECK (NOT archived);
CREATE POLICY pipette_tip_models_archived_delete ON pipette_tip_models AS RESTRICTIVE FOR DELETE TO app
    USING (NOT archived);
CREATE POLICY pipette_tip_models_blocked ON pipette_tip_models AS RESTRICTIVE FOR ALL TO app
    USING (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))))
    WITH CHECK (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))));

GRANT SELECT, INSERT, DELETE ON commercial_pipette_tip_models TO app;
ALTER TABLE commercial_pipette_tip_models ENABLE ROW LEVEL SECURITY;
ALTER TABLE commercial_pipette_tip_models FORCE ROW LEVEL SECURITY;
CREATE POLICY commercial_pipette_tip_models_select ON commercial_pipette_tip_models FOR SELECT TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY commercial_pipette_tip_models_insert ON commercial_pipette_tip_models FOR INSERT TO app
    WITH CHECK (id = ANY (ARRAY(SELECT authz.resources_with('writer')))
        AND pipette_tip_model_id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY commercial_pipette_tip_models_delete ON commercial_pipette_tip_models FOR DELETE TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('admin'))));
CREATE POLICY commercial_pipette_tip_models_archived_insert ON commercial_pipette_tip_models AS RESTRICTIVE FOR INSERT TO app
    WITH CHECK (NOT archived AND NOT pipette_tip_model_archived);
CREATE POLICY commercial_pipette_tip_models_archived_delete ON commercial_pipette_tip_models AS RESTRICTIVE FOR DELETE TO app
    USING (NOT archived);
CREATE POLICY commercial_pipette_tip_models_blocked ON commercial_pipette_tip_models AS RESTRICTIVE FOR ALL TO app
    USING (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))))
    WITH CHECK (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))));

GRANT SELECT, INSERT, DELETE ON commercial_pipette_tip_lots TO app;
ALTER TABLE commercial_pipette_tip_lots ENABLE ROW LEVEL SECURITY;
ALTER TABLE commercial_pipette_tip_lots FORCE ROW LEVEL SECURITY;
CREATE POLICY commercial_pipette_tip_lots_select ON commercial_pipette_tip_lots FOR SELECT TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY commercial_pipette_tip_lots_insert ON commercial_pipette_tip_lots FOR INSERT TO app
    WITH CHECK (id = ANY (ARRAY(SELECT authz.resources_with('writer')))
        AND commercial_pipette_tip_model_id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY commercial_pipette_tip_lots_delete ON commercial_pipette_tip_lots FOR DELETE TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('admin'))));
CREATE POLICY commercial_pipette_tip_lots_archived_insert ON commercial_pipette_tip_lots AS RESTRICTIVE FOR INSERT TO app
    WITH CHECK (NOT archived AND NOT commercial_pipette_tip_model_archived);
CREATE POLICY commercial_pipette_tip_lots_archived_delete ON commercial_pipette_tip_lots AS RESTRICTIVE FOR DELETE TO app
    USING (NOT archived);
CREATE POLICY commercial_pipette_tip_lots_blocked ON commercial_pipette_tip_lots AS RESTRICTIVE FOR ALL TO app
    USING (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))))
    WITH CHECK (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))));

GRANT SELECT, INSERT, DELETE ON pipette_tips TO app;
ALTER TABLE pipette_tips ENABLE ROW LEVEL SECURITY;
ALTER TABLE pipette_tips FORCE ROW LEVEL SECURITY;
CREATE POLICY pipette_tips_select ON pipette_tips FOR SELECT TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY pipette_tips_insert ON pipette_tips FOR INSERT TO app
    WITH CHECK (id = ANY (ARRAY(SELECT authz.resources_with('writer')))
        AND pipette_tip_model_id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY pipette_tips_delete ON pipette_tips FOR DELETE TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('admin'))));
CREATE POLICY pipette_tips_archived_insert ON pipette_tips AS RESTRICTIVE FOR INSERT TO app
    WITH CHECK (NOT archived AND NOT pipette_tip_model_archived);
CREATE POLICY pipette_tips_archived_delete ON pipette_tips AS RESTRICTIVE FOR DELETE TO app
    USING (NOT archived);
CREATE POLICY pipette_tips_blocked ON pipette_tips AS RESTRICTIVE FOR ALL TO app
    USING (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))))
    WITH CHECK (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))));

-- Records every change in connetto's `audit_events`.
CREATE TRIGGER pipette_tip_models_audit AFTER INSERT OR DELETE ON pipette_tip_models
    FOR EACH ROW EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER pipette_tip_models_update_audit AFTER UPDATE ON pipette_tip_models
    FOR EACH ROW WHEN (OLD IS DISTINCT FROM NEW) EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER commercial_pipette_tip_models_audit AFTER INSERT OR DELETE ON commercial_pipette_tip_models
    FOR EACH ROW EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER commercial_pipette_tip_models_update_audit AFTER UPDATE ON commercial_pipette_tip_models
    FOR EACH ROW WHEN (OLD IS DISTINCT FROM NEW) EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER commercial_pipette_tip_lots_audit AFTER INSERT OR DELETE ON commercial_pipette_tip_lots
    FOR EACH ROW EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER commercial_pipette_tip_lots_update_audit AFTER UPDATE ON commercial_pipette_tip_lots
    FOR EACH ROW WHEN (OLD IS DISTINCT FROM NEW) EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER pipette_tips_audit AFTER INSERT OR DELETE ON pipette_tips
    FOR EACH ROW EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER pipette_tips_update_audit AFTER UPDATE ON pipette_tips
    FOR EACH ROW WHEN (OLD IS DISTINCT FROM NEW) EXECUTE FUNCTION authz_write.record_audit_event();
