-- Migration: 003-capping-procedures
-- Purpose: Typed container sealing slots and execution records.
-- APS placement
-- ownables
-- +-- procedure_templates -- container_sealing_procedure_templates
-- +-- procedures -- container_sealing_procedures
-- Slot | Role | Meaning
-- `sealable_container` | `input` | The container remains after sealing.
-- `sealed_with` | `input` | The cap remains attached to the container.

-- Typed slots and parameters for container sealing.
CREATE TABLE container_sealing_procedure_templates (
    -- Identity shared with the parent template.
    id UUID PRIMARY KEY REFERENCES procedure_templates (id) ON DELETE CASCADE,
    -- Organization containing the parent resource.
    organization_id UUID NOT NULL,
    -- Slot declaring the `sealable_container` assets.
    sealable_container_slot_id UUID NOT NULL,
    -- Admissible model for the `sealable_container` slot.
    sealable_container_model_id UUID NOT NULL REFERENCES volumetric_container_models (id),
    -- Required role of the `sealable_container` slot.
    sealable_container_slot_role_id TEXT GENERATED ALWAYS AS ('input') STORED NOT NULL,
    -- Slot declaring the `sealed_with` assets.
    sealed_with_slot_id UUID NOT NULL,
    -- Admissible model for the `sealed_with` slot.
    sealed_with_model_id UUID NOT NULL REFERENCES container_sealer_models (id),
    -- Required role of the `sealed_with` slot.
    sealed_with_slot_role_id TEXT GENERATED ALWAYS AS ('input') STORED NOT NULL,
    FOREIGN KEY (sealable_container_slot_id, id, sealable_container_model_id, sealable_container_slot_role_id, organization_id)
        REFERENCES procedure_template_slots (id, procedure_template_id, asset_model_id, role_id, organization_id),
    FOREIGN KEY (sealed_with_slot_id, id, sealed_with_model_id, sealed_with_slot_role_id, organization_id)
        REFERENCES procedure_template_slots (id, procedure_template_id, asset_model_id, role_id, organization_id),
    FOREIGN KEY (organization_id, id) REFERENCES resources (organization_id, id) ON DELETE CASCADE
);
CREATE INDEX ON container_sealing_procedure_templates (sealable_container_model_id);
CREATE INDEX ON container_sealing_procedure_templates (sealed_with_model_id);
INSERT INTO table_names (id) VALUES ('container_sealing_procedure_templates');
INSERT INTO table_name_ancestors (descendant_table_name_id, ancestor_table_name_id)
    SELECT 'container_sealing_procedure_templates', ancestor_table_name_id FROM table_name_ancestors WHERE descendant_table_name_id = 'procedure_templates'
    UNION SELECT 'container_sealing_procedure_templates', 'container_sealing_procedure_templates';
CREATE TRIGGER container_sealing_procedure_templates_guard AFTER INSERT OR UPDATE ON container_sealing_procedure_templates
    FOR EACH ROW EXECUTE FUNCTION guard_procedure_template_subtype();
CREATE TRIGGER container_sealing_procedure_templates_delete_guard BEFORE DELETE ON container_sealing_procedure_templates
    FOR EACH ROW EXECUTE FUNCTION guard_procedure_template_subtype();

CREATE FUNCTION require_container_sealing_procedure_templates_row() RETURNS TRIGGER
LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
DECLARE
    row_id UUID := CASE TG_OP WHEN 'DELETE' THEN OLD.id ELSE NEW.id END;
BEGIN
    IF EXISTS (
        SELECT 1 FROM public.procedure_templates root
        JOIN public.table_name_ancestors path ON path.descendant_table_name_id = root.table_name_id
        WHERE root.id = row_id AND path.ancestor_table_name_id = 'container_sealing_procedure_templates'
    ) AND NOT EXISTS (SELECT 1 FROM public.container_sealing_procedure_templates row WHERE row.id = row_id) THEN
        RAISE EXCEPTION '% names a table extending % but has no row there', row_id, 'container_sealing_procedure_templates'
            USING ERRCODE = 'foreign_key_violation';
    END IF;
    RETURN NULL;
END;
$$;
ALTER FUNCTION require_container_sealing_procedure_templates_row() OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION require_container_sealing_procedure_templates_row() FROM PUBLIC;
CREATE CONSTRAINT TRIGGER container_sealing_procedure_templates_row AFTER INSERT ON procedure_templates
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW WHEN (NEW.table_name_id = 'container_sealing_procedure_templates')
    EXECUTE FUNCTION require_container_sealing_procedure_templates_row();
CREATE CONSTRAINT TRIGGER container_sealing_procedure_templates_kept AFTER DELETE ON container_sealing_procedure_templates
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION require_container_sealing_procedure_templates_row();

GRANT SELECT, INSERT, DELETE ON container_sealing_procedure_templates TO app;
ALTER TABLE container_sealing_procedure_templates ENABLE ROW LEVEL SECURITY;
ALTER TABLE container_sealing_procedure_templates FORCE ROW LEVEL SECURITY;
CREATE POLICY container_sealing_procedure_templates_select ON container_sealing_procedure_templates FOR SELECT TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY container_sealing_procedure_templates_insert ON container_sealing_procedure_templates FOR INSERT TO app
    WITH CHECK (id = ANY (ARRAY(SELECT authz.resources_with('writer'))) AND sealable_container_model_id = ANY (ARRAY(SELECT authz.resources_with('reader'))) AND sealed_with_model_id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY container_sealing_procedure_templates_delete ON container_sealing_procedure_templates FOR DELETE TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('admin'))));
CREATE POLICY container_sealing_procedure_templates_archived_insert ON container_sealing_procedure_templates AS RESTRICTIVE FOR INSERT TO app
    WITH CHECK (NOT authz.is_archived(id) AND NOT authz.is_archived(sealable_container_model_id) AND NOT authz.is_archived(sealed_with_model_id));
CREATE POLICY container_sealing_procedure_templates_archived_delete ON container_sealing_procedure_templates AS RESTRICTIVE FOR DELETE TO app
    USING (NOT authz.is_archived(id));
CREATE POLICY container_sealing_procedure_templates_blocked ON container_sealing_procedure_templates AS RESTRICTIVE FOR ALL TO app
    USING (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))))
    WITH CHECK (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))));

-- Recorded execution of a container sealing template.
CREATE TABLE container_sealing_procedures (
    -- Identity shared with the parent procedure.
    id UUID PRIMARY KEY REFERENCES procedures (id) ON DELETE CASCADE,
    -- Organization containing the parent resource.
    organization_id UUID NOT NULL,
    -- Typed template executed by this procedure.
    container_sealing_procedure_template_id UUID NOT NULL REFERENCES container_sealing_procedure_templates (id),
    -- Execution status copied from the parent procedure.
    status_id TEXT NOT NULL,
    FOREIGN KEY (id, container_sealing_procedure_template_id, status_id, organization_id)
        REFERENCES procedures (id, procedure_template_id, status_id, organization_id) ON UPDATE CASCADE ON DELETE CASCADE,
    FOREIGN KEY (organization_id, id) REFERENCES resources (organization_id, id) ON DELETE CASCADE
);
CREATE INDEX ON container_sealing_procedures (container_sealing_procedure_template_id);
INSERT INTO table_names (id) VALUES ('container_sealing_procedures');
INSERT INTO table_name_instances (id, instance_table_name_id)
    VALUES ('container_sealing_procedure_templates', 'container_sealing_procedures');
INSERT INTO table_name_ancestors (descendant_table_name_id, ancestor_table_name_id)
    SELECT 'container_sealing_procedures', ancestor_table_name_id FROM table_name_ancestors WHERE descendant_table_name_id = 'procedures'
    UNION SELECT 'container_sealing_procedures', 'container_sealing_procedures';
CREATE TRIGGER container_sealing_procedures_guard AFTER INSERT OR UPDATE ON container_sealing_procedures
    FOR EACH ROW EXECUTE FUNCTION guard_procedure_subtype();
CREATE TRIGGER container_sealing_procedures_delete_guard BEFORE DELETE ON container_sealing_procedures
    FOR EACH ROW EXECUTE FUNCTION guard_procedure_subtype();

CREATE FUNCTION require_container_sealing_procedures_row() RETURNS TRIGGER
LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
DECLARE
    row_id UUID := CASE TG_OP WHEN 'DELETE' THEN OLD.id ELSE NEW.id END;
BEGIN
    IF EXISTS (
        SELECT 1 FROM public.procedures root
        JOIN public.table_name_ancestors path ON path.descendant_table_name_id = root.table_name_id
        WHERE root.id = row_id AND path.ancestor_table_name_id = 'container_sealing_procedures'
    ) AND NOT EXISTS (SELECT 1 FROM public.container_sealing_procedures row WHERE row.id = row_id) THEN
        RAISE EXCEPTION '% names a table extending % but has no row there', row_id, 'container_sealing_procedures'
            USING ERRCODE = 'foreign_key_violation';
    END IF;
    RETURN NULL;
END;
$$;
ALTER FUNCTION require_container_sealing_procedures_row() OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION require_container_sealing_procedures_row() FROM PUBLIC;
CREATE CONSTRAINT TRIGGER container_sealing_procedures_row AFTER INSERT ON procedures
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW WHEN (NEW.table_name_id = 'container_sealing_procedures')
    EXECUTE FUNCTION require_container_sealing_procedures_row();
CREATE CONSTRAINT TRIGGER container_sealing_procedures_kept AFTER DELETE ON container_sealing_procedures
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION require_container_sealing_procedures_row();

GRANT SELECT, INSERT, DELETE ON container_sealing_procedures TO app;
ALTER TABLE container_sealing_procedures ENABLE ROW LEVEL SECURITY;
ALTER TABLE container_sealing_procedures FORCE ROW LEVEL SECURITY;
CREATE POLICY container_sealing_procedures_select ON container_sealing_procedures FOR SELECT TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY container_sealing_procedures_insert ON container_sealing_procedures FOR INSERT TO app
    WITH CHECK (id = ANY (ARRAY(SELECT authz.resources_with('writer'))) AND container_sealing_procedure_template_id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY container_sealing_procedures_delete ON container_sealing_procedures FOR DELETE TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('admin'))));
CREATE POLICY container_sealing_procedures_archived_insert ON container_sealing_procedures AS RESTRICTIVE FOR INSERT TO app
    WITH CHECK (NOT authz.is_archived(id) AND NOT authz.is_archived(container_sealing_procedure_template_id));
CREATE POLICY container_sealing_procedures_archived_delete ON container_sealing_procedures AS RESTRICTIVE FOR DELETE TO app
    USING (NOT authz.is_archived(id));
CREATE POLICY container_sealing_procedures_blocked ON container_sealing_procedures AS RESTRICTIVE FOR ALL TO app
    USING (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))))
    WITH CHECK (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))));

-- Records every change in connetto's `audit_events`.
CREATE TRIGGER container_sealing_procedure_templates_audit AFTER INSERT OR DELETE ON container_sealing_procedure_templates
    FOR EACH ROW EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER container_sealing_procedure_templates_update_audit AFTER UPDATE ON container_sealing_procedure_templates
    FOR EACH ROW WHEN (OLD IS DISTINCT FROM NEW) EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER container_sealing_procedures_audit AFTER INSERT OR DELETE ON container_sealing_procedures
    FOR EACH ROW EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER container_sealing_procedures_update_audit AFTER UPDATE ON container_sealing_procedures
    FOR EACH ROW WHEN (OLD IS DISTINCT FROM NEW) EXECUTE FUNCTION authz_write.record_audit_event();
