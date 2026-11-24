-- Migration: 010-fractioning-procedures
-- Purpose: Typed fractioning slots and execution records.
-- APS placement
-- ownables
-- +-- procedure_templates -- fractioning_procedure_templates
-- +-- procedures -- fractioning_procedures
-- Slot | Role | Meaning
-- `weighed_with` | `instrument` | The balance measures the fraction.
-- `fragment_container` | `input` | The source retains the remaining material.
-- `fragment_placed_into` | `input` | The destination container survives transfer.
-- `fraction` | `produced` | The procedure creates the sample.
-- `source_sample` | `input` | The sample supplies the transferred material.

-- Typed slots and parameters for fractioning.
CREATE TABLE fractioning_procedure_templates (
    -- Identity shared with the parent template.
    id UUID PRIMARY KEY REFERENCES procedure_templates (id) ON DELETE CASCADE,
    -- Organization containing the parent resource.
    organization_id UUID NOT NULL,
    -- Slot declaring the source sample.
    source_sample_slot_id UUID NOT NULL,
    -- Admissible model for the source sample.
    source_sample_model_id UUID NOT NULL REFERENCES sample_models (id),
    -- Required role of the source sample.
    source_sample_slot_role_id TEXT GENERATED ALWAYS AS ('input') STORED NOT NULL,
    -- Slot declaring the `weighed_with` assets.
    weighed_with_slot_id UUID NOT NULL,
    -- Admissible model for the `weighed_with` slot.
    weighed_with_model_id UUID NOT NULL REFERENCES weighing_device_models (id),
    -- Required role of the `weighed_with` slot.
    weighed_with_slot_role_id TEXT GENERATED ALWAYS AS ('instrument') STORED NOT NULL,
    -- Slot declaring the `fragment_container` assets.
    fragment_container_slot_id UUID NOT NULL,
    -- Admissible model for the `fragment_container` slot.
    fragment_container_model_id UUID NOT NULL REFERENCES volumetric_container_models (id),
    -- Required role of the `fragment_container` slot.
    fragment_container_slot_role_id TEXT GENERATED ALWAYS AS ('input') STORED NOT NULL,
    -- Slot declaring the `fragment_placed_into` assets.
    fragment_placed_into_slot_id UUID NOT NULL,
    -- Admissible model for the `fragment_placed_into` slot.
    fragment_placed_into_model_id UUID NOT NULL REFERENCES volumetric_container_models (id),
    -- Required role of the `fragment_placed_into` slot.
    fragment_placed_into_slot_role_id TEXT GENERATED ALWAYS AS ('input') STORED NOT NULL,
    -- Slot declaring the produced `fraction` sample.
    fraction_slot_id UUID NOT NULL,
    -- Admissible model for the `fraction` sample.
    fraction_model_id UUID NOT NULL REFERENCES sample_models (id),
    -- Required role of the `fraction` slot.
    fraction_slot_role_id TEXT GENERATED ALWAYS AS ('produced') STORED NOT NULL,
    -- Mass in kilograms.
    mass_kg DOUBLE PRECISION NOT NULL CHECK (mass_kg > 0.0 AND mass_kg < 'Infinity'::double precision),
    -- Allowed mass deviation as a percentage of the target.
    tolerance_percentage DOUBLE PRECISION NOT NULL CHECK ( tolerance_percentage > 0.0 AND tolerance_percentage <= 100.0 ),
    CHECK (fragment_container_slot_id <> fragment_placed_into_slot_id),
    FOREIGN KEY (source_sample_slot_id, id, source_sample_model_id, source_sample_slot_role_id, organization_id)
        REFERENCES procedure_template_slots (id, procedure_template_id, asset_model_id, role_id, organization_id),
    FOREIGN KEY (weighed_with_slot_id, id, weighed_with_model_id, weighed_with_slot_role_id, organization_id)
        REFERENCES procedure_template_slots (id, procedure_template_id, asset_model_id, role_id, organization_id),
    FOREIGN KEY (fragment_container_slot_id, id, fragment_container_model_id, fragment_container_slot_role_id, organization_id)
        REFERENCES procedure_template_slots (id, procedure_template_id, asset_model_id, role_id, organization_id),
    FOREIGN KEY (fragment_placed_into_slot_id, id, fragment_placed_into_model_id, fragment_placed_into_slot_role_id, organization_id)
        REFERENCES procedure_template_slots (id, procedure_template_id, asset_model_id, role_id, organization_id),
    FOREIGN KEY (fraction_slot_id, id, fraction_model_id, fraction_slot_role_id, organization_id)
        REFERENCES procedure_template_slots (id, procedure_template_id, asset_model_id, role_id, organization_id),
    FOREIGN KEY (fraction_slot_id, id, fragment_placed_into_slot_id, organization_id)
        REFERENCES procedure_template_slots (id, procedure_template_id, placed_into_slot_id, organization_id),
    FOREIGN KEY (organization_id, id) REFERENCES resources (organization_id, id) ON DELETE CASCADE
);
CREATE INDEX ON fractioning_procedure_templates (fraction_model_id);
CREATE INDEX ON fractioning_procedure_templates (fragment_container_model_id);
CREATE INDEX ON fractioning_procedure_templates (fragment_placed_into_model_id);
CREATE INDEX ON fractioning_procedure_templates (weighed_with_model_id);
INSERT INTO table_names (id) VALUES ('fractioning_procedure_templates');
INSERT INTO table_name_ancestors (descendant_table_name_id, ancestor_table_name_id)
    SELECT 'fractioning_procedure_templates', ancestor_table_name_id FROM table_name_ancestors WHERE descendant_table_name_id = 'procedure_templates'
    UNION SELECT 'fractioning_procedure_templates', 'fractioning_procedure_templates';
CREATE TRIGGER fractioning_procedure_templates_guard AFTER INSERT OR UPDATE ON fractioning_procedure_templates
    FOR EACH ROW EXECUTE FUNCTION guard_procedure_template_subtype();
CREATE TRIGGER fractioning_procedure_templates_delete_guard BEFORE DELETE ON fractioning_procedure_templates
    FOR EACH ROW EXECUTE FUNCTION guard_procedure_template_subtype();

CREATE FUNCTION require_fractioning_procedure_templates_row() RETURNS TRIGGER
LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
DECLARE
    row_id UUID := CASE TG_OP WHEN 'DELETE' THEN OLD.id ELSE NEW.id END;
BEGIN
    IF EXISTS (
        SELECT 1 FROM public.procedure_templates root
        JOIN public.table_name_ancestors path ON path.descendant_table_name_id = root.table_name_id
        WHERE root.id = row_id AND path.ancestor_table_name_id = 'fractioning_procedure_templates'
    ) AND NOT EXISTS (SELECT 1 FROM public.fractioning_procedure_templates row WHERE row.id = row_id) THEN
        RAISE EXCEPTION '% names a table extending % but has no row there', row_id, 'fractioning_procedure_templates'
            USING ERRCODE = 'foreign_key_violation';
    END IF;
    RETURN NULL;
END;
$$;
ALTER FUNCTION require_fractioning_procedure_templates_row() OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION require_fractioning_procedure_templates_row() FROM PUBLIC;
CREATE CONSTRAINT TRIGGER fractioning_procedure_templates_row AFTER INSERT ON procedure_templates
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW WHEN (NEW.table_name_id = 'fractioning_procedure_templates')
    EXECUTE FUNCTION require_fractioning_procedure_templates_row();
CREATE CONSTRAINT TRIGGER fractioning_procedure_templates_kept AFTER DELETE ON fractioning_procedure_templates
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION require_fractioning_procedure_templates_row();

GRANT SELECT, INSERT, UPDATE (mass_kg, tolerance_percentage), DELETE ON fractioning_procedure_templates TO app;
ALTER TABLE fractioning_procedure_templates ENABLE ROW LEVEL SECURITY;
ALTER TABLE fractioning_procedure_templates FORCE ROW LEVEL SECURITY;
CREATE POLICY fractioning_procedure_templates_select ON fractioning_procedure_templates FOR SELECT TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY fractioning_procedure_templates_insert ON fractioning_procedure_templates FOR INSERT TO app
    WITH CHECK (id = ANY (ARRAY(SELECT authz.resources_with('writer'))) AND weighed_with_model_id = ANY (ARRAY(SELECT authz.resources_with('reader'))) AND fragment_container_model_id = ANY (ARRAY(SELECT authz.resources_with('reader'))) AND fragment_placed_into_model_id = ANY (ARRAY(SELECT authz.resources_with('reader'))) AND fraction_model_id = ANY (ARRAY(SELECT authz.resources_with('reader'))) AND source_sample_model_id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY fractioning_procedure_templates_update ON fractioning_procedure_templates FOR UPDATE TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('writer'))))
    WITH CHECK (id = ANY (ARRAY(SELECT authz.resources_with('writer'))));
CREATE POLICY fractioning_procedure_templates_delete ON fractioning_procedure_templates FOR DELETE TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('admin'))));
CREATE POLICY fractioning_procedure_templates_archived_insert ON fractioning_procedure_templates AS RESTRICTIVE FOR INSERT TO app
    WITH CHECK (NOT authz.is_archived(id) AND NOT authz.is_archived(weighed_with_model_id) AND NOT authz.is_archived(fragment_container_model_id) AND NOT authz.is_archived(fragment_placed_into_model_id) AND NOT authz.is_archived(fraction_model_id) AND NOT authz.is_archived(source_sample_model_id));
CREATE POLICY fractioning_procedure_templates_archived_update ON fractioning_procedure_templates AS RESTRICTIVE FOR UPDATE TO app
    USING (NOT authz.is_archived(id)) WITH CHECK (NOT authz.is_archived(id));
CREATE POLICY fractioning_procedure_templates_archived_delete ON fractioning_procedure_templates AS RESTRICTIVE FOR DELETE TO app
    USING (NOT authz.is_archived(id));
CREATE POLICY fractioning_procedure_templates_blocked ON fractioning_procedure_templates AS RESTRICTIVE FOR ALL TO app
    USING (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))))
    WITH CHECK (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))));

-- Recorded execution of a fractioning template.
CREATE TABLE fractioning_procedures (
    -- Identity shared with the parent procedure.
    id UUID PRIMARY KEY REFERENCES procedures (id) ON DELETE CASCADE,
    -- Organization containing the parent resource.
    organization_id UUID NOT NULL,
    -- Typed template executed by this procedure.
    fractioning_procedure_template_id UUID NOT NULL REFERENCES fractioning_procedure_templates (id),
    -- Execution status copied from the parent procedure.
    status_id TEXT NOT NULL,
    -- Mass in kilograms.
    mass_kg DOUBLE PRECISION CHECK (mass_kg > 0.0 AND mass_kg < 'Infinity'::double precision),
    FOREIGN KEY (id, fractioning_procedure_template_id, status_id, organization_id)
        REFERENCES procedures (id, procedure_template_id, status_id, organization_id) ON DELETE CASCADE
        DEFERRABLE INITIALLY DEFERRED,
    CHECK (status_id <> 'completed' OR mass_kg IS NOT NULL),
    FOREIGN KEY (organization_id, id) REFERENCES resources (organization_id, id) ON DELETE CASCADE
);
CREATE INDEX ON fractioning_procedures (fractioning_procedure_template_id);
INSERT INTO table_names (id) VALUES ('fractioning_procedures');
INSERT INTO table_name_instances (id, instance_table_name_id)
    VALUES ('fractioning_procedure_templates', 'fractioning_procedures');
INSERT INTO table_name_ancestors (descendant_table_name_id, ancestor_table_name_id)
    SELECT 'fractioning_procedures', ancestor_table_name_id FROM table_name_ancestors WHERE descendant_table_name_id = 'procedures'
    UNION SELECT 'fractioning_procedures', 'fractioning_procedures';
CREATE TRIGGER fractioning_procedures_guard AFTER INSERT OR UPDATE ON fractioning_procedures
    FOR EACH ROW EXECUTE FUNCTION guard_procedure_subtype();
CREATE TRIGGER fractioning_procedures_delete_guard BEFORE DELETE ON fractioning_procedures
    FOR EACH ROW EXECUTE FUNCTION guard_procedure_subtype();

-- Writes a procedure's new status into its `fractioning_procedures` row, whose deferred
-- foreign key holds it equal to the procedure at commit.
CREATE FUNCTION copy_fractioning_procedures_status() RETURNS TRIGGER
LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
BEGIN
    UPDATE public.fractioning_procedures SET status_id = NEW.status_id WHERE id = NEW.id;
    RETURN NULL;
END;
$$;
GRANT UPDATE (status_id) ON fractioning_procedures TO aps_writer;
ALTER FUNCTION copy_fractioning_procedures_status() OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION copy_fractioning_procedures_status() FROM PUBLIC;
CREATE TRIGGER procedures_status_copy_fractioning_procedures AFTER UPDATE OF status_id ON procedures
    FOR EACH ROW WHEN (OLD.status_id IS DISTINCT FROM NEW.status_id) EXECUTE FUNCTION copy_fractioning_procedures_status();

CREATE FUNCTION require_fractioning_procedures_row() RETURNS TRIGGER
LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
DECLARE
    row_id UUID := CASE TG_OP WHEN 'DELETE' THEN OLD.id ELSE NEW.id END;
BEGIN
    IF EXISTS (
        SELECT 1 FROM public.procedures root
        JOIN public.table_name_ancestors path ON path.descendant_table_name_id = root.table_name_id
        WHERE root.id = row_id AND path.ancestor_table_name_id = 'fractioning_procedures'
    ) AND NOT EXISTS (SELECT 1 FROM public.fractioning_procedures row WHERE row.id = row_id) THEN
        RAISE EXCEPTION '% names a table extending % but has no row there', row_id, 'fractioning_procedures'
            USING ERRCODE = 'foreign_key_violation';
    END IF;
    RETURN NULL;
END;
$$;
ALTER FUNCTION require_fractioning_procedures_row() OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION require_fractioning_procedures_row() FROM PUBLIC;
CREATE CONSTRAINT TRIGGER fractioning_procedures_row AFTER INSERT ON procedures
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW WHEN (NEW.table_name_id = 'fractioning_procedures')
    EXECUTE FUNCTION require_fractioning_procedures_row();
CREATE CONSTRAINT TRIGGER fractioning_procedures_kept AFTER DELETE ON fractioning_procedures
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION require_fractioning_procedures_row();

GRANT SELECT, INSERT, UPDATE (mass_kg), DELETE ON fractioning_procedures TO app;
ALTER TABLE fractioning_procedures ENABLE ROW LEVEL SECURITY;
ALTER TABLE fractioning_procedures FORCE ROW LEVEL SECURITY;
CREATE POLICY fractioning_procedures_select ON fractioning_procedures FOR SELECT TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY fractioning_procedures_insert ON fractioning_procedures FOR INSERT TO app
    WITH CHECK (id = ANY (ARRAY(SELECT authz.resources_with('writer'))) AND fractioning_procedure_template_id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY fractioning_procedures_update ON fractioning_procedures FOR UPDATE TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('writer'))))
    WITH CHECK (id = ANY (ARRAY(SELECT authz.resources_with('writer'))));
CREATE POLICY fractioning_procedures_delete ON fractioning_procedures FOR DELETE TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('admin'))));
CREATE POLICY fractioning_procedures_archived_insert ON fractioning_procedures AS RESTRICTIVE FOR INSERT TO app
    WITH CHECK (NOT authz.is_archived(id) AND NOT authz.is_archived(fractioning_procedure_template_id));
CREATE POLICY fractioning_procedures_archived_update ON fractioning_procedures AS RESTRICTIVE FOR UPDATE TO app
    USING (NOT authz.is_archived(id)) WITH CHECK (NOT authz.is_archived(id));
CREATE POLICY fractioning_procedures_archived_delete ON fractioning_procedures AS RESTRICTIVE FOR DELETE TO app
    USING (NOT authz.is_archived(id));
CREATE POLICY fractioning_procedures_blocked ON fractioning_procedures AS RESTRICTIVE FOR ALL TO app
    USING (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))))
    WITH CHECK (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))));

-- Records every change in connetto's `audit_events`.
CREATE TRIGGER fractioning_procedure_templates_audit AFTER INSERT OR DELETE ON fractioning_procedure_templates
    FOR EACH ROW EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER fractioning_procedure_templates_update_audit AFTER UPDATE ON fractioning_procedure_templates
    FOR EACH ROW WHEN (OLD IS DISTINCT FROM NEW) EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER fractioning_procedures_audit AFTER INSERT OR DELETE ON fractioning_procedures
    FOR EACH ROW EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER fractioning_procedures_update_audit AFTER UPDATE ON fractioning_procedures
    FOR EACH ROW WHEN (OLD IS DISTINCT FROM NEW) EXECUTE FUNCTION authz_write.record_audit_event();

CREATE INDEX ON public.fractioning_procedure_templates (source_sample_model_id);
CREATE INDEX ON public.fractioning_procedure_templates (source_sample_slot_id);

-- Checks the typed slot contract after definition edits.
CREATE FUNCTION public.check_fractioning_slots() RETURNS TRIGGER
LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
DECLARE
    fragment_placed_into_slot UUID;
    fraction_slot UUID;
BEGIN
    SELECT template.fragment_placed_into_slot_id, template.fraction_slot_id
    INTO fragment_placed_into_slot, fraction_slot
    FROM public.fractioning_procedure_templates template
    WHERE template.id = NEW.id;
    PERFORM public.require_single_typed_slot(fragment_placed_into_slot);
    PERFORM public.require_single_typed_slot(fraction_slot);
    RETURN NULL;
END;
$$;
ALTER FUNCTION public.check_fractioning_slots() OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION public.check_fractioning_slots() FROM PUBLIC;

-- Checks the fractioning slot contract after cardinality edits.
CREATE FUNCTION public.check_fractioning_slot_cardinality() RETURNS TRIGGER
LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
DECLARE
    fragment_placed_into_slot UUID;
    fraction_slot UUID;
BEGIN
    SELECT template.fragment_placed_into_slot_id, template.fraction_slot_id
    INTO fragment_placed_into_slot, fraction_slot
    FROM public.fractioning_procedure_templates template
    WHERE template.id = NEW.procedure_template_id
      AND (template.fragment_placed_into_slot_id = NEW.id OR template.fraction_slot_id = NEW.id);
    PERFORM public.require_single_typed_slot(fragment_placed_into_slot);
    PERFORM public.require_single_typed_slot(fraction_slot);
    RETURN NULL;
END;
$$;
ALTER FUNCTION public.check_fractioning_slot_cardinality() OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION public.check_fractioning_slot_cardinality() FROM PUBLIC;
GRANT SELECT ON public.fractioning_procedure_templates TO aps_writer;
CREATE CONSTRAINT TRIGGER fractioning_template_slots AFTER INSERT OR UPDATE ON public.fractioning_procedure_templates
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION public.check_fractioning_slots();
CREATE CONSTRAINT TRIGGER fractioning_slot_cardinality AFTER UPDATE ON public.procedure_template_slots
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW
    WHEN (OLD.minimum_count IS DISTINCT FROM NEW.minimum_count OR OLD.maximum_count IS DISTINCT FROM NEW.maximum_count)
    EXECUTE FUNCTION public.check_fractioning_slot_cardinality();
