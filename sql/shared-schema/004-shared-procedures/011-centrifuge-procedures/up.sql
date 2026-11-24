-- Migration: 011-centrifuge-procedures
-- Purpose: Typed centrifuge slots and execution records.
-- APS placement
-- ownables
-- +-- procedure_templates -- centrifuge_procedure_templates
-- +-- procedures -- centrifuge_procedures
-- Slot | Role | Meaning
-- `centrifuged_with` | `instrument` | The centrifuge separates the contents.
-- `centrifuged_container` | `input` | The container remains after centrifugation.

-- Typed slots and parameters for centrifuge.
CREATE TABLE centrifuge_procedure_templates (
    -- Identity shared with the parent template.
    id UUID PRIMARY KEY REFERENCES procedure_templates (id) ON DELETE CASCADE,
    -- Organization containing the parent resource.
    organization_id UUID NOT NULL,
    -- Slot declaring the `centrifuged_with` assets.
    centrifuged_with_slot_id UUID NOT NULL,
    -- Admissible model for the `centrifuged_with` slot.
    centrifuged_with_model_id UUID NOT NULL REFERENCES centrifuge_models (id),
    -- Required role of the `centrifuged_with` slot.
    centrifuged_with_slot_role_id TEXT GENERATED ALWAYS AS ('instrument') STORED NOT NULL,
    -- Slot declaring the `centrifuged_container` assets.
    centrifuged_container_slot_id UUID NOT NULL,
    -- Admissible model for the `centrifuged_container` slot.
    centrifuged_container_model_id UUID NOT NULL REFERENCES volumetric_container_models (id),
    -- Required role of the `centrifuged_container` slot.
    centrifuged_container_slot_role_id TEXT GENERATED ALWAYS AS ('input') STORED NOT NULL,
    -- Target temperature in kelvin.
    temperature_k DOUBLE PRECISION NOT NULL DEFAULT 293.15 CHECK (temperature_k > 0.0 AND temperature_k < 'Infinity'::double precision),
    -- Allowed temperature deviation as a percentage of the target.
    temperature_tolerance_percentage DOUBLE PRECISION NOT NULL DEFAULT 1.0 CHECK ( temperature_tolerance_percentage > 0.0 AND temperature_tolerance_percentage <= 100.0 ),
    -- Treatment duration in seconds.
    duration_s DOUBLE PRECISION NOT NULL DEFAULT 120.0 CHECK ( duration_s >= 30.0 AND duration_s <= 1800.0 ),
    -- Allowed duration deviation as a percentage of the target.
    duration_tolerance_percentage DOUBLE PRECISION NOT NULL DEFAULT 5.0 CHECK (duration_tolerance_percentage > 0.0 AND duration_tolerance_percentage <= 100.0),
    -- Rotational speed in revolutions per minute.
    speed_rpm DOUBLE PRECISION NOT NULL DEFAULT 13000.0 CHECK ( speed_rpm >= 5000.0 AND speed_rpm <= 30000.0 ),
    -- Allowed speed deviation as a percentage of the target.
    speed_rpm_tolerance_percentage DOUBLE PRECISION NOT NULL DEFAULT 5.0 CHECK (speed_rpm_tolerance_percentage > 0.0 AND speed_rpm_tolerance_percentage <= 100.0),
    FOREIGN KEY (centrifuged_with_slot_id, id, centrifuged_with_model_id, centrifuged_with_slot_role_id, organization_id)
        REFERENCES procedure_template_slots (id, procedure_template_id, asset_model_id, role_id, organization_id),
    FOREIGN KEY (centrifuged_container_slot_id, id, centrifuged_container_model_id, centrifuged_container_slot_role_id, organization_id)
        REFERENCES procedure_template_slots (id, procedure_template_id, asset_model_id, role_id, organization_id),
    FOREIGN KEY (organization_id, id) REFERENCES resources (organization_id, id) ON DELETE CASCADE
);
CREATE INDEX ON centrifuge_procedure_templates (centrifuged_container_model_id);
CREATE INDEX ON centrifuge_procedure_templates (centrifuged_with_model_id);
INSERT INTO table_names (id) VALUES ('centrifuge_procedure_templates');
INSERT INTO table_name_ancestors (descendant_table_name_id, ancestor_table_name_id)
    SELECT 'centrifuge_procedure_templates', ancestor_table_name_id FROM table_name_ancestors WHERE descendant_table_name_id = 'procedure_templates'
    UNION SELECT 'centrifuge_procedure_templates', 'centrifuge_procedure_templates';
CREATE TRIGGER centrifuge_procedure_templates_guard AFTER INSERT OR UPDATE ON centrifuge_procedure_templates
    FOR EACH ROW EXECUTE FUNCTION guard_procedure_template_subtype();
CREATE TRIGGER centrifuge_procedure_templates_delete_guard BEFORE DELETE ON centrifuge_procedure_templates
    FOR EACH ROW EXECUTE FUNCTION guard_procedure_template_subtype();

CREATE FUNCTION require_centrifuge_procedure_templates_row() RETURNS TRIGGER
LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
DECLARE
    row_id UUID := CASE TG_OP WHEN 'DELETE' THEN OLD.id ELSE NEW.id END;
BEGIN
    IF EXISTS (
        SELECT 1 FROM public.procedure_templates root
        JOIN public.table_name_ancestors path ON path.descendant_table_name_id = root.table_name_id
        WHERE root.id = row_id AND path.ancestor_table_name_id = 'centrifuge_procedure_templates'
    ) AND NOT EXISTS (SELECT 1 FROM public.centrifuge_procedure_templates row WHERE row.id = row_id) THEN
        RAISE EXCEPTION '% names a table extending % but has no row there', row_id, 'centrifuge_procedure_templates'
            USING ERRCODE = 'foreign_key_violation';
    END IF;
    RETURN NULL;
END;
$$;
ALTER FUNCTION require_centrifuge_procedure_templates_row() OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION require_centrifuge_procedure_templates_row() FROM PUBLIC;
CREATE CONSTRAINT TRIGGER centrifuge_procedure_templates_row AFTER INSERT ON procedure_templates
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW WHEN (NEW.table_name_id = 'centrifuge_procedure_templates')
    EXECUTE FUNCTION require_centrifuge_procedure_templates_row();
CREATE CONSTRAINT TRIGGER centrifuge_procedure_templates_kept AFTER DELETE ON centrifuge_procedure_templates
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION require_centrifuge_procedure_templates_row();

GRANT SELECT, INSERT, UPDATE (temperature_k, temperature_tolerance_percentage, duration_s, duration_tolerance_percentage, speed_rpm, speed_rpm_tolerance_percentage), DELETE ON centrifuge_procedure_templates TO app;
ALTER TABLE centrifuge_procedure_templates ENABLE ROW LEVEL SECURITY;
ALTER TABLE centrifuge_procedure_templates FORCE ROW LEVEL SECURITY;
CREATE POLICY centrifuge_procedure_templates_select ON centrifuge_procedure_templates FOR SELECT TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY centrifuge_procedure_templates_insert ON centrifuge_procedure_templates FOR INSERT TO app
    WITH CHECK (id = ANY (ARRAY(SELECT authz.resources_with('writer'))) AND centrifuged_with_model_id = ANY (ARRAY(SELECT authz.resources_with('reader'))) AND centrifuged_container_model_id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY centrifuge_procedure_templates_update ON centrifuge_procedure_templates FOR UPDATE TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('writer'))))
    WITH CHECK (id = ANY (ARRAY(SELECT authz.resources_with('writer'))));
CREATE POLICY centrifuge_procedure_templates_delete ON centrifuge_procedure_templates FOR DELETE TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('admin'))));
CREATE POLICY centrifuge_procedure_templates_archived_insert ON centrifuge_procedure_templates AS RESTRICTIVE FOR INSERT TO app
    WITH CHECK (NOT authz.is_archived(id) AND NOT authz.is_archived(centrifuged_with_model_id) AND NOT authz.is_archived(centrifuged_container_model_id));
CREATE POLICY centrifuge_procedure_templates_archived_update ON centrifuge_procedure_templates AS RESTRICTIVE FOR UPDATE TO app
    USING (NOT authz.is_archived(id)) WITH CHECK (NOT authz.is_archived(id));
CREATE POLICY centrifuge_procedure_templates_archived_delete ON centrifuge_procedure_templates AS RESTRICTIVE FOR DELETE TO app
    USING (NOT authz.is_archived(id));
CREATE POLICY centrifuge_procedure_templates_blocked ON centrifuge_procedure_templates AS RESTRICTIVE FOR ALL TO app
    USING (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))))
    WITH CHECK (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))));

-- Recorded execution of a centrifuge template.
CREATE TABLE centrifuge_procedures (
    -- Identity shared with the parent procedure.
    id UUID PRIMARY KEY REFERENCES procedures (id) ON DELETE CASCADE,
    -- Organization containing the parent resource.
    organization_id UUID NOT NULL,
    -- Typed template executed by this procedure.
    centrifuge_procedure_template_id UUID NOT NULL REFERENCES centrifuge_procedure_templates (id),
    -- Execution status copied from the parent procedure.
    status_id TEXT NOT NULL,
    -- Measured temperature k.
    temperature_k DOUBLE PRECISION CHECK (temperature_k > 0.0 AND temperature_k < 'Infinity'::double precision),
    CHECK (status_id <> 'completed' OR temperature_k IS NOT NULL),
    -- Measured speed rpm.
    speed_rpm DOUBLE PRECISION CHECK (speed_rpm >= 0.0 AND speed_rpm < 'Infinity'::double precision),
    CHECK (status_id <> 'completed' OR speed_rpm IS NOT NULL),
    FOREIGN KEY (id, centrifuge_procedure_template_id, status_id, organization_id)
        REFERENCES procedures (id, procedure_template_id, status_id, organization_id) ON DELETE CASCADE
        DEFERRABLE INITIALLY DEFERRED,
    FOREIGN KEY (organization_id, id) REFERENCES resources (organization_id, id) ON DELETE CASCADE
);
CREATE INDEX ON centrifuge_procedures (centrifuge_procedure_template_id);
INSERT INTO table_names (id) VALUES ('centrifuge_procedures');
INSERT INTO table_name_instances (id, instance_table_name_id)
    VALUES ('centrifuge_procedure_templates', 'centrifuge_procedures');
INSERT INTO table_name_ancestors (descendant_table_name_id, ancestor_table_name_id)
    SELECT 'centrifuge_procedures', ancestor_table_name_id FROM table_name_ancestors WHERE descendant_table_name_id = 'procedures'
    UNION SELECT 'centrifuge_procedures', 'centrifuge_procedures';
CREATE TRIGGER centrifuge_procedures_guard AFTER INSERT OR UPDATE ON centrifuge_procedures
    FOR EACH ROW EXECUTE FUNCTION guard_procedure_subtype();
CREATE TRIGGER centrifuge_procedures_delete_guard BEFORE DELETE ON centrifuge_procedures
    FOR EACH ROW EXECUTE FUNCTION guard_procedure_subtype();

-- Writes a procedure's new status into its `centrifuge_procedures` row, whose deferred
-- foreign key holds it equal to the procedure at commit.
CREATE FUNCTION copy_centrifuge_procedures_status() RETURNS TRIGGER
LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
BEGIN
    UPDATE public.centrifuge_procedures SET status_id = NEW.status_id WHERE id = NEW.id;
    RETURN NULL;
END;
$$;
GRANT UPDATE (status_id) ON centrifuge_procedures TO aps_writer;
ALTER FUNCTION copy_centrifuge_procedures_status() OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION copy_centrifuge_procedures_status() FROM PUBLIC;
CREATE TRIGGER procedures_status_copy_centrifuge_procedures AFTER UPDATE OF status_id ON procedures
    FOR EACH ROW WHEN (OLD.status_id IS DISTINCT FROM NEW.status_id) EXECUTE FUNCTION copy_centrifuge_procedures_status();

CREATE FUNCTION require_centrifuge_procedures_row() RETURNS TRIGGER
LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
DECLARE
    row_id UUID := CASE TG_OP WHEN 'DELETE' THEN OLD.id ELSE NEW.id END;
BEGIN
    IF EXISTS (
        SELECT 1 FROM public.procedures root
        JOIN public.table_name_ancestors path ON path.descendant_table_name_id = root.table_name_id
        WHERE root.id = row_id AND path.ancestor_table_name_id = 'centrifuge_procedures'
    ) AND NOT EXISTS (SELECT 1 FROM public.centrifuge_procedures row WHERE row.id = row_id) THEN
        RAISE EXCEPTION '% names a table extending % but has no row there', row_id, 'centrifuge_procedures'
            USING ERRCODE = 'foreign_key_violation';
    END IF;
    RETURN NULL;
END;
$$;
ALTER FUNCTION require_centrifuge_procedures_row() OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION require_centrifuge_procedures_row() FROM PUBLIC;
CREATE CONSTRAINT TRIGGER centrifuge_procedures_row AFTER INSERT ON procedures
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW WHEN (NEW.table_name_id = 'centrifuge_procedures')
    EXECUTE FUNCTION require_centrifuge_procedures_row();
CREATE CONSTRAINT TRIGGER centrifuge_procedures_kept AFTER DELETE ON centrifuge_procedures
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION require_centrifuge_procedures_row();

GRANT SELECT, INSERT, UPDATE (temperature_k, speed_rpm), DELETE ON centrifuge_procedures TO app;
ALTER TABLE centrifuge_procedures ENABLE ROW LEVEL SECURITY;
ALTER TABLE centrifuge_procedures FORCE ROW LEVEL SECURITY;
CREATE POLICY centrifuge_procedures_select ON centrifuge_procedures FOR SELECT TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY centrifuge_procedures_insert ON centrifuge_procedures FOR INSERT TO app
    WITH CHECK (id = ANY (ARRAY(SELECT authz.resources_with('writer'))) AND centrifuge_procedure_template_id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY centrifuge_procedures_delete ON centrifuge_procedures FOR DELETE TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('admin'))));
CREATE POLICY centrifuge_procedures_archived_insert ON centrifuge_procedures AS RESTRICTIVE FOR INSERT TO app
    WITH CHECK (NOT authz.is_archived(id) AND NOT authz.is_archived(centrifuge_procedure_template_id));
CREATE POLICY centrifuge_procedures_archived_delete ON centrifuge_procedures AS RESTRICTIVE FOR DELETE TO app
    USING (NOT authz.is_archived(id));
CREATE POLICY centrifuge_procedures_blocked ON centrifuge_procedures AS RESTRICTIVE FOR ALL TO app
    USING (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))))
    WITH CHECK (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))));

-- Records every change in connetto's `audit_events`.
CREATE TRIGGER centrifuge_procedure_templates_audit AFTER INSERT OR DELETE ON centrifuge_procedure_templates
    FOR EACH ROW EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER centrifuge_procedure_templates_update_audit AFTER UPDATE ON centrifuge_procedure_templates
    FOR EACH ROW WHEN (OLD IS DISTINCT FROM NEW) EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER centrifuge_procedures_audit AFTER INSERT OR DELETE ON centrifuge_procedures
    FOR EACH ROW EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER centrifuge_procedures_update_audit AFTER UPDATE ON centrifuge_procedures
    FOR EACH ROW WHEN (OLD IS DISTINCT FROM NEW) EXECUTE FUNCTION authz_write.record_audit_event();
CREATE POLICY centrifuge_procedures_update ON centrifuge_procedures FOR UPDATE TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('writer'))))
    WITH CHECK (id = ANY (ARRAY(SELECT authz.resources_with('writer'))));
CREATE POLICY centrifuge_procedures_archived_update ON centrifuge_procedures AS RESTRICTIVE FOR UPDATE TO app
    USING (NOT authz.is_archived(id)) WITH CHECK (NOT authz.is_archived(id));
