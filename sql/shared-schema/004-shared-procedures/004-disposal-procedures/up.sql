-- Migration: 004-disposal-procedures
-- Purpose: Typed disposal slots and execution records.
-- APS placement
-- ownables
-- +-- procedure_templates -- disposal_procedure_templates
-- +-- procedures -- disposal_procedures
-- Slot | Role | Meaning
-- `disposed_asset` | `consumed` | Disposal removes the asset from inventory.

-- Typed slots and parameters for disposal.
CREATE TABLE disposal_procedure_templates (
    -- Identity shared with the parent template.
    id UUID PRIMARY KEY REFERENCES procedure_templates (id) ON DELETE CASCADE,
    -- Organization containing the parent resource.
    organization_id UUID NOT NULL,
    -- Slot declaring the `disposed_asset` assets.
    disposed_asset_slot_id UUID NOT NULL,
    -- Admissible model for the `disposed_asset` slot.
    disposed_asset_model_id UUID NOT NULL REFERENCES physical_asset_models (id),
    -- Required role of the `disposed_asset` slot.
    disposed_asset_slot_role_id TEXT GENERATED ALWAYS AS ('consumed') STORED NOT NULL,
    FOREIGN KEY (disposed_asset_slot_id, id, disposed_asset_model_id, disposed_asset_slot_role_id, organization_id)
        REFERENCES procedure_template_slots (id, procedure_template_id, asset_model_id, role_id, organization_id),
    FOREIGN KEY (organization_id, id) REFERENCES resources (organization_id, id) ON DELETE CASCADE
);
CREATE INDEX ON disposal_procedure_templates (disposed_asset_model_id);
INSERT INTO table_names (id) VALUES ('disposal_procedure_templates');
INSERT INTO table_name_ancestors (descendant_table_name_id, ancestor_table_name_id)
    SELECT 'disposal_procedure_templates', ancestor_table_name_id FROM table_name_ancestors WHERE descendant_table_name_id = 'procedure_templates'
    UNION SELECT 'disposal_procedure_templates', 'disposal_procedure_templates';
CREATE TRIGGER disposal_procedure_templates_guard AFTER INSERT OR UPDATE ON disposal_procedure_templates
    FOR EACH ROW EXECUTE FUNCTION guard_procedure_template_subtype();
CREATE TRIGGER disposal_procedure_templates_delete_guard BEFORE DELETE ON disposal_procedure_templates
    FOR EACH ROW EXECUTE FUNCTION guard_procedure_template_subtype();

CREATE FUNCTION require_disposal_procedure_templates_row() RETURNS TRIGGER
LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
DECLARE
    row_id UUID := CASE TG_OP WHEN 'DELETE' THEN OLD.id ELSE NEW.id END;
BEGIN
    IF EXISTS (
        SELECT 1 FROM public.procedure_templates root
        JOIN public.table_name_ancestors path ON path.descendant_table_name_id = root.table_name_id
        WHERE root.id = row_id AND path.ancestor_table_name_id = 'disposal_procedure_templates'
    ) AND NOT EXISTS (SELECT 1 FROM public.disposal_procedure_templates row WHERE row.id = row_id) THEN
        RAISE EXCEPTION '% names a table extending % but has no row there', row_id, 'disposal_procedure_templates'
            USING ERRCODE = 'foreign_key_violation';
    END IF;
    RETURN NULL;
END;
$$;
ALTER FUNCTION require_disposal_procedure_templates_row() OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION require_disposal_procedure_templates_row() FROM PUBLIC;
CREATE CONSTRAINT TRIGGER disposal_procedure_templates_row AFTER INSERT ON procedure_templates
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW WHEN (NEW.table_name_id = 'disposal_procedure_templates')
    EXECUTE FUNCTION require_disposal_procedure_templates_row();
CREATE CONSTRAINT TRIGGER disposal_procedure_templates_kept AFTER DELETE ON disposal_procedure_templates
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION require_disposal_procedure_templates_row();

REVOKE ALL ON disposal_procedure_templates FROM PUBLIC, app, identity_writer;
GRANT SELECT, INSERT, DELETE ON disposal_procedure_templates TO app;
ALTER TABLE disposal_procedure_templates ENABLE ROW LEVEL SECURITY;
ALTER TABLE disposal_procedure_templates FORCE ROW LEVEL SECURITY;
CREATE POLICY disposal_procedure_templates_select ON disposal_procedure_templates FOR SELECT TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY disposal_procedure_templates_insert ON disposal_procedure_templates FOR INSERT TO app
    WITH CHECK (id = ANY (ARRAY(SELECT authz.resources_with('writer'))) AND disposed_asset_model_id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY disposal_procedure_templates_delete ON disposal_procedure_templates FOR DELETE TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('admin'))));
CREATE POLICY disposal_procedure_templates_archived_insert ON disposal_procedure_templates AS RESTRICTIVE FOR INSERT TO app
    WITH CHECK (NOT authz.is_archived(id) AND NOT authz.is_archived(disposed_asset_model_id));
CREATE POLICY disposal_procedure_templates_archived_delete ON disposal_procedure_templates AS RESTRICTIVE FOR DELETE TO app
    USING (NOT authz.is_archived(id));
CREATE POLICY disposal_procedure_templates_blocked ON disposal_procedure_templates AS RESTRICTIVE FOR ALL TO app
    USING (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))))
    WITH CHECK (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))));

-- Recorded execution of a disposal template.
CREATE TABLE disposal_procedures (
    -- Identity shared with the parent procedure.
    id UUID PRIMARY KEY REFERENCES procedures (id) ON DELETE CASCADE,
    -- Organization containing the parent resource.
    organization_id UUID NOT NULL,
    -- Typed template executed by this procedure.
    disposal_procedure_template_id UUID NOT NULL REFERENCES disposal_procedure_templates (id),
    -- Execution status copied from the parent procedure.
    status_id TEXT NOT NULL,
    FOREIGN KEY (id, disposal_procedure_template_id, status_id, organization_id)
        REFERENCES procedures (id, procedure_template_id, status_id, organization_id) ON DELETE CASCADE
        DEFERRABLE INITIALLY DEFERRED,
    FOREIGN KEY (organization_id, id) REFERENCES resources (organization_id, id) ON DELETE CASCADE
);
CREATE INDEX ON disposal_procedures (disposal_procedure_template_id);
INSERT INTO table_names (id) VALUES ('disposal_procedures');
INSERT INTO table_name_instances (id, instance_table_name_id)
    VALUES ('disposal_procedure_templates', 'disposal_procedures');
INSERT INTO table_name_ancestors (descendant_table_name_id, ancestor_table_name_id)
    SELECT 'disposal_procedures', ancestor_table_name_id FROM table_name_ancestors WHERE descendant_table_name_id = 'procedures'
    UNION SELECT 'disposal_procedures', 'disposal_procedures';
CREATE TRIGGER disposal_procedures_guard AFTER INSERT OR UPDATE ON disposal_procedures
    FOR EACH ROW EXECUTE FUNCTION guard_procedure_subtype();
CREATE TRIGGER disposal_procedures_delete_guard BEFORE DELETE ON disposal_procedures
    FOR EACH ROW EXECUTE FUNCTION guard_procedure_subtype();

-- Writes a procedure's new status into its `disposal_procedures` row, whose deferred
-- foreign key holds it equal to the procedure at commit.
CREATE FUNCTION copy_disposal_procedures_status() RETURNS TRIGGER
LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
BEGIN
    UPDATE public.disposal_procedures SET status_id = NEW.status_id WHERE id = NEW.id;
    RETURN NULL;
END;
$$;
REVOKE ALL ON disposal_procedures FROM PUBLIC, app, identity_writer;
GRANT UPDATE (status_id) ON disposal_procedures TO aps_writer;
ALTER FUNCTION copy_disposal_procedures_status() OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION copy_disposal_procedures_status() FROM PUBLIC;
CREATE TRIGGER procedures_status_copy_disposal_procedures AFTER UPDATE OF status_id ON procedures
    FOR EACH ROW WHEN (OLD.status_id IS DISTINCT FROM NEW.status_id) EXECUTE FUNCTION copy_disposal_procedures_status();

CREATE FUNCTION require_disposal_procedures_row() RETURNS TRIGGER
LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
DECLARE
    row_id UUID := CASE TG_OP WHEN 'DELETE' THEN OLD.id ELSE NEW.id END;
BEGIN
    IF EXISTS (
        SELECT 1 FROM public.procedures root
        JOIN public.table_name_ancestors path ON path.descendant_table_name_id = root.table_name_id
        WHERE root.id = row_id AND path.ancestor_table_name_id = 'disposal_procedures'
    ) AND NOT EXISTS (SELECT 1 FROM public.disposal_procedures row WHERE row.id = row_id) THEN
        RAISE EXCEPTION '% names a table extending % but has no row there', row_id, 'disposal_procedures'
            USING ERRCODE = 'foreign_key_violation';
    END IF;
    RETURN NULL;
END;
$$;
ALTER FUNCTION require_disposal_procedures_row() OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION require_disposal_procedures_row() FROM PUBLIC;
CREATE CONSTRAINT TRIGGER disposal_procedures_row AFTER INSERT ON procedures
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW WHEN (NEW.table_name_id = 'disposal_procedures')
    EXECUTE FUNCTION require_disposal_procedures_row();
CREATE CONSTRAINT TRIGGER disposal_procedures_kept AFTER DELETE ON disposal_procedures
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION require_disposal_procedures_row();

GRANT SELECT, INSERT, DELETE ON disposal_procedures TO app;
ALTER TABLE disposal_procedures ENABLE ROW LEVEL SECURITY;
ALTER TABLE disposal_procedures FORCE ROW LEVEL SECURITY;
CREATE POLICY disposal_procedures_select ON disposal_procedures FOR SELECT TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY disposal_procedures_insert ON disposal_procedures FOR INSERT TO app
    WITH CHECK (id = ANY (ARRAY(SELECT authz.resources_with('writer'))) AND disposal_procedure_template_id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY disposal_procedures_delete ON disposal_procedures FOR DELETE TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('admin'))));
CREATE POLICY disposal_procedures_archived_insert ON disposal_procedures AS RESTRICTIVE FOR INSERT TO app
    WITH CHECK (NOT authz.is_archived(id) AND NOT authz.is_archived(disposal_procedure_template_id));
CREATE POLICY disposal_procedures_archived_delete ON disposal_procedures AS RESTRICTIVE FOR DELETE TO app
    USING (NOT authz.is_archived(id));
CREATE POLICY disposal_procedures_blocked ON disposal_procedures AS RESTRICTIVE FOR ALL TO app
    USING (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))))
    WITH CHECK (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))));

-- Records every change in connetto's `audit_events`.
CREATE TRIGGER disposal_procedure_templates_audit AFTER INSERT OR DELETE ON disposal_procedure_templates
    FOR EACH ROW EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER disposal_procedure_templates_update_audit AFTER UPDATE ON disposal_procedure_templates
    FOR EACH ROW WHEN (OLD IS DISTINCT FROM NEW) EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER disposal_procedures_audit AFTER INSERT OR DELETE ON disposal_procedures
    FOR EACH ROW EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER disposal_procedures_update_audit AFTER UPDATE ON disposal_procedures
    FOR EACH ROW WHEN (OLD IS DISTINCT FROM NEW) EXECUTE FUNCTION authz_write.record_audit_event();
