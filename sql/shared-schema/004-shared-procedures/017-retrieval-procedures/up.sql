-- Migration: 017-retrieval-procedures
-- Purpose: Typed retrieval slots and execution records.
-- APS placement
-- ownables
-- +-- procedure_templates -- retrieval_procedure_templates
-- +-- procedures -- retrieval_procedures
-- Slot | Role | Meaning
-- `retrieved_from` | `input` | The retrieval container remains in use.
-- `retrieved_asset` | `input` | The stored asset remains available.

-- Typed slots and parameters for retrieval.
CREATE TABLE retrieval_procedure_templates (
    -- Identity shared with the parent template.
    id UUID PRIMARY KEY REFERENCES procedure_templates (id) ON DELETE CASCADE,
    -- Organization containing the parent resource.
    organization_id UUID NOT NULL,
    -- Slot declaring the `retrieved_from` assets.
    retrieved_from_slot_id UUID NOT NULL,
    -- Admissible model for the `retrieved_from` slot.
    retrieved_from_model_id UUID NOT NULL REFERENCES container_models (id),
    -- Required role of the `retrieved_from` slot.
    retrieved_from_slot_role_id TEXT GENERATED ALWAYS AS ('input') STORED NOT NULL,
    -- Slot declaring the `retrieved_asset` assets.
    retrieved_asset_slot_id UUID NOT NULL,
    -- Admissible model for the `retrieved_asset` slot.
    retrieved_asset_model_id UUID NOT NULL REFERENCES physical_asset_models (id),
    -- Required role of the `retrieved_asset` slot.
    retrieved_asset_slot_role_id TEXT GENERATED ALWAYS AS ('input') STORED NOT NULL,
    CHECK (retrieved_from_slot_id <> retrieved_asset_slot_id),
    FOREIGN KEY (retrieved_from_slot_id, id, retrieved_from_model_id, retrieved_from_slot_role_id, organization_id)
        REFERENCES procedure_template_slots (id, procedure_template_id, asset_model_id, role_id, organization_id),
    FOREIGN KEY (retrieved_asset_slot_id, id, retrieved_asset_model_id, retrieved_asset_slot_role_id, organization_id)
        REFERENCES procedure_template_slots (id, procedure_template_id, asset_model_id, role_id, organization_id),
    FOREIGN KEY (retrieved_asset_slot_id, id, retrieved_from_slot_id, organization_id)
        REFERENCES procedure_template_slots (id, procedure_template_id, retrieved_from_slot_id, organization_id),
    FOREIGN KEY (organization_id, id) REFERENCES resources (organization_id, id) ON DELETE CASCADE
);
CREATE INDEX ON retrieval_procedure_templates (retrieved_asset_model_id);
CREATE INDEX ON retrieval_procedure_templates (retrieved_from_model_id, retrieved_asset_model_id);
INSERT INTO table_names (id) VALUES ('retrieval_procedure_templates');
INSERT INTO table_name_ancestors (descendant_table_name_id, ancestor_table_name_id)
    SELECT 'retrieval_procedure_templates', ancestor_table_name_id FROM table_name_ancestors WHERE descendant_table_name_id = 'procedure_templates'
    UNION SELECT 'retrieval_procedure_templates', 'retrieval_procedure_templates';
CREATE TRIGGER retrieval_procedure_templates_guard AFTER INSERT OR UPDATE ON retrieval_procedure_templates
    FOR EACH ROW EXECUTE FUNCTION guard_procedure_template_subtype();
CREATE TRIGGER retrieval_procedure_templates_delete_guard BEFORE DELETE ON retrieval_procedure_templates
    FOR EACH ROW EXECUTE FUNCTION guard_procedure_template_subtype();

CREATE FUNCTION require_retrieval_procedure_templates_row() RETURNS TRIGGER
LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
DECLARE
    row_id UUID := CASE TG_OP WHEN 'DELETE' THEN OLD.id ELSE NEW.id END;
BEGIN
    IF EXISTS (
        SELECT 1 FROM public.procedure_templates root
        JOIN public.table_name_ancestors path ON path.descendant_table_name_id = root.table_name_id
        WHERE root.id = row_id AND path.ancestor_table_name_id = 'retrieval_procedure_templates'
    ) AND NOT EXISTS (SELECT 1 FROM public.retrieval_procedure_templates row WHERE row.id = row_id) THEN
        RAISE EXCEPTION '% names a table extending % but has no row there', row_id, 'retrieval_procedure_templates'
            USING ERRCODE = 'foreign_key_violation';
    END IF;
    RETURN NULL;
END;
$$;
ALTER FUNCTION require_retrieval_procedure_templates_row() OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION require_retrieval_procedure_templates_row() FROM PUBLIC;
CREATE CONSTRAINT TRIGGER retrieval_procedure_templates_row AFTER INSERT ON procedure_templates
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW WHEN (NEW.table_name_id = 'retrieval_procedure_templates')
    EXECUTE FUNCTION require_retrieval_procedure_templates_row();
CREATE CONSTRAINT TRIGGER retrieval_procedure_templates_kept AFTER DELETE ON retrieval_procedure_templates
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION require_retrieval_procedure_templates_row();

GRANT SELECT, INSERT, DELETE ON retrieval_procedure_templates TO app;
ALTER TABLE retrieval_procedure_templates ENABLE ROW LEVEL SECURITY;
ALTER TABLE retrieval_procedure_templates FORCE ROW LEVEL SECURITY;
CREATE POLICY retrieval_procedure_templates_select ON retrieval_procedure_templates FOR SELECT TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY retrieval_procedure_templates_insert ON retrieval_procedure_templates FOR INSERT TO app
    WITH CHECK (id = ANY (ARRAY(SELECT authz.resources_with('writer'))) AND retrieved_from_model_id = ANY (ARRAY(SELECT authz.resources_with('reader'))) AND retrieved_asset_model_id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY retrieval_procedure_templates_delete ON retrieval_procedure_templates FOR DELETE TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('admin'))));
CREATE POLICY retrieval_procedure_templates_archived_insert ON retrieval_procedure_templates AS RESTRICTIVE FOR INSERT TO app
    WITH CHECK (NOT authz.is_archived(id) AND NOT authz.is_archived(retrieved_from_model_id) AND NOT authz.is_archived(retrieved_asset_model_id));
CREATE POLICY retrieval_procedure_templates_archived_delete ON retrieval_procedure_templates AS RESTRICTIVE FOR DELETE TO app
    USING (NOT authz.is_archived(id));
CREATE POLICY retrieval_procedure_templates_blocked ON retrieval_procedure_templates AS RESTRICTIVE FOR ALL TO app
    USING (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))))
    WITH CHECK (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))));

-- Recorded execution of a retrieval template.
CREATE TABLE retrieval_procedures (
    -- Identity shared with the parent procedure.
    id UUID PRIMARY KEY REFERENCES procedures (id) ON DELETE CASCADE,
    -- Organization containing the parent resource.
    organization_id UUID NOT NULL,
    -- Typed template executed by this procedure.
    retrieval_procedure_template_id UUID NOT NULL REFERENCES retrieval_procedure_templates (id),
    -- Execution status copied from the parent procedure.
    status_id TEXT NOT NULL,
    FOREIGN KEY (id, retrieval_procedure_template_id, status_id, organization_id)
        REFERENCES procedures (id, procedure_template_id, status_id, organization_id) ON DELETE CASCADE
        DEFERRABLE INITIALLY DEFERRED,
    FOREIGN KEY (organization_id, id) REFERENCES resources (organization_id, id) ON DELETE CASCADE
);
CREATE INDEX ON retrieval_procedures (retrieval_procedure_template_id);
INSERT INTO table_names (id) VALUES ('retrieval_procedures');
INSERT INTO table_name_instances (id, instance_table_name_id)
    VALUES ('retrieval_procedure_templates', 'retrieval_procedures');
INSERT INTO table_name_ancestors (descendant_table_name_id, ancestor_table_name_id)
    SELECT 'retrieval_procedures', ancestor_table_name_id FROM table_name_ancestors WHERE descendant_table_name_id = 'procedures'
    UNION SELECT 'retrieval_procedures', 'retrieval_procedures';
CREATE TRIGGER retrieval_procedures_guard AFTER INSERT OR UPDATE ON retrieval_procedures
    FOR EACH ROW EXECUTE FUNCTION guard_procedure_subtype();
CREATE TRIGGER retrieval_procedures_delete_guard BEFORE DELETE ON retrieval_procedures
    FOR EACH ROW EXECUTE FUNCTION guard_procedure_subtype();

-- Writes a procedure's new status into its `retrieval_procedures` row, whose deferred
-- foreign key holds it equal to the procedure at commit.
CREATE FUNCTION copy_retrieval_procedures_status() RETURNS TRIGGER
LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
BEGIN
    UPDATE public.retrieval_procedures SET status_id = NEW.status_id WHERE id = NEW.id;
    RETURN NULL;
END;
$$;
GRANT UPDATE (status_id) ON retrieval_procedures TO aps_writer;
ALTER FUNCTION copy_retrieval_procedures_status() OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION copy_retrieval_procedures_status() FROM PUBLIC;
CREATE TRIGGER procedures_status_copy_retrieval_procedures AFTER UPDATE OF status_id ON procedures
    FOR EACH ROW WHEN (OLD.status_id IS DISTINCT FROM NEW.status_id) EXECUTE FUNCTION copy_retrieval_procedures_status();

CREATE FUNCTION require_retrieval_procedures_row() RETURNS TRIGGER
LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
DECLARE
    row_id UUID := CASE TG_OP WHEN 'DELETE' THEN OLD.id ELSE NEW.id END;
BEGIN
    IF EXISTS (
        SELECT 1 FROM public.procedures root
        JOIN public.table_name_ancestors path ON path.descendant_table_name_id = root.table_name_id
        WHERE root.id = row_id AND path.ancestor_table_name_id = 'retrieval_procedures'
    ) AND NOT EXISTS (SELECT 1 FROM public.retrieval_procedures row WHERE row.id = row_id) THEN
        RAISE EXCEPTION '% names a table extending % but has no row there', row_id, 'retrieval_procedures'
            USING ERRCODE = 'foreign_key_violation';
    END IF;
    RETURN NULL;
END;
$$;
ALTER FUNCTION require_retrieval_procedures_row() OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION require_retrieval_procedures_row() FROM PUBLIC;
CREATE CONSTRAINT TRIGGER retrieval_procedures_row AFTER INSERT ON procedures
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW WHEN (NEW.table_name_id = 'retrieval_procedures')
    EXECUTE FUNCTION require_retrieval_procedures_row();
CREATE CONSTRAINT TRIGGER retrieval_procedures_kept AFTER DELETE ON retrieval_procedures
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION require_retrieval_procedures_row();

GRANT SELECT, INSERT, DELETE ON retrieval_procedures TO app;
ALTER TABLE retrieval_procedures ENABLE ROW LEVEL SECURITY;
ALTER TABLE retrieval_procedures FORCE ROW LEVEL SECURITY;
CREATE POLICY retrieval_procedures_select ON retrieval_procedures FOR SELECT TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY retrieval_procedures_insert ON retrieval_procedures FOR INSERT TO app
    WITH CHECK (id = ANY (ARRAY(SELECT authz.resources_with('writer'))) AND retrieval_procedure_template_id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY retrieval_procedures_delete ON retrieval_procedures FOR DELETE TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('admin'))));
CREATE POLICY retrieval_procedures_archived_insert ON retrieval_procedures AS RESTRICTIVE FOR INSERT TO app
    WITH CHECK (NOT authz.is_archived(id) AND NOT authz.is_archived(retrieval_procedure_template_id));
CREATE POLICY retrieval_procedures_archived_delete ON retrieval_procedures AS RESTRICTIVE FOR DELETE TO app
    USING (NOT authz.is_archived(id));
CREATE POLICY retrieval_procedures_blocked ON retrieval_procedures AS RESTRICTIVE FOR ALL TO app
    USING (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))))
    WITH CHECK (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))));

-- Records every change in connetto's `audit_events`.
CREATE TRIGGER retrieval_procedure_templates_audit AFTER INSERT OR DELETE ON retrieval_procedure_templates
    FOR EACH ROW EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER retrieval_procedure_templates_update_audit AFTER UPDATE ON retrieval_procedure_templates
    FOR EACH ROW WHEN (OLD IS DISTINCT FROM NEW) EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER retrieval_procedures_audit AFTER INSERT OR DELETE ON retrieval_procedures
    FOR EACH ROW EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER retrieval_procedures_update_audit AFTER UPDATE ON retrieval_procedures
    FOR EACH ROW WHEN (OLD IS DISTINCT FROM NEW) EXECUTE FUNCTION authz_write.record_audit_event();

-- Checks the typed slot contract after definition edits.
CREATE FUNCTION public.check_retrieval_slots() RETURNS TRIGGER
LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
DECLARE
    retrieved_from_slot UUID;
BEGIN
    FOR retrieved_from_slot IN
        SELECT template.retrieved_from_slot_id FROM public.retrieval_procedure_templates template
        WHERE template.id = NEW.id OR template.retrieved_from_slot_id = NEW.id
    LOOP
        PERFORM public.require_single_typed_slot(retrieved_from_slot);
    END LOOP;
    RETURN NULL;
END;
$$;
ALTER FUNCTION public.check_retrieval_slots() OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION public.check_retrieval_slots() FROM PUBLIC;
GRANT SELECT ON public.retrieval_procedure_templates, public.retrieval_procedures TO aps_writer;
CREATE CONSTRAINT TRIGGER retrieval_template_slots AFTER INSERT OR UPDATE ON public.retrieval_procedure_templates
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION public.check_retrieval_slots();
CREATE CONSTRAINT TRIGGER retrieval_slot_cardinality AFTER UPDATE ON public.procedure_template_slots
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW
    WHEN (OLD.minimum_count IS DISTINCT FROM NEW.minimum_count OR OLD.maximum_count IS DISTINCT FROM NEW.maximum_count)
    EXECUTE FUNCTION public.check_retrieval_slots();
