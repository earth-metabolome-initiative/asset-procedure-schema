-- Migration: 009-freezing-procedures
-- Purpose: Typed freezing slots and execution records.
-- APS placement
-- ownables
-- +-- procedure_templates -- freezing_procedure_templates
-- +-- procedures -- freezing_procedures
-- Slot | Role | Meaning
-- `frozen_with` | `instrument` | The freezer cools the container.
-- `frozen_container` | `input` | The container remains after freezing.

-- Typed slots and parameters for freezing.
CREATE TABLE freezing_procedure_templates (
    -- Identity shared with the parent template.
    id UUID PRIMARY KEY REFERENCES procedure_templates (id) ON DELETE CASCADE,
    -- Organization containing the parent resource.
    organization_id UUID NOT NULL,
    -- Slot declaring the `frozen_with` assets.
    frozen_with_slot_id UUID NOT NULL,
    -- Admissible model for the `frozen_with` slot.
    frozen_with_model_id UUID NOT NULL REFERENCES freezer_models (id),
    -- Required role of the `frozen_with` slot.
    frozen_with_slot_role_id TEXT GENERATED ALWAYS AS ('instrument') STORED NOT NULL,
    -- Slot declaring the `frozen_container` assets.
    frozen_container_slot_id UUID NOT NULL,
    -- Admissible model for the `frozen_container` slot.
    frozen_container_model_id UUID NOT NULL REFERENCES volumetric_container_models (id),
    -- Required role of the `frozen_container` slot.
    frozen_container_slot_role_id TEXT GENERATED ALWAYS AS ('input') STORED NOT NULL,
    -- Target temperature in kelvin.
    temperature_k DOUBLE PRECISION NOT NULL DEFAULT 203.15 CHECK (temperature_k > 0.0 AND temperature_k < 'Infinity'::double precision),
    -- Allowed temperature deviation as a percentage of the target.
    temperature_tolerance_percentage DOUBLE PRECISION NOT NULL DEFAULT 5.0 CHECK ( temperature_tolerance_percentage > 0.0 AND temperature_tolerance_percentage <= 100.0 ),
    -- Treatment duration in seconds.
    duration_s DOUBLE PRECISION NOT NULL DEFAULT 43200.0 CHECK (duration_s > 1800.0 AND duration_s <= 604800.0),
    -- Allowed duration deviation as a percentage of the target.
    duration_tolerance_percentage DOUBLE PRECISION NOT NULL DEFAULT 5.0 CHECK (duration_tolerance_percentage > 0.0 AND duration_tolerance_percentage <= 100.0),
    FOREIGN KEY (frozen_with_slot_id, id, frozen_with_model_id, frozen_with_slot_role_id, organization_id)
        REFERENCES procedure_template_slots (id, procedure_template_id, asset_model_id, role_id, organization_id),
    FOREIGN KEY (frozen_container_slot_id, id, frozen_container_model_id, frozen_container_slot_role_id, organization_id)
        REFERENCES procedure_template_slots (id, procedure_template_id, asset_model_id, role_id, organization_id),
    FOREIGN KEY (organization_id, id) REFERENCES resources (organization_id, id) ON DELETE CASCADE
);
CREATE INDEX ON freezing_procedure_templates (frozen_container_model_id);
CREATE INDEX ON freezing_procedure_templates (frozen_with_model_id);
INSERT INTO table_names (id) VALUES ('freezing_procedure_templates');
INSERT INTO table_name_ancestors (descendant_table_name_id, ancestor_table_name_id)
    SELECT 'freezing_procedure_templates', ancestor_table_name_id FROM table_name_ancestors WHERE descendant_table_name_id = 'procedure_templates'
    UNION SELECT 'freezing_procedure_templates', 'freezing_procedure_templates';
CREATE TRIGGER freezing_procedure_templates_guard AFTER INSERT OR UPDATE ON freezing_procedure_templates
    FOR EACH ROW EXECUTE FUNCTION guard_procedure_template_subtype();
CREATE TRIGGER freezing_procedure_templates_delete_guard BEFORE DELETE ON freezing_procedure_templates
    FOR EACH ROW EXECUTE FUNCTION guard_procedure_template_subtype();

CREATE FUNCTION require_freezing_procedure_templates_row() RETURNS TRIGGER
LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
DECLARE
    row_id UUID := CASE TG_OP WHEN 'DELETE' THEN OLD.id ELSE NEW.id END;
BEGIN
    IF EXISTS (
        SELECT 1 FROM public.procedure_templates root
        JOIN public.table_name_ancestors path ON path.descendant_table_name_id = root.table_name_id
        WHERE root.id = row_id AND path.ancestor_table_name_id = 'freezing_procedure_templates'
    ) AND NOT EXISTS (SELECT 1 FROM public.freezing_procedure_templates row WHERE row.id = row_id) THEN
        RAISE EXCEPTION '% names a table extending % but has no row there', row_id, 'freezing_procedure_templates'
            USING ERRCODE = 'foreign_key_violation';
    END IF;
    RETURN NULL;
END;
$$;
ALTER FUNCTION require_freezing_procedure_templates_row() OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION require_freezing_procedure_templates_row() FROM PUBLIC;
CREATE CONSTRAINT TRIGGER freezing_procedure_templates_row AFTER INSERT ON procedure_templates
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW WHEN (NEW.table_name_id = 'freezing_procedure_templates')
    EXECUTE FUNCTION require_freezing_procedure_templates_row();
CREATE CONSTRAINT TRIGGER freezing_procedure_templates_kept AFTER DELETE ON freezing_procedure_templates
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION require_freezing_procedure_templates_row();

GRANT SELECT, INSERT, UPDATE (temperature_k, temperature_tolerance_percentage, duration_s, duration_tolerance_percentage), DELETE ON freezing_procedure_templates TO app;
ALTER TABLE freezing_procedure_templates ENABLE ROW LEVEL SECURITY;
ALTER TABLE freezing_procedure_templates FORCE ROW LEVEL SECURITY;
CREATE POLICY freezing_procedure_templates_select ON freezing_procedure_templates FOR SELECT TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY freezing_procedure_templates_insert ON freezing_procedure_templates FOR INSERT TO app
    WITH CHECK (id = ANY (ARRAY(SELECT authz.resources_with('writer'))) AND frozen_with_model_id = ANY (ARRAY(SELECT authz.resources_with('reader'))) AND frozen_container_model_id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY freezing_procedure_templates_update ON freezing_procedure_templates FOR UPDATE TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('writer'))))
    WITH CHECK (id = ANY (ARRAY(SELECT authz.resources_with('writer'))));
CREATE POLICY freezing_procedure_templates_delete ON freezing_procedure_templates FOR DELETE TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('admin'))));
CREATE POLICY freezing_procedure_templates_archived_insert ON freezing_procedure_templates AS RESTRICTIVE FOR INSERT TO app
    WITH CHECK (NOT authz.is_archived(id) AND NOT authz.is_archived(frozen_with_model_id) AND NOT authz.is_archived(frozen_container_model_id));
CREATE POLICY freezing_procedure_templates_archived_update ON freezing_procedure_templates AS RESTRICTIVE FOR UPDATE TO app
    USING (NOT authz.is_archived(id)) WITH CHECK (NOT authz.is_archived(id));
CREATE POLICY freezing_procedure_templates_archived_delete ON freezing_procedure_templates AS RESTRICTIVE FOR DELETE TO app
    USING (NOT authz.is_archived(id));
CREATE POLICY freezing_procedure_templates_blocked ON freezing_procedure_templates AS RESTRICTIVE FOR ALL TO app
    USING (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))))
    WITH CHECK (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))));

-- Recorded execution of a freezing template.
CREATE TABLE freezing_procedures (
    -- Identity shared with the parent procedure.
    id UUID PRIMARY KEY REFERENCES procedures (id) ON DELETE CASCADE,
    -- Organization containing the parent resource.
    organization_id UUID NOT NULL,
    -- Typed template executed by this procedure.
    freezing_procedure_template_id UUID NOT NULL REFERENCES freezing_procedure_templates (id),
    -- Execution status copied from the parent procedure.
    status_id TEXT NOT NULL,
    -- Measured temperature k.
    temperature_k DOUBLE PRECISION CHECK (temperature_k > 0.0 AND temperature_k < 'Infinity'::double precision),
    CHECK (status_id <> 'completed' OR temperature_k IS NOT NULL),
    FOREIGN KEY (id, freezing_procedure_template_id, status_id, organization_id)
        REFERENCES procedures (id, procedure_template_id, status_id, organization_id) ON DELETE CASCADE
        DEFERRABLE INITIALLY DEFERRED,
    FOREIGN KEY (organization_id, id) REFERENCES resources (organization_id, id) ON DELETE CASCADE
);
CREATE INDEX ON freezing_procedures (freezing_procedure_template_id);
INSERT INTO table_names (id) VALUES ('freezing_procedures');
INSERT INTO table_name_instances (id, instance_table_name_id)
    VALUES ('freezing_procedure_templates', 'freezing_procedures');
INSERT INTO table_name_ancestors (descendant_table_name_id, ancestor_table_name_id)
    SELECT 'freezing_procedures', ancestor_table_name_id FROM table_name_ancestors WHERE descendant_table_name_id = 'procedures'
    UNION SELECT 'freezing_procedures', 'freezing_procedures';
CREATE TRIGGER freezing_procedures_guard AFTER INSERT OR UPDATE ON freezing_procedures
    FOR EACH ROW EXECUTE FUNCTION guard_procedure_subtype();
CREATE TRIGGER freezing_procedures_delete_guard BEFORE DELETE ON freezing_procedures
    FOR EACH ROW EXECUTE FUNCTION guard_procedure_subtype();

-- Writes a procedure's new status into its `freezing_procedures` row, whose deferred
-- foreign key holds it equal to the procedure at commit.
CREATE FUNCTION copy_freezing_procedures_status() RETURNS TRIGGER
LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
BEGIN
    UPDATE public.freezing_procedures SET status_id = NEW.status_id WHERE id = NEW.id;
    RETURN NULL;
END;
$$;
GRANT UPDATE (status_id) ON freezing_procedures TO aps_writer;
ALTER FUNCTION copy_freezing_procedures_status() OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION copy_freezing_procedures_status() FROM PUBLIC;
CREATE TRIGGER procedures_status_copy_freezing_procedures AFTER UPDATE OF status_id ON procedures
    FOR EACH ROW WHEN (OLD.status_id IS DISTINCT FROM NEW.status_id) EXECUTE FUNCTION copy_freezing_procedures_status();

CREATE FUNCTION require_freezing_procedures_row() RETURNS TRIGGER
LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
DECLARE
    row_id UUID := CASE TG_OP WHEN 'DELETE' THEN OLD.id ELSE NEW.id END;
BEGIN
    IF EXISTS (
        SELECT 1 FROM public.procedures root
        JOIN public.table_name_ancestors path ON path.descendant_table_name_id = root.table_name_id
        WHERE root.id = row_id AND path.ancestor_table_name_id = 'freezing_procedures'
    ) AND NOT EXISTS (SELECT 1 FROM public.freezing_procedures row WHERE row.id = row_id) THEN
        RAISE EXCEPTION '% names a table extending % but has no row there', row_id, 'freezing_procedures'
            USING ERRCODE = 'foreign_key_violation';
    END IF;
    RETURN NULL;
END;
$$;
ALTER FUNCTION require_freezing_procedures_row() OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION require_freezing_procedures_row() FROM PUBLIC;
CREATE CONSTRAINT TRIGGER freezing_procedures_row AFTER INSERT ON procedures
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW WHEN (NEW.table_name_id = 'freezing_procedures')
    EXECUTE FUNCTION require_freezing_procedures_row();
CREATE CONSTRAINT TRIGGER freezing_procedures_kept AFTER DELETE ON freezing_procedures
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION require_freezing_procedures_row();

GRANT SELECT, INSERT, UPDATE (temperature_k), DELETE ON freezing_procedures TO app;
ALTER TABLE freezing_procedures ENABLE ROW LEVEL SECURITY;
ALTER TABLE freezing_procedures FORCE ROW LEVEL SECURITY;
CREATE POLICY freezing_procedures_select ON freezing_procedures FOR SELECT TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY freezing_procedures_insert ON freezing_procedures FOR INSERT TO app
    WITH CHECK (id = ANY (ARRAY(SELECT authz.resources_with('writer'))) AND freezing_procedure_template_id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY freezing_procedures_delete ON freezing_procedures FOR DELETE TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('admin'))));
CREATE POLICY freezing_procedures_archived_insert ON freezing_procedures AS RESTRICTIVE FOR INSERT TO app
    WITH CHECK (NOT authz.is_archived(id) AND NOT authz.is_archived(freezing_procedure_template_id));
CREATE POLICY freezing_procedures_archived_delete ON freezing_procedures AS RESTRICTIVE FOR DELETE TO app
    USING (NOT authz.is_archived(id));
CREATE POLICY freezing_procedures_blocked ON freezing_procedures AS RESTRICTIVE FOR ALL TO app
    USING (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))))
    WITH CHECK (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))));

-- Records every change in connetto's `audit_events`.
CREATE TRIGGER freezing_procedure_templates_audit AFTER INSERT OR DELETE ON freezing_procedure_templates
    FOR EACH ROW EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER freezing_procedure_templates_update_audit AFTER UPDATE ON freezing_procedure_templates
    FOR EACH ROW WHEN (OLD IS DISTINCT FROM NEW) EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER freezing_procedures_audit AFTER INSERT OR DELETE ON freezing_procedures
    FOR EACH ROW EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER freezing_procedures_update_audit AFTER UPDATE ON freezing_procedures
    FOR EACH ROW WHEN (OLD IS DISTINCT FROM NEW) EXECUTE FUNCTION authz_write.record_audit_event();
CREATE POLICY freezing_procedures_update ON freezing_procedures FOR UPDATE TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('writer'))))
    WITH CHECK (id = ANY (ARRAY(SELECT authz.resources_with('writer'))));
CREATE POLICY freezing_procedures_archived_update ON freezing_procedures AS RESTRICTIVE FOR UPDATE TO app
    USING (NOT authz.is_archived(id)) WITH CHECK (NOT authz.is_archived(id));
