-- Migration: 007-geolocation-procedures
-- Purpose: Typed geopositioning slots and execution records.
-- APS placement
-- `resources` of kind `'ownable'`
-- +-- procedure_templates -- geopositioning_procedure_templates
-- +-- procedures -- geopositioning_procedures
-- Slot | Role | Meaning
-- `geopositioned_with` | `instrument` | The device measures position.
-- `geopositioned_asset` | `input` | The positioned asset survives measurement.

-- Typed slots and parameters for geopositioning.
CREATE TABLE geopositioning_procedure_templates (
    -- Identity shared with the parent template.
    id UUID PRIMARY KEY REFERENCES procedure_templates (id) ON DELETE CASCADE,
    -- Organization containing the parent resource.
    organization_id UUID NOT NULL,
    -- Slot declaring the `geopositioned_with` assets.
    geopositioned_with_slot_id UUID NOT NULL,
    -- Admissible model for the `geopositioned_with` slot.
    geopositioned_with_model_id UUID NOT NULL REFERENCES geopositioning_device_models (id),
    -- Required role of the `geopositioned_with` slot.
    geopositioned_with_slot_role_id TEXT GENERATED ALWAYS AS ('instrument') STORED NOT NULL,
    -- Slot declaring the `geopositioned_asset` assets.
    geopositioned_asset_slot_id UUID NOT NULL,
    -- Admissible model for the `geopositioned_asset` slot.
    geopositioned_asset_model_id UUID NOT NULL REFERENCES physical_asset_models (id),
    -- Required role of the `geopositioned_asset` slot.
    geopositioned_asset_slot_role_id TEXT GENERATED ALWAYS AS ('input') STORED NOT NULL,
    FOREIGN KEY (geopositioned_with_slot_id, id, geopositioned_with_model_id, geopositioned_with_slot_role_id, organization_id)
        REFERENCES procedure_template_slots (id, procedure_template_id, asset_model_id, role_id, organization_id),
    FOREIGN KEY (geopositioned_asset_slot_id, id, geopositioned_asset_model_id, geopositioned_asset_slot_role_id, organization_id)
        REFERENCES procedure_template_slots (id, procedure_template_id, asset_model_id, role_id, organization_id),
    -- Archive state inherited from this resource.
    archived BOOLEAN NOT NULL DEFAULT FALSE,
    -- Archive state of the row named by `geopositioned_with_model_id`.
    geopositioned_with_model_archived BOOLEAN NOT NULL DEFAULT FALSE,
    -- Archive state of the row named by `geopositioned_asset_model_id`.
    geopositioned_asset_model_archived BOOLEAN NOT NULL DEFAULT FALSE,
    UNIQUE (id, archived),
    FOREIGN KEY (geopositioned_with_model_id, geopositioned_with_model_archived) REFERENCES geopositioning_device_models (id, archived) ON UPDATE CASCADE,
    FOREIGN KEY (geopositioned_asset_model_id, geopositioned_asset_model_archived) REFERENCES physical_asset_models (id, archived) ON UPDATE CASCADE,
    FOREIGN KEY (organization_id, id, archived) REFERENCES resources (organization_id, id, archived) ON UPDATE CASCADE ON DELETE CASCADE
);
CREATE INDEX ON geopositioning_procedure_templates (geopositioned_asset_model_id);
CREATE INDEX ON geopositioning_procedure_templates (geopositioned_with_model_id);
INSERT INTO table_names (id) VALUES ('geopositioning_procedure_templates');
INSERT INTO table_name_ancestors (descendant_table_name_id, ancestor_table_name_id)
    SELECT 'geopositioning_procedure_templates', ancestor_table_name_id FROM table_name_ancestors WHERE descendant_table_name_id = 'procedure_templates'
    UNION SELECT 'geopositioning_procedure_templates', 'geopositioning_procedure_templates';
CREATE TRIGGER geopositioning_procedure_templates_guard AFTER INSERT ON geopositioning_procedure_templates
    FOR EACH ROW EXECUTE FUNCTION guard_procedure_template_subtype();
CREATE TRIGGER geopositioning_procedure_templates_guard_update AFTER UPDATE OF id, geopositioned_with_slot_id, geopositioned_with_model_id, geopositioned_with_slot_role_id, geopositioned_asset_slot_id, geopositioned_asset_model_id, geopositioned_asset_slot_role_id ON geopositioning_procedure_templates
    FOR EACH ROW WHEN (OLD.id IS DISTINCT FROM NEW.id
        OR OLD.geopositioned_with_slot_id IS DISTINCT FROM NEW.geopositioned_with_slot_id
        OR OLD.geopositioned_with_model_id IS DISTINCT FROM NEW.geopositioned_with_model_id
        OR OLD.geopositioned_with_slot_role_id IS DISTINCT FROM NEW.geopositioned_with_slot_role_id
        OR OLD.geopositioned_asset_slot_id IS DISTINCT FROM NEW.geopositioned_asset_slot_id
        OR OLD.geopositioned_asset_model_id IS DISTINCT FROM NEW.geopositioned_asset_model_id
        OR OLD.geopositioned_asset_slot_role_id IS DISTINCT FROM NEW.geopositioned_asset_slot_role_id) EXECUTE FUNCTION guard_procedure_template_subtype();
CREATE TRIGGER geopositioning_procedure_templates_delete_guard BEFORE DELETE ON geopositioning_procedure_templates
    FOR EACH ROW EXECUTE FUNCTION guard_procedure_template_subtype();

CREATE FUNCTION require_geopositioning_procedure_templates_row() RETURNS TRIGGER
LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
DECLARE
    row_id UUID := CASE TG_OP WHEN 'DELETE' THEN OLD.id ELSE NEW.id END;
BEGIN
    IF EXISTS (
        SELECT 1 FROM public.procedure_templates root
        JOIN public.table_name_ancestors path ON path.descendant_table_name_id = root.table_name_id
        WHERE root.id = row_id AND path.ancestor_table_name_id = 'geopositioning_procedure_templates'
    ) AND NOT EXISTS (SELECT 1 FROM public.geopositioning_procedure_templates row WHERE row.id = row_id) THEN
        RAISE EXCEPTION '% names a table extending % but has no row there', row_id, 'geopositioning_procedure_templates'
            USING ERRCODE = 'foreign_key_violation';
    END IF;
    RETURN NULL;
END;
$$;
ALTER FUNCTION require_geopositioning_procedure_templates_row() OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION require_geopositioning_procedure_templates_row() FROM PUBLIC;
CREATE CONSTRAINT TRIGGER geopositioning_procedure_templates_row AFTER INSERT ON procedure_templates
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW WHEN (NEW.table_name_id = 'geopositioning_procedure_templates')
    EXECUTE FUNCTION require_geopositioning_procedure_templates_row();
CREATE CONSTRAINT TRIGGER geopositioning_procedure_templates_kept AFTER DELETE ON geopositioning_procedure_templates
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION require_geopositioning_procedure_templates_row();

REVOKE ALL ON geopositioning_procedure_templates FROM PUBLIC, app, identity_writer;
GRANT SELECT, INSERT, DELETE ON geopositioning_procedure_templates TO app;
ALTER TABLE geopositioning_procedure_templates ENABLE ROW LEVEL SECURITY;
ALTER TABLE geopositioning_procedure_templates FORCE ROW LEVEL SECURITY;
CREATE POLICY geopositioning_procedure_templates_select ON geopositioning_procedure_templates FOR SELECT TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY geopositioning_procedure_templates_insert ON geopositioning_procedure_templates FOR INSERT TO app
    WITH CHECK (id = ANY (ARRAY(SELECT authz.resources_with('writer'))) AND geopositioned_with_model_id = ANY (ARRAY(SELECT authz.resources_with('reader'))) AND geopositioned_asset_model_id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY geopositioning_procedure_templates_delete ON geopositioning_procedure_templates FOR DELETE TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('admin'))));
CREATE POLICY geopositioning_procedure_templates_archived_insert ON geopositioning_procedure_templates AS RESTRICTIVE FOR INSERT TO app
    WITH CHECK (NOT archived AND NOT geopositioned_with_model_archived AND NOT geopositioned_asset_model_archived);
CREATE POLICY geopositioning_procedure_templates_archived_delete ON geopositioning_procedure_templates AS RESTRICTIVE FOR DELETE TO app
    USING (NOT archived);
CREATE POLICY geopositioning_procedure_templates_blocked ON geopositioning_procedure_templates AS RESTRICTIVE FOR ALL TO app
    USING (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))))
    WITH CHECK (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))));

-- Recorded execution of a geopositioning template.
CREATE TABLE geopositioning_procedures (
    -- Identity shared with the parent procedure.
    id UUID PRIMARY KEY REFERENCES procedures (id) ON DELETE CASCADE,
    -- Organization containing the parent resource.
    organization_id UUID NOT NULL,
    -- Typed template executed by this procedure.
    geopositioning_procedure_template_id UUID NOT NULL REFERENCES geopositioning_procedure_templates (id),
    -- Execution status copied from the parent procedure.
    status_id TEXT NOT NULL DEFAULT 'planned',
    -- Measured geographic position in the WGS 84 reference system.
    location GEOGRAPHY(POINT, 4326),
    FOREIGN KEY (id, geopositioning_procedure_template_id, status_id, organization_id)
        REFERENCES procedures (id, procedure_template_id, status_id, organization_id) ON DELETE CASCADE
        DEFERRABLE INITIALLY DEFERRED,
    CHECK (status_id <> 'completed' OR location IS NOT NULL),
    -- Archive state inherited from this resource.
    archived BOOLEAN NOT NULL DEFAULT FALSE,
    -- Archive state of the row named by `geopositioning_procedure_template_id`.
    geopositioning_procedure_template_archived BOOLEAN NOT NULL DEFAULT FALSE,
    FOREIGN KEY (geopositioning_procedure_template_id, geopositioning_procedure_template_archived) REFERENCES geopositioning_procedure_templates (id, archived) ON UPDATE CASCADE,
    FOREIGN KEY (organization_id, id, archived) REFERENCES resources (organization_id, id, archived) ON UPDATE CASCADE ON DELETE CASCADE
);
CREATE INDEX ON geopositioning_procedures (geopositioning_procedure_template_id);
INSERT INTO table_names (id) VALUES ('geopositioning_procedures');
INSERT INTO table_name_instances (id, instance_table_name_id)
    VALUES ('geopositioning_procedure_templates', 'geopositioning_procedures');
INSERT INTO table_name_ancestors (descendant_table_name_id, ancestor_table_name_id)
    SELECT 'geopositioning_procedures', ancestor_table_name_id FROM table_name_ancestors WHERE descendant_table_name_id = 'procedures'
    UNION SELECT 'geopositioning_procedures', 'geopositioning_procedures';
CREATE TRIGGER geopositioning_procedures_guard AFTER INSERT ON geopositioning_procedures
    FOR EACH ROW EXECUTE FUNCTION guard_procedure_subtype();
CREATE TRIGGER geopositioning_procedures_guard_update AFTER UPDATE OF id, geopositioning_procedure_template_id, status_id, location ON geopositioning_procedures
    FOR EACH ROW WHEN (OLD.id IS DISTINCT FROM NEW.id
        OR OLD.geopositioning_procedure_template_id IS DISTINCT FROM NEW.geopositioning_procedure_template_id
        OR OLD.status_id IS DISTINCT FROM NEW.status_id
        OR OLD.location IS DISTINCT FROM NEW.location) EXECUTE FUNCTION guard_procedure_subtype();
CREATE TRIGGER geopositioning_procedures_delete_guard BEFORE DELETE ON geopositioning_procedures
    FOR EACH ROW EXECUTE FUNCTION guard_procedure_subtype();

-- Writes a procedure's new status into its `geopositioning_procedures` row, whose deferred
-- foreign key holds it equal to the procedure at commit.
CREATE FUNCTION copy_geopositioning_procedures_status() RETURNS TRIGGER
LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
BEGIN
    UPDATE public.geopositioning_procedures SET status_id = NEW.status_id WHERE id = NEW.id;
    RETURN NULL;
END;
$$;
REVOKE ALL ON geopositioning_procedures FROM PUBLIC, app, identity_writer;
GRANT UPDATE (status_id) ON geopositioning_procedures TO aps_writer;
ALTER FUNCTION copy_geopositioning_procedures_status() OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION copy_geopositioning_procedures_status() FROM PUBLIC;
CREATE TRIGGER procedures_status_copy_geopositioning_procedures AFTER UPDATE OF status_id ON procedures
    FOR EACH ROW WHEN (OLD.status_id IS DISTINCT FROM NEW.status_id) EXECUTE FUNCTION copy_geopositioning_procedures_status();

CREATE FUNCTION require_geopositioning_procedures_row() RETURNS TRIGGER
LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
DECLARE
    row_id UUID := CASE TG_OP WHEN 'DELETE' THEN OLD.id ELSE NEW.id END;
BEGIN
    IF EXISTS (
        SELECT 1 FROM public.procedures root
        JOIN public.table_name_ancestors path ON path.descendant_table_name_id = root.table_name_id
        WHERE root.id = row_id AND path.ancestor_table_name_id = 'geopositioning_procedures'
    ) AND NOT EXISTS (SELECT 1 FROM public.geopositioning_procedures row WHERE row.id = row_id) THEN
        RAISE EXCEPTION '% names a table extending % but has no row there', row_id, 'geopositioning_procedures'
            USING ERRCODE = 'foreign_key_violation';
    END IF;
    RETURN NULL;
END;
$$;
ALTER FUNCTION require_geopositioning_procedures_row() OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION require_geopositioning_procedures_row() FROM PUBLIC;
CREATE CONSTRAINT TRIGGER geopositioning_procedures_row AFTER INSERT ON procedures
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW WHEN (NEW.table_name_id = 'geopositioning_procedures')
    EXECUTE FUNCTION require_geopositioning_procedures_row();
CREATE CONSTRAINT TRIGGER geopositioning_procedures_kept AFTER DELETE ON geopositioning_procedures
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION require_geopositioning_procedures_row();

CREATE FUNCTION initialize_geopositioning_procedures_status() RETURNS TRIGGER
LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
DECLARE
    parent_status TEXT;
BEGIN
    SELECT procedure.status_id INTO parent_status
    FROM public.procedures procedure
    WHERE procedure.id = NEW.id
        AND procedure.procedure_template_id = NEW.geopositioning_procedure_template_id
        AND procedure.organization_id = NEW.organization_id;
    IF FOUND THEN
        NEW.status_id := parent_status;
    END IF;
    RETURN NEW;
END;
$$;
ALTER FUNCTION initialize_geopositioning_procedures_status() OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION initialize_geopositioning_procedures_status() FROM PUBLIC;
CREATE TRIGGER geopositioning_procedures_status_initialize BEFORE INSERT ON geopositioning_procedures
    FOR EACH ROW EXECUTE FUNCTION initialize_geopositioning_procedures_status();

GRANT SELECT, INSERT (id, organization_id, geopositioning_procedure_template_id, location, archived, geopositioning_procedure_template_archived), UPDATE (location), DELETE ON geopositioning_procedures TO app;
ALTER TABLE geopositioning_procedures ENABLE ROW LEVEL SECURITY;
ALTER TABLE geopositioning_procedures FORCE ROW LEVEL SECURITY;
CREATE POLICY geopositioning_procedures_select ON geopositioning_procedures FOR SELECT TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY geopositioning_procedures_insert ON geopositioning_procedures FOR INSERT TO app
    WITH CHECK (id = ANY (ARRAY(SELECT authz.resources_with('writer'))) AND geopositioning_procedure_template_id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY geopositioning_procedures_update ON geopositioning_procedures FOR UPDATE TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('writer'))))
    WITH CHECK (id = ANY (ARRAY(SELECT authz.resources_with('writer'))));
CREATE POLICY geopositioning_procedures_delete ON geopositioning_procedures FOR DELETE TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('admin'))));
CREATE POLICY geopositioning_procedures_archived_insert ON geopositioning_procedures AS RESTRICTIVE FOR INSERT TO app
    WITH CHECK (NOT archived AND NOT geopositioning_procedure_template_archived);
CREATE POLICY geopositioning_procedures_archived_update ON geopositioning_procedures AS RESTRICTIVE FOR UPDATE TO app
    USING (NOT archived) WITH CHECK (NOT archived);
CREATE POLICY geopositioning_procedures_archived_delete ON geopositioning_procedures AS RESTRICTIVE FOR DELETE TO app
    USING (NOT archived);
CREATE POLICY geopositioning_procedures_blocked ON geopositioning_procedures AS RESTRICTIVE FOR ALL TO app
    USING (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))))
    WITH CHECK (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))));

-- Records every change in connetto's `audit_events`.
CREATE TRIGGER geopositioning_procedure_templates_audit AFTER INSERT OR DELETE ON geopositioning_procedure_templates
    FOR EACH ROW EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER geopositioning_procedure_templates_update_audit AFTER UPDATE ON geopositioning_procedure_templates
    FOR EACH ROW WHEN (OLD IS DISTINCT FROM NEW) EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER geopositioning_procedures_audit AFTER INSERT OR DELETE ON geopositioning_procedures
    FOR EACH ROW EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER geopositioning_procedures_update_audit AFTER UPDATE ON geopositioning_procedures
    FOR EACH ROW WHEN (OLD IS DISTINCT FROM NEW) EXECUTE FUNCTION authz_write.record_audit_event();
