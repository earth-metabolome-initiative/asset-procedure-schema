-- Migration: 015-supernatant-procedures
-- Purpose: Typed supernatant slots and execution records.
-- APS placement
-- `resources` of kind `'ownable'`
-- +-- procedure_templates -- supernatant_procedure_templates
-- +-- procedures -- supernatant_procedures
-- Slot | Role | Meaning
-- `stratified_source` | `input` | The source retains the settled material.
-- `supernatant_destination` | `input` | The destination container survives transfer.
-- `supernatant` | `produced` | The procedure creates the sample.
-- `transferred_with` | `instrument` | The device transfers the supernatant.
-- `source_sample` | `input` | The sample supplies the transferred material.

-- Typed slots and parameters for supernatant.
CREATE TABLE supernatant_procedure_templates (
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
    -- Slot declaring the `stratified_source` assets.
    stratified_source_slot_id UUID NOT NULL,
    -- Admissible model for the `stratified_source` slot.
    stratified_source_model_id UUID NOT NULL REFERENCES volumetric_container_models (id),
    -- Required role of the `stratified_source` slot.
    stratified_source_slot_role_id TEXT GENERATED ALWAYS AS ('input') STORED NOT NULL,
    -- Slot declaring the `supernatant_destination` assets.
    supernatant_destination_slot_id UUID NOT NULL,
    -- Admissible model for the `supernatant_destination` slot.
    supernatant_destination_model_id UUID NOT NULL REFERENCES volumetric_container_models (id),
    -- Required role of the `supernatant_destination` slot.
    supernatant_destination_slot_role_id TEXT GENERATED ALWAYS AS ('input') STORED NOT NULL,
    -- Slot declaring the produced `supernatant` sample.
    supernatant_slot_id UUID NOT NULL,
    -- Admissible model for the `supernatant` sample.
    supernatant_model_id UUID NOT NULL REFERENCES sample_models (id),
    -- Required role of the `supernatant` slot.
    supernatant_slot_role_id TEXT GENERATED ALWAYS AS ('produced') STORED NOT NULL,
    -- Slot declaring the `transferred_with` assets.
    transferred_with_slot_id UUID NOT NULL,
    -- Admissible model for the `transferred_with` slot.
    transferred_with_model_id UUID NOT NULL REFERENCES volume_measuring_device_models (id),
    -- Required role of the `transferred_with` slot.
    transferred_with_slot_role_id TEXT GENERATED ALWAYS AS ('instrument') STORED NOT NULL,
    -- Volume in liters.
    volume_l DOUBLE PRECISION NOT NULL CHECK (volume_l > 0.0 AND volume_l < 'Infinity'::double precision),
    -- Allowed volume deviation as a percentage of the target.
    volume_l_tolerance_percentage DOUBLE PRECISION NOT NULL DEFAULT 5.0
        CHECK (volume_l_tolerance_percentage > 0.0 AND volume_l_tolerance_percentage <= 100.0),
    CHECK (stratified_source_slot_id <> supernatant_destination_slot_id),
    FOREIGN KEY (source_sample_slot_id, id, source_sample_model_id, source_sample_slot_role_id, organization_id)
        REFERENCES procedure_template_slots (id, procedure_template_id, asset_model_id, role_id, organization_id),
    FOREIGN KEY (stratified_source_slot_id, id, stratified_source_model_id, stratified_source_slot_role_id, organization_id)
        REFERENCES procedure_template_slots (id, procedure_template_id, asset_model_id, role_id, organization_id),
    FOREIGN KEY (supernatant_destination_slot_id, id, supernatant_destination_model_id, supernatant_destination_slot_role_id, organization_id)
        REFERENCES procedure_template_slots (id, procedure_template_id, asset_model_id, role_id, organization_id),
    FOREIGN KEY (transferred_with_slot_id, id, transferred_with_model_id, transferred_with_slot_role_id, organization_id)
        REFERENCES procedure_template_slots (id, procedure_template_id, asset_model_id, role_id, organization_id),
    FOREIGN KEY (supernatant_slot_id, id, supernatant_model_id, supernatant_slot_role_id, organization_id)
        REFERENCES procedure_template_slots (id, procedure_template_id, asset_model_id, role_id, organization_id),
    FOREIGN KEY (supernatant_slot_id, id, supernatant_destination_slot_id, organization_id)
        REFERENCES procedure_template_slots (id, procedure_template_id, placed_into_slot_id, organization_id),
    -- Archive state inherited from this resource.
    archived BOOLEAN NOT NULL DEFAULT FALSE,
    -- Archive state of the row named by `stratified_source_model_id`.
    stratified_source_model_archived BOOLEAN NOT NULL DEFAULT FALSE,
    -- Archive state of the row named by `supernatant_destination_model_id`.
    supernatant_destination_model_archived BOOLEAN NOT NULL DEFAULT FALSE,
    -- Archive state of the row named by `supernatant_model_id`.
    supernatant_model_archived BOOLEAN NOT NULL DEFAULT FALSE,
    -- Archive state of the row named by `transferred_with_model_id`.
    transferred_with_model_archived BOOLEAN NOT NULL DEFAULT FALSE,
    -- Archive state of the row named by `source_sample_model_id`.
    source_sample_model_archived BOOLEAN NOT NULL DEFAULT FALSE,
    UNIQUE (id, archived),
    FOREIGN KEY (stratified_source_model_id, stratified_source_model_archived) REFERENCES volumetric_container_models (id, archived) ON UPDATE CASCADE,
    FOREIGN KEY (supernatant_destination_model_id, supernatant_destination_model_archived) REFERENCES volumetric_container_models (id, archived) ON UPDATE CASCADE,
    FOREIGN KEY (supernatant_model_id, supernatant_model_archived) REFERENCES sample_models (id, archived) ON UPDATE CASCADE,
    FOREIGN KEY (transferred_with_model_id, transferred_with_model_archived) REFERENCES volume_measuring_device_models (id, archived) ON UPDATE CASCADE,
    FOREIGN KEY (source_sample_model_id, source_sample_model_archived) REFERENCES sample_models (id, archived) ON UPDATE CASCADE,
    FOREIGN KEY (organization_id, id, archived) REFERENCES resources (organization_id, id, archived) ON UPDATE CASCADE ON DELETE CASCADE
);
CREATE INDEX ON supernatant_procedure_templates (stratified_source_model_id);
CREATE INDEX ON supernatant_procedure_templates (supernatant_destination_model_id);
CREATE INDEX ON supernatant_procedure_templates (supernatant_model_id);
CREATE INDEX ON supernatant_procedure_templates (transferred_with_model_id);
INSERT INTO table_names (id) VALUES ('supernatant_procedure_templates');
INSERT INTO table_name_ancestors (descendant_table_name_id, ancestor_table_name_id)
    SELECT 'supernatant_procedure_templates', ancestor_table_name_id FROM table_name_ancestors WHERE descendant_table_name_id = 'procedure_templates'
    UNION SELECT 'supernatant_procedure_templates', 'supernatant_procedure_templates';
CREATE TRIGGER supernatant_procedure_templates_guard AFTER INSERT ON supernatant_procedure_templates
    FOR EACH ROW EXECUTE FUNCTION guard_procedure_template_subtype();
CREATE TRIGGER supernatant_procedure_templates_guard_update AFTER UPDATE OF id, source_sample_slot_id, source_sample_model_id, source_sample_slot_role_id, stratified_source_slot_id, stratified_source_model_id, stratified_source_slot_role_id, supernatant_destination_slot_id, supernatant_destination_model_id, supernatant_destination_slot_role_id, supernatant_slot_id, supernatant_model_id, supernatant_slot_role_id, transferred_with_slot_id, transferred_with_model_id, transferred_with_slot_role_id, volume_l, volume_l_tolerance_percentage ON supernatant_procedure_templates
    FOR EACH ROW WHEN (OLD.id IS DISTINCT FROM NEW.id
        OR OLD.source_sample_slot_id IS DISTINCT FROM NEW.source_sample_slot_id
        OR OLD.source_sample_model_id IS DISTINCT FROM NEW.source_sample_model_id
        OR OLD.source_sample_slot_role_id IS DISTINCT FROM NEW.source_sample_slot_role_id
        OR OLD.stratified_source_slot_id IS DISTINCT FROM NEW.stratified_source_slot_id
        OR OLD.stratified_source_model_id IS DISTINCT FROM NEW.stratified_source_model_id
        OR OLD.stratified_source_slot_role_id IS DISTINCT FROM NEW.stratified_source_slot_role_id
        OR OLD.supernatant_destination_slot_id IS DISTINCT FROM NEW.supernatant_destination_slot_id
        OR OLD.supernatant_destination_model_id IS DISTINCT FROM NEW.supernatant_destination_model_id
        OR OLD.supernatant_destination_slot_role_id IS DISTINCT FROM NEW.supernatant_destination_slot_role_id
        OR OLD.supernatant_slot_id IS DISTINCT FROM NEW.supernatant_slot_id
        OR OLD.supernatant_model_id IS DISTINCT FROM NEW.supernatant_model_id
        OR OLD.supernatant_slot_role_id IS DISTINCT FROM NEW.supernatant_slot_role_id
        OR OLD.transferred_with_slot_id IS DISTINCT FROM NEW.transferred_with_slot_id
        OR OLD.transferred_with_model_id IS DISTINCT FROM NEW.transferred_with_model_id
        OR OLD.transferred_with_slot_role_id IS DISTINCT FROM NEW.transferred_with_slot_role_id
        OR OLD.volume_l IS DISTINCT FROM NEW.volume_l
        OR OLD.volume_l_tolerance_percentage IS DISTINCT FROM NEW.volume_l_tolerance_percentage) EXECUTE FUNCTION guard_procedure_template_subtype();
CREATE TRIGGER supernatant_procedure_templates_delete_guard BEFORE DELETE ON supernatant_procedure_templates
    FOR EACH ROW EXECUTE FUNCTION guard_procedure_template_subtype();

CREATE FUNCTION require_supernatant_procedure_templates_row() RETURNS TRIGGER
LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
DECLARE
    row_id UUID := CASE TG_OP WHEN 'DELETE' THEN OLD.id ELSE NEW.id END;
BEGIN
    IF EXISTS (
        SELECT 1 FROM public.procedure_templates root
        JOIN public.table_name_ancestors path ON path.descendant_table_name_id = root.table_name_id
        WHERE root.id = row_id AND path.ancestor_table_name_id = 'supernatant_procedure_templates'
    ) AND NOT EXISTS (SELECT 1 FROM public.supernatant_procedure_templates row WHERE row.id = row_id) THEN
        RAISE EXCEPTION '% names a table extending % but has no row there', row_id, 'supernatant_procedure_templates'
            USING ERRCODE = 'foreign_key_violation';
    END IF;
    RETURN NULL;
END;
$$;
ALTER FUNCTION require_supernatant_procedure_templates_row() OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION require_supernatant_procedure_templates_row() FROM PUBLIC;
CREATE CONSTRAINT TRIGGER supernatant_procedure_templates_row AFTER INSERT ON procedure_templates
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW WHEN (NEW.table_name_id = 'supernatant_procedure_templates')
    EXECUTE FUNCTION require_supernatant_procedure_templates_row();
CREATE CONSTRAINT TRIGGER supernatant_procedure_templates_kept AFTER DELETE ON supernatant_procedure_templates
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION require_supernatant_procedure_templates_row();

REVOKE ALL ON supernatant_procedure_templates FROM PUBLIC, app, identity_writer;
GRANT SELECT, INSERT, UPDATE (volume_l, volume_l_tolerance_percentage), DELETE ON supernatant_procedure_templates TO app;
ALTER TABLE supernatant_procedure_templates ENABLE ROW LEVEL SECURITY;
ALTER TABLE supernatant_procedure_templates FORCE ROW LEVEL SECURITY;
CREATE POLICY supernatant_procedure_templates_select ON supernatant_procedure_templates FOR SELECT TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY supernatant_procedure_templates_insert ON supernatant_procedure_templates FOR INSERT TO app
    WITH CHECK (id = ANY (ARRAY(SELECT authz.resources_with('writer'))) AND stratified_source_model_id = ANY (ARRAY(SELECT authz.resources_with('reader'))) AND supernatant_destination_model_id = ANY (ARRAY(SELECT authz.resources_with('reader'))) AND supernatant_model_id = ANY (ARRAY(SELECT authz.resources_with('reader'))) AND transferred_with_model_id = ANY (ARRAY(SELECT authz.resources_with('reader'))) AND source_sample_model_id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY supernatant_procedure_templates_update ON supernatant_procedure_templates FOR UPDATE TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('writer'))) AND supernatant_destination_model_id = ANY (ARRAY(SELECT authz.resources_with('reader'))))
    WITH CHECK (id = ANY (ARRAY(SELECT authz.resources_with('writer'))));
CREATE POLICY supernatant_procedure_templates_delete ON supernatant_procedure_templates FOR DELETE TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('admin'))));
CREATE POLICY supernatant_procedure_templates_archived_insert ON supernatant_procedure_templates AS RESTRICTIVE FOR INSERT TO app
    WITH CHECK (NOT archived AND NOT stratified_source_model_archived AND NOT supernatant_destination_model_archived AND NOT supernatant_model_archived AND NOT transferred_with_model_archived AND NOT source_sample_model_archived);
CREATE POLICY supernatant_procedure_templates_archived_update ON supernatant_procedure_templates AS RESTRICTIVE FOR UPDATE TO app
    USING (NOT archived) WITH CHECK (NOT archived);
CREATE POLICY supernatant_procedure_templates_archived_delete ON supernatant_procedure_templates AS RESTRICTIVE FOR DELETE TO app
    USING (NOT archived);
CREATE POLICY supernatant_procedure_templates_blocked ON supernatant_procedure_templates AS RESTRICTIVE FOR ALL TO app
    USING (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))))
    WITH CHECK (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))));

-- Recorded execution of a supernatant template.
CREATE TABLE supernatant_procedures (
    -- Identity shared with the parent procedure.
    id UUID PRIMARY KEY REFERENCES procedures (id) ON DELETE CASCADE,
    -- Organization containing the parent resource.
    organization_id UUID NOT NULL,
    -- Typed template executed by this procedure.
    supernatant_procedure_template_id UUID NOT NULL REFERENCES supernatant_procedure_templates (id),
    -- Execution status copied from the parent procedure.
    status_id TEXT NOT NULL DEFAULT 'planned',
    -- Measured volume l.
    volume_l DOUBLE PRECISION CHECK (volume_l > 0.0 AND volume_l < 'Infinity'::double precision),
    CHECK (status_id <> 'completed' OR volume_l IS NOT NULL),
    FOREIGN KEY (id, supernatant_procedure_template_id, status_id, organization_id)
        REFERENCES procedures (id, procedure_template_id, status_id, organization_id) ON DELETE CASCADE
        DEFERRABLE INITIALLY DEFERRED,
    -- Archive state inherited from this resource.
    archived BOOLEAN NOT NULL DEFAULT FALSE,
    -- Archive state of the row named by `supernatant_procedure_template_id`.
    supernatant_procedure_template_archived BOOLEAN NOT NULL DEFAULT FALSE,
    FOREIGN KEY (supernatant_procedure_template_id, supernatant_procedure_template_archived) REFERENCES supernatant_procedure_templates (id, archived) ON UPDATE CASCADE,
    FOREIGN KEY (organization_id, id, archived) REFERENCES resources (organization_id, id, archived) ON UPDATE CASCADE ON DELETE CASCADE
);
CREATE INDEX ON supernatant_procedures (supernatant_procedure_template_id);
INSERT INTO table_names (id) VALUES ('supernatant_procedures');
INSERT INTO table_name_instances (id, instance_table_name_id)
    VALUES ('supernatant_procedure_templates', 'supernatant_procedures');
INSERT INTO table_name_ancestors (descendant_table_name_id, ancestor_table_name_id)
    SELECT 'supernatant_procedures', ancestor_table_name_id FROM table_name_ancestors WHERE descendant_table_name_id = 'procedures'
    UNION SELECT 'supernatant_procedures', 'supernatant_procedures';
CREATE TRIGGER supernatant_procedures_guard AFTER INSERT ON supernatant_procedures
    FOR EACH ROW EXECUTE FUNCTION guard_procedure_subtype();
CREATE TRIGGER supernatant_procedures_guard_update AFTER UPDATE OF id, supernatant_procedure_template_id, status_id, volume_l ON supernatant_procedures
    FOR EACH ROW WHEN (OLD.id IS DISTINCT FROM NEW.id
        OR OLD.supernatant_procedure_template_id IS DISTINCT FROM NEW.supernatant_procedure_template_id
        OR OLD.status_id IS DISTINCT FROM NEW.status_id
        OR OLD.volume_l IS DISTINCT FROM NEW.volume_l) EXECUTE FUNCTION guard_procedure_subtype();
CREATE TRIGGER supernatant_procedures_delete_guard BEFORE DELETE ON supernatant_procedures
    FOR EACH ROW EXECUTE FUNCTION guard_procedure_subtype();

-- Writes a procedure's new status into its `supernatant_procedures` row, whose deferred
-- foreign key holds it equal to the procedure at commit.
CREATE FUNCTION copy_supernatant_procedures_status() RETURNS TRIGGER
LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
BEGIN
    UPDATE public.supernatant_procedures SET status_id = NEW.status_id WHERE id = NEW.id;
    RETURN NULL;
END;
$$;
REVOKE ALL ON supernatant_procedures FROM PUBLIC, app, identity_writer;
GRANT UPDATE (status_id) ON supernatant_procedures TO aps_writer;
ALTER FUNCTION copy_supernatant_procedures_status() OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION copy_supernatant_procedures_status() FROM PUBLIC;
CREATE TRIGGER procedures_status_copy_supernatant_procedures AFTER UPDATE OF status_id ON procedures
    FOR EACH ROW WHEN (OLD.status_id IS DISTINCT FROM NEW.status_id) EXECUTE FUNCTION copy_supernatant_procedures_status();

CREATE FUNCTION require_supernatant_procedures_row() RETURNS TRIGGER
LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
DECLARE
    row_id UUID := CASE TG_OP WHEN 'DELETE' THEN OLD.id ELSE NEW.id END;
BEGIN
    IF EXISTS (
        SELECT 1 FROM public.procedures root
        JOIN public.table_name_ancestors path ON path.descendant_table_name_id = root.table_name_id
        WHERE root.id = row_id AND path.ancestor_table_name_id = 'supernatant_procedures'
    ) AND NOT EXISTS (SELECT 1 FROM public.supernatant_procedures row WHERE row.id = row_id) THEN
        RAISE EXCEPTION '% names a table extending % but has no row there', row_id, 'supernatant_procedures'
            USING ERRCODE = 'foreign_key_violation';
    END IF;
    RETURN NULL;
END;
$$;
ALTER FUNCTION require_supernatant_procedures_row() OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION require_supernatant_procedures_row() FROM PUBLIC;
CREATE CONSTRAINT TRIGGER supernatant_procedures_row AFTER INSERT ON procedures
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW WHEN (NEW.table_name_id = 'supernatant_procedures')
    EXECUTE FUNCTION require_supernatant_procedures_row();
CREATE CONSTRAINT TRIGGER supernatant_procedures_kept AFTER DELETE ON supernatant_procedures
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION require_supernatant_procedures_row();

CREATE FUNCTION initialize_supernatant_procedures_status() RETURNS TRIGGER
LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
DECLARE
    parent_status TEXT;
BEGIN
    SELECT procedure.status_id INTO parent_status
    FROM public.procedures procedure
    WHERE procedure.id = NEW.id
        AND procedure.procedure_template_id = NEW.supernatant_procedure_template_id
        AND procedure.organization_id = NEW.organization_id;
    IF FOUND THEN
        NEW.status_id := parent_status;
    END IF;
    RETURN NEW;
END;
$$;
ALTER FUNCTION initialize_supernatant_procedures_status() OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION initialize_supernatant_procedures_status() FROM PUBLIC;
CREATE TRIGGER supernatant_procedures_status_initialize BEFORE INSERT ON supernatant_procedures
    FOR EACH ROW EXECUTE FUNCTION initialize_supernatant_procedures_status();

GRANT SELECT, INSERT (id, organization_id, supernatant_procedure_template_id, volume_l, archived, supernatant_procedure_template_archived), UPDATE (volume_l), DELETE ON supernatant_procedures TO app;
ALTER TABLE supernatant_procedures ENABLE ROW LEVEL SECURITY;
ALTER TABLE supernatant_procedures FORCE ROW LEVEL SECURITY;
CREATE POLICY supernatant_procedures_select ON supernatant_procedures FOR SELECT TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY supernatant_procedures_insert ON supernatant_procedures FOR INSERT TO app
    WITH CHECK (id = ANY (ARRAY(SELECT authz.resources_with('writer'))) AND supernatant_procedure_template_id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY supernatant_procedures_delete ON supernatant_procedures FOR DELETE TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('admin'))));
CREATE POLICY supernatant_procedures_archived_insert ON supernatant_procedures AS RESTRICTIVE FOR INSERT TO app
    WITH CHECK (NOT archived AND NOT supernatant_procedure_template_archived);
CREATE POLICY supernatant_procedures_archived_delete ON supernatant_procedures AS RESTRICTIVE FOR DELETE TO app
    USING (NOT archived);
CREATE POLICY supernatant_procedures_blocked ON supernatant_procedures AS RESTRICTIVE FOR ALL TO app
    USING (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))))
    WITH CHECK (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))));

-- Records every change in connetto's `audit_events`.
CREATE TRIGGER supernatant_procedure_templates_audit AFTER INSERT OR DELETE ON supernatant_procedure_templates
    FOR EACH ROW EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER supernatant_procedure_templates_update_audit AFTER UPDATE ON supernatant_procedure_templates
    FOR EACH ROW WHEN (OLD IS DISTINCT FROM NEW) EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER supernatant_procedures_audit AFTER INSERT OR DELETE ON supernatant_procedures
    FOR EACH ROW EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER supernatant_procedures_update_audit AFTER UPDATE ON supernatant_procedures
    FOR EACH ROW WHEN (OLD IS DISTINCT FROM NEW) EXECUTE FUNCTION authz_write.record_audit_event();
CREATE POLICY supernatant_procedures_update ON supernatant_procedures FOR UPDATE TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('writer'))))
    WITH CHECK (id = ANY (ARRAY(SELECT authz.resources_with('writer'))));
CREATE POLICY supernatant_procedures_archived_update ON supernatant_procedures AS RESTRICTIVE FOR UPDATE TO app
    USING (NOT archived) WITH CHECK (NOT archived);

CREATE INDEX ON public.supernatant_procedure_templates (source_sample_model_id);
CREATE INDEX ON public.supernatant_procedure_templates (source_sample_slot_id);

-- Checks the typed slot contract after definition edits.
CREATE FUNCTION public.check_supernatant_slots() RETURNS TRIGGER
LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
DECLARE
    supernatant_destination_slot UUID;
BEGIN
    SELECT template.supernatant_destination_slot_id INTO supernatant_destination_slot
    FROM public.supernatant_procedure_templates template
    WHERE template.id = NEW.id;
    PERFORM public.require_single_typed_slot(supernatant_destination_slot);
    RETURN NULL;
END;
$$;
ALTER FUNCTION public.check_supernatant_slots() OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION public.check_supernatant_slots() FROM PUBLIC;

-- Checks the supernatant slot contract after cardinality edits.
CREATE FUNCTION public.check_supernatant_slot_cardinality() RETURNS TRIGGER
LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
DECLARE
    supernatant_destination_slot UUID;
BEGIN
    SELECT template.supernatant_destination_slot_id INTO supernatant_destination_slot
    FROM public.supernatant_procedure_templates template
    WHERE template.id = NEW.procedure_template_id AND template.supernatant_destination_slot_id = NEW.id;
    PERFORM public.require_single_typed_slot(supernatant_destination_slot);
    RETURN NULL;
END;
$$;
ALTER FUNCTION public.check_supernatant_slot_cardinality() OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION public.check_supernatant_slot_cardinality() FROM PUBLIC;
GRANT SELECT ON public.supernatant_procedure_templates TO aps_writer;
CREATE CONSTRAINT TRIGGER supernatant_template_slots AFTER INSERT OR UPDATE OF id, source_sample_slot_id, source_sample_model_id, source_sample_slot_role_id, stratified_source_slot_id, stratified_source_model_id, stratified_source_slot_role_id, supernatant_destination_slot_id, supernatant_destination_model_id, supernatant_destination_slot_role_id, supernatant_slot_id, supernatant_model_id, supernatant_slot_role_id, transferred_with_slot_id, transferred_with_model_id, transferred_with_slot_role_id, volume_l, volume_l_tolerance_percentage ON public.supernatant_procedure_templates
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION public.check_supernatant_slots();
CREATE CONSTRAINT TRIGGER supernatant_slot_cardinality AFTER UPDATE OF minimum_count, maximum_count ON public.procedure_template_slots
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW
    WHEN (OLD.minimum_count IS DISTINCT FROM NEW.minimum_count OR OLD.maximum_count IS DISTINCT FROM NEW.maximum_count)
    EXECUTE FUNCTION public.check_supernatant_slot_cardinality();
