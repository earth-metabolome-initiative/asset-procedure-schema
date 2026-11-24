-- Migration: 001-weighing-procedures
-- Purpose: Typed weighing slots and execution records.
-- APS placement
-- ownables
-- +-- procedure_templates -- weighing_procedure_templates
-- +-- procedures -- weighing_procedures
-- Slot | Role | Meaning
-- `weighed_asset` | `input` | The weighed asset survives measurement.
-- `weighed_with` | `instrument` | The balance measures mass.

-- Typed slots and parameters for weighing.
CREATE TABLE weighing_procedure_templates (
    -- Identity shared with the parent template.
    id UUID PRIMARY KEY REFERENCES procedure_templates (id) ON DELETE CASCADE,
    -- Organization containing the parent resource.
    organization_id UUID NOT NULL,
    -- Slot declaring the `weighed_asset` assets.
    weighed_asset_slot_id UUID NOT NULL,
    -- Admissible model for the `weighed_asset` slot.
    weighed_asset_model_id UUID NOT NULL REFERENCES physical_asset_models (id),
    -- Required role of the `weighed_asset` slot.
    weighed_asset_slot_role_id TEXT GENERATED ALWAYS AS ('input') STORED NOT NULL,
    -- Slot declaring the `weighed_with` assets.
    weighed_with_slot_id UUID NOT NULL,
    -- Admissible model for the `weighed_with` slot.
    weighed_with_model_id UUID NOT NULL REFERENCES weighing_device_models (id),
    -- Required role of the `weighed_with` slot.
    weighed_with_slot_role_id TEXT GENERATED ALWAYS AS ('instrument') STORED NOT NULL,
    FOREIGN KEY (weighed_asset_slot_id, id, weighed_asset_model_id, weighed_asset_slot_role_id, organization_id)
        REFERENCES procedure_template_slots (id, procedure_template_id, asset_model_id, role_id, organization_id),
    FOREIGN KEY (weighed_with_slot_id, id, weighed_with_model_id, weighed_with_slot_role_id, organization_id)
        REFERENCES procedure_template_slots (id, procedure_template_id, asset_model_id, role_id, organization_id),
    FOREIGN KEY (organization_id, id) REFERENCES resources (organization_id, id) ON DELETE CASCADE
);
CREATE INDEX ON weighing_procedure_templates (weighed_asset_model_id);
CREATE INDEX ON weighing_procedure_templates (weighed_with_model_id);
INSERT INTO table_names (id) VALUES ('weighing_procedure_templates');
INSERT INTO table_name_ancestors (descendant_table_name_id, ancestor_table_name_id)
    SELECT 'weighing_procedure_templates', ancestor_table_name_id FROM table_name_ancestors WHERE descendant_table_name_id = 'procedure_templates'
    UNION SELECT 'weighing_procedure_templates', 'weighing_procedure_templates';
CREATE TRIGGER weighing_procedure_templates_guard AFTER INSERT OR UPDATE ON weighing_procedure_templates
    FOR EACH ROW EXECUTE FUNCTION guard_procedure_template_subtype();
CREATE TRIGGER weighing_procedure_templates_delete_guard BEFORE DELETE ON weighing_procedure_templates
    FOR EACH ROW EXECUTE FUNCTION guard_procedure_template_subtype();

CREATE FUNCTION require_weighing_procedure_templates_row() RETURNS TRIGGER
LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
DECLARE
    row_id UUID := CASE TG_OP WHEN 'DELETE' THEN OLD.id ELSE NEW.id END;
BEGIN
    IF EXISTS (
        SELECT 1 FROM public.procedure_templates root
        JOIN public.table_name_ancestors path ON path.descendant_table_name_id = root.table_name_id
        WHERE root.id = row_id AND path.ancestor_table_name_id = 'weighing_procedure_templates'
    ) AND NOT EXISTS (SELECT 1 FROM public.weighing_procedure_templates row WHERE row.id = row_id) THEN
        RAISE EXCEPTION '% names a table extending % but has no row there', row_id, 'weighing_procedure_templates'
            USING ERRCODE = 'foreign_key_violation';
    END IF;
    RETURN NULL;
END;
$$;
ALTER FUNCTION require_weighing_procedure_templates_row() OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION require_weighing_procedure_templates_row() FROM PUBLIC;
CREATE CONSTRAINT TRIGGER weighing_procedure_templates_row AFTER INSERT ON procedure_templates
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW WHEN (NEW.table_name_id = 'weighing_procedure_templates')
    EXECUTE FUNCTION require_weighing_procedure_templates_row();
CREATE CONSTRAINT TRIGGER weighing_procedure_templates_kept AFTER DELETE ON weighing_procedure_templates
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION require_weighing_procedure_templates_row();

GRANT SELECT, INSERT, DELETE ON weighing_procedure_templates TO app;
ALTER TABLE weighing_procedure_templates ENABLE ROW LEVEL SECURITY;
ALTER TABLE weighing_procedure_templates FORCE ROW LEVEL SECURITY;
CREATE POLICY weighing_procedure_templates_select ON weighing_procedure_templates FOR SELECT TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY weighing_procedure_templates_insert ON weighing_procedure_templates FOR INSERT TO app
    WITH CHECK (id = ANY (ARRAY(SELECT authz.resources_with('writer'))) AND weighed_asset_model_id = ANY (ARRAY(SELECT authz.resources_with('reader'))) AND weighed_with_model_id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY weighing_procedure_templates_delete ON weighing_procedure_templates FOR DELETE TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('admin'))));
CREATE POLICY weighing_procedure_templates_archived_insert ON weighing_procedure_templates AS RESTRICTIVE FOR INSERT TO app
    WITH CHECK (NOT authz.is_archived(id) AND NOT authz.is_archived(weighed_asset_model_id) AND NOT authz.is_archived(weighed_with_model_id));
CREATE POLICY weighing_procedure_templates_archived_delete ON weighing_procedure_templates AS RESTRICTIVE FOR DELETE TO app
    USING (NOT authz.is_archived(id));
CREATE POLICY weighing_procedure_templates_blocked ON weighing_procedure_templates AS RESTRICTIVE FOR ALL TO app
    USING (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))))
    WITH CHECK (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))));

-- Recorded execution of a weighing template.
CREATE TABLE weighing_procedures (
    -- Identity shared with the parent procedure.
    id UUID PRIMARY KEY REFERENCES procedures (id) ON DELETE CASCADE,
    -- Organization containing the parent resource.
    organization_id UUID NOT NULL,
    -- Typed template executed by this procedure.
    weighing_procedure_template_id UUID NOT NULL REFERENCES weighing_procedure_templates (id),
    -- Execution status copied from the parent procedure.
    status_id TEXT NOT NULL,
    -- Mass in kilograms.
    mass_kg DOUBLE PRECISION CHECK (mass_kg > 0.0 AND mass_kg < 'Infinity'::double precision),
    FOREIGN KEY (id, weighing_procedure_template_id, status_id, organization_id)
        REFERENCES procedures (id, procedure_template_id, status_id, organization_id) ON DELETE CASCADE
        DEFERRABLE INITIALLY DEFERRED,
    CHECK (status_id <> 'completed' OR mass_kg IS NOT NULL),
    FOREIGN KEY (organization_id, id) REFERENCES resources (organization_id, id) ON DELETE CASCADE
);
CREATE INDEX ON weighing_procedures (weighing_procedure_template_id);
INSERT INTO table_names (id) VALUES ('weighing_procedures');
INSERT INTO table_name_instances (id, instance_table_name_id)
    VALUES ('weighing_procedure_templates', 'weighing_procedures');
INSERT INTO table_name_ancestors (descendant_table_name_id, ancestor_table_name_id)
    SELECT 'weighing_procedures', ancestor_table_name_id FROM table_name_ancestors WHERE descendant_table_name_id = 'procedures'
    UNION SELECT 'weighing_procedures', 'weighing_procedures';
CREATE TRIGGER weighing_procedures_guard AFTER INSERT OR UPDATE ON weighing_procedures
    FOR EACH ROW EXECUTE FUNCTION guard_procedure_subtype();
CREATE TRIGGER weighing_procedures_delete_guard BEFORE DELETE ON weighing_procedures
    FOR EACH ROW EXECUTE FUNCTION guard_procedure_subtype();

-- Writes a procedure's new status into its `weighing_procedures` row, whose deferred
-- foreign key holds it equal to the procedure at commit.
CREATE FUNCTION copy_weighing_procedures_status() RETURNS TRIGGER
LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
BEGIN
    UPDATE public.weighing_procedures SET status_id = NEW.status_id WHERE id = NEW.id;
    RETURN NULL;
END;
$$;
GRANT UPDATE (status_id) ON weighing_procedures TO aps_writer;
ALTER FUNCTION copy_weighing_procedures_status() OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION copy_weighing_procedures_status() FROM PUBLIC;
CREATE TRIGGER procedures_status_copy_weighing_procedures AFTER UPDATE OF status_id ON procedures
    FOR EACH ROW WHEN (OLD.status_id IS DISTINCT FROM NEW.status_id) EXECUTE FUNCTION copy_weighing_procedures_status();

CREATE FUNCTION require_weighing_procedures_row() RETURNS TRIGGER
LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
DECLARE
    row_id UUID := CASE TG_OP WHEN 'DELETE' THEN OLD.id ELSE NEW.id END;
BEGIN
    IF EXISTS (
        SELECT 1 FROM public.procedures root
        JOIN public.table_name_ancestors path ON path.descendant_table_name_id = root.table_name_id
        WHERE root.id = row_id AND path.ancestor_table_name_id = 'weighing_procedures'
    ) AND NOT EXISTS (SELECT 1 FROM public.weighing_procedures row WHERE row.id = row_id) THEN
        RAISE EXCEPTION '% names a table extending % but has no row there', row_id, 'weighing_procedures'
            USING ERRCODE = 'foreign_key_violation';
    END IF;
    RETURN NULL;
END;
$$;
ALTER FUNCTION require_weighing_procedures_row() OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION require_weighing_procedures_row() FROM PUBLIC;
CREATE CONSTRAINT TRIGGER weighing_procedures_row AFTER INSERT ON procedures
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW WHEN (NEW.table_name_id = 'weighing_procedures')
    EXECUTE FUNCTION require_weighing_procedures_row();
CREATE CONSTRAINT TRIGGER weighing_procedures_kept AFTER DELETE ON weighing_procedures
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION require_weighing_procedures_row();

GRANT SELECT, INSERT, UPDATE (mass_kg), DELETE ON weighing_procedures TO app;
ALTER TABLE weighing_procedures ENABLE ROW LEVEL SECURITY;
ALTER TABLE weighing_procedures FORCE ROW LEVEL SECURITY;
CREATE POLICY weighing_procedures_select ON weighing_procedures FOR SELECT TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY weighing_procedures_insert ON weighing_procedures FOR INSERT TO app
    WITH CHECK (id = ANY (ARRAY(SELECT authz.resources_with('writer'))) AND weighing_procedure_template_id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY weighing_procedures_update ON weighing_procedures FOR UPDATE TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('writer'))))
    WITH CHECK (id = ANY (ARRAY(SELECT authz.resources_with('writer'))));
CREATE POLICY weighing_procedures_delete ON weighing_procedures FOR DELETE TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('admin'))));
CREATE POLICY weighing_procedures_archived_insert ON weighing_procedures AS RESTRICTIVE FOR INSERT TO app
    WITH CHECK (NOT authz.is_archived(id) AND NOT authz.is_archived(weighing_procedure_template_id));
CREATE POLICY weighing_procedures_archived_update ON weighing_procedures AS RESTRICTIVE FOR UPDATE TO app
    USING (NOT authz.is_archived(id)) WITH CHECK (NOT authz.is_archived(id));
CREATE POLICY weighing_procedures_archived_delete ON weighing_procedures AS RESTRICTIVE FOR DELETE TO app
    USING (NOT authz.is_archived(id));
CREATE POLICY weighing_procedures_blocked ON weighing_procedures AS RESTRICTIVE FOR ALL TO app
    USING (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))))
    WITH CHECK (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))));

-- Records every change in connetto's `audit_events`.
CREATE TRIGGER weighing_procedure_templates_audit AFTER INSERT OR DELETE ON weighing_procedure_templates
    FOR EACH ROW EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER weighing_procedure_templates_update_audit AFTER UPDATE ON weighing_procedure_templates
    FOR EACH ROW WHEN (OLD IS DISTINCT FROM NEW) EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER weighing_procedures_audit AFTER INSERT OR DELETE ON weighing_procedures
    FOR EACH ROW EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER weighing_procedures_update_audit AFTER UPDATE ON weighing_procedures
    FOR EACH ROW WHEN (OLD IS DISTINCT FROM NEW) EXECUTE FUNCTION authz_write.record_audit_event();

-- Requires a typed slot to bind exactly one asset.
CREATE FUNCTION public.require_single_typed_slot(slot_identifier UUID) RETURNS VOID
LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
BEGIN
    IF EXISTS (SELECT 1 FROM public.procedure_template_slots slot
        WHERE slot.id = slot_identifier AND (slot.minimum_count <> 1 OR slot.maximum_count IS DISTINCT FROM 1)) THEN
        RAISE EXCEPTION 'typed slot requires exactly one asset' USING ERRCODE = 'check_violation';
    END IF;
END;
$$;
ALTER FUNCTION public.require_single_typed_slot(UUID) OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION public.require_single_typed_slot(UUID) FROM PUBLIC;

-- Checks the typed slot contract after definition edits.
CREATE FUNCTION public.check_weighing_slots() RETURNS TRIGGER
LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
DECLARE
    weighed_asset_slot UUID;
BEGIN
    FOR weighed_asset_slot IN
        SELECT template.weighed_asset_slot_id FROM public.weighing_procedure_templates template
        WHERE template.id = NEW.id OR template.weighed_asset_slot_id = NEW.id
    LOOP
        PERFORM public.require_single_typed_slot(weighed_asset_slot);
    END LOOP;
    RETURN NULL;
END;
$$;
ALTER FUNCTION public.check_weighing_slots() OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION public.check_weighing_slots() FROM PUBLIC;
GRANT SELECT ON public.weighing_procedure_templates TO aps_writer;
CREATE CONSTRAINT TRIGGER weighing_template_slots AFTER INSERT OR UPDATE ON public.weighing_procedure_templates
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION public.check_weighing_slots();
CREATE CONSTRAINT TRIGGER weighing_slot_cardinality AFTER UPDATE ON public.procedure_template_slots
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW
    WHEN (OLD.minimum_count IS DISTINCT FROM NEW.minimum_count OR OLD.maximum_count IS DISTINCT FROM NEW.maximum_count)
    EXECUTE FUNCTION public.check_weighing_slots();
