-- Migration: Shared Assets / Supplies / Personal Protective Equipment
-- Purpose: Personal Protective Equipment for project inventories.
-- APS placement
--   ownables
--       +-- asset_models
--           +-- physical_asset_models
--               +-- personal_protective_equipment_models
--                   +-- commercial_personal_protective_equipment_models
--               +-- commercial_product_lots
--                   +-- commercial_personal_protective_equipment_lots
--       +-- assets
--           +-- physical_assets
--               +-- personal_protective_equipments
-- Commercial tables decision: Vendor models and lots provide purchase traceability.

-- Catalog of personal protective equipment model definitions.
CREATE TABLE personal_protective_equipment_models (
    -- Stable model identifier inherited from physical_asset_models.
    id UUID PRIMARY KEY REFERENCES physical_asset_models (id) ON DELETE CASCADE,
    -- Organization containing this resource.
    organization_id UUID NOT NULL,
    FOREIGN KEY (organization_id, id) REFERENCES resources (organization_id, id) ON DELETE CASCADE
);
INSERT INTO table_names (id) VALUES ('personal_protective_equipment_models');
INSERT INTO table_name_ancestors (descendant_table_name_id, ancestor_table_name_id)
    SELECT 'personal_protective_equipment_models', ancestor_table_name_id FROM table_name_ancestors WHERE descendant_table_name_id IN ('physical_asset_models')
    UNION SELECT 'personal_protective_equipment_models', 'personal_protective_equipment_models';
CREATE TRIGGER personal_protective_equipment_models_path AFTER INSERT ON personal_protective_equipment_models
    FOR EACH ROW EXECUTE FUNCTION check_asset_model_subtype();

CREATE FUNCTION require_personal_protective_equipment_models_row() RETURNS TRIGGER
LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
DECLARE
    row_id UUID := CASE TG_OP WHEN 'DELETE' THEN OLD.id ELSE NEW.id END;
BEGIN
    IF EXISTS (
        SELECT 1 FROM public.asset_models root
        JOIN public.table_name_ancestors path ON path.descendant_table_name_id = root.table_name_id
        WHERE root.id = row_id AND path.ancestor_table_name_id = 'personal_protective_equipment_models'
    ) AND NOT EXISTS (SELECT 1 FROM public.personal_protective_equipment_models subtype WHERE subtype.id = row_id) THEN
        RAISE EXCEPTION '% names a table extending % but has no row there', row_id, 'personal_protective_equipment_models'
            USING ERRCODE = 'foreign_key_violation';
    END IF;
    RETURN NULL;
END;
$$;
ALTER FUNCTION require_personal_protective_equipment_models_row() OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION require_personal_protective_equipment_models_row() FROM PUBLIC;
GRANT SELECT ON personal_protective_equipment_models TO aps_writer;
CREATE CONSTRAINT TRIGGER personal_protective_equipment_models_row AFTER INSERT ON asset_models
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW WHEN (NEW.table_name_id = 'personal_protective_equipment_models')
    EXECUTE FUNCTION require_personal_protective_equipment_models_row();
CREATE CONSTRAINT TRIGGER personal_protective_equipment_models_kept AFTER DELETE ON personal_protective_equipment_models
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION require_personal_protective_equipment_models_row();

-- Catalog of commercial personal protective equipment models.
CREATE TABLE commercial_personal_protective_equipment_models (
    -- Stable commercial model identifier shared with parent model tables.
    id UUID PRIMARY KEY REFERENCES personal_protective_equipment_models (id) ON DELETE CASCADE,
    -- Organization containing this resource.
    organization_id UUID NOT NULL,
    -- Base PPE model represented by this commercial model.
    personal_protective_equipment_model_id UUID NOT NULL REFERENCES personal_protective_equipment_models (id) ON DELETE RESTRICT,
    FOREIGN KEY (id) REFERENCES commercial_products (id) ON DELETE CASCADE,
    FOREIGN KEY (id, personal_protective_equipment_model_id) REFERENCES asset_models (id, parent_model_id) ON DELETE CASCADE,
    FOREIGN KEY (organization_id, id) REFERENCES resources (organization_id, id) ON DELETE CASCADE
);
CREATE INDEX ON commercial_personal_protective_equipment_models (personal_protective_equipment_model_id);
INSERT INTO table_names (id) VALUES ('commercial_personal_protective_equipment_models');
INSERT INTO table_name_ancestors (descendant_table_name_id, ancestor_table_name_id)
    SELECT 'commercial_personal_protective_equipment_models', ancestor_table_name_id FROM table_name_ancestors WHERE descendant_table_name_id IN ('personal_protective_equipment_models', 'commercial_products')
    UNION SELECT 'commercial_personal_protective_equipment_models', 'commercial_personal_protective_equipment_models';
CREATE TRIGGER commercial_personal_protective_equipment_models_path AFTER INSERT ON commercial_personal_protective_equipment_models
    FOR EACH ROW EXECUTE FUNCTION check_asset_model_subtype();

CREATE FUNCTION require_commercial_personal_protective_equipment_models_row() RETURNS TRIGGER
LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
DECLARE
    row_id UUID := CASE TG_OP WHEN 'DELETE' THEN OLD.id ELSE NEW.id END;
BEGIN
    IF EXISTS (
        SELECT 1 FROM public.asset_models root
        JOIN public.table_name_ancestors path ON path.descendant_table_name_id = root.table_name_id
        WHERE root.id = row_id AND path.ancestor_table_name_id = 'commercial_personal_protective_equipment_models'
    ) AND NOT EXISTS (SELECT 1 FROM public.commercial_personal_protective_equipment_models subtype WHERE subtype.id = row_id) THEN
        RAISE EXCEPTION '% names a table extending % but has no row there', row_id, 'commercial_personal_protective_equipment_models'
            USING ERRCODE = 'foreign_key_violation';
    END IF;
    RETURN NULL;
END;
$$;
ALTER FUNCTION require_commercial_personal_protective_equipment_models_row() OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION require_commercial_personal_protective_equipment_models_row() FROM PUBLIC;
GRANT SELECT ON commercial_personal_protective_equipment_models TO aps_writer;
CREATE CONSTRAINT TRIGGER commercial_personal_protective_equipment_models_row AFTER INSERT ON asset_models
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW WHEN (NEW.table_name_id = 'commercial_personal_protective_equipment_models')
    EXECUTE FUNCTION require_commercial_personal_protective_equipment_models_row();
CREATE CONSTRAINT TRIGGER commercial_personal_protective_equipment_models_kept AFTER DELETE ON commercial_personal_protective_equipment_models
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION require_commercial_personal_protective_equipment_models_row();

-- Catalog of lot-specific commercial personal protective equipment models.
CREATE TABLE commercial_personal_protective_equipment_lots (
    -- Stable lot-model identifier shared with parent lot/model tables.
    id UUID PRIMARY KEY REFERENCES commercial_product_lots (id) ON DELETE CASCADE,
    -- Organization containing this resource.
    organization_id UUID NOT NULL,
    FOREIGN KEY (id) REFERENCES personal_protective_equipment_models (id) ON DELETE CASCADE,
    -- Commercial PPE model from which this lot derives.
    commercial_personal_protective_equipment_model_id UUID NOT NULL REFERENCES commercial_personal_protective_equipment_models (id),
    FOREIGN KEY (id, commercial_personal_protective_equipment_model_id) REFERENCES asset_models (id, parent_model_id) ON DELETE CASCADE,
    FOREIGN KEY (organization_id, id) REFERENCES resources (organization_id, id) ON DELETE CASCADE
);
CREATE INDEX ON commercial_personal_protective_equipment_lots (commercial_personal_protective_equipment_model_id);
INSERT INTO table_names (id) VALUES ('commercial_personal_protective_equipment_lots');
INSERT INTO table_name_ancestors (descendant_table_name_id, ancestor_table_name_id)
    SELECT 'commercial_personal_protective_equipment_lots', ancestor_table_name_id FROM table_name_ancestors WHERE descendant_table_name_id IN ('commercial_product_lots', 'personal_protective_equipment_models')
    UNION SELECT 'commercial_personal_protective_equipment_lots', 'commercial_personal_protective_equipment_lots';
INSERT INTO table_name_admitted_parents (table_name_id, parent_table_name_id)
    VALUES ('commercial_personal_protective_equipment_lots', 'commercial_personal_protective_equipment_models');
CREATE TRIGGER commercial_personal_protective_equipment_lots_path AFTER INSERT ON commercial_personal_protective_equipment_lots
    FOR EACH ROW EXECUTE FUNCTION check_asset_model_subtype();

CREATE FUNCTION require_commercial_personal_protective_equipment_lots_row() RETURNS TRIGGER
LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
DECLARE
    row_id UUID := CASE TG_OP WHEN 'DELETE' THEN OLD.id ELSE NEW.id END;
BEGIN
    IF EXISTS (
        SELECT 1 FROM public.asset_models root
        JOIN public.table_name_ancestors path ON path.descendant_table_name_id = root.table_name_id
        WHERE root.id = row_id AND path.ancestor_table_name_id = 'commercial_personal_protective_equipment_lots'
    ) AND NOT EXISTS (SELECT 1 FROM public.commercial_personal_protective_equipment_lots subtype WHERE subtype.id = row_id) THEN
        RAISE EXCEPTION '% names a table extending % but has no row there', row_id, 'commercial_personal_protective_equipment_lots'
            USING ERRCODE = 'foreign_key_violation';
    END IF;
    RETURN NULL;
END;
$$;
ALTER FUNCTION require_commercial_personal_protective_equipment_lots_row() OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION require_commercial_personal_protective_equipment_lots_row() FROM PUBLIC;
GRANT SELECT ON commercial_personal_protective_equipment_lots TO aps_writer;
CREATE CONSTRAINT TRIGGER commercial_personal_protective_equipment_lots_row AFTER INSERT ON asset_models
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW WHEN (NEW.table_name_id = 'commercial_personal_protective_equipment_lots')
    EXECUTE FUNCTION require_commercial_personal_protective_equipment_lots_row();
CREATE CONSTRAINT TRIGGER commercial_personal_protective_equipment_lots_kept AFTER DELETE ON commercial_personal_protective_equipment_lots
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION require_commercial_personal_protective_equipment_lots_row();

-- Physical personal protective equipment assets tracked in APS inventory.
CREATE TABLE personal_protective_equipments (
    -- Stable asset identifier inherited from physical_assets.
    id UUID PRIMARY KEY REFERENCES physical_assets (id) ON DELETE CASCADE,
    -- Organization containing this resource.
    organization_id UUID NOT NULL,
    -- PPE model instantiated by this physical asset.
    personal_protective_equipment_model_id UUID NOT NULL REFERENCES personal_protective_equipment_models (id),
    FOREIGN KEY (id, personal_protective_equipment_model_id) REFERENCES assets (id, model_id) ON DELETE CASCADE,
    FOREIGN KEY (organization_id, id) REFERENCES resources (organization_id, id) ON DELETE CASCADE
);
CREATE INDEX ON personal_protective_equipments (personal_protective_equipment_model_id);
INSERT INTO table_names (id) VALUES ('personal_protective_equipments');
INSERT INTO table_name_instances (id, instance_table_name_id)
    VALUES ('personal_protective_equipment_models', 'personal_protective_equipments');
INSERT INTO table_name_ancestors (descendant_table_name_id, ancestor_table_name_id)
    SELECT 'personal_protective_equipments', ancestor_table_name_id FROM table_name_ancestors WHERE descendant_table_name_id IN ('physical_assets')
    UNION SELECT 'personal_protective_equipments', 'personal_protective_equipments';
CREATE TRIGGER personal_protective_equipments_path AFTER INSERT ON personal_protective_equipments
    FOR EACH ROW EXECUTE FUNCTION check_asset_subtype();

CREATE FUNCTION require_personal_protective_equipments_row() RETURNS TRIGGER
LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
DECLARE
    row_id UUID := CASE TG_OP WHEN 'DELETE' THEN OLD.id ELSE NEW.id END;
BEGIN
    IF EXISTS (
        SELECT 1 FROM public.assets root
        JOIN public.table_name_ancestors path ON path.descendant_table_name_id = root.table_name_id
        WHERE root.id = row_id AND path.ancestor_table_name_id = 'personal_protective_equipments'
    ) AND NOT EXISTS (SELECT 1 FROM public.personal_protective_equipments subtype WHERE subtype.id = row_id) THEN
        RAISE EXCEPTION '% names a table extending % but has no row there', row_id, 'personal_protective_equipments'
            USING ERRCODE = 'foreign_key_violation';
    END IF;
    RETURN NULL;
END;
$$;
ALTER FUNCTION require_personal_protective_equipments_row() OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION require_personal_protective_equipments_row() FROM PUBLIC;
GRANT SELECT ON personal_protective_equipments TO aps_writer;
CREATE CONSTRAINT TRIGGER personal_protective_equipments_row AFTER INSERT ON assets
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW WHEN (NEW.table_name_id = 'personal_protective_equipments')
    EXECUTE FUNCTION require_personal_protective_equipments_row();
CREATE CONSTRAINT TRIGGER personal_protective_equipments_kept AFTER DELETE ON personal_protective_equipments
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION require_personal_protective_equipments_row();

GRANT SELECT, INSERT, DELETE ON personal_protective_equipment_models TO app;
ALTER TABLE personal_protective_equipment_models ENABLE ROW LEVEL SECURITY;
ALTER TABLE personal_protective_equipment_models FORCE ROW LEVEL SECURITY;
CREATE POLICY personal_protective_equipment_models_select ON personal_protective_equipment_models FOR SELECT TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY personal_protective_equipment_models_insert ON personal_protective_equipment_models FOR INSERT TO app
    WITH CHECK (id = ANY (ARRAY(SELECT authz.resources_with('writer'))));
CREATE POLICY personal_protective_equipment_models_delete ON personal_protective_equipment_models FOR DELETE TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('admin'))));
CREATE POLICY personal_protective_equipment_models_archived_insert ON personal_protective_equipment_models AS RESTRICTIVE FOR INSERT TO app
    WITH CHECK (NOT authz.is_archived(id));
CREATE POLICY personal_protective_equipment_models_archived_delete ON personal_protective_equipment_models AS RESTRICTIVE FOR DELETE TO app
    USING (NOT authz.is_archived(id));
CREATE POLICY personal_protective_equipment_models_blocked ON personal_protective_equipment_models AS RESTRICTIVE FOR ALL TO app
    USING (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))))
    WITH CHECK (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))));

GRANT SELECT, INSERT, DELETE ON commercial_personal_protective_equipment_models TO app;
ALTER TABLE commercial_personal_protective_equipment_models ENABLE ROW LEVEL SECURITY;
ALTER TABLE commercial_personal_protective_equipment_models FORCE ROW LEVEL SECURITY;
CREATE POLICY commercial_personal_protective_equipment_models_select ON commercial_personal_protective_equipment_models FOR SELECT TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY commercial_personal_protective_equipment_models_insert ON commercial_personal_protective_equipment_models FOR INSERT TO app
    WITH CHECK (id = ANY (ARRAY(SELECT authz.resources_with('writer')))
        AND personal_protective_equipment_model_id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY commercial_personal_protective_equipment_models_delete ON commercial_personal_protective_equipment_models FOR DELETE TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('admin'))));
CREATE POLICY commercial_personal_protective_equipment_models_archived_insert ON commercial_personal_protective_equipment_models AS RESTRICTIVE FOR INSERT TO app
    WITH CHECK (NOT authz.is_archived(id) AND NOT authz.is_archived(personal_protective_equipment_model_id));
CREATE POLICY commercial_personal_protective_equipment_models_archived_delete ON commercial_personal_protective_equipment_models AS RESTRICTIVE FOR DELETE TO app
    USING (NOT authz.is_archived(id));
CREATE POLICY commercial_personal_protective_equipment_models_blocked ON commercial_personal_protective_equipment_models AS RESTRICTIVE FOR ALL TO app
    USING (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))))
    WITH CHECK (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))));

GRANT SELECT, INSERT, DELETE ON commercial_personal_protective_equipment_lots TO app;
ALTER TABLE commercial_personal_protective_equipment_lots ENABLE ROW LEVEL SECURITY;
ALTER TABLE commercial_personal_protective_equipment_lots FORCE ROW LEVEL SECURITY;
CREATE POLICY commercial_personal_protective_equipment_lots_select ON commercial_personal_protective_equipment_lots FOR SELECT TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY commercial_personal_protective_equipment_lots_insert ON commercial_personal_protective_equipment_lots FOR INSERT TO app
    WITH CHECK (id = ANY (ARRAY(SELECT authz.resources_with('writer')))
        AND commercial_personal_protective_equipment_model_id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY commercial_personal_protective_equipment_lots_delete ON commercial_personal_protective_equipment_lots FOR DELETE TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('admin'))));
CREATE POLICY commercial_personal_protective_equipment_lots_archived_insert ON commercial_personal_protective_equipment_lots AS RESTRICTIVE FOR INSERT TO app
    WITH CHECK (NOT authz.is_archived(id) AND NOT authz.is_archived(commercial_personal_protective_equipment_model_id));
CREATE POLICY commercial_personal_protective_equipment_lots_archived_delete ON commercial_personal_protective_equipment_lots AS RESTRICTIVE FOR DELETE TO app
    USING (NOT authz.is_archived(id));
CREATE POLICY commercial_personal_protective_equipment_lots_blocked ON commercial_personal_protective_equipment_lots AS RESTRICTIVE FOR ALL TO app
    USING (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))))
    WITH CHECK (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))));

GRANT SELECT, INSERT, DELETE ON personal_protective_equipments TO app;
ALTER TABLE personal_protective_equipments ENABLE ROW LEVEL SECURITY;
ALTER TABLE personal_protective_equipments FORCE ROW LEVEL SECURITY;
CREATE POLICY personal_protective_equipments_select ON personal_protective_equipments FOR SELECT TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY personal_protective_equipments_insert ON personal_protective_equipments FOR INSERT TO app
    WITH CHECK (id = ANY (ARRAY(SELECT authz.resources_with('writer')))
        AND personal_protective_equipment_model_id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY personal_protective_equipments_delete ON personal_protective_equipments FOR DELETE TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('admin'))));
CREATE POLICY personal_protective_equipments_archived_insert ON personal_protective_equipments AS RESTRICTIVE FOR INSERT TO app
    WITH CHECK (NOT authz.is_archived(id) AND NOT authz.is_archived(personal_protective_equipment_model_id));
CREATE POLICY personal_protective_equipments_archived_delete ON personal_protective_equipments AS RESTRICTIVE FOR DELETE TO app
    USING (NOT authz.is_archived(id));
CREATE POLICY personal_protective_equipments_blocked ON personal_protective_equipments AS RESTRICTIVE FOR ALL TO app
    USING (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))))
    WITH CHECK (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))));

-- Records every change in connetto's `audit_events`.
CREATE TRIGGER personal_protective_equipment_models_audit AFTER INSERT OR DELETE ON personal_protective_equipment_models
    FOR EACH ROW EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER personal_protective_equipment_models_update_audit AFTER UPDATE ON personal_protective_equipment_models
    FOR EACH ROW WHEN (OLD IS DISTINCT FROM NEW) EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER commercial_personal_protective_equipment_models_audit AFTER INSERT OR DELETE ON commercial_personal_protective_equipment_models
    FOR EACH ROW EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER commercial_personal_protective_equipment_models_update_audit AFTER UPDATE ON commercial_personal_protective_equipment_models
    FOR EACH ROW WHEN (OLD IS DISTINCT FROM NEW) EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER commercial_personal_protective_equipment_lots_audit AFTER INSERT OR DELETE ON commercial_personal_protective_equipment_lots
    FOR EACH ROW EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER commercial_personal_protective_equipment_lots_update_audit AFTER UPDATE ON commercial_personal_protective_equipment_lots
    FOR EACH ROW WHEN (OLD IS DISTINCT FROM NEW) EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER personal_protective_equipments_audit AFTER INSERT OR DELETE ON personal_protective_equipments
    FOR EACH ROW EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER personal_protective_equipments_update_audit AFTER UPDATE ON personal_protective_equipments
    FOR EACH ROW WHEN (OLD IS DISTINCT FROM NEW) EXECUTE FUNCTION authz_write.record_audit_event();
