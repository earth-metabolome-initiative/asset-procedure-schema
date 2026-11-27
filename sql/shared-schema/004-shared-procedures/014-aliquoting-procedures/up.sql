-- Migration: 014-aliquoting-procedures
-- Purpose: Typed aliquoting slots and execution records.
-- APS placement
-- `resources` of kind `'ownable'`
-- +-- procedure_templates -- aliquoting_procedure_templates
-- +-- procedures -- aliquoting_procedures
-- Slot | Role | Meaning
-- `aliquoted_from` | `input` | The source retains the remaining material.
-- `aliquoted_into` | `input` | The destination container survives transfer.
-- `aliquot` | `produced` | The procedure creates the sample.
-- `aliquoted_with` | `instrument` | The device measures and transfers the aliquot.
-- `source_sample` | `input` | The sample supplies the transferred material.

-- Typed slots and parameters for aliquoting.
CREATE TABLE aliquoting_procedure_templates (
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
    -- Slot declaring the `aliquoted_from` assets.
    aliquoted_from_slot_id UUID NOT NULL,
    -- Admissible model for the `aliquoted_from` slot.
    aliquoted_from_model_id UUID NOT NULL REFERENCES volumetric_container_models (id),
    -- Required role of the `aliquoted_from` slot.
    aliquoted_from_slot_role_id TEXT GENERATED ALWAYS AS ('input') STORED NOT NULL,
    -- Slot declaring the `aliquoted_into` assets.
    aliquoted_into_slot_id UUID NOT NULL,
    -- Admissible model for the `aliquoted_into` slot.
    aliquoted_into_model_id UUID NOT NULL REFERENCES volumetric_container_models (id),
    -- Required role of the `aliquoted_into` slot.
    aliquoted_into_slot_role_id TEXT GENERATED ALWAYS AS ('input') STORED NOT NULL,
    -- Slot declaring the produced `aliquot` sample.
    aliquot_slot_id UUID NOT NULL,
    -- Admissible model for the `aliquot` sample.
    aliquot_model_id UUID NOT NULL REFERENCES sample_models (id),
    -- Required role of the `aliquot` slot.
    aliquot_slot_role_id TEXT GENERATED ALWAYS AS ('produced') STORED NOT NULL,
    -- Slot declaring the `aliquoted_with` assets.
    aliquoted_with_slot_id UUID NOT NULL,
    -- Admissible model for the `aliquoted_with` slot.
    aliquoted_with_model_id UUID NOT NULL REFERENCES volume_measuring_device_models (id),
    -- Required role of the `aliquoted_with` slot.
    aliquoted_with_slot_role_id TEXT GENERATED ALWAYS AS ('instrument') STORED NOT NULL,
    -- Volume in liters.
    volume_l DOUBLE PRECISION NOT NULL CHECK (volume_l > 0.0 AND volume_l < 'Infinity'::double precision),
    -- Allowed volume deviation as a percentage of the target.
    volume_l_tolerance_percentage DOUBLE PRECISION NOT NULL DEFAULT 5.0
        CHECK (volume_l_tolerance_percentage > 0.0 AND volume_l_tolerance_percentage <= 100.0),
    CHECK (aliquoted_from_slot_id <> aliquoted_into_slot_id),
    FOREIGN KEY (source_sample_slot_id, id, source_sample_model_id, source_sample_slot_role_id, organization_id)
        REFERENCES procedure_template_slots (id, procedure_template_id, asset_model_id, role_id, organization_id),
    FOREIGN KEY (aliquoted_from_slot_id, id, aliquoted_from_model_id, aliquoted_from_slot_role_id, organization_id)
        REFERENCES procedure_template_slots (id, procedure_template_id, asset_model_id, role_id, organization_id),
    FOREIGN KEY (aliquoted_into_slot_id, id, aliquoted_into_model_id, aliquoted_into_slot_role_id, organization_id)
        REFERENCES procedure_template_slots (id, procedure_template_id, asset_model_id, role_id, organization_id),
    FOREIGN KEY (aliquoted_with_slot_id, id, aliquoted_with_model_id, aliquoted_with_slot_role_id, organization_id)
        REFERENCES procedure_template_slots (id, procedure_template_id, asset_model_id, role_id, organization_id),
    FOREIGN KEY (aliquot_slot_id, id, aliquot_model_id, aliquot_slot_role_id, organization_id)
        REFERENCES procedure_template_slots (id, procedure_template_id, asset_model_id, role_id, organization_id),
    FOREIGN KEY (aliquot_slot_id, id, aliquoted_into_slot_id, organization_id)
        REFERENCES procedure_template_slots (id, procedure_template_id, placed_into_slot_id, organization_id),
    -- Archive state inherited from this resource.
    archived BOOLEAN NOT NULL DEFAULT FALSE,
    -- Archive state of the row named by `aliquoted_from_model_id`.
    aliquoted_from_model_archived BOOLEAN NOT NULL DEFAULT FALSE,
    -- Archive state of the row named by `aliquoted_into_model_id`.
    aliquoted_into_model_archived BOOLEAN NOT NULL DEFAULT FALSE,
    -- Archive state of the row named by `aliquot_model_id`.
    aliquot_model_archived BOOLEAN NOT NULL DEFAULT FALSE,
    -- Archive state of the row named by `aliquoted_with_model_id`.
    aliquoted_with_model_archived BOOLEAN NOT NULL DEFAULT FALSE,
    -- Archive state of the row named by `source_sample_model_id`.
    source_sample_model_archived BOOLEAN NOT NULL DEFAULT FALSE,
    UNIQUE (id, archived),
    FOREIGN KEY (aliquoted_from_model_id, aliquoted_from_model_archived) REFERENCES volumetric_container_models (id, archived) ON UPDATE CASCADE,
    FOREIGN KEY (aliquoted_into_model_id, aliquoted_into_model_archived) REFERENCES volumetric_container_models (id, archived) ON UPDATE CASCADE,
    FOREIGN KEY (aliquot_model_id, aliquot_model_archived) REFERENCES sample_models (id, archived) ON UPDATE CASCADE,
    FOREIGN KEY (aliquoted_with_model_id, aliquoted_with_model_archived) REFERENCES volume_measuring_device_models (id, archived) ON UPDATE CASCADE,
    FOREIGN KEY (source_sample_model_id, source_sample_model_archived) REFERENCES sample_models (id, archived) ON UPDATE CASCADE,
    FOREIGN KEY (organization_id, id, archived) REFERENCES resources (organization_id, id, archived) ON UPDATE CASCADE ON DELETE CASCADE
);
CREATE INDEX ON aliquoting_procedure_templates (aliquot_model_id);
CREATE INDEX ON aliquoting_procedure_templates (aliquoted_from_model_id);
CREATE INDEX ON aliquoting_procedure_templates (aliquoted_into_model_id);
CREATE INDEX ON aliquoting_procedure_templates (aliquoted_with_model_id);
INSERT INTO table_names (id) VALUES ('aliquoting_procedure_templates');
INSERT INTO table_name_ancestors (descendant_table_name_id, ancestor_table_name_id)
    SELECT 'aliquoting_procedure_templates', ancestor_table_name_id FROM table_name_ancestors WHERE descendant_table_name_id = 'procedure_templates'
    UNION SELECT 'aliquoting_procedure_templates', 'aliquoting_procedure_templates';
CREATE TRIGGER aliquoting_procedure_templates_guard AFTER INSERT ON aliquoting_procedure_templates
    FOR EACH ROW EXECUTE FUNCTION guard_procedure_template_subtype();
CREATE TRIGGER aliquoting_procedure_templates_guard_update AFTER UPDATE OF id, source_sample_slot_id, source_sample_model_id, source_sample_slot_role_id, aliquoted_from_slot_id, aliquoted_from_model_id, aliquoted_from_slot_role_id, aliquoted_into_slot_id, aliquoted_into_model_id, aliquoted_into_slot_role_id, aliquot_slot_id, aliquot_model_id, aliquot_slot_role_id, aliquoted_with_slot_id, aliquoted_with_model_id, aliquoted_with_slot_role_id, volume_l, volume_l_tolerance_percentage ON aliquoting_procedure_templates
    FOR EACH ROW WHEN (OLD.id IS DISTINCT FROM NEW.id
        OR OLD.source_sample_slot_id IS DISTINCT FROM NEW.source_sample_slot_id
        OR OLD.source_sample_model_id IS DISTINCT FROM NEW.source_sample_model_id
        OR OLD.source_sample_slot_role_id IS DISTINCT FROM NEW.source_sample_slot_role_id
        OR OLD.aliquoted_from_slot_id IS DISTINCT FROM NEW.aliquoted_from_slot_id
        OR OLD.aliquoted_from_model_id IS DISTINCT FROM NEW.aliquoted_from_model_id
        OR OLD.aliquoted_from_slot_role_id IS DISTINCT FROM NEW.aliquoted_from_slot_role_id
        OR OLD.aliquoted_into_slot_id IS DISTINCT FROM NEW.aliquoted_into_slot_id
        OR OLD.aliquoted_into_model_id IS DISTINCT FROM NEW.aliquoted_into_model_id
        OR OLD.aliquoted_into_slot_role_id IS DISTINCT FROM NEW.aliquoted_into_slot_role_id
        OR OLD.aliquot_slot_id IS DISTINCT FROM NEW.aliquot_slot_id
        OR OLD.aliquot_model_id IS DISTINCT FROM NEW.aliquot_model_id
        OR OLD.aliquot_slot_role_id IS DISTINCT FROM NEW.aliquot_slot_role_id
        OR OLD.aliquoted_with_slot_id IS DISTINCT FROM NEW.aliquoted_with_slot_id
        OR OLD.aliquoted_with_model_id IS DISTINCT FROM NEW.aliquoted_with_model_id
        OR OLD.aliquoted_with_slot_role_id IS DISTINCT FROM NEW.aliquoted_with_slot_role_id
        OR OLD.volume_l IS DISTINCT FROM NEW.volume_l
        OR OLD.volume_l_tolerance_percentage IS DISTINCT FROM NEW.volume_l_tolerance_percentage) EXECUTE FUNCTION guard_procedure_template_subtype();
CREATE TRIGGER aliquoting_procedure_templates_delete_guard BEFORE DELETE ON aliquoting_procedure_templates
    FOR EACH ROW EXECUTE FUNCTION guard_procedure_template_subtype();

CREATE FUNCTION require_aliquoting_procedure_templates_row() RETURNS TRIGGER
LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
DECLARE
    row_id UUID := CASE TG_OP WHEN 'DELETE' THEN OLD.id ELSE NEW.id END;
BEGIN
    IF EXISTS (
        SELECT 1 FROM public.procedure_templates root
        JOIN public.table_name_ancestors path ON path.descendant_table_name_id = root.table_name_id
        WHERE root.id = row_id AND path.ancestor_table_name_id = 'aliquoting_procedure_templates'
    ) AND NOT EXISTS (SELECT 1 FROM public.aliquoting_procedure_templates row WHERE row.id = row_id) THEN
        RAISE EXCEPTION '% names a table extending % but has no row there', row_id, 'aliquoting_procedure_templates'
            USING ERRCODE = 'foreign_key_violation';
    END IF;
    RETURN NULL;
END;
$$;
ALTER FUNCTION require_aliquoting_procedure_templates_row() OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION require_aliquoting_procedure_templates_row() FROM PUBLIC;
CREATE CONSTRAINT TRIGGER aliquoting_procedure_templates_row AFTER INSERT ON procedure_templates
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW WHEN (NEW.table_name_id = 'aliquoting_procedure_templates')
    EXECUTE FUNCTION require_aliquoting_procedure_templates_row();
CREATE CONSTRAINT TRIGGER aliquoting_procedure_templates_kept AFTER DELETE ON aliquoting_procedure_templates
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION require_aliquoting_procedure_templates_row();

REVOKE ALL ON aliquoting_procedure_templates FROM PUBLIC, app, identity_writer;
GRANT SELECT, INSERT, UPDATE (volume_l, volume_l_tolerance_percentage), DELETE ON aliquoting_procedure_templates TO app;
ALTER TABLE aliquoting_procedure_templates ENABLE ROW LEVEL SECURITY;
ALTER TABLE aliquoting_procedure_templates FORCE ROW LEVEL SECURITY;
CREATE POLICY aliquoting_procedure_templates_select ON aliquoting_procedure_templates FOR SELECT TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY aliquoting_procedure_templates_insert ON aliquoting_procedure_templates FOR INSERT TO app
    WITH CHECK (id = ANY (ARRAY(SELECT authz.resources_with('writer'))) AND aliquoted_from_model_id = ANY (ARRAY(SELECT authz.resources_with('reader'))) AND aliquoted_into_model_id = ANY (ARRAY(SELECT authz.resources_with('reader'))) AND aliquot_model_id = ANY (ARRAY(SELECT authz.resources_with('reader'))) AND aliquoted_with_model_id = ANY (ARRAY(SELECT authz.resources_with('reader'))) AND source_sample_model_id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY aliquoting_procedure_templates_update ON aliquoting_procedure_templates FOR UPDATE TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('writer'))) AND aliquoted_into_model_id = ANY (ARRAY(SELECT authz.resources_with('reader'))))
    WITH CHECK (id = ANY (ARRAY(SELECT authz.resources_with('writer'))));
CREATE POLICY aliquoting_procedure_templates_delete ON aliquoting_procedure_templates FOR DELETE TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('admin'))));
CREATE POLICY aliquoting_procedure_templates_archived_insert ON aliquoting_procedure_templates AS RESTRICTIVE FOR INSERT TO app
    WITH CHECK (NOT archived AND NOT aliquoted_from_model_archived AND NOT aliquoted_into_model_archived AND NOT aliquot_model_archived AND NOT aliquoted_with_model_archived AND NOT source_sample_model_archived);
CREATE POLICY aliquoting_procedure_templates_archived_update ON aliquoting_procedure_templates AS RESTRICTIVE FOR UPDATE TO app
    USING (NOT archived) WITH CHECK (NOT archived);
CREATE POLICY aliquoting_procedure_templates_archived_delete ON aliquoting_procedure_templates AS RESTRICTIVE FOR DELETE TO app
    USING (NOT archived);
CREATE POLICY aliquoting_procedure_templates_blocked ON aliquoting_procedure_templates AS RESTRICTIVE FOR ALL TO app
    USING (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))))
    WITH CHECK (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))));

-- Recorded execution of a aliquoting template.
CREATE TABLE aliquoting_procedures (
    -- Identity shared with the parent procedure.
    id UUID PRIMARY KEY REFERENCES procedures (id) ON DELETE CASCADE,
    -- Organization containing the parent resource.
    organization_id UUID NOT NULL,
    -- Typed template executed by this procedure.
    aliquoting_procedure_template_id UUID NOT NULL REFERENCES aliquoting_procedure_templates (id),
    -- Execution status copied from the parent procedure.
    status_id TEXT NOT NULL DEFAULT 'planned',
    -- Measured volume l.
    volume_l DOUBLE PRECISION CHECK (volume_l > 0.0 AND volume_l < 'Infinity'::double precision),
    CHECK (status_id <> 'completed' OR volume_l IS NOT NULL),
    FOREIGN KEY (id, aliquoting_procedure_template_id, status_id, organization_id)
        REFERENCES procedures (id, procedure_template_id, status_id, organization_id) ON DELETE CASCADE
        DEFERRABLE INITIALLY DEFERRED,
    -- Archive state inherited from this resource.
    archived BOOLEAN NOT NULL DEFAULT FALSE,
    -- Archive state of the row named by `aliquoting_procedure_template_id`.
    aliquoting_procedure_template_archived BOOLEAN NOT NULL DEFAULT FALSE,
    FOREIGN KEY (aliquoting_procedure_template_id, aliquoting_procedure_template_archived) REFERENCES aliquoting_procedure_templates (id, archived) ON UPDATE CASCADE,
    FOREIGN KEY (organization_id, id, archived) REFERENCES resources (organization_id, id, archived) ON UPDATE CASCADE ON DELETE CASCADE
);
CREATE INDEX ON aliquoting_procedures (aliquoting_procedure_template_id);
INSERT INTO table_names (id) VALUES ('aliquoting_procedures');
INSERT INTO table_name_instances (id, instance_table_name_id)
    VALUES ('aliquoting_procedure_templates', 'aliquoting_procedures');
INSERT INTO table_name_ancestors (descendant_table_name_id, ancestor_table_name_id)
    SELECT 'aliquoting_procedures', ancestor_table_name_id FROM table_name_ancestors WHERE descendant_table_name_id = 'procedures'
    UNION SELECT 'aliquoting_procedures', 'aliquoting_procedures';
CREATE TRIGGER aliquoting_procedures_guard AFTER INSERT ON aliquoting_procedures
    FOR EACH ROW EXECUTE FUNCTION guard_procedure_subtype();
CREATE TRIGGER aliquoting_procedures_guard_update AFTER UPDATE OF id, aliquoting_procedure_template_id, status_id, volume_l ON aliquoting_procedures
    FOR EACH ROW WHEN (OLD.id IS DISTINCT FROM NEW.id
        OR OLD.aliquoting_procedure_template_id IS DISTINCT FROM NEW.aliquoting_procedure_template_id
        OR OLD.status_id IS DISTINCT FROM NEW.status_id
        OR OLD.volume_l IS DISTINCT FROM NEW.volume_l) EXECUTE FUNCTION guard_procedure_subtype();
CREATE TRIGGER aliquoting_procedures_delete_guard BEFORE DELETE ON aliquoting_procedures
    FOR EACH ROW EXECUTE FUNCTION guard_procedure_subtype();

-- Writes a procedure's new status into its `aliquoting_procedures` row, whose deferred
-- foreign key holds it equal to the procedure at commit.
CREATE FUNCTION copy_aliquoting_procedures_status() RETURNS TRIGGER
LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
BEGIN
    UPDATE public.aliquoting_procedures SET status_id = NEW.status_id WHERE id = NEW.id;
    RETURN NULL;
END;
$$;
REVOKE ALL ON aliquoting_procedures FROM PUBLIC, app, identity_writer;
GRANT UPDATE (status_id) ON aliquoting_procedures TO aps_writer;
ALTER FUNCTION copy_aliquoting_procedures_status() OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION copy_aliquoting_procedures_status() FROM PUBLIC;
CREATE TRIGGER procedures_status_copy_aliquoting_procedures AFTER UPDATE OF status_id ON procedures
    FOR EACH ROW WHEN (OLD.status_id IS DISTINCT FROM NEW.status_id) EXECUTE FUNCTION copy_aliquoting_procedures_status();

CREATE FUNCTION require_aliquoting_procedures_row() RETURNS TRIGGER
LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
DECLARE
    row_id UUID := CASE TG_OP WHEN 'DELETE' THEN OLD.id ELSE NEW.id END;
BEGIN
    IF EXISTS (
        SELECT 1 FROM public.procedures root
        JOIN public.table_name_ancestors path ON path.descendant_table_name_id = root.table_name_id
        WHERE root.id = row_id AND path.ancestor_table_name_id = 'aliquoting_procedures'
    ) AND NOT EXISTS (SELECT 1 FROM public.aliquoting_procedures row WHERE row.id = row_id) THEN
        RAISE EXCEPTION '% names a table extending % but has no row there', row_id, 'aliquoting_procedures'
            USING ERRCODE = 'foreign_key_violation';
    END IF;
    RETURN NULL;
END;
$$;
ALTER FUNCTION require_aliquoting_procedures_row() OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION require_aliquoting_procedures_row() FROM PUBLIC;
CREATE CONSTRAINT TRIGGER aliquoting_procedures_row AFTER INSERT ON procedures
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW WHEN (NEW.table_name_id = 'aliquoting_procedures')
    EXECUTE FUNCTION require_aliquoting_procedures_row();
CREATE CONSTRAINT TRIGGER aliquoting_procedures_kept AFTER DELETE ON aliquoting_procedures
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION require_aliquoting_procedures_row();

CREATE FUNCTION initialize_aliquoting_procedures_status() RETURNS TRIGGER
LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
DECLARE
    parent_status TEXT;
BEGIN
    SELECT procedure.status_id INTO parent_status
    FROM public.procedures procedure
    WHERE procedure.id = NEW.id
        AND procedure.procedure_template_id = NEW.aliquoting_procedure_template_id
        AND procedure.organization_id = NEW.organization_id;
    IF FOUND THEN
        NEW.status_id := parent_status;
    END IF;
    RETURN NEW;
END;
$$;
ALTER FUNCTION initialize_aliquoting_procedures_status() OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION initialize_aliquoting_procedures_status() FROM PUBLIC;
CREATE TRIGGER aliquoting_procedures_status_initialize BEFORE INSERT ON aliquoting_procedures
    FOR EACH ROW EXECUTE FUNCTION initialize_aliquoting_procedures_status();

GRANT SELECT, INSERT (id, organization_id, aliquoting_procedure_template_id, volume_l, archived, aliquoting_procedure_template_archived), UPDATE (volume_l), DELETE ON aliquoting_procedures TO app;
ALTER TABLE aliquoting_procedures ENABLE ROW LEVEL SECURITY;
ALTER TABLE aliquoting_procedures FORCE ROW LEVEL SECURITY;
CREATE POLICY aliquoting_procedures_select ON aliquoting_procedures FOR SELECT TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY aliquoting_procedures_insert ON aliquoting_procedures FOR INSERT TO app
    WITH CHECK (id = ANY (ARRAY(SELECT authz.resources_with('writer'))) AND aliquoting_procedure_template_id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY aliquoting_procedures_delete ON aliquoting_procedures FOR DELETE TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('admin'))));
CREATE POLICY aliquoting_procedures_archived_insert ON aliquoting_procedures AS RESTRICTIVE FOR INSERT TO app
    WITH CHECK (NOT archived AND NOT aliquoting_procedure_template_archived);
CREATE POLICY aliquoting_procedures_archived_delete ON aliquoting_procedures AS RESTRICTIVE FOR DELETE TO app
    USING (NOT archived);
CREATE POLICY aliquoting_procedures_blocked ON aliquoting_procedures AS RESTRICTIVE FOR ALL TO app
    USING (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))))
    WITH CHECK (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))));

-- Records every change in connetto's `audit_events`.
CREATE TRIGGER aliquoting_procedure_templates_audit AFTER INSERT OR DELETE ON aliquoting_procedure_templates
    FOR EACH ROW EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER aliquoting_procedure_templates_update_audit AFTER UPDATE ON aliquoting_procedure_templates
    FOR EACH ROW WHEN (OLD IS DISTINCT FROM NEW) EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER aliquoting_procedures_audit AFTER INSERT OR DELETE ON aliquoting_procedures
    FOR EACH ROW EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER aliquoting_procedures_update_audit AFTER UPDATE ON aliquoting_procedures
    FOR EACH ROW WHEN (OLD IS DISTINCT FROM NEW) EXECUTE FUNCTION authz_write.record_audit_event();
CREATE POLICY aliquoting_procedures_update ON aliquoting_procedures FOR UPDATE TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('writer'))))
    WITH CHECK (id = ANY (ARRAY(SELECT authz.resources_with('writer'))));
CREATE POLICY aliquoting_procedures_archived_update ON aliquoting_procedures AS RESTRICTIVE FOR UPDATE TO app
    USING (NOT archived) WITH CHECK (NOT archived);

CREATE INDEX ON public.aliquoting_procedure_templates (source_sample_model_id);
CREATE INDEX ON public.aliquoting_procedure_templates (source_sample_slot_id);

-- Checks the typed slot contract after definition edits.
CREATE FUNCTION public.check_aliquoting_slots() RETURNS TRIGGER
LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
DECLARE
    aliquoted_into_slot UUID;
BEGIN
    SELECT template.aliquoted_into_slot_id INTO aliquoted_into_slot
    FROM public.aliquoting_procedure_templates template
    WHERE template.id = NEW.id;
    PERFORM public.require_single_typed_slot(aliquoted_into_slot);
    RETURN NULL;
END;
$$;
ALTER FUNCTION public.check_aliquoting_slots() OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION public.check_aliquoting_slots() FROM PUBLIC;

-- Checks the aliquoting slot contract after cardinality edits.
CREATE FUNCTION public.check_aliquoting_slot_cardinality() RETURNS TRIGGER
LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
DECLARE
    aliquoted_into_slot UUID;
BEGIN
    SELECT template.aliquoted_into_slot_id INTO aliquoted_into_slot
    FROM public.aliquoting_procedure_templates template
    WHERE template.id = NEW.procedure_template_id AND template.aliquoted_into_slot_id = NEW.id;
    PERFORM public.require_single_typed_slot(aliquoted_into_slot);
    RETURN NULL;
END;
$$;
ALTER FUNCTION public.check_aliquoting_slot_cardinality() OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION public.check_aliquoting_slot_cardinality() FROM PUBLIC;
GRANT SELECT ON public.aliquoting_procedure_templates TO aps_writer;
CREATE CONSTRAINT TRIGGER aliquoting_template_slots AFTER INSERT OR UPDATE OF id, source_sample_slot_id, source_sample_model_id, source_sample_slot_role_id, aliquoted_from_slot_id, aliquoted_from_model_id, aliquoted_from_slot_role_id, aliquoted_into_slot_id, aliquoted_into_model_id, aliquoted_into_slot_role_id, aliquot_slot_id, aliquot_model_id, aliquot_slot_role_id, aliquoted_with_slot_id, aliquoted_with_model_id, aliquoted_with_slot_role_id, volume_l, volume_l_tolerance_percentage ON public.aliquoting_procedure_templates
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION public.check_aliquoting_slots();
CREATE CONSTRAINT TRIGGER aliquoting_slot_cardinality AFTER UPDATE OF minimum_count, maximum_count ON public.procedure_template_slots
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW
    WHEN (OLD.minimum_count IS DISTINCT FROM NEW.minimum_count OR OLD.maximum_count IS DISTINCT FROM NEW.maximum_count)
    EXECUTE FUNCTION public.check_aliquoting_slot_cardinality();
