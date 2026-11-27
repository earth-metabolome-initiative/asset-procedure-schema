-- Migration: 006-photograph-procedures
-- Purpose: Typed photograph slots and execution records.
-- APS placement
-- `resources` of kind `'ownable'`
-- +-- procedure_templates -- photograph_procedure_templates
-- +-- procedures -- photograph_procedures
-- Slot | Role | Meaning
-- `photographed_with` | `instrument` | The camera takes the photograph.
-- `photographed_asset` | `input` | The subject survives photography.
-- `photograph` | `produced` | The procedure creates the photograph.

-- Typed slots and parameters for photograph.
CREATE TABLE photograph_procedure_templates (
    -- Identity shared with the parent template.
    id UUID PRIMARY KEY REFERENCES procedure_templates (id) ON DELETE CASCADE,
    -- Organization containing the parent resource.
    organization_id UUID NOT NULL,
    -- Slot declaring the `photographed_with` assets.
    photographed_with_slot_id UUID NOT NULL,
    -- Admissible model for the `photographed_with` slot.
    photographed_with_model_id UUID NOT NULL REFERENCES camera_models (id),
    -- Required role of the `photographed_with` slot.
    photographed_with_slot_role_id TEXT GENERATED ALWAYS AS ('instrument') STORED NOT NULL,
    -- Slot declaring the `photographed_asset` assets.
    photographed_asset_slot_id UUID NOT NULL,
    -- Admissible model for the `photographed_asset` slot.
    photographed_asset_model_id UUID NOT NULL REFERENCES physical_asset_models (id),
    -- Required role of the `photographed_asset` slot.
    photographed_asset_slot_role_id TEXT GENERATED ALWAYS AS ('input') STORED NOT NULL,
    -- Slot declaring the `photograph` assets.
    photograph_slot_id UUID NOT NULL,
    -- Admissible model for the `photograph` slot.
    photograph_model_id UUID NOT NULL REFERENCES photograph_models (id),
    -- Required role of the `photograph` slot.
    photograph_slot_role_id TEXT GENERATED ALWAYS AS ('produced') STORED NOT NULL,
    FOREIGN KEY (photographed_with_slot_id, id, photographed_with_model_id, photographed_with_slot_role_id, organization_id)
        REFERENCES procedure_template_slots (id, procedure_template_id, asset_model_id, role_id, organization_id),
    FOREIGN KEY (photographed_asset_slot_id, id, photographed_asset_model_id, photographed_asset_slot_role_id, organization_id)
        REFERENCES procedure_template_slots (id, procedure_template_id, asset_model_id, role_id, organization_id),
    FOREIGN KEY (photograph_slot_id, id, photograph_model_id, photograph_slot_role_id, organization_id)
        REFERENCES procedure_template_slots (id, procedure_template_id, asset_model_id, role_id, organization_id),
    archived BOOLEAN NOT NULL DEFAULT FALSE,
    photographed_with_model_archived BOOLEAN NOT NULL DEFAULT FALSE,
    photographed_asset_model_archived BOOLEAN NOT NULL DEFAULT FALSE,
    photograph_model_archived BOOLEAN NOT NULL DEFAULT FALSE,
    UNIQUE (id, archived),
    FOREIGN KEY (photographed_with_model_id, photographed_with_model_archived) REFERENCES camera_models (id, archived) ON UPDATE CASCADE,
    FOREIGN KEY (photographed_asset_model_id, photographed_asset_model_archived) REFERENCES physical_asset_models (id, archived) ON UPDATE CASCADE,
    FOREIGN KEY (photograph_model_id, photograph_model_archived) REFERENCES photograph_models (id, archived) ON UPDATE CASCADE,
    FOREIGN KEY (organization_id, id, archived) REFERENCES resources (organization_id, id, archived) ON UPDATE CASCADE ON DELETE CASCADE
);
CREATE INDEX ON photograph_procedure_templates (photograph_model_id);
CREATE INDEX ON photograph_procedure_templates (photographed_asset_model_id);
CREATE INDEX ON photograph_procedure_templates (photographed_with_model_id);
INSERT INTO table_names (id) VALUES ('photograph_procedure_templates');
INSERT INTO table_name_ancestors (descendant_table_name_id, ancestor_table_name_id)
    SELECT 'photograph_procedure_templates', ancestor_table_name_id FROM table_name_ancestors WHERE descendant_table_name_id = 'procedure_templates'
    UNION SELECT 'photograph_procedure_templates', 'photograph_procedure_templates';
CREATE TRIGGER photograph_procedure_templates_guard AFTER INSERT ON photograph_procedure_templates
    FOR EACH ROW EXECUTE FUNCTION guard_procedure_template_subtype();
CREATE TRIGGER photograph_procedure_templates_guard_update AFTER UPDATE OF id, photographed_with_slot_id, photographed_with_model_id, photographed_with_slot_role_id, photographed_asset_slot_id, photographed_asset_model_id, photographed_asset_slot_role_id, photograph_slot_id, photograph_model_id, photograph_slot_role_id ON photograph_procedure_templates
    FOR EACH ROW WHEN (OLD.id IS DISTINCT FROM NEW.id
        OR OLD.photographed_with_slot_id IS DISTINCT FROM NEW.photographed_with_slot_id
        OR OLD.photographed_with_model_id IS DISTINCT FROM NEW.photographed_with_model_id
        OR OLD.photographed_with_slot_role_id IS DISTINCT FROM NEW.photographed_with_slot_role_id
        OR OLD.photographed_asset_slot_id IS DISTINCT FROM NEW.photographed_asset_slot_id
        OR OLD.photographed_asset_model_id IS DISTINCT FROM NEW.photographed_asset_model_id
        OR OLD.photographed_asset_slot_role_id IS DISTINCT FROM NEW.photographed_asset_slot_role_id
        OR OLD.photograph_slot_id IS DISTINCT FROM NEW.photograph_slot_id
        OR OLD.photograph_model_id IS DISTINCT FROM NEW.photograph_model_id
        OR OLD.photograph_slot_role_id IS DISTINCT FROM NEW.photograph_slot_role_id) EXECUTE FUNCTION guard_procedure_template_subtype();
CREATE TRIGGER photograph_procedure_templates_delete_guard BEFORE DELETE ON photograph_procedure_templates
    FOR EACH ROW EXECUTE FUNCTION guard_procedure_template_subtype();

CREATE FUNCTION require_photograph_procedure_templates_row() RETURNS TRIGGER
LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
DECLARE
    row_id UUID := CASE TG_OP WHEN 'DELETE' THEN OLD.id ELSE NEW.id END;
BEGIN
    IF EXISTS (
        SELECT 1 FROM public.procedure_templates root
        JOIN public.table_name_ancestors path ON path.descendant_table_name_id = root.table_name_id
        WHERE root.id = row_id AND path.ancestor_table_name_id = 'photograph_procedure_templates'
    ) AND NOT EXISTS (SELECT 1 FROM public.photograph_procedure_templates row WHERE row.id = row_id) THEN
        RAISE EXCEPTION '% names a table extending % but has no row there', row_id, 'photograph_procedure_templates'
            USING ERRCODE = 'foreign_key_violation';
    END IF;
    RETURN NULL;
END;
$$;
ALTER FUNCTION require_photograph_procedure_templates_row() OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION require_photograph_procedure_templates_row() FROM PUBLIC;
CREATE CONSTRAINT TRIGGER photograph_procedure_templates_row AFTER INSERT ON procedure_templates
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW WHEN (NEW.table_name_id = 'photograph_procedure_templates')
    EXECUTE FUNCTION require_photograph_procedure_templates_row();
CREATE CONSTRAINT TRIGGER photograph_procedure_templates_kept AFTER DELETE ON photograph_procedure_templates
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION require_photograph_procedure_templates_row();

REVOKE ALL ON photograph_procedure_templates FROM PUBLIC, app, identity_writer;
GRANT SELECT, INSERT, DELETE ON photograph_procedure_templates TO app;
ALTER TABLE photograph_procedure_templates ENABLE ROW LEVEL SECURITY;
ALTER TABLE photograph_procedure_templates FORCE ROW LEVEL SECURITY;
CREATE POLICY photograph_procedure_templates_select ON photograph_procedure_templates FOR SELECT TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY photograph_procedure_templates_insert ON photograph_procedure_templates FOR INSERT TO app
    WITH CHECK (id = ANY (ARRAY(SELECT authz.resources_with('writer'))) AND photographed_with_model_id = ANY (ARRAY(SELECT authz.resources_with('reader'))) AND photographed_asset_model_id = ANY (ARRAY(SELECT authz.resources_with('reader'))) AND photograph_model_id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY photograph_procedure_templates_delete ON photograph_procedure_templates FOR DELETE TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('admin'))));
CREATE POLICY photograph_procedure_templates_archived_insert ON photograph_procedure_templates AS RESTRICTIVE FOR INSERT TO app
    WITH CHECK (NOT archived AND NOT photographed_with_model_archived AND NOT photographed_asset_model_archived AND NOT photograph_model_archived);
CREATE POLICY photograph_procedure_templates_archived_delete ON photograph_procedure_templates AS RESTRICTIVE FOR DELETE TO app
    USING (NOT archived);
CREATE POLICY photograph_procedure_templates_blocked ON photograph_procedure_templates AS RESTRICTIVE FOR ALL TO app
    USING (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))))
    WITH CHECK (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))));

-- Recorded execution of a photograph template.
CREATE TABLE photograph_procedures (
    -- Identity shared with the parent procedure.
    id UUID PRIMARY KEY REFERENCES procedures (id) ON DELETE CASCADE,
    -- Organization containing the parent resource.
    organization_id UUID NOT NULL,
    -- Typed template executed by this procedure.
    photograph_procedure_template_id UUID NOT NULL REFERENCES photograph_procedure_templates (id),
    -- Execution status copied from the parent procedure.
    status_id TEXT NOT NULL,
    FOREIGN KEY (id, photograph_procedure_template_id, status_id, organization_id)
        REFERENCES procedures (id, procedure_template_id, status_id, organization_id) ON DELETE CASCADE
        DEFERRABLE INITIALLY DEFERRED,
    archived BOOLEAN NOT NULL DEFAULT FALSE,
    photograph_procedure_template_archived BOOLEAN NOT NULL DEFAULT FALSE,
    FOREIGN KEY (photograph_procedure_template_id, photograph_procedure_template_archived) REFERENCES photograph_procedure_templates (id, archived) ON UPDATE CASCADE,
    FOREIGN KEY (organization_id, id, archived) REFERENCES resources (organization_id, id, archived) ON UPDATE CASCADE ON DELETE CASCADE
);
CREATE INDEX ON photograph_procedures (photograph_procedure_template_id);
INSERT INTO table_names (id) VALUES ('photograph_procedures');
INSERT INTO table_name_instances (id, instance_table_name_id)
    VALUES ('photograph_procedure_templates', 'photograph_procedures');
INSERT INTO table_name_ancestors (descendant_table_name_id, ancestor_table_name_id)
    SELECT 'photograph_procedures', ancestor_table_name_id FROM table_name_ancestors WHERE descendant_table_name_id = 'procedures'
    UNION SELECT 'photograph_procedures', 'photograph_procedures';
CREATE TRIGGER photograph_procedures_guard AFTER INSERT ON photograph_procedures
    FOR EACH ROW EXECUTE FUNCTION guard_procedure_subtype();
CREATE TRIGGER photograph_procedures_guard_update AFTER UPDATE OF id, photograph_procedure_template_id, status_id ON photograph_procedures
    FOR EACH ROW WHEN (OLD.id IS DISTINCT FROM NEW.id
        OR OLD.photograph_procedure_template_id IS DISTINCT FROM NEW.photograph_procedure_template_id
        OR OLD.status_id IS DISTINCT FROM NEW.status_id) EXECUTE FUNCTION guard_procedure_subtype();
CREATE TRIGGER photograph_procedures_delete_guard BEFORE DELETE ON photograph_procedures
    FOR EACH ROW EXECUTE FUNCTION guard_procedure_subtype();

-- Writes a procedure's new status into its `photograph_procedures` row, whose deferred
-- foreign key holds it equal to the procedure at commit.
CREATE FUNCTION copy_photograph_procedures_status() RETURNS TRIGGER
LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
BEGIN
    UPDATE public.photograph_procedures SET status_id = NEW.status_id WHERE id = NEW.id;
    RETURN NULL;
END;
$$;
REVOKE ALL ON photograph_procedures FROM PUBLIC, app, identity_writer;
GRANT UPDATE (status_id) ON photograph_procedures TO aps_writer;
ALTER FUNCTION copy_photograph_procedures_status() OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION copy_photograph_procedures_status() FROM PUBLIC;
CREATE TRIGGER procedures_status_copy_photograph_procedures AFTER UPDATE OF status_id ON procedures
    FOR EACH ROW WHEN (OLD.status_id IS DISTINCT FROM NEW.status_id) EXECUTE FUNCTION copy_photograph_procedures_status();

CREATE FUNCTION require_photograph_procedures_row() RETURNS TRIGGER
LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
DECLARE
    row_id UUID := CASE TG_OP WHEN 'DELETE' THEN OLD.id ELSE NEW.id END;
BEGIN
    IF EXISTS (
        SELECT 1 FROM public.procedures root
        JOIN public.table_name_ancestors path ON path.descendant_table_name_id = root.table_name_id
        WHERE root.id = row_id AND path.ancestor_table_name_id = 'photograph_procedures'
    ) AND NOT EXISTS (SELECT 1 FROM public.photograph_procedures row WHERE row.id = row_id) THEN
        RAISE EXCEPTION '% names a table extending % but has no row there', row_id, 'photograph_procedures'
            USING ERRCODE = 'foreign_key_violation';
    END IF;
    RETURN NULL;
END;
$$;
ALTER FUNCTION require_photograph_procedures_row() OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION require_photograph_procedures_row() FROM PUBLIC;
CREATE CONSTRAINT TRIGGER photograph_procedures_row AFTER INSERT ON procedures
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW WHEN (NEW.table_name_id = 'photograph_procedures')
    EXECUTE FUNCTION require_photograph_procedures_row();
CREATE CONSTRAINT TRIGGER photograph_procedures_kept AFTER DELETE ON photograph_procedures
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION require_photograph_procedures_row();

CREATE FUNCTION initialize_photograph_procedures_status() RETURNS TRIGGER
LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
BEGIN
    PERFORM public.lock_procedure(NEW.id);
    SELECT procedure.status_id INTO NEW.status_id
    FROM public.procedures procedure
    WHERE procedure.id = NEW.id
        AND procedure.procedure_template_id = NEW.photograph_procedure_template_id
        AND procedure.organization_id = NEW.organization_id;
    RETURN NEW;
END;
$$;
ALTER FUNCTION initialize_photograph_procedures_status() OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION initialize_photograph_procedures_status() FROM PUBLIC;
CREATE TRIGGER photograph_procedures_status_initialize BEFORE INSERT ON photograph_procedures
    FOR EACH ROW EXECUTE FUNCTION initialize_photograph_procedures_status();

GRANT SELECT, INSERT (id, organization_id, photograph_procedure_template_id, archived, photograph_procedure_template_archived), DELETE ON photograph_procedures TO app;
ALTER TABLE photograph_procedures ENABLE ROW LEVEL SECURITY;
ALTER TABLE photograph_procedures FORCE ROW LEVEL SECURITY;
CREATE POLICY photograph_procedures_select ON photograph_procedures FOR SELECT TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY photograph_procedures_insert ON photograph_procedures FOR INSERT TO app
    WITH CHECK (id = ANY (ARRAY(SELECT authz.resources_with('writer'))) AND photograph_procedure_template_id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY photograph_procedures_delete ON photograph_procedures FOR DELETE TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('admin'))));
CREATE POLICY photograph_procedures_archived_insert ON photograph_procedures AS RESTRICTIVE FOR INSERT TO app
    WITH CHECK (NOT archived AND NOT photograph_procedure_template_archived);
CREATE POLICY photograph_procedures_archived_delete ON photograph_procedures AS RESTRICTIVE FOR DELETE TO app
    USING (NOT archived);
CREATE POLICY photograph_procedures_blocked ON photograph_procedures AS RESTRICTIVE FOR ALL TO app
    USING (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))))
    WITH CHECK (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))));

-- Records every change in connetto's `audit_events`.
CREATE TRIGGER photograph_procedure_templates_audit AFTER INSERT OR DELETE ON photograph_procedure_templates
    FOR EACH ROW EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER photograph_procedure_templates_update_audit AFTER UPDATE ON photograph_procedure_templates
    FOR EACH ROW WHEN (OLD IS DISTINCT FROM NEW) EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER photograph_procedures_audit AFTER INSERT OR DELETE ON photograph_procedures
    FOR EACH ROW EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER photograph_procedures_update_audit AFTER UPDATE ON photograph_procedures
    FOR EACH ROW WHEN (OLD IS DISTINCT FROM NEW) EXECUTE FUNCTION authz_write.record_audit_event();
