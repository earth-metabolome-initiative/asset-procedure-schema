-- Migration: 002-storage-procedures
-- Purpose: Typed storage slots and execution records.
-- APS placement
-- `resources` of kind `'ownable'`
-- +-- procedure_templates -- storage_procedure_templates
-- +-- procedures -- storage_procedures
-- Slot | Role | Meaning
-- `stored_into` | `input` | The storage container remains in use.
-- `stored_asset` | `input` | The stored asset remains available.

-- Typed slots and parameters for storage.
CREATE TABLE storage_procedure_templates (
    -- Identity shared with the parent template.
    id UUID PRIMARY KEY REFERENCES procedure_templates (id) ON DELETE CASCADE,
    -- Organization containing the parent resource.
    organization_id UUID NOT NULL,
    -- Slot declaring the `stored_into` assets.
    stored_into_slot_id UUID NOT NULL,
    -- Admissible model for the `stored_into` slot.
    stored_into_model_id UUID NOT NULL REFERENCES container_models (id),
    -- Required role of the `stored_into` slot.
    stored_into_slot_role_id TEXT GENERATED ALWAYS AS ('input') STORED NOT NULL,
    -- Slot declaring the `stored_asset` assets.
    stored_asset_slot_id UUID NOT NULL,
    -- Admissible model for the `stored_asset` slot.
    stored_asset_model_id UUID NOT NULL REFERENCES physical_asset_models (id),
    -- Required role of the `stored_asset` slot.
    stored_asset_slot_role_id TEXT GENERATED ALWAYS AS ('input') STORED NOT NULL,
    -- Target temperature in kelvin.
    temperature_k DOUBLE PRECISION NOT NULL DEFAULT 293.15 CHECK (temperature_k > 0.0 AND temperature_k < 'Infinity'::double precision),
    -- Allowed temperature deviation as a percentage of the target.
    temperature_tolerance_percentage DOUBLE PRECISION NOT NULL DEFAULT 1.0 CHECK ( temperature_tolerance_percentage > 0.0 AND temperature_tolerance_percentage <= 100.0 ),
    FOREIGN KEY (stored_into_slot_id, id, stored_into_model_id, stored_into_slot_role_id, organization_id)
        REFERENCES procedure_template_slots (id, procedure_template_id, asset_model_id, role_id, organization_id),
    FOREIGN KEY (stored_asset_slot_id, id, stored_asset_model_id, stored_asset_slot_role_id, organization_id)
        REFERENCES procedure_template_slots (id, procedure_template_id, asset_model_id, role_id, organization_id),
    FOREIGN KEY (stored_asset_slot_id, id, stored_into_slot_id, organization_id)
        REFERENCES procedure_template_slots (id, procedure_template_id, placed_into_slot_id, organization_id),
    -- Archive state inherited from this resource.
    archived BOOLEAN NOT NULL DEFAULT FALSE,
    -- Archive state of the row named by `stored_into_model_id`.
    stored_into_model_archived BOOLEAN NOT NULL DEFAULT FALSE,
    -- Archive state of the row named by `stored_asset_model_id`.
    stored_asset_model_archived BOOLEAN NOT NULL DEFAULT FALSE,
    UNIQUE (id, archived),
    FOREIGN KEY (stored_into_model_id, stored_into_model_archived) REFERENCES container_models (id, archived) ON UPDATE CASCADE,
    FOREIGN KEY (stored_asset_model_id, stored_asset_model_archived) REFERENCES physical_asset_models (id, archived) ON UPDATE CASCADE,
    FOREIGN KEY (organization_id, id, archived) REFERENCES resources (organization_id, id, archived) ON UPDATE CASCADE ON DELETE CASCADE
);
CREATE INDEX ON storage_procedure_templates (stored_asset_model_id);
CREATE INDEX ON storage_procedure_templates (stored_into_model_id, stored_asset_model_id);
INSERT INTO table_names (id) VALUES ('storage_procedure_templates');
INSERT INTO table_name_ancestors (descendant_table_name_id, ancestor_table_name_id)
    SELECT 'storage_procedure_templates', ancestor_table_name_id FROM table_name_ancestors WHERE descendant_table_name_id = 'procedure_templates'
    UNION SELECT 'storage_procedure_templates', 'storage_procedure_templates';
CREATE TRIGGER storage_procedure_templates_guard AFTER INSERT ON storage_procedure_templates
    FOR EACH ROW EXECUTE FUNCTION guard_procedure_template_subtype();
CREATE TRIGGER storage_procedure_templates_guard_update AFTER UPDATE OF id, stored_into_slot_id, stored_into_model_id, stored_into_slot_role_id, stored_asset_slot_id, stored_asset_model_id, stored_asset_slot_role_id, temperature_k, temperature_tolerance_percentage ON storage_procedure_templates
    FOR EACH ROW WHEN (OLD.id IS DISTINCT FROM NEW.id
        OR OLD.stored_into_slot_id IS DISTINCT FROM NEW.stored_into_slot_id
        OR OLD.stored_into_model_id IS DISTINCT FROM NEW.stored_into_model_id
        OR OLD.stored_into_slot_role_id IS DISTINCT FROM NEW.stored_into_slot_role_id
        OR OLD.stored_asset_slot_id IS DISTINCT FROM NEW.stored_asset_slot_id
        OR OLD.stored_asset_model_id IS DISTINCT FROM NEW.stored_asset_model_id
        OR OLD.stored_asset_slot_role_id IS DISTINCT FROM NEW.stored_asset_slot_role_id
        OR OLD.temperature_k IS DISTINCT FROM NEW.temperature_k
        OR OLD.temperature_tolerance_percentage IS DISTINCT FROM NEW.temperature_tolerance_percentage) EXECUTE FUNCTION guard_procedure_template_subtype();
CREATE TRIGGER storage_procedure_templates_delete_guard BEFORE DELETE ON storage_procedure_templates
    FOR EACH ROW EXECUTE FUNCTION guard_procedure_template_subtype();

CREATE FUNCTION require_storage_procedure_templates_row() RETURNS TRIGGER
LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
DECLARE
    row_id UUID := CASE TG_OP WHEN 'DELETE' THEN OLD.id ELSE NEW.id END;
BEGIN
    IF EXISTS (
        SELECT 1 FROM public.procedure_templates root
        JOIN public.table_name_ancestors path ON path.descendant_table_name_id = root.table_name_id
        WHERE root.id = row_id AND path.ancestor_table_name_id = 'storage_procedure_templates'
    ) AND NOT EXISTS (SELECT 1 FROM public.storage_procedure_templates row WHERE row.id = row_id) THEN
        RAISE EXCEPTION '% names a table extending % but has no row there', row_id, 'storage_procedure_templates'
            USING ERRCODE = 'foreign_key_violation';
    END IF;
    RETURN NULL;
END;
$$;
ALTER FUNCTION require_storage_procedure_templates_row() OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION require_storage_procedure_templates_row() FROM PUBLIC;
CREATE CONSTRAINT TRIGGER storage_procedure_templates_row AFTER INSERT ON procedure_templates
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW WHEN (NEW.table_name_id = 'storage_procedure_templates')
    EXECUTE FUNCTION require_storage_procedure_templates_row();
CREATE CONSTRAINT TRIGGER storage_procedure_templates_kept AFTER DELETE ON storage_procedure_templates
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION require_storage_procedure_templates_row();

REVOKE ALL ON storage_procedure_templates FROM PUBLIC, app, identity_writer;
GRANT SELECT, INSERT, UPDATE (temperature_k, temperature_tolerance_percentage), DELETE ON storage_procedure_templates TO app;
ALTER TABLE storage_procedure_templates ENABLE ROW LEVEL SECURITY;
ALTER TABLE storage_procedure_templates FORCE ROW LEVEL SECURITY;
CREATE POLICY storage_procedure_templates_select ON storage_procedure_templates FOR SELECT TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY storage_procedure_templates_insert ON storage_procedure_templates FOR INSERT TO app
    WITH CHECK (id = ANY (ARRAY(SELECT authz.resources_with('writer'))) AND stored_into_model_id = ANY (ARRAY(SELECT authz.resources_with('reader'))) AND stored_asset_model_id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY storage_procedure_templates_update ON storage_procedure_templates FOR UPDATE TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('writer'))))
    WITH CHECK (id = ANY (ARRAY(SELECT authz.resources_with('writer'))));
CREATE POLICY storage_procedure_templates_delete ON storage_procedure_templates FOR DELETE TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('admin'))));
CREATE POLICY storage_procedure_templates_archived_insert ON storage_procedure_templates AS RESTRICTIVE FOR INSERT TO app
    WITH CHECK (NOT archived AND NOT stored_into_model_archived AND NOT stored_asset_model_archived);
CREATE POLICY storage_procedure_templates_archived_update ON storage_procedure_templates AS RESTRICTIVE FOR UPDATE TO app
    USING (NOT archived) WITH CHECK (NOT archived);
CREATE POLICY storage_procedure_templates_archived_delete ON storage_procedure_templates AS RESTRICTIVE FOR DELETE TO app
    USING (NOT archived);
CREATE POLICY storage_procedure_templates_blocked ON storage_procedure_templates AS RESTRICTIVE FOR ALL TO app
    USING (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))))
    WITH CHECK (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))));

-- Recorded execution of a storage template.
CREATE TABLE storage_procedures (
    -- Identity shared with the parent procedure.
    id UUID PRIMARY KEY REFERENCES procedures (id) ON DELETE CASCADE,
    -- Organization containing the parent resource.
    organization_id UUID NOT NULL,
    -- Typed template executed by this procedure.
    storage_procedure_template_id UUID NOT NULL REFERENCES storage_procedure_templates (id),
    -- Execution status copied from the parent procedure.
    status_id TEXT NOT NULL DEFAULT 'planned',
    -- Measured temperature k.
    temperature_k DOUBLE PRECISION CHECK (temperature_k > 0.0 AND temperature_k < 'Infinity'::double precision),
    CHECK (status_id <> 'completed' OR temperature_k IS NOT NULL),
    FOREIGN KEY (id, storage_procedure_template_id, status_id, organization_id)
        REFERENCES procedures (id, procedure_template_id, status_id, organization_id) ON DELETE CASCADE
        DEFERRABLE INITIALLY DEFERRED,
    -- Archive state inherited from this resource.
    archived BOOLEAN NOT NULL DEFAULT FALSE,
    -- Archive state of the row named by `storage_procedure_template_id`.
    storage_procedure_template_archived BOOLEAN NOT NULL DEFAULT FALSE,
    FOREIGN KEY (storage_procedure_template_id, storage_procedure_template_archived) REFERENCES storage_procedure_templates (id, archived) ON UPDATE CASCADE,
    FOREIGN KEY (organization_id, id, archived) REFERENCES resources (organization_id, id, archived) ON UPDATE CASCADE ON DELETE CASCADE
);
CREATE INDEX ON storage_procedures (storage_procedure_template_id);
INSERT INTO table_names (id) VALUES ('storage_procedures');
INSERT INTO table_name_instances (id, instance_table_name_id)
    VALUES ('storage_procedure_templates', 'storage_procedures');
INSERT INTO table_name_ancestors (descendant_table_name_id, ancestor_table_name_id)
    SELECT 'storage_procedures', ancestor_table_name_id FROM table_name_ancestors WHERE descendant_table_name_id = 'procedures'
    UNION SELECT 'storage_procedures', 'storage_procedures';
CREATE TRIGGER storage_procedures_guard AFTER INSERT ON storage_procedures
    FOR EACH ROW EXECUTE FUNCTION guard_procedure_subtype();
CREATE TRIGGER storage_procedures_guard_update AFTER UPDATE OF id, storage_procedure_template_id, status_id, temperature_k ON storage_procedures
    FOR EACH ROW WHEN (OLD.id IS DISTINCT FROM NEW.id
        OR OLD.storage_procedure_template_id IS DISTINCT FROM NEW.storage_procedure_template_id
        OR OLD.status_id IS DISTINCT FROM NEW.status_id
        OR OLD.temperature_k IS DISTINCT FROM NEW.temperature_k) EXECUTE FUNCTION guard_procedure_subtype();
CREATE TRIGGER storage_procedures_delete_guard BEFORE DELETE ON storage_procedures
    FOR EACH ROW EXECUTE FUNCTION guard_procedure_subtype();

-- Writes a procedure's new status into its `storage_procedures` row, whose deferred
-- foreign key holds it equal to the procedure at commit.
CREATE FUNCTION copy_storage_procedures_status() RETURNS TRIGGER
LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
BEGIN
    UPDATE public.storage_procedures SET status_id = NEW.status_id WHERE id = NEW.id;
    RETURN NULL;
END;
$$;
REVOKE ALL ON storage_procedures FROM PUBLIC, app, identity_writer;
GRANT UPDATE (status_id) ON storage_procedures TO aps_writer;
ALTER FUNCTION copy_storage_procedures_status() OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION copy_storage_procedures_status() FROM PUBLIC;
CREATE TRIGGER procedures_status_copy_storage_procedures AFTER UPDATE OF status_id ON procedures
    FOR EACH ROW WHEN (OLD.status_id IS DISTINCT FROM NEW.status_id) EXECUTE FUNCTION copy_storage_procedures_status();

CREATE FUNCTION require_storage_procedures_row() RETURNS TRIGGER
LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
DECLARE
    row_id UUID := CASE TG_OP WHEN 'DELETE' THEN OLD.id ELSE NEW.id END;
BEGIN
    IF EXISTS (
        SELECT 1 FROM public.procedures root
        JOIN public.table_name_ancestors path ON path.descendant_table_name_id = root.table_name_id
        WHERE root.id = row_id AND path.ancestor_table_name_id = 'storage_procedures'
    ) AND NOT EXISTS (SELECT 1 FROM public.storage_procedures row WHERE row.id = row_id) THEN
        RAISE EXCEPTION '% names a table extending % but has no row there', row_id, 'storage_procedures'
            USING ERRCODE = 'foreign_key_violation';
    END IF;
    RETURN NULL;
END;
$$;
ALTER FUNCTION require_storage_procedures_row() OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION require_storage_procedures_row() FROM PUBLIC;
CREATE CONSTRAINT TRIGGER storage_procedures_row AFTER INSERT ON procedures
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW WHEN (NEW.table_name_id = 'storage_procedures')
    EXECUTE FUNCTION require_storage_procedures_row();
CREATE CONSTRAINT TRIGGER storage_procedures_kept AFTER DELETE ON storage_procedures
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION require_storage_procedures_row();

CREATE FUNCTION initialize_storage_procedures_status() RETURNS TRIGGER
LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
DECLARE
    parent_status TEXT;
BEGIN
    SELECT procedure.status_id INTO parent_status
    FROM public.procedures procedure
    WHERE procedure.id = NEW.id
        AND procedure.procedure_template_id = NEW.storage_procedure_template_id
        AND procedure.organization_id = NEW.organization_id;
    IF FOUND THEN
        NEW.status_id := parent_status;
    END IF;
    RETURN NEW;
END;
$$;
ALTER FUNCTION initialize_storage_procedures_status() OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION initialize_storage_procedures_status() FROM PUBLIC;
CREATE TRIGGER storage_procedures_status_initialize BEFORE INSERT ON storage_procedures
    FOR EACH ROW EXECUTE FUNCTION initialize_storage_procedures_status();

GRANT SELECT, INSERT (id, organization_id, storage_procedure_template_id, temperature_k, archived, storage_procedure_template_archived), UPDATE (temperature_k), DELETE ON storage_procedures TO app;
ALTER TABLE storage_procedures ENABLE ROW LEVEL SECURITY;
ALTER TABLE storage_procedures FORCE ROW LEVEL SECURITY;
CREATE POLICY storage_procedures_select ON storage_procedures FOR SELECT TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY storage_procedures_insert ON storage_procedures FOR INSERT TO app
    WITH CHECK (id = ANY (ARRAY(SELECT authz.resources_with('writer'))) AND storage_procedure_template_id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY storage_procedures_delete ON storage_procedures FOR DELETE TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('admin'))));
CREATE POLICY storage_procedures_archived_insert ON storage_procedures AS RESTRICTIVE FOR INSERT TO app
    WITH CHECK (NOT archived AND NOT storage_procedure_template_archived);
CREATE POLICY storage_procedures_archived_delete ON storage_procedures AS RESTRICTIVE FOR DELETE TO app
    USING (NOT archived);
CREATE POLICY storage_procedures_blocked ON storage_procedures AS RESTRICTIVE FOR ALL TO app
    USING (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))))
    WITH CHECK (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))));

-- Records every change in connetto's `audit_events`.
CREATE TRIGGER storage_procedure_templates_audit AFTER INSERT OR DELETE ON storage_procedure_templates
    FOR EACH ROW EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER storage_procedure_templates_update_audit AFTER UPDATE ON storage_procedure_templates
    FOR EACH ROW WHEN (OLD IS DISTINCT FROM NEW) EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER storage_procedures_audit AFTER INSERT OR DELETE ON storage_procedures
    FOR EACH ROW EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER storage_procedures_update_audit AFTER UPDATE ON storage_procedures
    FOR EACH ROW WHEN (OLD IS DISTINCT FROM NEW) EXECUTE FUNCTION authz_write.record_audit_event();
CREATE POLICY storage_procedures_update ON storage_procedures FOR UPDATE TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('writer'))))
    WITH CHECK (id = ANY (ARRAY(SELECT authz.resources_with('writer'))));
CREATE POLICY storage_procedures_archived_update ON storage_procedures AS RESTRICTIVE FOR UPDATE TO app
    USING (NOT archived) WITH CHECK (NOT archived);

-- Keeps the storage destination slot singular.
CREATE FUNCTION check_storage_slots() RETURNS TRIGGER
LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
DECLARE
    stored_into_slot UUID;
BEGIN
    SELECT template.stored_into_slot_id INTO stored_into_slot
    FROM public.storage_procedure_templates template
    WHERE template.id = NEW.id;
    PERFORM public.require_single_typed_slot(stored_into_slot);
    RETURN NULL;
END;
$$;
ALTER FUNCTION check_storage_slots() OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION check_storage_slots() FROM PUBLIC;

-- Checks the storage destination slot after cardinality edits.
CREATE FUNCTION public.check_storage_slot_cardinality() RETURNS TRIGGER
LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
DECLARE
    stored_into_slot UUID;
BEGIN
    SELECT template.stored_into_slot_id INTO stored_into_slot
    FROM public.storage_procedure_templates template
    WHERE template.id = NEW.procedure_template_id AND template.stored_into_slot_id = NEW.id;
    PERFORM public.require_single_typed_slot(stored_into_slot);
    RETURN NULL;
END;
$$;
ALTER FUNCTION public.check_storage_slot_cardinality() OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION public.check_storage_slot_cardinality() FROM PUBLIC;
CREATE CONSTRAINT TRIGGER storage_template_slots AFTER INSERT OR UPDATE OF id, stored_into_slot_id, stored_into_model_id, stored_into_slot_role_id, stored_asset_slot_id, stored_asset_model_id, stored_asset_slot_role_id, temperature_k, temperature_tolerance_percentage ON storage_procedure_templates
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION check_storage_slots();
CREATE CONSTRAINT TRIGGER storage_slot_counts AFTER UPDATE OF minimum_count, maximum_count ON procedure_template_slots
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW WHEN (OLD.minimum_count IS DISTINCT FROM NEW.minimum_count OR OLD.maximum_count IS DISTINCT FROM NEW.maximum_count) EXECUTE FUNCTION public.check_storage_slot_cardinality();
