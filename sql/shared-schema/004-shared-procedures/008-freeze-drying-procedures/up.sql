-- Migration: 008-freeze-drying-procedures
-- Purpose: Typed freeze drying slots and execution records.
-- APS placement
-- `resources` of kind `'ownable'`
-- +-- procedure_templates -- freeze_drying_procedure_templates
-- +-- procedures -- freeze_drying_procedures
-- Slot | Role | Meaning
-- `freeze_dried_with` | `instrument` | The dryer performs lyophilization.
-- `freeze_dried_container` | `input` | The container remains after drying.

-- Typed slots and parameters for freeze drying.
CREATE TABLE freeze_drying_procedure_templates (
    -- Identity shared with the parent template.
    id UUID PRIMARY KEY REFERENCES procedure_templates (id) ON DELETE CASCADE,
    -- Organization containing the parent resource.
    organization_id UUID NOT NULL,
    -- Slot declaring the `freeze_dried_with` assets.
    freeze_dried_with_slot_id UUID NOT NULL,
    -- Admissible model for the `freeze_dried_with` slot.
    freeze_dried_with_model_id UUID NOT NULL REFERENCES freeze_dryer_models (id),
    -- Required role of the `freeze_dried_with` slot.
    freeze_dried_with_slot_role_id TEXT GENERATED ALWAYS AS ('instrument') STORED NOT NULL,
    -- Slot declaring the `freeze_dried_container` assets.
    freeze_dried_container_slot_id UUID NOT NULL,
    -- Admissible model for the `freeze_dried_container` slot.
    freeze_dried_container_model_id UUID NOT NULL REFERENCES volumetric_container_models (id),
    -- Required role of the `freeze_dried_container` slot.
    freeze_dried_container_slot_role_id TEXT GENERATED ALWAYS AS ('input') STORED NOT NULL,
    -- Target temperature in kelvin.
    temperature_k DOUBLE PRECISION NOT NULL DEFAULT 203.15 CHECK (temperature_k > 0.0 AND temperature_k < 'Infinity'::double precision),
    -- Allowed temperature deviation as a percentage of the target.
    temperature_tolerance_percentage DOUBLE PRECISION NOT NULL DEFAULT 5.0 CHECK ( temperature_tolerance_percentage > 0.0 AND temperature_tolerance_percentage <= 100.0 ),
    -- Target pressure in pascals.
    pressure_pa DOUBLE PRECISION NOT NULL DEFAULT 4.0 CHECK ( pressure_pa >= 0.0 AND pressure_pa <= 500.0 ),
    -- Allowed pressure deviation as a percentage of the target.
    pressure_pa_tolerance_percentage DOUBLE PRECISION NOT NULL DEFAULT 5.0 CHECK (pressure_pa_tolerance_percentage > 0.0 AND pressure_pa_tolerance_percentage <= 100.0),
    -- Treatment duration in seconds.
    duration_s DOUBLE PRECISION NOT NULL DEFAULT 259200.0 CHECK ( duration_s >= 7200.0 AND duration_s <= 604800.0 ),
    -- Allowed duration deviation as a percentage of the target.
    duration_tolerance_percentage DOUBLE PRECISION NOT NULL DEFAULT 5.0 CHECK (duration_tolerance_percentage > 0.0 AND duration_tolerance_percentage <= 100.0),
    FOREIGN KEY (freeze_dried_with_slot_id, id, freeze_dried_with_model_id, freeze_dried_with_slot_role_id, organization_id)
        REFERENCES procedure_template_slots (id, procedure_template_id, asset_model_id, role_id, organization_id),
    FOREIGN KEY (freeze_dried_container_slot_id, id, freeze_dried_container_model_id, freeze_dried_container_slot_role_id, organization_id)
        REFERENCES procedure_template_slots (id, procedure_template_id, asset_model_id, role_id, organization_id),
    archived BOOLEAN NOT NULL DEFAULT FALSE,
    freeze_dried_with_model_archived BOOLEAN NOT NULL DEFAULT FALSE,
    freeze_dried_container_model_archived BOOLEAN NOT NULL DEFAULT FALSE,
    UNIQUE (id, archived),
    FOREIGN KEY (freeze_dried_with_model_id, freeze_dried_with_model_archived) REFERENCES freeze_dryer_models (id, archived) ON UPDATE CASCADE,
    FOREIGN KEY (freeze_dried_container_model_id, freeze_dried_container_model_archived) REFERENCES volumetric_container_models (id, archived) ON UPDATE CASCADE,
    FOREIGN KEY (organization_id, id, archived) REFERENCES resources (organization_id, id, archived) ON UPDATE CASCADE ON DELETE CASCADE
);
CREATE INDEX ON freeze_drying_procedure_templates (freeze_dried_container_model_id);
CREATE INDEX ON freeze_drying_procedure_templates (freeze_dried_with_model_id);
INSERT INTO table_names (id) VALUES ('freeze_drying_procedure_templates');
INSERT INTO table_name_ancestors (descendant_table_name_id, ancestor_table_name_id)
    SELECT 'freeze_drying_procedure_templates', ancestor_table_name_id FROM table_name_ancestors WHERE descendant_table_name_id = 'procedure_templates'
    UNION SELECT 'freeze_drying_procedure_templates', 'freeze_drying_procedure_templates';
CREATE TRIGGER freeze_drying_procedure_templates_guard AFTER INSERT ON freeze_drying_procedure_templates
    FOR EACH ROW EXECUTE FUNCTION guard_procedure_template_subtype();
CREATE TRIGGER freeze_drying_procedure_templates_guard_update AFTER UPDATE OF id, freeze_dried_with_slot_id, freeze_dried_with_model_id, freeze_dried_with_slot_role_id, freeze_dried_container_slot_id, freeze_dried_container_model_id, freeze_dried_container_slot_role_id, temperature_k, temperature_tolerance_percentage, pressure_pa, pressure_pa_tolerance_percentage, duration_s, duration_tolerance_percentage ON freeze_drying_procedure_templates
    FOR EACH ROW WHEN (OLD.id IS DISTINCT FROM NEW.id
        OR OLD.freeze_dried_with_slot_id IS DISTINCT FROM NEW.freeze_dried_with_slot_id
        OR OLD.freeze_dried_with_model_id IS DISTINCT FROM NEW.freeze_dried_with_model_id
        OR OLD.freeze_dried_with_slot_role_id IS DISTINCT FROM NEW.freeze_dried_with_slot_role_id
        OR OLD.freeze_dried_container_slot_id IS DISTINCT FROM NEW.freeze_dried_container_slot_id
        OR OLD.freeze_dried_container_model_id IS DISTINCT FROM NEW.freeze_dried_container_model_id
        OR OLD.freeze_dried_container_slot_role_id IS DISTINCT FROM NEW.freeze_dried_container_slot_role_id
        OR OLD.temperature_k IS DISTINCT FROM NEW.temperature_k
        OR OLD.temperature_tolerance_percentage IS DISTINCT FROM NEW.temperature_tolerance_percentage
        OR OLD.pressure_pa IS DISTINCT FROM NEW.pressure_pa
        OR OLD.pressure_pa_tolerance_percentage IS DISTINCT FROM NEW.pressure_pa_tolerance_percentage
        OR OLD.duration_s IS DISTINCT FROM NEW.duration_s
        OR OLD.duration_tolerance_percentage IS DISTINCT FROM NEW.duration_tolerance_percentage) EXECUTE FUNCTION guard_procedure_template_subtype();
CREATE TRIGGER freeze_drying_procedure_templates_delete_guard BEFORE DELETE ON freeze_drying_procedure_templates
    FOR EACH ROW EXECUTE FUNCTION guard_procedure_template_subtype();

CREATE FUNCTION require_freeze_drying_procedure_templates_row() RETURNS TRIGGER
LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
DECLARE
    row_id UUID := CASE TG_OP WHEN 'DELETE' THEN OLD.id ELSE NEW.id END;
BEGIN
    IF EXISTS (
        SELECT 1 FROM public.procedure_templates root
        JOIN public.table_name_ancestors path ON path.descendant_table_name_id = root.table_name_id
        WHERE root.id = row_id AND path.ancestor_table_name_id = 'freeze_drying_procedure_templates'
    ) AND NOT EXISTS (SELECT 1 FROM public.freeze_drying_procedure_templates row WHERE row.id = row_id) THEN
        RAISE EXCEPTION '% names a table extending % but has no row there', row_id, 'freeze_drying_procedure_templates'
            USING ERRCODE = 'foreign_key_violation';
    END IF;
    RETURN NULL;
END;
$$;
ALTER FUNCTION require_freeze_drying_procedure_templates_row() OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION require_freeze_drying_procedure_templates_row() FROM PUBLIC;
CREATE CONSTRAINT TRIGGER freeze_drying_procedure_templates_row AFTER INSERT ON procedure_templates
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW WHEN (NEW.table_name_id = 'freeze_drying_procedure_templates')
    EXECUTE FUNCTION require_freeze_drying_procedure_templates_row();
CREATE CONSTRAINT TRIGGER freeze_drying_procedure_templates_kept AFTER DELETE ON freeze_drying_procedure_templates
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION require_freeze_drying_procedure_templates_row();

REVOKE ALL ON freeze_drying_procedure_templates FROM PUBLIC, app, identity_writer;
GRANT SELECT, INSERT, UPDATE (temperature_k, temperature_tolerance_percentage, pressure_pa, pressure_pa_tolerance_percentage, duration_s, duration_tolerance_percentage), DELETE ON freeze_drying_procedure_templates TO app;
ALTER TABLE freeze_drying_procedure_templates ENABLE ROW LEVEL SECURITY;
ALTER TABLE freeze_drying_procedure_templates FORCE ROW LEVEL SECURITY;
CREATE POLICY freeze_drying_procedure_templates_select ON freeze_drying_procedure_templates FOR SELECT TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY freeze_drying_procedure_templates_insert ON freeze_drying_procedure_templates FOR INSERT TO app
    WITH CHECK (id = ANY (ARRAY(SELECT authz.resources_with('writer'))) AND freeze_dried_with_model_id = ANY (ARRAY(SELECT authz.resources_with('reader'))) AND freeze_dried_container_model_id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY freeze_drying_procedure_templates_update ON freeze_drying_procedure_templates FOR UPDATE TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('writer'))))
    WITH CHECK (id = ANY (ARRAY(SELECT authz.resources_with('writer'))));
CREATE POLICY freeze_drying_procedure_templates_delete ON freeze_drying_procedure_templates FOR DELETE TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('admin'))));
CREATE POLICY freeze_drying_procedure_templates_archived_insert ON freeze_drying_procedure_templates AS RESTRICTIVE FOR INSERT TO app
    WITH CHECK (NOT archived AND NOT freeze_dried_with_model_archived AND NOT freeze_dried_container_model_archived);
CREATE POLICY freeze_drying_procedure_templates_archived_update ON freeze_drying_procedure_templates AS RESTRICTIVE FOR UPDATE TO app
    USING (NOT archived) WITH CHECK (NOT archived);
CREATE POLICY freeze_drying_procedure_templates_archived_delete ON freeze_drying_procedure_templates AS RESTRICTIVE FOR DELETE TO app
    USING (NOT archived);
CREATE POLICY freeze_drying_procedure_templates_blocked ON freeze_drying_procedure_templates AS RESTRICTIVE FOR ALL TO app
    USING (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))))
    WITH CHECK (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))));

-- Recorded execution of a freeze drying template.
CREATE TABLE freeze_drying_procedures (
    -- Identity shared with the parent procedure.
    id UUID PRIMARY KEY REFERENCES procedures (id) ON DELETE CASCADE,
    -- Organization containing the parent resource.
    organization_id UUID NOT NULL,
    -- Typed template executed by this procedure.
    freeze_drying_procedure_template_id UUID NOT NULL REFERENCES freeze_drying_procedure_templates (id),
    -- Execution status copied from the parent procedure.
    status_id TEXT NOT NULL,
    -- Measured temperature k.
    temperature_k DOUBLE PRECISION CHECK (temperature_k > 0.0 AND temperature_k < 'Infinity'::double precision),
    CHECK (status_id <> 'completed' OR temperature_k IS NOT NULL),
    -- Measured pressure pa.
    pressure_pa DOUBLE PRECISION CHECK (pressure_pa >= 0.0 AND pressure_pa < 'Infinity'::double precision),
    CHECK (status_id <> 'completed' OR pressure_pa IS NOT NULL),
    FOREIGN KEY (id, freeze_drying_procedure_template_id, status_id, organization_id)
        REFERENCES procedures (id, procedure_template_id, status_id, organization_id) ON DELETE CASCADE
        DEFERRABLE INITIALLY DEFERRED,
    archived BOOLEAN NOT NULL DEFAULT FALSE,
    freeze_drying_procedure_template_archived BOOLEAN NOT NULL DEFAULT FALSE,
    FOREIGN KEY (freeze_drying_procedure_template_id, freeze_drying_procedure_template_archived) REFERENCES freeze_drying_procedure_templates (id, archived) ON UPDATE CASCADE,
    FOREIGN KEY (organization_id, id, archived) REFERENCES resources (organization_id, id, archived) ON UPDATE CASCADE ON DELETE CASCADE
);
CREATE INDEX ON freeze_drying_procedures (freeze_drying_procedure_template_id);
INSERT INTO table_names (id) VALUES ('freeze_drying_procedures');
INSERT INTO table_name_instances (id, instance_table_name_id)
    VALUES ('freeze_drying_procedure_templates', 'freeze_drying_procedures');
INSERT INTO table_name_ancestors (descendant_table_name_id, ancestor_table_name_id)
    SELECT 'freeze_drying_procedures', ancestor_table_name_id FROM table_name_ancestors WHERE descendant_table_name_id = 'procedures'
    UNION SELECT 'freeze_drying_procedures', 'freeze_drying_procedures';
CREATE TRIGGER freeze_drying_procedures_guard AFTER INSERT ON freeze_drying_procedures
    FOR EACH ROW EXECUTE FUNCTION guard_procedure_subtype();
CREATE TRIGGER freeze_drying_procedures_guard_update AFTER UPDATE OF id, freeze_drying_procedure_template_id, status_id, temperature_k, pressure_pa ON freeze_drying_procedures
    FOR EACH ROW WHEN (OLD.id IS DISTINCT FROM NEW.id
        OR OLD.freeze_drying_procedure_template_id IS DISTINCT FROM NEW.freeze_drying_procedure_template_id
        OR OLD.status_id IS DISTINCT FROM NEW.status_id
        OR OLD.temperature_k IS DISTINCT FROM NEW.temperature_k
        OR OLD.pressure_pa IS DISTINCT FROM NEW.pressure_pa) EXECUTE FUNCTION guard_procedure_subtype();
CREATE TRIGGER freeze_drying_procedures_delete_guard BEFORE DELETE ON freeze_drying_procedures
    FOR EACH ROW EXECUTE FUNCTION guard_procedure_subtype();

-- Writes a procedure's new status into its `freeze_drying_procedures` row, whose deferred
-- foreign key holds it equal to the procedure at commit.
CREATE FUNCTION copy_freeze_drying_procedures_status() RETURNS TRIGGER
LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
BEGIN
    UPDATE public.freeze_drying_procedures SET status_id = NEW.status_id WHERE id = NEW.id;
    RETURN NULL;
END;
$$;
REVOKE ALL ON freeze_drying_procedures FROM PUBLIC, app, identity_writer;
GRANT UPDATE (status_id) ON freeze_drying_procedures TO aps_writer;
ALTER FUNCTION copy_freeze_drying_procedures_status() OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION copy_freeze_drying_procedures_status() FROM PUBLIC;
CREATE TRIGGER procedures_status_copy_freeze_drying_procedures AFTER UPDATE OF status_id ON procedures
    FOR EACH ROW WHEN (OLD.status_id IS DISTINCT FROM NEW.status_id) EXECUTE FUNCTION copy_freeze_drying_procedures_status();

CREATE FUNCTION require_freeze_drying_procedures_row() RETURNS TRIGGER
LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
DECLARE
    row_id UUID := CASE TG_OP WHEN 'DELETE' THEN OLD.id ELSE NEW.id END;
BEGIN
    IF EXISTS (
        SELECT 1 FROM public.procedures root
        JOIN public.table_name_ancestors path ON path.descendant_table_name_id = root.table_name_id
        WHERE root.id = row_id AND path.ancestor_table_name_id = 'freeze_drying_procedures'
    ) AND NOT EXISTS (SELECT 1 FROM public.freeze_drying_procedures row WHERE row.id = row_id) THEN
        RAISE EXCEPTION '% names a table extending % but has no row there', row_id, 'freeze_drying_procedures'
            USING ERRCODE = 'foreign_key_violation';
    END IF;
    RETURN NULL;
END;
$$;
ALTER FUNCTION require_freeze_drying_procedures_row() OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION require_freeze_drying_procedures_row() FROM PUBLIC;
CREATE CONSTRAINT TRIGGER freeze_drying_procedures_row AFTER INSERT ON procedures
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW WHEN (NEW.table_name_id = 'freeze_drying_procedures')
    EXECUTE FUNCTION require_freeze_drying_procedures_row();
CREATE CONSTRAINT TRIGGER freeze_drying_procedures_kept AFTER DELETE ON freeze_drying_procedures
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION require_freeze_drying_procedures_row();

CREATE FUNCTION initialize_freeze_drying_procedures_status() RETURNS TRIGGER
LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
BEGIN
    PERFORM public.lock_procedure(NEW.id);
    SELECT procedure.status_id INTO NEW.status_id
    FROM public.procedures procedure
    WHERE procedure.id = NEW.id
        AND procedure.procedure_template_id = NEW.freeze_drying_procedure_template_id
        AND procedure.organization_id = NEW.organization_id;
    RETURN NEW;
END;
$$;
ALTER FUNCTION initialize_freeze_drying_procedures_status() OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION initialize_freeze_drying_procedures_status() FROM PUBLIC;
CREATE TRIGGER freeze_drying_procedures_status_initialize BEFORE INSERT ON freeze_drying_procedures
    FOR EACH ROW EXECUTE FUNCTION initialize_freeze_drying_procedures_status();

GRANT SELECT, INSERT (id, organization_id, freeze_drying_procedure_template_id, temperature_k, pressure_pa, archived, freeze_drying_procedure_template_archived), UPDATE (temperature_k, pressure_pa), DELETE ON freeze_drying_procedures TO app;
ALTER TABLE freeze_drying_procedures ENABLE ROW LEVEL SECURITY;
ALTER TABLE freeze_drying_procedures FORCE ROW LEVEL SECURITY;
CREATE POLICY freeze_drying_procedures_select ON freeze_drying_procedures FOR SELECT TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY freeze_drying_procedures_insert ON freeze_drying_procedures FOR INSERT TO app
    WITH CHECK (id = ANY (ARRAY(SELECT authz.resources_with('writer'))) AND freeze_drying_procedure_template_id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY freeze_drying_procedures_delete ON freeze_drying_procedures FOR DELETE TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('admin'))));
CREATE POLICY freeze_drying_procedures_archived_insert ON freeze_drying_procedures AS RESTRICTIVE FOR INSERT TO app
    WITH CHECK (NOT archived AND NOT freeze_drying_procedure_template_archived);
CREATE POLICY freeze_drying_procedures_archived_delete ON freeze_drying_procedures AS RESTRICTIVE FOR DELETE TO app
    USING (NOT archived);
CREATE POLICY freeze_drying_procedures_blocked ON freeze_drying_procedures AS RESTRICTIVE FOR ALL TO app
    USING (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))))
    WITH CHECK (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))));

-- Records every change in connetto's `audit_events`.
CREATE TRIGGER freeze_drying_procedure_templates_audit AFTER INSERT OR DELETE ON freeze_drying_procedure_templates
    FOR EACH ROW EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER freeze_drying_procedure_templates_update_audit AFTER UPDATE ON freeze_drying_procedure_templates
    FOR EACH ROW WHEN (OLD IS DISTINCT FROM NEW) EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER freeze_drying_procedures_audit AFTER INSERT OR DELETE ON freeze_drying_procedures
    FOR EACH ROW EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER freeze_drying_procedures_update_audit AFTER UPDATE ON freeze_drying_procedures
    FOR EACH ROW WHEN (OLD IS DISTINCT FROM NEW) EXECUTE FUNCTION authz_write.record_audit_event();
CREATE POLICY freeze_drying_procedures_update ON freeze_drying_procedures FOR UPDATE TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('writer'))))
    WITH CHECK (id = ANY (ARRAY(SELECT authz.resources_with('writer'))));
CREATE POLICY freeze_drying_procedures_archived_update ON freeze_drying_procedures AS RESTRICTIVE FOR UPDATE TO app
    USING (NOT archived) WITH CHECK (NOT archived);
