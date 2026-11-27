-- Migration: Shared Assets / Devices / Ball Mill Machines
-- Purpose: Ball Mill Machines for project inventories.
-- APS placement
--   `resources` of kind `'ownable'`
--       +-- asset_models
--           +-- physical_asset_models
--               +-- ball_mill_machine_models
--                   +-- commercial_ball_mill_machine_models
--               +-- commercial_product_lots
--                   +-- commercial_ball_mill_machine_lots
--       +-- assets
--           +-- physical_assets
--               +-- ball_mill_machines
-- Commercial tables decision: Vendor models and lots provide purchase traceability.

-- Catalog of ball mill machine model definitions.
CREATE TABLE ball_mill_machine_models (
    -- Stable model identifier inherited from `physical_asset_models`.
    id UUID PRIMARY KEY REFERENCES physical_asset_models (id) ON DELETE CASCADE,
    -- Organization containing this resource.
    organization_id UUID NOT NULL,
    -- Archive state inherited from this resource.
    archived BOOLEAN NOT NULL DEFAULT FALSE,
    UNIQUE (id, archived),
    FOREIGN KEY (organization_id, id, archived) REFERENCES resources (organization_id, id, archived) ON UPDATE CASCADE ON DELETE CASCADE
);
INSERT INTO table_names (id) VALUES ('ball_mill_machine_models');
INSERT INTO table_name_ancestors (descendant_table_name_id, ancestor_table_name_id)
    SELECT 'ball_mill_machine_models', ancestor_table_name_id FROM table_name_ancestors WHERE descendant_table_name_id IN ('physical_asset_models')
    UNION SELECT 'ball_mill_machine_models', 'ball_mill_machine_models';
CREATE TRIGGER ball_mill_machine_models_path AFTER INSERT ON ball_mill_machine_models
    FOR EACH ROW EXECUTE FUNCTION check_asset_model_subtype();

CREATE FUNCTION require_ball_mill_machine_models_row() RETURNS TRIGGER
LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
DECLARE
    row_id UUID := CASE TG_OP WHEN 'DELETE' THEN OLD.id ELSE NEW.id END;
BEGIN
    IF EXISTS (
        SELECT 1 FROM public.asset_models root
        JOIN public.table_name_ancestors path ON path.descendant_table_name_id = root.table_name_id
        WHERE root.id = row_id AND path.ancestor_table_name_id = 'ball_mill_machine_models'
    ) AND NOT EXISTS (SELECT 1 FROM public.ball_mill_machine_models subtype WHERE subtype.id = row_id) THEN
        RAISE EXCEPTION '% names a table extending % but has no row there', row_id, 'ball_mill_machine_models'
            USING ERRCODE = 'foreign_key_violation';
    END IF;
    RETURN NULL;
END;
$$;
ALTER FUNCTION require_ball_mill_machine_models_row() OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION require_ball_mill_machine_models_row() FROM PUBLIC;
REVOKE ALL ON ball_mill_machine_models FROM PUBLIC, app, identity_writer;
GRANT SELECT ON ball_mill_machine_models TO aps_writer;
CREATE CONSTRAINT TRIGGER ball_mill_machine_models_row AFTER INSERT ON asset_models
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW WHEN (NEW.table_name_id = 'ball_mill_machine_models')
    EXECUTE FUNCTION require_ball_mill_machine_models_row();
CREATE CONSTRAINT TRIGGER ball_mill_machine_models_kept AFTER DELETE ON ball_mill_machine_models
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION require_ball_mill_machine_models_row();

-- Catalog of commercial ball mill machine models.
CREATE TABLE commercial_ball_mill_machine_models (
    -- Stable commercial model identifier shared with parent model tables.
    id UUID PRIMARY KEY REFERENCES ball_mill_machine_models (id) ON DELETE CASCADE,
    -- Organization containing this resource.
    organization_id UUID NOT NULL,
    -- Base ball-mill model represented by this commercial model.
    ball_mill_machine_model_id UUID NOT NULL REFERENCES ball_mill_machine_models (id) ON DELETE RESTRICT,
    FOREIGN KEY (id) REFERENCES commercial_products (id) ON DELETE CASCADE,
    FOREIGN KEY (id, ball_mill_machine_model_id) REFERENCES asset_models (id, parent_model_id) ON DELETE CASCADE,
    -- Archive state inherited from this resource.
    archived BOOLEAN NOT NULL DEFAULT FALSE,
    -- Archive state of the row named by `ball_mill_machine_model_id`.
    ball_mill_machine_model_archived BOOLEAN NOT NULL DEFAULT FALSE,
    UNIQUE (id, archived),
    FOREIGN KEY (ball_mill_machine_model_id, ball_mill_machine_model_archived) REFERENCES ball_mill_machine_models (id, archived) ON UPDATE CASCADE,
    FOREIGN KEY (organization_id, id, archived) REFERENCES resources (organization_id, id, archived) ON UPDATE CASCADE ON DELETE CASCADE
);
CREATE INDEX ON commercial_ball_mill_machine_models (ball_mill_machine_model_id);
INSERT INTO table_names (id) VALUES ('commercial_ball_mill_machine_models');
INSERT INTO table_name_ancestors (descendant_table_name_id, ancestor_table_name_id)
    SELECT 'commercial_ball_mill_machine_models', ancestor_table_name_id FROM table_name_ancestors WHERE descendant_table_name_id IN ('ball_mill_machine_models', 'commercial_products')
    UNION SELECT 'commercial_ball_mill_machine_models', 'commercial_ball_mill_machine_models';
CREATE TRIGGER commercial_ball_mill_machine_models_path AFTER INSERT ON commercial_ball_mill_machine_models
    FOR EACH ROW EXECUTE FUNCTION check_asset_model_subtype();

CREATE FUNCTION require_commercial_ball_mill_machine_models_row() RETURNS TRIGGER
LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
DECLARE
    row_id UUID := CASE TG_OP WHEN 'DELETE' THEN OLD.id ELSE NEW.id END;
BEGIN
    IF EXISTS (
        SELECT 1 FROM public.asset_models root
        JOIN public.table_name_ancestors path ON path.descendant_table_name_id = root.table_name_id
        WHERE root.id = row_id AND path.ancestor_table_name_id = 'commercial_ball_mill_machine_models'
    ) AND NOT EXISTS (SELECT 1 FROM public.commercial_ball_mill_machine_models subtype WHERE subtype.id = row_id) THEN
        RAISE EXCEPTION '% names a table extending % but has no row there', row_id, 'commercial_ball_mill_machine_models'
            USING ERRCODE = 'foreign_key_violation';
    END IF;
    RETURN NULL;
END;
$$;
ALTER FUNCTION require_commercial_ball_mill_machine_models_row() OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION require_commercial_ball_mill_machine_models_row() FROM PUBLIC;
REVOKE ALL ON commercial_ball_mill_machine_models FROM PUBLIC, app, identity_writer;
GRANT SELECT ON commercial_ball_mill_machine_models TO aps_writer;
CREATE CONSTRAINT TRIGGER commercial_ball_mill_machine_models_row AFTER INSERT ON asset_models
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW WHEN (NEW.table_name_id = 'commercial_ball_mill_machine_models')
    EXECUTE FUNCTION require_commercial_ball_mill_machine_models_row();
CREATE CONSTRAINT TRIGGER commercial_ball_mill_machine_models_kept AFTER DELETE ON commercial_ball_mill_machine_models
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION require_commercial_ball_mill_machine_models_row();

-- Catalog of lot-specific commercial ball mill machine models.
CREATE TABLE commercial_ball_mill_machine_lots (
    -- Stable lot-model identifier shared with parent lot/model tables.
    id UUID PRIMARY KEY REFERENCES commercial_product_lots (id) ON DELETE CASCADE,
    -- Organization containing this resource.
    organization_id UUID NOT NULL,
    FOREIGN KEY (id) REFERENCES ball_mill_machine_models (id) ON DELETE CASCADE,
    -- Commercial ball-mill model from which this lot derives.
    commercial_ball_mill_machine_model_id UUID NOT NULL REFERENCES commercial_ball_mill_machine_models (id),
    FOREIGN KEY (id, commercial_ball_mill_machine_model_id) REFERENCES asset_models (id, parent_model_id) ON DELETE CASCADE,
    -- Archive state inherited from this resource.
    archived BOOLEAN NOT NULL DEFAULT FALSE,
    -- Archive state of the row named by `commercial_ball_mill_machine_model_id`.
    commercial_ball_mill_machine_model_archived BOOLEAN NOT NULL DEFAULT FALSE,
    UNIQUE (id, archived),
    FOREIGN KEY (commercial_ball_mill_machine_model_id, commercial_ball_mill_machine_model_archived) REFERENCES commercial_ball_mill_machine_models (id, archived) ON UPDATE CASCADE,
    FOREIGN KEY (organization_id, id, archived) REFERENCES resources (organization_id, id, archived) ON UPDATE CASCADE ON DELETE CASCADE
);
CREATE INDEX ON commercial_ball_mill_machine_lots (commercial_ball_mill_machine_model_id);
INSERT INTO table_names (id) VALUES ('commercial_ball_mill_machine_lots');
INSERT INTO table_name_ancestors (descendant_table_name_id, ancestor_table_name_id)
    SELECT 'commercial_ball_mill_machine_lots', ancestor_table_name_id FROM table_name_ancestors WHERE descendant_table_name_id IN ('commercial_product_lots', 'ball_mill_machine_models')
    UNION SELECT 'commercial_ball_mill_machine_lots', 'commercial_ball_mill_machine_lots';
INSERT INTO table_name_admitted_parents (table_name_id, parent_table_name_id)
    VALUES ('commercial_ball_mill_machine_lots', 'commercial_ball_mill_machine_models');
CREATE TRIGGER commercial_ball_mill_machine_lots_path AFTER INSERT ON commercial_ball_mill_machine_lots
    FOR EACH ROW EXECUTE FUNCTION check_asset_model_subtype();

CREATE FUNCTION require_commercial_ball_mill_machine_lots_row() RETURNS TRIGGER
LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
DECLARE
    row_id UUID := CASE TG_OP WHEN 'DELETE' THEN OLD.id ELSE NEW.id END;
BEGIN
    IF EXISTS (
        SELECT 1 FROM public.asset_models root
        JOIN public.table_name_ancestors path ON path.descendant_table_name_id = root.table_name_id
        WHERE root.id = row_id AND path.ancestor_table_name_id = 'commercial_ball_mill_machine_lots'
    ) AND NOT EXISTS (SELECT 1 FROM public.commercial_ball_mill_machine_lots subtype WHERE subtype.id = row_id) THEN
        RAISE EXCEPTION '% names a table extending % but has no row there', row_id, 'commercial_ball_mill_machine_lots'
            USING ERRCODE = 'foreign_key_violation';
    END IF;
    RETURN NULL;
END;
$$;
ALTER FUNCTION require_commercial_ball_mill_machine_lots_row() OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION require_commercial_ball_mill_machine_lots_row() FROM PUBLIC;
REVOKE ALL ON commercial_ball_mill_machine_lots FROM PUBLIC, app, identity_writer;
GRANT SELECT ON commercial_ball_mill_machine_lots TO aps_writer;
CREATE CONSTRAINT TRIGGER commercial_ball_mill_machine_lots_row AFTER INSERT ON asset_models
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW WHEN (NEW.table_name_id = 'commercial_ball_mill_machine_lots')
    EXECUTE FUNCTION require_commercial_ball_mill_machine_lots_row();
CREATE CONSTRAINT TRIGGER commercial_ball_mill_machine_lots_kept AFTER DELETE ON commercial_ball_mill_machine_lots
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION require_commercial_ball_mill_machine_lots_row();

-- Physical ball mill machines tracked in APS inventory.
CREATE TABLE ball_mill_machines (
    -- Stable asset identifier inherited from `physical_assets`.
    id UUID PRIMARY KEY REFERENCES physical_assets (id) ON DELETE CASCADE,
    -- Organization containing this resource.
    organization_id UUID NOT NULL,
    -- Commercial lot model instantiated by this physical machine.
    commercial_ball_mill_machine_lot_id UUID NOT NULL REFERENCES commercial_ball_mill_machine_lots (id),
    FOREIGN KEY (id, commercial_ball_mill_machine_lot_id) REFERENCES assets (id, model_id) ON DELETE CASCADE,
    -- Archive state inherited from this resource.
    archived BOOLEAN NOT NULL DEFAULT FALSE,
    -- Archive state of the row named by `commercial_ball_mill_machine_lot_id`.
    commercial_ball_mill_machine_lot_archived BOOLEAN NOT NULL DEFAULT FALSE,
    FOREIGN KEY (commercial_ball_mill_machine_lot_id, commercial_ball_mill_machine_lot_archived) REFERENCES commercial_ball_mill_machine_lots (id, archived) ON UPDATE CASCADE,
    FOREIGN KEY (organization_id, id, archived) REFERENCES resources (organization_id, id, archived) ON UPDATE CASCADE ON DELETE CASCADE
);
CREATE INDEX ON ball_mill_machines (commercial_ball_mill_machine_lot_id);
INSERT INTO table_names (id) VALUES ('ball_mill_machines');
INSERT INTO table_name_instances (id, instance_table_name_id)
    VALUES ('commercial_ball_mill_machine_lots', 'ball_mill_machines');
INSERT INTO table_name_ancestors (descendant_table_name_id, ancestor_table_name_id)
    SELECT 'ball_mill_machines', ancestor_table_name_id FROM table_name_ancestors WHERE descendant_table_name_id IN ('physical_assets')
    UNION SELECT 'ball_mill_machines', 'ball_mill_machines';
CREATE TRIGGER ball_mill_machines_path AFTER INSERT ON ball_mill_machines
    FOR EACH ROW EXECUTE FUNCTION check_asset_subtype();

CREATE FUNCTION require_ball_mill_machines_row() RETURNS TRIGGER
LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
DECLARE
    row_id UUID := CASE TG_OP WHEN 'DELETE' THEN OLD.id ELSE NEW.id END;
BEGIN
    IF EXISTS (
        SELECT 1 FROM public.assets root
        JOIN public.table_name_ancestors path ON path.descendant_table_name_id = root.table_name_id
        WHERE root.id = row_id AND path.ancestor_table_name_id = 'ball_mill_machines'
    ) AND NOT EXISTS (SELECT 1 FROM public.ball_mill_machines subtype WHERE subtype.id = row_id) THEN
        RAISE EXCEPTION '% names a table extending % but has no row there', row_id, 'ball_mill_machines'
            USING ERRCODE = 'foreign_key_violation';
    END IF;
    RETURN NULL;
END;
$$;
ALTER FUNCTION require_ball_mill_machines_row() OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION require_ball_mill_machines_row() FROM PUBLIC;
REVOKE ALL ON ball_mill_machines FROM PUBLIC, app, identity_writer;
GRANT SELECT ON ball_mill_machines TO aps_writer;
CREATE CONSTRAINT TRIGGER ball_mill_machines_row AFTER INSERT ON assets
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW WHEN (NEW.table_name_id = 'ball_mill_machines')
    EXECUTE FUNCTION require_ball_mill_machines_row();
CREATE CONSTRAINT TRIGGER ball_mill_machines_kept AFTER DELETE ON ball_mill_machines
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION require_ball_mill_machines_row();

GRANT SELECT, INSERT, DELETE ON ball_mill_machine_models TO app;
ALTER TABLE ball_mill_machine_models ENABLE ROW LEVEL SECURITY;
ALTER TABLE ball_mill_machine_models FORCE ROW LEVEL SECURITY;
CREATE POLICY ball_mill_machine_models_select ON ball_mill_machine_models FOR SELECT TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY ball_mill_machine_models_insert ON ball_mill_machine_models FOR INSERT TO app
    WITH CHECK (id = ANY (ARRAY(SELECT authz.resources_with('writer'))));
CREATE POLICY ball_mill_machine_models_delete ON ball_mill_machine_models FOR DELETE TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('admin'))));
CREATE POLICY ball_mill_machine_models_archived_insert ON ball_mill_machine_models AS RESTRICTIVE FOR INSERT TO app
    WITH CHECK (NOT archived);
CREATE POLICY ball_mill_machine_models_archived_delete ON ball_mill_machine_models AS RESTRICTIVE FOR DELETE TO app
    USING (NOT archived);
CREATE POLICY ball_mill_machine_models_blocked ON ball_mill_machine_models AS RESTRICTIVE FOR ALL TO app
    USING (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))))
    WITH CHECK (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))));

GRANT SELECT, INSERT, DELETE ON commercial_ball_mill_machine_models TO app;
ALTER TABLE commercial_ball_mill_machine_models ENABLE ROW LEVEL SECURITY;
ALTER TABLE commercial_ball_mill_machine_models FORCE ROW LEVEL SECURITY;
CREATE POLICY commercial_ball_mill_machine_models_select ON commercial_ball_mill_machine_models FOR SELECT TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY commercial_ball_mill_machine_models_insert ON commercial_ball_mill_machine_models FOR INSERT TO app
    WITH CHECK (id = ANY (ARRAY(SELECT authz.resources_with('writer')))
        AND ball_mill_machine_model_id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY commercial_ball_mill_machine_models_delete ON commercial_ball_mill_machine_models FOR DELETE TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('admin'))));
CREATE POLICY commercial_ball_mill_machine_models_archived_insert ON commercial_ball_mill_machine_models AS RESTRICTIVE FOR INSERT TO app
    WITH CHECK (NOT archived AND NOT ball_mill_machine_model_archived);
CREATE POLICY commercial_ball_mill_machine_models_archived_delete ON commercial_ball_mill_machine_models AS RESTRICTIVE FOR DELETE TO app
    USING (NOT archived);
CREATE POLICY commercial_ball_mill_machine_models_blocked ON commercial_ball_mill_machine_models AS RESTRICTIVE FOR ALL TO app
    USING (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))))
    WITH CHECK (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))));

GRANT SELECT, INSERT, DELETE ON commercial_ball_mill_machine_lots TO app;
ALTER TABLE commercial_ball_mill_machine_lots ENABLE ROW LEVEL SECURITY;
ALTER TABLE commercial_ball_mill_machine_lots FORCE ROW LEVEL SECURITY;
CREATE POLICY commercial_ball_mill_machine_lots_select ON commercial_ball_mill_machine_lots FOR SELECT TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY commercial_ball_mill_machine_lots_insert ON commercial_ball_mill_machine_lots FOR INSERT TO app
    WITH CHECK (id = ANY (ARRAY(SELECT authz.resources_with('writer')))
        AND commercial_ball_mill_machine_model_id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY commercial_ball_mill_machine_lots_delete ON commercial_ball_mill_machine_lots FOR DELETE TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('admin'))));
CREATE POLICY commercial_ball_mill_machine_lots_archived_insert ON commercial_ball_mill_machine_lots AS RESTRICTIVE FOR INSERT TO app
    WITH CHECK (NOT archived AND NOT commercial_ball_mill_machine_model_archived);
CREATE POLICY commercial_ball_mill_machine_lots_archived_delete ON commercial_ball_mill_machine_lots AS RESTRICTIVE FOR DELETE TO app
    USING (NOT archived);
CREATE POLICY commercial_ball_mill_machine_lots_blocked ON commercial_ball_mill_machine_lots AS RESTRICTIVE FOR ALL TO app
    USING (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))))
    WITH CHECK (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))));

GRANT SELECT, INSERT, DELETE ON ball_mill_machines TO app;
ALTER TABLE ball_mill_machines ENABLE ROW LEVEL SECURITY;
ALTER TABLE ball_mill_machines FORCE ROW LEVEL SECURITY;
CREATE POLICY ball_mill_machines_select ON ball_mill_machines FOR SELECT TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY ball_mill_machines_insert ON ball_mill_machines FOR INSERT TO app
    WITH CHECK (id = ANY (ARRAY(SELECT authz.resources_with('writer')))
        AND commercial_ball_mill_machine_lot_id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY ball_mill_machines_delete ON ball_mill_machines FOR DELETE TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('admin'))));
CREATE POLICY ball_mill_machines_archived_insert ON ball_mill_machines AS RESTRICTIVE FOR INSERT TO app
    WITH CHECK (NOT archived AND NOT commercial_ball_mill_machine_lot_archived);
CREATE POLICY ball_mill_machines_archived_delete ON ball_mill_machines AS RESTRICTIVE FOR DELETE TO app
    USING (NOT archived);
CREATE POLICY ball_mill_machines_blocked ON ball_mill_machines AS RESTRICTIVE FOR ALL TO app
    USING (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))))
    WITH CHECK (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))));

-- Records every change in connetto's `audit_events`.
CREATE TRIGGER ball_mill_machine_models_audit AFTER INSERT OR DELETE ON ball_mill_machine_models
    FOR EACH ROW EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER ball_mill_machine_models_update_audit AFTER UPDATE ON ball_mill_machine_models
    FOR EACH ROW WHEN (OLD IS DISTINCT FROM NEW) EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER commercial_ball_mill_machine_models_audit AFTER INSERT OR DELETE ON commercial_ball_mill_machine_models
    FOR EACH ROW EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER commercial_ball_mill_machine_models_update_audit AFTER UPDATE ON commercial_ball_mill_machine_models
    FOR EACH ROW WHEN (OLD IS DISTINCT FROM NEW) EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER commercial_ball_mill_machine_lots_audit AFTER INSERT OR DELETE ON commercial_ball_mill_machine_lots
    FOR EACH ROW EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER commercial_ball_mill_machine_lots_update_audit AFTER UPDATE ON commercial_ball_mill_machine_lots
    FOR EACH ROW WHEN (OLD IS DISTINCT FROM NEW) EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER ball_mill_machines_audit AFTER INSERT OR DELETE ON ball_mill_machines
    FOR EACH ROW EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER ball_mill_machines_update_audit AFTER UPDATE ON ball_mill_machines
    FOR EACH ROW WHEN (OLD IS DISTINCT FROM NEW) EXECUTE FUNCTION authz_write.record_audit_event();
