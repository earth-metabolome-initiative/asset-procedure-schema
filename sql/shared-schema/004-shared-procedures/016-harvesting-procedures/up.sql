-- Migration: 016-harvesting-procedures
-- Purpose: Typed harvesting slots and execution records.
-- APS placement
-- `resources` of kind `'ownable'`
-- +-- procedure_templates -- harvesting_procedure_templates
-- +-- procedures -- harvesting_procedures
-- Slot | Role | Meaning
-- `sample_source` | `input` | The sample source survives sampling.
-- `sample` | `produced` | Harvesting creates the tracked sample.

-- Typed slots and parameters for harvesting.
CREATE TABLE harvesting_procedure_templates (
    -- Identity shared with the parent template.
    id UUID PRIMARY KEY REFERENCES procedure_templates (id) ON DELETE CASCADE,
    -- Organization containing the parent resource.
    organization_id UUID NOT NULL,
    -- Slot declaring the `sample_source` assets.
    sample_source_slot_id UUID NOT NULL,
    -- Admissible model for the `sample_source` slot.
    sample_source_model_id UUID NOT NULL REFERENCES sample_source_models (id),
    -- Required role of the `sample_source` slot.
    sample_source_slot_role_id TEXT GENERATED ALWAYS AS ('input') STORED NOT NULL,
    -- Slot declaring the `sample` assets.
    sample_slot_id UUID NOT NULL,
    -- Admissible model for the `sample` slot.
    sample_model_id UUID NOT NULL,
    -- Source family required by the sample model.
    source_family_model_id UUID NOT NULL,
    -- Required role of the `sample` slot.
    sample_slot_role_id TEXT GENERATED ALWAYS AS ('produced') STORED NOT NULL,
    FOREIGN KEY (sample_source_slot_id, id, sample_source_model_id, sample_source_slot_role_id, organization_id)
        REFERENCES procedure_template_slots (id, procedure_template_id, asset_model_id, role_id, organization_id),
    FOREIGN KEY (sample_slot_id, id, sample_model_id, sample_slot_role_id, organization_id)
        REFERENCES procedure_template_slots (id, procedure_template_id, asset_model_id, role_id, organization_id),
    FOREIGN KEY (sample_model_id, source_family_model_id) REFERENCES sample_models (id, sample_source_model_id),
    FOREIGN KEY (sample_source_model_id, source_family_model_id)
        REFERENCES asset_model_ancestors (descendant_model_id, ancestor_model_id),
    -- Archive state inherited from this resource.
    archived BOOLEAN NOT NULL DEFAULT FALSE,
    -- Archive state of the row named by `sample_source_model_id`.
    sample_source_model_archived BOOLEAN NOT NULL DEFAULT FALSE,
    -- Archive state of the row named by `sample_model_id`.
    sample_model_archived BOOLEAN NOT NULL DEFAULT FALSE,
    -- Archive state of the row named by `source_family_model_id`.
    source_family_model_archived BOOLEAN NOT NULL DEFAULT FALSE,
    UNIQUE (id, archived),
    FOREIGN KEY (sample_source_model_id, sample_source_model_archived) REFERENCES sample_source_models (id, archived) ON UPDATE CASCADE,
    FOREIGN KEY (sample_model_id, sample_model_archived) REFERENCES sample_models (id, archived) ON UPDATE CASCADE,
    FOREIGN KEY (source_family_model_id, source_family_model_archived) REFERENCES sample_source_models (id, archived) ON UPDATE CASCADE,
    FOREIGN KEY (organization_id, id, archived) REFERENCES resources (organization_id, id, archived) ON UPDATE CASCADE ON DELETE CASCADE
);
CREATE INDEX ON harvesting_procedure_templates (sample_model_id, source_family_model_id);
CREATE INDEX ON harvesting_procedure_templates (sample_source_model_id, source_family_model_id);
CREATE INDEX ON harvesting_procedure_templates (source_family_model_id);
INSERT INTO table_names (id) VALUES ('harvesting_procedure_templates');
INSERT INTO table_name_ancestors (descendant_table_name_id, ancestor_table_name_id)
    SELECT 'harvesting_procedure_templates', ancestor_table_name_id FROM table_name_ancestors WHERE descendant_table_name_id = 'procedure_templates'
    UNION SELECT 'harvesting_procedure_templates', 'harvesting_procedure_templates';
CREATE TRIGGER harvesting_procedure_templates_guard AFTER INSERT ON harvesting_procedure_templates
    FOR EACH ROW EXECUTE FUNCTION guard_procedure_template_subtype();
CREATE TRIGGER harvesting_procedure_templates_guard_update AFTER UPDATE OF id, sample_source_slot_id, sample_source_model_id, sample_source_slot_role_id, sample_slot_id, sample_model_id, source_family_model_id, sample_slot_role_id ON harvesting_procedure_templates
    FOR EACH ROW WHEN (OLD.id IS DISTINCT FROM NEW.id
        OR OLD.sample_source_slot_id IS DISTINCT FROM NEW.sample_source_slot_id
        OR OLD.sample_source_model_id IS DISTINCT FROM NEW.sample_source_model_id
        OR OLD.sample_source_slot_role_id IS DISTINCT FROM NEW.sample_source_slot_role_id
        OR OLD.sample_slot_id IS DISTINCT FROM NEW.sample_slot_id
        OR OLD.sample_model_id IS DISTINCT FROM NEW.sample_model_id
        OR OLD.source_family_model_id IS DISTINCT FROM NEW.source_family_model_id
        OR OLD.sample_slot_role_id IS DISTINCT FROM NEW.sample_slot_role_id) EXECUTE FUNCTION guard_procedure_template_subtype();
CREATE TRIGGER harvesting_procedure_templates_delete_guard BEFORE DELETE ON harvesting_procedure_templates
    FOR EACH ROW EXECUTE FUNCTION guard_procedure_template_subtype();

CREATE FUNCTION require_harvesting_procedure_templates_row() RETURNS TRIGGER
LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
DECLARE
    row_id UUID := CASE TG_OP WHEN 'DELETE' THEN OLD.id ELSE NEW.id END;
BEGIN
    IF EXISTS (
        SELECT 1 FROM public.procedure_templates root
        JOIN public.table_name_ancestors path ON path.descendant_table_name_id = root.table_name_id
        WHERE root.id = row_id AND path.ancestor_table_name_id = 'harvesting_procedure_templates'
    ) AND NOT EXISTS (SELECT 1 FROM public.harvesting_procedure_templates row WHERE row.id = row_id) THEN
        RAISE EXCEPTION '% names a table extending % but has no row there', row_id, 'harvesting_procedure_templates'
            USING ERRCODE = 'foreign_key_violation';
    END IF;
    RETURN NULL;
END;
$$;
ALTER FUNCTION require_harvesting_procedure_templates_row() OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION require_harvesting_procedure_templates_row() FROM PUBLIC;
CREATE CONSTRAINT TRIGGER harvesting_procedure_templates_row AFTER INSERT ON procedure_templates
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW WHEN (NEW.table_name_id = 'harvesting_procedure_templates')
    EXECUTE FUNCTION require_harvesting_procedure_templates_row();
CREATE CONSTRAINT TRIGGER harvesting_procedure_templates_kept AFTER DELETE ON harvesting_procedure_templates
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION require_harvesting_procedure_templates_row();

REVOKE ALL ON harvesting_procedure_templates FROM PUBLIC, app, identity_writer;
GRANT SELECT, INSERT, DELETE ON harvesting_procedure_templates TO app;
ALTER TABLE harvesting_procedure_templates ENABLE ROW LEVEL SECURITY;
ALTER TABLE harvesting_procedure_templates FORCE ROW LEVEL SECURITY;
CREATE POLICY harvesting_procedure_templates_select ON harvesting_procedure_templates FOR SELECT TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY harvesting_procedure_templates_insert ON harvesting_procedure_templates FOR INSERT TO app
    WITH CHECK (id = ANY (ARRAY(SELECT authz.resources_with('writer'))) AND sample_source_model_id = ANY (ARRAY(SELECT authz.resources_with('reader'))) AND sample_model_id = ANY (ARRAY(SELECT authz.resources_with('reader'))) AND source_family_model_id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY harvesting_procedure_templates_delete ON harvesting_procedure_templates FOR DELETE TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('admin'))));
CREATE POLICY harvesting_procedure_templates_archived_insert ON harvesting_procedure_templates AS RESTRICTIVE FOR INSERT TO app
    WITH CHECK (NOT archived AND NOT sample_source_model_archived AND NOT sample_model_archived AND NOT source_family_model_archived);
CREATE POLICY harvesting_procedure_templates_archived_delete ON harvesting_procedure_templates AS RESTRICTIVE FOR DELETE TO app
    USING (NOT archived);
CREATE POLICY harvesting_procedure_templates_blocked ON harvesting_procedure_templates AS RESTRICTIVE FOR ALL TO app
    USING (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))))
    WITH CHECK (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))));

-- Recorded execution of a harvesting template.
CREATE TABLE harvesting_procedures (
    -- Identity shared with the parent procedure.
    id UUID PRIMARY KEY REFERENCES procedures (id) ON DELETE CASCADE,
    -- Organization containing the parent resource.
    organization_id UUID NOT NULL,
    -- Typed template executed by this procedure.
    harvesting_procedure_template_id UUID NOT NULL REFERENCES harvesting_procedure_templates (id),
    -- Execution status copied from the parent procedure.
    status_id TEXT NOT NULL DEFAULT 'planned',
    FOREIGN KEY (id, harvesting_procedure_template_id, status_id, organization_id)
        REFERENCES procedures (id, procedure_template_id, status_id, organization_id) ON DELETE CASCADE
        DEFERRABLE INITIALLY DEFERRED,
    -- Archive state inherited from this resource.
    archived BOOLEAN NOT NULL DEFAULT FALSE,
    -- Archive state of the row named by `harvesting_procedure_template_id`.
    harvesting_procedure_template_archived BOOLEAN NOT NULL DEFAULT FALSE,
    FOREIGN KEY (harvesting_procedure_template_id, harvesting_procedure_template_archived) REFERENCES harvesting_procedure_templates (id, archived) ON UPDATE CASCADE,
    FOREIGN KEY (organization_id, id, archived) REFERENCES resources (organization_id, id, archived) ON UPDATE CASCADE ON DELETE CASCADE
);
CREATE INDEX ON harvesting_procedures (harvesting_procedure_template_id);
INSERT INTO table_names (id) VALUES ('harvesting_procedures');
UPDATE table_names SET instance_table_name_id = 'harvesting_procedures'
    WHERE id = 'harvesting_procedure_templates';
INSERT INTO table_name_ancestors (descendant_table_name_id, ancestor_table_name_id)
    SELECT 'harvesting_procedures', ancestor_table_name_id FROM table_name_ancestors WHERE descendant_table_name_id = 'procedures'
    UNION SELECT 'harvesting_procedures', 'harvesting_procedures';
CREATE TRIGGER harvesting_procedures_guard AFTER INSERT ON harvesting_procedures
    FOR EACH ROW EXECUTE FUNCTION guard_procedure_subtype();
CREATE TRIGGER harvesting_procedures_guard_update AFTER UPDATE OF id, harvesting_procedure_template_id, status_id ON harvesting_procedures
    FOR EACH ROW WHEN (OLD.id IS DISTINCT FROM NEW.id
        OR OLD.harvesting_procedure_template_id IS DISTINCT FROM NEW.harvesting_procedure_template_id
        OR OLD.status_id IS DISTINCT FROM NEW.status_id) EXECUTE FUNCTION guard_procedure_subtype();
CREATE TRIGGER harvesting_procedures_delete_guard BEFORE DELETE ON harvesting_procedures
    FOR EACH ROW EXECUTE FUNCTION guard_procedure_subtype();

-- Writes a procedure's new status into its `harvesting_procedures` row, whose deferred
-- foreign key holds it equal to the procedure at commit.
CREATE FUNCTION copy_harvesting_procedures_status() RETURNS TRIGGER
LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
BEGIN
    UPDATE public.harvesting_procedures SET status_id = NEW.status_id WHERE id = NEW.id;
    RETURN NULL;
END;
$$;
REVOKE ALL ON harvesting_procedures FROM PUBLIC, app, identity_writer;
GRANT UPDATE (status_id) ON harvesting_procedures TO aps_writer;
ALTER FUNCTION copy_harvesting_procedures_status() OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION copy_harvesting_procedures_status() FROM PUBLIC;
CREATE TRIGGER procedures_status_copy_harvesting_procedures AFTER UPDATE OF status_id ON procedures
    FOR EACH ROW WHEN (OLD.status_id IS DISTINCT FROM NEW.status_id) EXECUTE FUNCTION copy_harvesting_procedures_status();

CREATE FUNCTION require_harvesting_procedures_row() RETURNS TRIGGER
LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
DECLARE
    row_id UUID := CASE TG_OP WHEN 'DELETE' THEN OLD.id ELSE NEW.id END;
BEGIN
    IF EXISTS (
        SELECT 1 FROM public.procedures root
        JOIN public.table_name_ancestors path ON path.descendant_table_name_id = root.table_name_id
        WHERE root.id = row_id AND path.ancestor_table_name_id = 'harvesting_procedures'
    ) AND NOT EXISTS (SELECT 1 FROM public.harvesting_procedures row WHERE row.id = row_id) THEN
        RAISE EXCEPTION '% names a table extending % but has no row there', row_id, 'harvesting_procedures'
            USING ERRCODE = 'foreign_key_violation';
    END IF;
    RETURN NULL;
END;
$$;
ALTER FUNCTION require_harvesting_procedures_row() OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION require_harvesting_procedures_row() FROM PUBLIC;
CREATE CONSTRAINT TRIGGER harvesting_procedures_row AFTER INSERT ON procedures
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW WHEN (NEW.table_name_id = 'harvesting_procedures')
    EXECUTE FUNCTION require_harvesting_procedures_row();
CREATE CONSTRAINT TRIGGER harvesting_procedures_kept AFTER DELETE ON harvesting_procedures
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION require_harvesting_procedures_row();

CREATE FUNCTION initialize_harvesting_procedures_status() RETURNS TRIGGER
LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
DECLARE
    parent_status TEXT;
BEGIN
    SELECT procedure.status_id INTO parent_status
    FROM public.procedures procedure
    WHERE procedure.id = NEW.id
        AND procedure.procedure_template_id = NEW.harvesting_procedure_template_id
        AND procedure.organization_id = NEW.organization_id;
    IF FOUND THEN
        NEW.status_id := parent_status;
    END IF;
    RETURN NEW;
END;
$$;
ALTER FUNCTION initialize_harvesting_procedures_status() OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION initialize_harvesting_procedures_status() FROM PUBLIC;
CREATE TRIGGER harvesting_procedures_status_initialize BEFORE INSERT ON harvesting_procedures
    FOR EACH ROW EXECUTE FUNCTION initialize_harvesting_procedures_status();

GRANT SELECT, INSERT (id, organization_id, harvesting_procedure_template_id, archived, harvesting_procedure_template_archived), DELETE ON harvesting_procedures TO app;
ALTER TABLE harvesting_procedures ENABLE ROW LEVEL SECURITY;
ALTER TABLE harvesting_procedures FORCE ROW LEVEL SECURITY;
CREATE POLICY harvesting_procedures_select ON harvesting_procedures FOR SELECT TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY harvesting_procedures_insert ON harvesting_procedures FOR INSERT TO app
    WITH CHECK (id = ANY (ARRAY(SELECT authz.resources_with('writer'))) AND harvesting_procedure_template_id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY harvesting_procedures_delete ON harvesting_procedures FOR DELETE TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('admin'))));
CREATE POLICY harvesting_procedures_archived_insert ON harvesting_procedures AS RESTRICTIVE FOR INSERT TO app
    WITH CHECK (NOT archived AND NOT harvesting_procedure_template_archived);
CREATE POLICY harvesting_procedures_archived_delete ON harvesting_procedures AS RESTRICTIVE FOR DELETE TO app
    USING (NOT archived);
CREATE POLICY harvesting_procedures_blocked ON harvesting_procedures AS RESTRICTIVE FOR ALL TO app
    USING (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))))
    WITH CHECK (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))));

-- Records every change in connetto's `audit_events`.
CREATE TRIGGER harvesting_procedure_templates_audit AFTER INSERT OR DELETE ON harvesting_procedure_templates
    FOR EACH ROW EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER harvesting_procedure_templates_update_audit AFTER UPDATE ON harvesting_procedure_templates
    FOR EACH ROW WHEN (OLD IS DISTINCT FROM NEW) EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER harvesting_procedures_audit AFTER INSERT OR DELETE ON harvesting_procedures
    FOR EACH ROW EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER harvesting_procedures_update_audit AFTER UPDATE ON harvesting_procedures
    FOR EACH ROW WHEN (OLD IS DISTINCT FROM NEW) EXECUTE FUNCTION authz_write.record_audit_event();
