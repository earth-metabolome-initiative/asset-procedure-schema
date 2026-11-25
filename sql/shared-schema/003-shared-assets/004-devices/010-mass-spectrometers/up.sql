-- Migration: Shared Assets / Mass Spectrometers
-- Purpose: Mass Spectrometers for project inventories.
-- APS placement
--   ownables
--       +-- asset_models
--           +-- physical_asset_models
--               +-- mass_spectrometer_models
--                   +-- commercial_mass_spectrometer_models
--               +-- commercial_product_lots
--                   +-- commercial_mass_spectrometer_lots
--       +-- assets
--           +-- physical_assets
--               +-- mass_spectrometers
-- Commercial tables decision: Vendor models and lots provide purchase traceability.

-- Catalog of mass spectrometer model definitions.
CREATE TABLE mass_spectrometer_models (
    -- Stable model identifier inherited from physical_asset_models.
    id UUID PRIMARY KEY REFERENCES physical_asset_models (id) ON DELETE CASCADE,
    -- Organization containing this resource.
    organization_id UUID NOT NULL,
    FOREIGN KEY (organization_id, id) REFERENCES resources (organization_id, id) ON DELETE CASCADE
);
INSERT INTO table_names (id) VALUES ('mass_spectrometer_models');
INSERT INTO table_name_ancestors (descendant_table_name_id, ancestor_table_name_id)
    SELECT 'mass_spectrometer_models', ancestor_table_name_id FROM table_name_ancestors WHERE descendant_table_name_id IN ('physical_asset_models')
    UNION SELECT 'mass_spectrometer_models', 'mass_spectrometer_models';
CREATE TRIGGER mass_spectrometer_models_path AFTER INSERT ON mass_spectrometer_models
    FOR EACH ROW EXECUTE FUNCTION check_asset_model_subtype();

CREATE FUNCTION require_mass_spectrometer_models_row() RETURNS TRIGGER
LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
DECLARE
    row_id UUID := CASE TG_OP WHEN 'DELETE' THEN OLD.id ELSE NEW.id END;
BEGIN
    IF EXISTS (
        SELECT 1 FROM public.asset_models root
        JOIN public.table_name_ancestors path ON path.descendant_table_name_id = root.table_name_id
        WHERE root.id = row_id AND path.ancestor_table_name_id = 'mass_spectrometer_models'
    ) AND NOT EXISTS (SELECT 1 FROM public.mass_spectrometer_models subtype WHERE subtype.id = row_id) THEN
        RAISE EXCEPTION '% names a table extending % but has no row there', row_id, 'mass_spectrometer_models'
            USING ERRCODE = 'foreign_key_violation';
    END IF;
    RETURN NULL;
END;
$$;
ALTER FUNCTION require_mass_spectrometer_models_row() OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION require_mass_spectrometer_models_row() FROM PUBLIC;
REVOKE ALL ON mass_spectrometer_models FROM PUBLIC, app, identity_writer;
GRANT SELECT ON mass_spectrometer_models TO aps_writer;
CREATE CONSTRAINT TRIGGER mass_spectrometer_models_row AFTER INSERT ON asset_models
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW WHEN (NEW.table_name_id = 'mass_spectrometer_models')
    EXECUTE FUNCTION require_mass_spectrometer_models_row();
CREATE CONSTRAINT TRIGGER mass_spectrometer_models_kept AFTER DELETE ON mass_spectrometer_models
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION require_mass_spectrometer_models_row();

-- Catalog of commercial mass spectrometer models sold by brands.
CREATE TABLE commercial_mass_spectrometer_models (
    -- Stable commercial model identifier shared with parent model tables.
    id UUID PRIMARY KEY REFERENCES mass_spectrometer_models (id) ON DELETE CASCADE,
    -- Organization containing this resource.
    organization_id UUID NOT NULL,
    -- Base mass spectrometer model represented by this commercial model.
    mass_spectrometer_model_id UUID NOT NULL REFERENCES mass_spectrometer_models (id) ON DELETE RESTRICT,
    FOREIGN KEY (id) REFERENCES commercial_products (id) ON DELETE CASCADE,
    FOREIGN KEY (id, mass_spectrometer_model_id) REFERENCES asset_models (id, parent_model_id) ON DELETE CASCADE,
    FOREIGN KEY (organization_id, id) REFERENCES resources (organization_id, id) ON DELETE CASCADE
);
CREATE INDEX ON commercial_mass_spectrometer_models (mass_spectrometer_model_id);
INSERT INTO table_names (id) VALUES ('commercial_mass_spectrometer_models');
INSERT INTO table_name_ancestors (descendant_table_name_id, ancestor_table_name_id)
    SELECT 'commercial_mass_spectrometer_models', ancestor_table_name_id FROM table_name_ancestors WHERE descendant_table_name_id IN ('mass_spectrometer_models', 'commercial_products')
    UNION SELECT 'commercial_mass_spectrometer_models', 'commercial_mass_spectrometer_models';
CREATE TRIGGER commercial_mass_spectrometer_models_path AFTER INSERT ON commercial_mass_spectrometer_models
    FOR EACH ROW EXECUTE FUNCTION check_asset_model_subtype();

CREATE FUNCTION require_commercial_mass_spectrometer_models_row() RETURNS TRIGGER
LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
DECLARE
    row_id UUID := CASE TG_OP WHEN 'DELETE' THEN OLD.id ELSE NEW.id END;
BEGIN
    IF EXISTS (
        SELECT 1 FROM public.asset_models root
        JOIN public.table_name_ancestors path ON path.descendant_table_name_id = root.table_name_id
        WHERE root.id = row_id AND path.ancestor_table_name_id = 'commercial_mass_spectrometer_models'
    ) AND NOT EXISTS (SELECT 1 FROM public.commercial_mass_spectrometer_models subtype WHERE subtype.id = row_id) THEN
        RAISE EXCEPTION '% names a table extending % but has no row there', row_id, 'commercial_mass_spectrometer_models'
            USING ERRCODE = 'foreign_key_violation';
    END IF;
    RETURN NULL;
END;
$$;
ALTER FUNCTION require_commercial_mass_spectrometer_models_row() OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION require_commercial_mass_spectrometer_models_row() FROM PUBLIC;
REVOKE ALL ON commercial_mass_spectrometer_models FROM PUBLIC, app, identity_writer;
GRANT SELECT ON commercial_mass_spectrometer_models TO aps_writer;
CREATE CONSTRAINT TRIGGER commercial_mass_spectrometer_models_row AFTER INSERT ON asset_models
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW WHEN (NEW.table_name_id = 'commercial_mass_spectrometer_models')
    EXECUTE FUNCTION require_commercial_mass_spectrometer_models_row();
CREATE CONSTRAINT TRIGGER commercial_mass_spectrometer_models_kept AFTER DELETE ON commercial_mass_spectrometer_models
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION require_commercial_mass_spectrometer_models_row();

-- Catalog of lot-specific commercial mass spectrometer models.
CREATE TABLE commercial_mass_spectrometer_lots (
    -- Stable lot-model identifier shared with parent lot/model tables.
    id UUID PRIMARY KEY REFERENCES commercial_product_lots (id) ON DELETE CASCADE,
    -- Organization containing this resource.
    organization_id UUID NOT NULL,
    FOREIGN KEY (id) REFERENCES mass_spectrometer_models (id) ON DELETE CASCADE,
    -- Commercial model from which this lot-level model descends.
    commercial_mass_spectrometer_model_id UUID NOT NULL REFERENCES commercial_mass_spectrometer_models (id),
    FOREIGN KEY (id, commercial_mass_spectrometer_model_id) REFERENCES asset_models (id, parent_model_id) ON DELETE CASCADE,
    FOREIGN KEY (organization_id, id) REFERENCES resources (organization_id, id) ON DELETE CASCADE
);
CREATE INDEX ON commercial_mass_spectrometer_lots (commercial_mass_spectrometer_model_id);
INSERT INTO table_names (id) VALUES ('commercial_mass_spectrometer_lots');
INSERT INTO table_name_ancestors (descendant_table_name_id, ancestor_table_name_id)
    SELECT 'commercial_mass_spectrometer_lots', ancestor_table_name_id FROM table_name_ancestors WHERE descendant_table_name_id IN ('commercial_product_lots', 'mass_spectrometer_models')
    UNION SELECT 'commercial_mass_spectrometer_lots', 'commercial_mass_spectrometer_lots';
INSERT INTO table_name_admitted_parents (table_name_id, parent_table_name_id)
    VALUES ('commercial_mass_spectrometer_lots', 'commercial_mass_spectrometer_models');
CREATE TRIGGER commercial_mass_spectrometer_lots_path AFTER INSERT ON commercial_mass_spectrometer_lots
    FOR EACH ROW EXECUTE FUNCTION check_asset_model_subtype();

CREATE FUNCTION require_commercial_mass_spectrometer_lots_row() RETURNS TRIGGER
LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
DECLARE
    row_id UUID := CASE TG_OP WHEN 'DELETE' THEN OLD.id ELSE NEW.id END;
BEGIN
    IF EXISTS (
        SELECT 1 FROM public.asset_models root
        JOIN public.table_name_ancestors path ON path.descendant_table_name_id = root.table_name_id
        WHERE root.id = row_id AND path.ancestor_table_name_id = 'commercial_mass_spectrometer_lots'
    ) AND NOT EXISTS (SELECT 1 FROM public.commercial_mass_spectrometer_lots subtype WHERE subtype.id = row_id) THEN
        RAISE EXCEPTION '% names a table extending % but has no row there', row_id, 'commercial_mass_spectrometer_lots'
            USING ERRCODE = 'foreign_key_violation';
    END IF;
    RETURN NULL;
END;
$$;
ALTER FUNCTION require_commercial_mass_spectrometer_lots_row() OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION require_commercial_mass_spectrometer_lots_row() FROM PUBLIC;
REVOKE ALL ON commercial_mass_spectrometer_lots FROM PUBLIC, app, identity_writer;
GRANT SELECT ON commercial_mass_spectrometer_lots TO aps_writer;
CREATE CONSTRAINT TRIGGER commercial_mass_spectrometer_lots_row AFTER INSERT ON asset_models
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW WHEN (NEW.table_name_id = 'commercial_mass_spectrometer_lots')
    EXECUTE FUNCTION require_commercial_mass_spectrometer_lots_row();
CREATE CONSTRAINT TRIGGER commercial_mass_spectrometer_lots_kept AFTER DELETE ON commercial_mass_spectrometer_lots
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION require_commercial_mass_spectrometer_lots_row();

-- Physical mass spectrometers tracked in APS inventory and procedures.
CREATE TABLE mass_spectrometers (
    -- Stable asset identifier inherited from physical_assets.
    id UUID PRIMARY KEY REFERENCES physical_assets (id) ON DELETE CASCADE,
    -- Organization containing this resource.
    organization_id UUID NOT NULL,
    -- Commercial lot model instantiated by this physical instrument.
    commercial_mass_spectrometer_lot_id UUID NOT NULL REFERENCES commercial_mass_spectrometer_lots (id),
    FOREIGN KEY (id, commercial_mass_spectrometer_lot_id) REFERENCES assets (id, model_id) ON DELETE CASCADE,
    FOREIGN KEY (organization_id, id) REFERENCES resources (organization_id, id) ON DELETE CASCADE
);
CREATE INDEX ON mass_spectrometers (commercial_mass_spectrometer_lot_id);
INSERT INTO table_names (id) VALUES ('mass_spectrometers');
INSERT INTO table_name_instances (id, instance_table_name_id)
    VALUES ('commercial_mass_spectrometer_lots', 'mass_spectrometers');
INSERT INTO table_name_ancestors (descendant_table_name_id, ancestor_table_name_id)
    SELECT 'mass_spectrometers', ancestor_table_name_id FROM table_name_ancestors WHERE descendant_table_name_id IN ('physical_assets')
    UNION SELECT 'mass_spectrometers', 'mass_spectrometers';
CREATE TRIGGER mass_spectrometers_path AFTER INSERT ON mass_spectrometers
    FOR EACH ROW EXECUTE FUNCTION check_asset_subtype();

CREATE FUNCTION require_mass_spectrometers_row() RETURNS TRIGGER
LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
DECLARE
    row_id UUID := CASE TG_OP WHEN 'DELETE' THEN OLD.id ELSE NEW.id END;
BEGIN
    IF EXISTS (
        SELECT 1 FROM public.assets root
        JOIN public.table_name_ancestors path ON path.descendant_table_name_id = root.table_name_id
        WHERE root.id = row_id AND path.ancestor_table_name_id = 'mass_spectrometers'
    ) AND NOT EXISTS (SELECT 1 FROM public.mass_spectrometers subtype WHERE subtype.id = row_id) THEN
        RAISE EXCEPTION '% names a table extending % but has no row there', row_id, 'mass_spectrometers'
            USING ERRCODE = 'foreign_key_violation';
    END IF;
    RETURN NULL;
END;
$$;
ALTER FUNCTION require_mass_spectrometers_row() OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION require_mass_spectrometers_row() FROM PUBLIC;
REVOKE ALL ON mass_spectrometers FROM PUBLIC, app, identity_writer;
GRANT SELECT ON mass_spectrometers TO aps_writer;
CREATE CONSTRAINT TRIGGER mass_spectrometers_row AFTER INSERT ON assets
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW WHEN (NEW.table_name_id = 'mass_spectrometers')
    EXECUTE FUNCTION require_mass_spectrometers_row();
CREATE CONSTRAINT TRIGGER mass_spectrometers_kept AFTER DELETE ON mass_spectrometers
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION require_mass_spectrometers_row();

GRANT SELECT, INSERT, DELETE ON mass_spectrometer_models TO app;
ALTER TABLE mass_spectrometer_models ENABLE ROW LEVEL SECURITY;
ALTER TABLE mass_spectrometer_models FORCE ROW LEVEL SECURITY;
CREATE POLICY mass_spectrometer_models_select ON mass_spectrometer_models FOR SELECT TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY mass_spectrometer_models_insert ON mass_spectrometer_models FOR INSERT TO app
    WITH CHECK (id = ANY (ARRAY(SELECT authz.resources_with('writer'))));
CREATE POLICY mass_spectrometer_models_delete ON mass_spectrometer_models FOR DELETE TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('admin'))));
CREATE POLICY mass_spectrometer_models_archived_insert ON mass_spectrometer_models AS RESTRICTIVE FOR INSERT TO app
    WITH CHECK (NOT authz.is_archived(id));
CREATE POLICY mass_spectrometer_models_archived_delete ON mass_spectrometer_models AS RESTRICTIVE FOR DELETE TO app
    USING (NOT authz.is_archived(id));
CREATE POLICY mass_spectrometer_models_blocked ON mass_spectrometer_models AS RESTRICTIVE FOR ALL TO app
    USING (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))))
    WITH CHECK (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))));

GRANT SELECT, INSERT, DELETE ON commercial_mass_spectrometer_models TO app;
ALTER TABLE commercial_mass_spectrometer_models ENABLE ROW LEVEL SECURITY;
ALTER TABLE commercial_mass_spectrometer_models FORCE ROW LEVEL SECURITY;
CREATE POLICY commercial_mass_spectrometer_models_select ON commercial_mass_spectrometer_models FOR SELECT TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY commercial_mass_spectrometer_models_insert ON commercial_mass_spectrometer_models FOR INSERT TO app
    WITH CHECK (id = ANY (ARRAY(SELECT authz.resources_with('writer')))
        AND mass_spectrometer_model_id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY commercial_mass_spectrometer_models_delete ON commercial_mass_spectrometer_models FOR DELETE TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('admin'))));
CREATE POLICY commercial_mass_spectrometer_models_archived_insert ON commercial_mass_spectrometer_models AS RESTRICTIVE FOR INSERT TO app
    WITH CHECK (NOT authz.is_archived(id) AND NOT authz.is_archived(mass_spectrometer_model_id));
CREATE POLICY commercial_mass_spectrometer_models_archived_delete ON commercial_mass_spectrometer_models AS RESTRICTIVE FOR DELETE TO app
    USING (NOT authz.is_archived(id));
CREATE POLICY commercial_mass_spectrometer_models_blocked ON commercial_mass_spectrometer_models AS RESTRICTIVE FOR ALL TO app
    USING (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))))
    WITH CHECK (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))));

GRANT SELECT, INSERT, DELETE ON commercial_mass_spectrometer_lots TO app;
ALTER TABLE commercial_mass_spectrometer_lots ENABLE ROW LEVEL SECURITY;
ALTER TABLE commercial_mass_spectrometer_lots FORCE ROW LEVEL SECURITY;
CREATE POLICY commercial_mass_spectrometer_lots_select ON commercial_mass_spectrometer_lots FOR SELECT TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY commercial_mass_spectrometer_lots_insert ON commercial_mass_spectrometer_lots FOR INSERT TO app
    WITH CHECK (id = ANY (ARRAY(SELECT authz.resources_with('writer')))
        AND commercial_mass_spectrometer_model_id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY commercial_mass_spectrometer_lots_delete ON commercial_mass_spectrometer_lots FOR DELETE TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('admin'))));
CREATE POLICY commercial_mass_spectrometer_lots_archived_insert ON commercial_mass_spectrometer_lots AS RESTRICTIVE FOR INSERT TO app
    WITH CHECK (NOT authz.is_archived(id) AND NOT authz.is_archived(commercial_mass_spectrometer_model_id));
CREATE POLICY commercial_mass_spectrometer_lots_archived_delete ON commercial_mass_spectrometer_lots AS RESTRICTIVE FOR DELETE TO app
    USING (NOT authz.is_archived(id));
CREATE POLICY commercial_mass_spectrometer_lots_blocked ON commercial_mass_spectrometer_lots AS RESTRICTIVE FOR ALL TO app
    USING (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))))
    WITH CHECK (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))));

GRANT SELECT, INSERT, DELETE ON mass_spectrometers TO app;
ALTER TABLE mass_spectrometers ENABLE ROW LEVEL SECURITY;
ALTER TABLE mass_spectrometers FORCE ROW LEVEL SECURITY;
CREATE POLICY mass_spectrometers_select ON mass_spectrometers FOR SELECT TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY mass_spectrometers_insert ON mass_spectrometers FOR INSERT TO app
    WITH CHECK (id = ANY (ARRAY(SELECT authz.resources_with('writer')))
        AND commercial_mass_spectrometer_lot_id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY mass_spectrometers_delete ON mass_spectrometers FOR DELETE TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('admin'))));
CREATE POLICY mass_spectrometers_archived_insert ON mass_spectrometers AS RESTRICTIVE FOR INSERT TO app
    WITH CHECK (NOT authz.is_archived(id) AND NOT authz.is_archived(commercial_mass_spectrometer_lot_id));
CREATE POLICY mass_spectrometers_archived_delete ON mass_spectrometers AS RESTRICTIVE FOR DELETE TO app
    USING (NOT authz.is_archived(id));
CREATE POLICY mass_spectrometers_blocked ON mass_spectrometers AS RESTRICTIVE FOR ALL TO app
    USING (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))))
    WITH CHECK (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))));

-- Records every change in connetto's `audit_events`.
CREATE TRIGGER mass_spectrometer_models_audit AFTER INSERT OR DELETE ON mass_spectrometer_models
    FOR EACH ROW EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER mass_spectrometer_models_update_audit AFTER UPDATE ON mass_spectrometer_models
    FOR EACH ROW WHEN (OLD IS DISTINCT FROM NEW) EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER commercial_mass_spectrometer_models_audit AFTER INSERT OR DELETE ON commercial_mass_spectrometer_models
    FOR EACH ROW EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER commercial_mass_spectrometer_models_update_audit AFTER UPDATE ON commercial_mass_spectrometer_models
    FOR EACH ROW WHEN (OLD IS DISTINCT FROM NEW) EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER commercial_mass_spectrometer_lots_audit AFTER INSERT OR DELETE ON commercial_mass_spectrometer_lots
    FOR EACH ROW EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER commercial_mass_spectrometer_lots_update_audit AFTER UPDATE ON commercial_mass_spectrometer_lots
    FOR EACH ROW WHEN (OLD IS DISTINCT FROM NEW) EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER mass_spectrometers_audit AFTER INSERT OR DELETE ON mass_spectrometers
    FOR EACH ROW EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER mass_spectrometers_update_audit AFTER UPDATE ON mass_spectrometers
    FOR EACH ROW WHEN (OLD IS DISTINCT FROM NEW) EXECUTE FUNCTION authz_write.record_audit_event();
