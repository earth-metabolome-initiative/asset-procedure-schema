-- Migration: 012-ball-mill-procedures
-- Purpose: Typed ball mill slots and execution records.
-- APS placement
-- ownables
-- +-- procedure_templates -- ball_mill_procedure_templates
-- +-- procedures -- ball_mill_procedures
-- Slot | Role | Meaning
-- `bead` | `instrument` | The beads grind the contents.
-- `milled_with` | `instrument` | The mill drives grinding.
-- `milled_container` | `input` | The container remains after milling.

-- Typed slots and parameters for ball mill.
CREATE TABLE ball_mill_procedure_templates (
    -- Identity shared with the parent template.
    id UUID PRIMARY KEY REFERENCES procedure_templates (id) ON DELETE CASCADE,
    -- Organization containing the parent resource.
    organization_id UUID NOT NULL,
    -- Slot declaring the `bead` assets.
    bead_slot_id UUID NOT NULL,
    -- Admissible model for the `bead` slot.
    bead_model_id UUID NOT NULL REFERENCES bead_models (id),
    -- Required role of the `bead` slot.
    bead_slot_role_id TEXT GENERATED ALWAYS AS ('instrument') STORED NOT NULL,
    -- Slot declaring the `milled_with` assets.
    milled_with_slot_id UUID NOT NULL,
    -- Admissible model for the `milled_with` slot.
    milled_with_model_id UUID NOT NULL REFERENCES ball_mill_machine_models (id),
    -- Required role of the `milled_with` slot.
    milled_with_slot_role_id TEXT GENERATED ALWAYS AS ('instrument') STORED NOT NULL,
    -- Slot declaring the `milled_container` assets.
    milled_container_slot_id UUID NOT NULL,
    -- Admissible model for the `milled_container` slot.
    milled_container_model_id UUID NOT NULL REFERENCES volumetric_container_models (id),
    -- Required role of the `milled_container` slot.
    milled_container_slot_role_id TEXT GENERATED ALWAYS AS ('input') STORED NOT NULL,
    -- Target temperature in kelvin.
    temperature_k DOUBLE PRECISION NOT NULL DEFAULT 293.15 CHECK (temperature_k > 0.0 AND temperature_k < 'Infinity'::double precision),
    -- Allowed temperature deviation as a percentage of the target.
    temperature_tolerance_percentage DOUBLE PRECISION NOT NULL DEFAULT 1.0 CHECK ( temperature_tolerance_percentage > 0.0 AND temperature_tolerance_percentage <= 100.0 ),
    -- Treatment duration in seconds.
    duration_s DOUBLE PRECISION NOT NULL DEFAULT 150.0 CHECK ( duration_s <= 900.0 AND duration_s >= 30.0 ),
    -- Allowed duration deviation as a percentage of the target.
    duration_tolerance_percentage DOUBLE PRECISION NOT NULL DEFAULT 5.0 CHECK (duration_tolerance_percentage > 0.0 AND duration_tolerance_percentage <= 100.0),
    -- Oscillation frequency in hertz.
    frequency_hz DOUBLE PRECISION NOT NULL DEFAULT 25.0 CHECK ( frequency_hz <= 50.0 AND frequency_hz >= 15.0 ),
    -- Allowed frequency deviation as a percentage of the target.
    frequency_hz_tolerance_percentage DOUBLE PRECISION NOT NULL DEFAULT 5.0 CHECK (frequency_hz_tolerance_percentage > 0.0 AND frequency_hz_tolerance_percentage <= 100.0),
    -- Number of grinding beads.
    bead_count SMALLINT NOT NULL DEFAULT 3 CHECK (bead_count > 0),
    FOREIGN KEY (bead_slot_id, id, bead_model_id, bead_slot_role_id, organization_id)
        REFERENCES procedure_template_slots (id, procedure_template_id, asset_model_id, role_id, organization_id),
    FOREIGN KEY (milled_with_slot_id, id, milled_with_model_id, milled_with_slot_role_id, organization_id)
        REFERENCES procedure_template_slots (id, procedure_template_id, asset_model_id, role_id, organization_id),
    FOREIGN KEY (milled_container_slot_id, id, milled_container_model_id, milled_container_slot_role_id, organization_id)
        REFERENCES procedure_template_slots (id, procedure_template_id, asset_model_id, role_id, organization_id),
    FOREIGN KEY (organization_id, id) REFERENCES resources (organization_id, id) ON DELETE CASCADE
);
CREATE INDEX ON ball_mill_procedure_templates (bead_model_id);
CREATE INDEX ON ball_mill_procedure_templates (milled_container_model_id);
CREATE INDEX ON ball_mill_procedure_templates (milled_with_model_id);
INSERT INTO table_names (id) VALUES ('ball_mill_procedure_templates');
INSERT INTO table_name_ancestors (descendant_table_name_id, ancestor_table_name_id)
    SELECT 'ball_mill_procedure_templates', ancestor_table_name_id FROM table_name_ancestors WHERE descendant_table_name_id = 'procedure_templates'
    UNION SELECT 'ball_mill_procedure_templates', 'ball_mill_procedure_templates';
CREATE TRIGGER ball_mill_procedure_templates_guard AFTER INSERT OR UPDATE ON ball_mill_procedure_templates
    FOR EACH ROW EXECUTE FUNCTION guard_procedure_template_subtype();
CREATE TRIGGER ball_mill_procedure_templates_delete_guard BEFORE DELETE ON ball_mill_procedure_templates
    FOR EACH ROW EXECUTE FUNCTION guard_procedure_template_subtype();

CREATE FUNCTION require_ball_mill_procedure_templates_row() RETURNS TRIGGER
LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
DECLARE
    row_id UUID := CASE TG_OP WHEN 'DELETE' THEN OLD.id ELSE NEW.id END;
BEGIN
    IF EXISTS (
        SELECT 1 FROM public.procedure_templates root
        JOIN public.table_name_ancestors path ON path.descendant_table_name_id = root.table_name_id
        WHERE root.id = row_id AND path.ancestor_table_name_id = 'ball_mill_procedure_templates'
    ) AND NOT EXISTS (SELECT 1 FROM public.ball_mill_procedure_templates row WHERE row.id = row_id) THEN
        RAISE EXCEPTION '% names a table extending % but has no row there', row_id, 'ball_mill_procedure_templates'
            USING ERRCODE = 'foreign_key_violation';
    END IF;
    RETURN NULL;
END;
$$;
ALTER FUNCTION require_ball_mill_procedure_templates_row() OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION require_ball_mill_procedure_templates_row() FROM PUBLIC;
CREATE CONSTRAINT TRIGGER ball_mill_procedure_templates_row AFTER INSERT ON procedure_templates
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW WHEN (NEW.table_name_id = 'ball_mill_procedure_templates')
    EXECUTE FUNCTION require_ball_mill_procedure_templates_row();
CREATE CONSTRAINT TRIGGER ball_mill_procedure_templates_kept AFTER DELETE ON ball_mill_procedure_templates
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION require_ball_mill_procedure_templates_row();

GRANT SELECT, INSERT, UPDATE (temperature_k, temperature_tolerance_percentage, duration_s, duration_tolerance_percentage, frequency_hz, frequency_hz_tolerance_percentage, bead_count), DELETE ON ball_mill_procedure_templates TO app;
ALTER TABLE ball_mill_procedure_templates ENABLE ROW LEVEL SECURITY;
ALTER TABLE ball_mill_procedure_templates FORCE ROW LEVEL SECURITY;
CREATE POLICY ball_mill_procedure_templates_select ON ball_mill_procedure_templates FOR SELECT TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY ball_mill_procedure_templates_insert ON ball_mill_procedure_templates FOR INSERT TO app
    WITH CHECK (id = ANY (ARRAY(SELECT authz.resources_with('writer'))) AND bead_model_id = ANY (ARRAY(SELECT authz.resources_with('reader'))) AND milled_with_model_id = ANY (ARRAY(SELECT authz.resources_with('reader'))) AND milled_container_model_id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY ball_mill_procedure_templates_update ON ball_mill_procedure_templates FOR UPDATE TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('writer'))))
    WITH CHECK (id = ANY (ARRAY(SELECT authz.resources_with('writer'))));
CREATE POLICY ball_mill_procedure_templates_delete ON ball_mill_procedure_templates FOR DELETE TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('admin'))));
CREATE POLICY ball_mill_procedure_templates_archived_insert ON ball_mill_procedure_templates AS RESTRICTIVE FOR INSERT TO app
    WITH CHECK (NOT authz.is_archived(id) AND NOT authz.is_archived(bead_model_id) AND NOT authz.is_archived(milled_with_model_id) AND NOT authz.is_archived(milled_container_model_id));
CREATE POLICY ball_mill_procedure_templates_archived_update ON ball_mill_procedure_templates AS RESTRICTIVE FOR UPDATE TO app
    USING (NOT authz.is_archived(id)) WITH CHECK (NOT authz.is_archived(id));
CREATE POLICY ball_mill_procedure_templates_archived_delete ON ball_mill_procedure_templates AS RESTRICTIVE FOR DELETE TO app
    USING (NOT authz.is_archived(id));
CREATE POLICY ball_mill_procedure_templates_blocked ON ball_mill_procedure_templates AS RESTRICTIVE FOR ALL TO app
    USING (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))))
    WITH CHECK (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))));

-- Recorded execution of a ball mill template.
CREATE TABLE ball_mill_procedures (
    -- Identity shared with the parent procedure.
    id UUID PRIMARY KEY REFERENCES procedures (id) ON DELETE CASCADE,
    -- Organization containing the parent resource.
    organization_id UUID NOT NULL,
    -- Typed template executed by this procedure.
    ball_mill_procedure_template_id UUID NOT NULL REFERENCES ball_mill_procedure_templates (id),
    -- Execution status copied from the parent procedure.
    status_id TEXT NOT NULL,
    -- Measured temperature k.
    temperature_k DOUBLE PRECISION CHECK (temperature_k > 0.0 AND temperature_k < 'Infinity'::double precision),
    CHECK (status_id <> 'completed' OR temperature_k IS NOT NULL),
    -- Measured frequency hz.
    frequency_hz DOUBLE PRECISION CHECK (frequency_hz >= 0.0 AND frequency_hz < 'Infinity'::double precision),
    CHECK (status_id <> 'completed' OR frequency_hz IS NOT NULL),
    FOREIGN KEY (id, ball_mill_procedure_template_id, status_id, organization_id)
        REFERENCES procedures (id, procedure_template_id, status_id, organization_id) ON UPDATE CASCADE ON DELETE CASCADE,
    FOREIGN KEY (organization_id, id) REFERENCES resources (organization_id, id) ON DELETE CASCADE
);
CREATE INDEX ON ball_mill_procedures (ball_mill_procedure_template_id);
INSERT INTO table_names (id) VALUES ('ball_mill_procedures');
INSERT INTO table_name_instances (id, instance_table_name_id)
    VALUES ('ball_mill_procedure_templates', 'ball_mill_procedures');
INSERT INTO table_name_ancestors (descendant_table_name_id, ancestor_table_name_id)
    SELECT 'ball_mill_procedures', ancestor_table_name_id FROM table_name_ancestors WHERE descendant_table_name_id = 'procedures'
    UNION SELECT 'ball_mill_procedures', 'ball_mill_procedures';
CREATE TRIGGER ball_mill_procedures_guard AFTER INSERT OR UPDATE ON ball_mill_procedures
    FOR EACH ROW EXECUTE FUNCTION guard_procedure_subtype();
CREATE TRIGGER ball_mill_procedures_delete_guard BEFORE DELETE ON ball_mill_procedures
    FOR EACH ROW EXECUTE FUNCTION guard_procedure_subtype();

CREATE FUNCTION require_ball_mill_procedures_row() RETURNS TRIGGER
LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
DECLARE
    row_id UUID := CASE TG_OP WHEN 'DELETE' THEN OLD.id ELSE NEW.id END;
BEGIN
    IF EXISTS (
        SELECT 1 FROM public.procedures root
        JOIN public.table_name_ancestors path ON path.descendant_table_name_id = root.table_name_id
        WHERE root.id = row_id AND path.ancestor_table_name_id = 'ball_mill_procedures'
    ) AND NOT EXISTS (SELECT 1 FROM public.ball_mill_procedures row WHERE row.id = row_id) THEN
        RAISE EXCEPTION '% names a table extending % but has no row there', row_id, 'ball_mill_procedures'
            USING ERRCODE = 'foreign_key_violation';
    END IF;
    RETURN NULL;
END;
$$;
ALTER FUNCTION require_ball_mill_procedures_row() OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION require_ball_mill_procedures_row() FROM PUBLIC;
CREATE CONSTRAINT TRIGGER ball_mill_procedures_row AFTER INSERT ON procedures
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW WHEN (NEW.table_name_id = 'ball_mill_procedures')
    EXECUTE FUNCTION require_ball_mill_procedures_row();
CREATE CONSTRAINT TRIGGER ball_mill_procedures_kept AFTER DELETE ON ball_mill_procedures
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION require_ball_mill_procedures_row();

GRANT SELECT, INSERT, UPDATE (temperature_k, frequency_hz), DELETE ON ball_mill_procedures TO app;
ALTER TABLE ball_mill_procedures ENABLE ROW LEVEL SECURITY;
ALTER TABLE ball_mill_procedures FORCE ROW LEVEL SECURITY;
CREATE POLICY ball_mill_procedures_select ON ball_mill_procedures FOR SELECT TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY ball_mill_procedures_insert ON ball_mill_procedures FOR INSERT TO app
    WITH CHECK (id = ANY (ARRAY(SELECT authz.resources_with('writer'))) AND ball_mill_procedure_template_id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY ball_mill_procedures_delete ON ball_mill_procedures FOR DELETE TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('admin'))));
CREATE POLICY ball_mill_procedures_archived_insert ON ball_mill_procedures AS RESTRICTIVE FOR INSERT TO app
    WITH CHECK (NOT authz.is_archived(id) AND NOT authz.is_archived(ball_mill_procedure_template_id));
CREATE POLICY ball_mill_procedures_archived_delete ON ball_mill_procedures AS RESTRICTIVE FOR DELETE TO app
    USING (NOT authz.is_archived(id));
CREATE POLICY ball_mill_procedures_blocked ON ball_mill_procedures AS RESTRICTIVE FOR ALL TO app
    USING (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))))
    WITH CHECK (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))));

-- Records every change in connetto's `audit_events`.
CREATE TRIGGER ball_mill_procedure_templates_audit AFTER INSERT OR DELETE ON ball_mill_procedure_templates
    FOR EACH ROW EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER ball_mill_procedure_templates_update_audit AFTER UPDATE ON ball_mill_procedure_templates
    FOR EACH ROW WHEN (OLD IS DISTINCT FROM NEW) EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER ball_mill_procedures_audit AFTER INSERT OR DELETE ON ball_mill_procedures
    FOR EACH ROW EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER ball_mill_procedures_update_audit AFTER UPDATE ON ball_mill_procedures
    FOR EACH ROW WHEN (OLD IS DISTINCT FROM NEW) EXECUTE FUNCTION authz_write.record_audit_event();
CREATE POLICY ball_mill_procedures_update ON ball_mill_procedures FOR UPDATE TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('writer'))))
    WITH CHECK (id = ANY (ARRAY(SELECT authz.resources_with('writer'))));
CREATE POLICY ball_mill_procedures_archived_update ON ball_mill_procedures AS RESTRICTIVE FOR UPDATE TO app
    USING (NOT authz.is_archived(id)) WITH CHECK (NOT authz.is_archived(id));
