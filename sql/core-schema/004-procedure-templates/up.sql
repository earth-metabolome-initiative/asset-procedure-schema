-- A procedure template is the blueprint of a procedure. It declares the slots
-- its assets fill, and, when composite, the steps it runs as other templates.
-- Only a `draft` changes. Publishing freezes it, and a new version is a new
-- draft naming the published one as `previous_version_id`.
CREATE TABLE procedure_templates (
    -- Identifier of the connetto ownable this template extends.
    id UUID PRIMARY KEY REFERENCES ownables (id) ON DELETE CASCADE,
    -- connetto organization owning the row.
    organization_id UUID NOT NULL,
    -- connetto project holding the template.
    project_id UUID NOT NULL,
    -- Most concrete APS table holding this template.
    table_name_id TEXT NOT NULL REFERENCES table_names (id),
    -- Name shared by every version of the template, unique per version within the project.
    name TEXT NOT NULL CHECK (name <> '' AND length(name) <= 255),
    -- Free-text description of what the procedure achieves.
    description TEXT CHECK (description <> '' AND length(description) <= 8192),
    -- Instruction a person follows to perform the procedure.
    instruction TEXT CHECK (instruction <> '' AND length(instruction) <= 65536),
    -- Lifecycle state, `draft`, `published` or `deprecated`.
    state_id TEXT NOT NULL DEFAULT 'draft' REFERENCES procedure_template_states (id),
    -- Version number, one more than the previous version's.
    version INTEGER NOT NULL DEFAULT 1 CHECK (version > 0),
    -- Published version this draft revises, absent on the first version.
    previous_version_id UUID,
    -- Version number of `previous_version_id`.
    previous_version INTEGER,
    CHECK ((previous_version_id IS NULL) = (version = 1)),
    CHECK ((previous_version_id IS NULL) = (previous_version IS NULL)),
    CHECK (version = previous_version + 1),
    UNIQUE (project_id, name, version),
    UNIQUE (id, project_id, name, version, organization_id),
    FOREIGN KEY (previous_version_id, project_id, name, previous_version, organization_id)
        REFERENCES procedure_templates (id, project_id, name, version, organization_id),
    FOREIGN KEY (id, project_id) REFERENCES ownables (id, project_id) ON UPDATE CASCADE ON DELETE CASCADE,
    FOREIGN KEY (organization_id, id) REFERENCES resources (organization_id, id) ON DELETE CASCADE
);
CREATE INDEX ON procedure_templates (previous_version_id);
CREATE INDEX ON procedure_templates (state_id);
CREATE INDEX ON procedure_templates (table_name_id);
INSERT INTO table_names (id) VALUES ('procedure_templates');
INSERT INTO table_name_ancestors (descendant_table_name_id, ancestor_table_name_id) VALUES ('procedure_templates', 'procedure_templates');

-- A slot is a place in the procedure that assets fill, typed by the most
-- general admissible asset model and by the role the assets play. A slot may
-- name another slot of its template whose assets end up holding its assets,
-- and another whose assets held them until the procedure took them out.
CREATE TABLE procedure_template_slots (
    -- Identifier of the slot.
    id UUID PRIMARY KEY,
    -- connetto organization owning the template.
    organization_id UUID NOT NULL,
    -- Template declaring the slot.
    procedure_template_id UUID NOT NULL REFERENCES procedure_templates (id) ON DELETE CASCADE,
    -- Name, unique within the template.
    name TEXT NOT NULL CHECK (name <> '' AND length(name) <= 255),
    -- Free-text description of what fills the slot.
    description TEXT CHECK (description <> '' AND length(description) <= 8192),
    -- Most general model the bound assets may have.
    asset_model_id UUID NOT NULL REFERENCES asset_models (id),
    -- What the procedure does to the bound assets.
    role_id TEXT NOT NULL REFERENCES slot_roles (id),
    -- Fewest assets a completed procedure binds.
    minimum_count INTEGER NOT NULL DEFAULT 1 CHECK (minimum_count >= 0),
    -- Most assets a procedure binds. NULL leaves the number unbounded.
    maximum_count INTEGER CHECK (maximum_count >= 1 AND maximum_count >= minimum_count),
    -- Slot whose assets hold this slot's assets once the procedure completes.
    placed_into_slot_id UUID CHECK (placed_into_slot_id <> id),
    -- Slot whose assets held this slot's assets until the procedure took them out.
    retrieved_from_slot_id UUID CHECK (retrieved_from_slot_id <> id),
    UNIQUE (procedure_template_id, name),
    UNIQUE (id, procedure_template_id, organization_id),
    UNIQUE (id, procedure_template_id, asset_model_id),
    UNIQUE (id, procedure_template_id, asset_model_id, role_id),
    UNIQUE (id, procedure_template_id, asset_model_id, role_id, organization_id),
    UNIQUE (id, procedure_template_id, placed_into_slot_id, organization_id),
    UNIQUE (id, procedure_template_id, retrieved_from_slot_id, organization_id),
    FOREIGN KEY (placed_into_slot_id, procedure_template_id, organization_id)
        REFERENCES procedure_template_slots (id, procedure_template_id, organization_id),
    FOREIGN KEY (retrieved_from_slot_id, procedure_template_id, organization_id)
        REFERENCES procedure_template_slots (id, procedure_template_id, organization_id),
    FOREIGN KEY (organization_id, procedure_template_id) REFERENCES resources (organization_id, id) ON DELETE CASCADE
);
CREATE INDEX ON procedure_template_slots (asset_model_id);
CREATE INDEX ON procedure_template_slots (role_id);
CREATE INDEX ON procedure_template_slots (placed_into_slot_id);
CREATE INDEX ON procedure_template_slots (retrieved_from_slot_id);
INSERT INTO table_names (id) VALUES ('procedure_template_slots');

-- A step runs a published child template inside a parent template. The same
-- child may run in several steps of one parent, told apart by the step name.
-- Steps with equal `position` may run in parallel. A step with a repeat slot
-- runs once for every asset the parent procedure binds to that slot.
CREATE TABLE procedure_template_steps (
    -- Identifier of the step.
    id UUID PRIMARY KEY,
    -- connetto organization owning the parent template.
    organization_id UUID NOT NULL,
    -- Template the step belongs to.
    parent_template_id UUID NOT NULL REFERENCES procedure_templates (id) ON DELETE CASCADE,
    -- Template the step runs.
    child_template_id UUID NOT NULL REFERENCES procedure_templates (id),
    -- Name, unique within the parent template.
    name TEXT NOT NULL CHECK (name <> '' AND length(name) <= 255),
    -- What the step achieves within the parent.
    description TEXT CHECK (description <> '' AND length(description) <= 8192),
    -- Order of the step. A predecessor sits at a strictly lower position.
    position INTEGER NOT NULL CHECK (position >= 0),
    -- An optional step may be left without a completed procedure.
    optional BOOLEAN NOT NULL DEFAULT FALSE,
    -- Parent slot whose every bound asset gets its own run of the step.
    repeat_slot_id UUID,
    CHECK (parent_template_id <> child_template_id),
    UNIQUE (parent_template_id, name),
    UNIQUE (id, parent_template_id),
    UNIQUE (id, parent_template_id, position, organization_id),
    UNIQUE (id, parent_template_id, child_template_id),
    UNIQUE (id, parent_template_id, child_template_id, organization_id),
    FOREIGN KEY (repeat_slot_id, parent_template_id, organization_id)
        REFERENCES procedure_template_slots (id, procedure_template_id, organization_id),
    FOREIGN KEY (organization_id, parent_template_id) REFERENCES resources (organization_id, id) ON DELETE CASCADE
);
CREATE INDEX ON procedure_template_steps (child_template_id);
CREATE INDEX ON procedure_template_steps (repeat_slot_id, parent_template_id);
INSERT INTO table_names (id) VALUES ('procedure_template_steps');

-- The successor step starts only after the predecessor completes. A
-- predecessor sits at a strictly lower position, so the steps of a parent
-- form a DAG without a recursive check.
CREATE TABLE procedure_template_step_edges (
    -- Step that completes first.
    predecessor_step_id UUID NOT NULL,
    -- Step that starts after the predecessor completed.
    successor_step_id UUID NOT NULL,
    -- Template both steps belong to.
    parent_template_id UUID NOT NULL,
    -- connetto organization owning the parent template.
    organization_id UUID NOT NULL,
    -- Position of the predecessor, kept equal by the cascade.
    predecessor_position INTEGER NOT NULL,
    -- Position of the successor, kept equal by the cascade.
    successor_position INTEGER NOT NULL,
    PRIMARY KEY (predecessor_step_id, parent_template_id, successor_step_id),
    CHECK (predecessor_position < successor_position),
    FOREIGN KEY (predecessor_step_id, parent_template_id, predecessor_position, organization_id)
        REFERENCES procedure_template_steps (id, parent_template_id, position, organization_id) ON UPDATE CASCADE ON DELETE CASCADE,
    FOREIGN KEY (successor_step_id, parent_template_id, successor_position, organization_id)
        REFERENCES procedure_template_steps (id, parent_template_id, position, organization_id) ON UPDATE CASCADE ON DELETE CASCADE,
    FOREIGN KEY (organization_id, parent_template_id) REFERENCES resources (organization_id, id) ON DELETE CASCADE
);
CREATE INDEX ON procedure_template_step_edges (successor_step_id);
CREATE INDEX ON procedure_template_step_edges (organization_id, parent_template_id);
INSERT INTO table_names (id) VALUES ('procedure_template_step_edges');

-- Binds a slot of a step's child template to a slot of the parent template,
-- so the child procedure must use the asset the parent procedure binds there.
-- Two steps bound to one parent slot share that asset. The parent slot may
-- narrow the child slot's model, and the pair of roles must be admitted by
-- `slot_role_pairs`. A parent slot is produced by at most one child slot and
-- consumed by at most one.
CREATE TABLE procedure_template_slot_bindings (
    -- Step whose child slot is bound.
    step_id UUID NOT NULL,
    -- Slot of the step's child template.
    child_slot_id UUID NOT NULL,
    -- Template the step belongs to.
    parent_template_id UUID NOT NULL,
    -- Template the step runs.
    child_template_id UUID NOT NULL,
    -- Slot of the parent template that fills the child slot.
    parent_slot_id UUID NOT NULL,
    -- Model of the child slot.
    child_asset_model_id UUID NOT NULL,
    -- Model of the parent slot, equal to or below the child slot's.
    parent_asset_model_id UUID NOT NULL,
    -- Role of the child slot.
    child_role_id TEXT NOT NULL,
    -- Role of the parent slot.
    parent_role_id TEXT NOT NULL,
    -- connetto organization owning the parent template.
    organization_id UUID NOT NULL,
    PRIMARY KEY (step_id, parent_template_id, child_slot_id),
    FOREIGN KEY (step_id, parent_template_id, child_template_id, organization_id)
        REFERENCES procedure_template_steps (id, parent_template_id, child_template_id, organization_id) ON DELETE CASCADE,
    FOREIGN KEY (child_slot_id, child_template_id, child_asset_model_id, child_role_id)
        REFERENCES procedure_template_slots (id, procedure_template_id, asset_model_id, role_id),
    FOREIGN KEY (parent_slot_id, parent_template_id, parent_asset_model_id, parent_role_id, organization_id)
        REFERENCES procedure_template_slots (id, procedure_template_id, asset_model_id, role_id, organization_id) ON DELETE CASCADE,
    FOREIGN KEY (parent_asset_model_id, child_asset_model_id)
        REFERENCES asset_model_ancestors (descendant_model_id, ancestor_model_id),
    FOREIGN KEY (child_role_id, parent_role_id) REFERENCES slot_role_pairs (child_role_id, parent_role_id),
    FOREIGN KEY (organization_id, parent_template_id) REFERENCES resources (organization_id, id) ON DELETE CASCADE
);
CREATE INDEX ON procedure_template_slot_bindings (parent_slot_id);
CREATE UNIQUE INDEX procedure_template_slot_bindings_produced_once ON procedure_template_slot_bindings (parent_slot_id, parent_template_id)
    WHERE child_role_id = 'produced';
CREATE UNIQUE INDEX procedure_template_slot_bindings_consumed_once ON procedure_template_slot_bindings (parent_slot_id, parent_template_id)
    WHERE child_role_id = 'consumed';
CREATE INDEX ON procedure_template_slot_bindings (child_role_id, parent_role_id);
CREATE INDEX ON procedure_template_slot_bindings (child_slot_id, child_template_id, child_asset_model_id, child_role_id);
CREATE INDEX ON procedure_template_slot_bindings (organization_id, parent_template_id);
CREATE INDEX ON procedure_template_slot_bindings (parent_asset_model_id, child_asset_model_id);
INSERT INTO table_names (id) VALUES ('procedure_template_slot_bindings');

-- Canonical units for generic numeric parameters.
CREATE TABLE quantity_kinds (
    -- Quantity name.
    id TEXT PRIMARY KEY CHECK (id <> '' AND length(id) <= 63),
    -- Symbol displayed beside a stored value.
    unit_symbol TEXT NOT NULL CHECK (unit_symbol <> '' AND length(unit_symbol) <= 63),
    -- UCUM code of the stored unit.
    ucum_code TEXT NOT NULL CHECK (ucum_code <> '' AND length(ucum_code) <= 63)
);
INSERT INTO quantity_kinds (id, unit_symbol, ucum_code) VALUES
    ('mass', 'kg', 'kg'), ('volume', 'L', 'L'), ('temperature', 'K', 'K'),
    ('pressure', 'Pa', 'Pa'), ('frequency', 'Hz', 'Hz'),
    ('rotational_speed', 'rpm', '{rpm}'), ('duration', 's', 's'), ('ratio', '%', '%');
ALTER TABLE quantity_kinds ENABLE ROW LEVEL SECURITY;
ALTER TABLE quantity_kinds FORCE ROW LEVEL SECURITY;
CREATE POLICY quantity_kinds_select ON quantity_kinds FOR SELECT TO app USING (true);
REVOKE ALL ON quantity_kinds FROM PUBLIC, app, identity_writer;
GRANT SELECT ON quantity_kinds TO app;

-- A named procedure value with an optional default in its quantity kind's unit.
CREATE TABLE procedure_template_parameters (
    -- Identifier of the parameter.
    id UUID PRIMARY KEY,
    -- connetto organization owning the template.
    organization_id UUID NOT NULL,
    -- Template declaring the parameter.
    procedure_template_id UUID NOT NULL REFERENCES procedure_templates (id) ON DELETE CASCADE,
    -- Name, unique within the template.
    name TEXT NOT NULL CHECK (name <> '' AND length(name) <= 255),
    -- Free-text description of what the value means.
    description TEXT CHECK (description <> '' AND length(description) <= 8192),
    -- Kind of the value, `number`, `text` or `boolean`.
    value_kind_id TEXT NOT NULL REFERENCES parameter_value_kinds (id),
    -- Quantity kind fixing the canonical unit of a number.
    quantity_kind_id TEXT REFERENCES quantity_kinds (id),
    -- Whether a procedure completes only with a value recorded.
    required BOOLEAN NOT NULL DEFAULT FALSE,
    -- Default of a number parameter.
    default_number DOUBLE PRECISION CHECK (default_number > '-Infinity'::double precision AND default_number < 'Infinity'::double precision),
    -- Lowest admissible value before a procedure records a deviation.
    minimum_number DOUBLE PRECISION CONSTRAINT parameter_minimum_number_finite
        CHECK (minimum_number > '-Infinity'::double precision AND minimum_number < 'Infinity'::double precision),
    -- Highest admissible value before a procedure records a deviation.
    maximum_number DOUBLE PRECISION CONSTRAINT parameter_maximum_number_finite
        CHECK (maximum_number > '-Infinity'::double precision AND maximum_number < 'Infinity'::double precision),
    -- Default of a text parameter.
    default_text TEXT CHECK (default_text <> '' AND length(default_text) <= 8192),
    -- Default of a boolean parameter.
    default_boolean BOOLEAN,
    CONSTRAINT parameter_quantity_kind CHECK ((quantity_kind_id IS NOT NULL) = (value_kind_id = 'number')),
    CONSTRAINT parameter_number_range_kind CHECK (value_kind_id = 'number' OR (minimum_number IS NULL AND maximum_number IS NULL)),
    CONSTRAINT parameter_number_range CHECK (minimum_number <= maximum_number),
    CONSTRAINT parameter_number_default_range CHECK (default_number >= minimum_number AND default_number <= maximum_number),
    CHECK (default_number IS NULL OR value_kind_id = 'number'),
    CHECK (default_text IS NULL OR value_kind_id = 'text'),
    CHECK (default_boolean IS NULL OR value_kind_id = 'boolean'),
    UNIQUE (procedure_template_id, name),
    UNIQUE (id, procedure_template_id, value_kind_id),
    FOREIGN KEY (organization_id, procedure_template_id) REFERENCES resources (organization_id, id) ON DELETE CASCADE
);
CREATE INDEX ON procedure_template_parameters (value_kind_id);
CREATE INDEX ON procedure_template_parameters (quantity_kind_id);
INSERT INTO table_names (id) VALUES ('procedure_template_parameters');

-- The value a parent template fixes for a parameter of a step's child
-- template, replacing the child's default in that step.
CREATE TABLE procedure_template_step_parameters (
    -- Step whose child parameter is fixed.
    step_id UUID NOT NULL,
    -- Parameter of the step's child template.
    parameter_id UUID NOT NULL,
    -- Template the step belongs to.
    parent_template_id UUID NOT NULL,
    -- Template the step runs.
    child_template_id UUID NOT NULL,
    -- Kind of the parameter's value.
    value_kind_id TEXT NOT NULL,
    -- connetto organization owning the parent template.
    organization_id UUID NOT NULL,
    -- Value of a number parameter.
    number_value DOUBLE PRECISION CHECK (number_value > '-Infinity'::double precision AND number_value < 'Infinity'::double precision),
    -- Value of a text parameter.
    text_value TEXT CHECK (text_value <> '' AND length(text_value) <= 8192),
    -- Value of a boolean parameter.
    boolean_value BOOLEAN,
    PRIMARY KEY (step_id, parent_template_id, parameter_id),
    CHECK ((number_value IS NOT NULL) = (value_kind_id = 'number')),
    CHECK ((text_value IS NOT NULL) = (value_kind_id = 'text')),
    CHECK ((boolean_value IS NOT NULL) = (value_kind_id = 'boolean')),
    FOREIGN KEY (step_id, parent_template_id, child_template_id, organization_id)
        REFERENCES procedure_template_steps (id, parent_template_id, child_template_id, organization_id) ON DELETE CASCADE,
    FOREIGN KEY (parameter_id, child_template_id, value_kind_id)
        REFERENCES procedure_template_parameters (id, procedure_template_id, value_kind_id),
    FOREIGN KEY (organization_id, parent_template_id) REFERENCES resources (organization_id, id) ON DELETE CASCADE
);
CREATE INDEX ON procedure_template_step_parameters (organization_id, parent_template_id);
CREATE INDEX ON procedure_template_step_parameters (parameter_id, child_template_id, value_kind_id);
INSERT INTO table_names (id) VALUES ('procedure_template_step_parameters');

-- Requires executable steps and complete, ordered slot effects before publication.
CREATE FUNCTION require_publishable_procedure_template(template_id UUID) RETURNS VOID
LANGUAGE plpgsql SET search_path = pg_catalog, pg_temp AS $$
DECLARE
    has_steps BOOLEAN := EXISTS (SELECT 1 FROM public.procedure_template_steps step WHERE step.parent_template_id = template_id);
BEGIN
    IF NOT has_steps AND NOT EXISTS (SELECT 1 FROM public.procedure_template_slots slot WHERE slot.procedure_template_id = template_id) THEN
        RAISE EXCEPTION 'procedure template % has neither slots nor steps', template_id USING ERRCODE = 'check_violation';
    END IF;
    IF EXISTS (
        SELECT 1 FROM public.procedure_template_slots slot
        WHERE slot.procedure_template_id = template_id
          AND ((slot.role_id IN ('produced', 'intermediate') AND (has_steps OR slot.role_id = 'intermediate') AND NOT EXISTS (
                  SELECT 1 FROM public.procedure_template_slot_bindings binding
                  WHERE binding.parent_slot_id = slot.id AND binding.child_role_id = 'produced'))
            OR (slot.role_id IN ('consumed', 'intermediate') AND (has_steps OR slot.role_id = 'intermediate') AND NOT EXISTS (
                  SELECT 1 FROM public.procedure_template_slot_bindings binding
                  WHERE binding.parent_slot_id = slot.id AND binding.child_role_id = 'consumed')))
    ) THEN
        RAISE EXCEPTION 'procedure template % has a slot that no step produces or consumes as declared', template_id
            USING ERRCODE = 'check_violation';
    END IF;
    IF EXISTS (
        SELECT 1 FROM public.procedure_template_steps step
        WHERE step.parent_template_id = template_id AND step.repeat_slot_id IS NOT NULL
          AND NOT EXISTS (
              SELECT 1 FROM public.procedure_template_slot_bindings binding
              WHERE binding.step_id = step.id AND binding.parent_slot_id = step.repeat_slot_id)
    ) THEN
        RAISE EXCEPTION 'procedure template % repeats a step over a slot the step does not bind', template_id
            USING ERRCODE = 'check_violation';
    END IF;
    IF EXISTS (
        SELECT 1 FROM public.procedure_template_steps step
        JOIN public.procedure_templates child ON child.id = step.child_template_id
        WHERE step.parent_template_id = template_id AND child.state_id <> 'published'
    ) THEN
        RAISE EXCEPTION 'procedure template % has a step whose template is not published', template_id
            USING ERRCODE = 'check_violation';
    END IF;
    IF EXISTS (
        SELECT 1 FROM public.procedure_template_slot_bindings binding
        JOIN public.procedure_template_steps step ON step.id = binding.step_id
        JOIN public.procedure_template_slots child ON child.id = binding.child_slot_id
        JOIN public.procedure_template_slots parent ON parent.id = binding.parent_slot_id
        WHERE binding.parent_template_id = template_id
          AND child.minimum_count > CASE WHEN step.repeat_slot_id = parent.id THEN 1 ELSE parent.maximum_count END
    ) THEN
        RAISE EXCEPTION 'procedure template % binds a child slot requiring more assets than the parent supplies', template_id
            USING ERRCODE = 'check_violation';
    END IF;
    IF EXISTS (
        SELECT 1 FROM public.procedure_template_slot_bindings binding
        JOIN public.procedure_template_steps step ON step.id = binding.step_id
        JOIN public.procedure_template_slots parent ON parent.id = binding.parent_slot_id
        WHERE binding.parent_template_id = template_id AND binding.child_role_id IN ('produced', 'consumed')
          AND step.optional AND parent.minimum_count > 0
    ) THEN
        RAISE EXCEPTION 'procedure template % has an optional step fulfilling a required slot', template_id
            USING ERRCODE = 'check_violation';
    END IF;
    IF EXISTS (
        SELECT 1 FROM public.procedure_template_slot_bindings binding
        JOIN public.procedure_template_slots child ON child.id = binding.child_slot_id
        JOIN public.procedure_template_slots parent ON parent.id = binding.parent_slot_id
        WHERE binding.parent_template_id = template_id AND binding.child_role_id IN ('produced', 'consumed')
          AND child.maximum_count IS NOT NULL
          AND (parent.maximum_count IS NULL OR child.maximum_count < parent.maximum_count)
    ) THEN
        RAISE EXCEPTION 'procedure template % has a fulfilling child slot with insufficient capacity', template_id
            USING ERRCODE = 'check_violation';
    END IF;
    IF EXISTS (
        WITH RECURSIVE paths (predecessor_step_id, successor_step_id) AS (
            SELECT edge.predecessor_step_id, edge.successor_step_id
            FROM public.procedure_template_step_edges edge WHERE edge.parent_template_id = template_id
            UNION
            SELECT path.predecessor_step_id, edge.successor_step_id
            FROM paths path
            JOIN public.procedure_template_step_edges edge ON edge.predecessor_step_id = path.successor_step_id
            WHERE edge.parent_template_id = template_id
        )
        SELECT 1 FROM public.procedure_template_slot_bindings effect
        JOIN public.procedure_template_slot_bindings other
          ON other.parent_slot_id = effect.parent_slot_id AND other.step_id <> effect.step_id
        WHERE effect.parent_template_id = template_id
          AND ((effect.child_role_id = 'produced' AND NOT EXISTS (
              SELECT 1 FROM paths path
              WHERE path.predecessor_step_id = effect.step_id AND path.successor_step_id = other.step_id))
            OR (effect.child_role_id = 'consumed' AND NOT EXISTS (
              SELECT 1 FROM paths path
              WHERE path.predecessor_step_id = other.step_id AND path.successor_step_id = effect.step_id)))
    ) THEN
        RAISE EXCEPTION 'procedure template % has unordered production or consumption', template_id
            USING ERRCODE = 'check_violation';
    END IF;
END;
$$;
ALTER FUNCTION require_publishable_procedure_template(UUID) OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION require_publishable_procedure_template(UUID) FROM PUBLIC;

CREATE FUNCTION guard_procedure_template() RETURNS TRIGGER LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
BEGIN
    IF TG_OP = 'INSERT' THEN
        PERFORM public.require_table_path(NEW.table_name_id, NEW.id, TG_TABLE_NAME);
        IF NEW.state_id <> 'draft' THEN
            RAISE EXCEPTION 'procedure template % must be inserted as draft', NEW.id USING ERRCODE = 'check_violation';
        END IF;
        IF NEW.previous_version_id IS NOT NULL AND NOT EXISTS (
            SELECT 1 FROM public.procedure_templates previous
            WHERE previous.id = NEW.previous_version_id AND previous.state_id <> 'draft'
        ) THEN
            RAISE EXCEPTION 'procedure template % follows a version that is not published', NEW.id
                USING ERRCODE = 'check_violation';
        END IF;
        RETURN NEW;
    END IF;
    IF NOT ((OLD.state_id = NEW.state_id)
        OR (OLD.state_id = 'draft' AND NEW.state_id = 'published')
        OR (OLD.state_id = 'published' AND NEW.state_id = 'deprecated')) THEN
        RAISE EXCEPTION 'procedure template % cannot go from % to %', NEW.id, OLD.state_id, NEW.state_id
            USING ERRCODE = 'check_violation';
    END IF;
    IF OLD.state_id <> 'draft' AND (NEW.name IS DISTINCT FROM OLD.name
        OR NEW.description IS DISTINCT FROM OLD.description
        OR NEW.instruction IS DISTINCT FROM OLD.instruction) THEN
        RAISE EXCEPTION 'procedure template % is %, so it cannot change', NEW.id, OLD.state_id
            USING ERRCODE = 'check_violation';
    END IF;
    IF OLD.state_id = 'draft' AND NEW.state_id = 'published' THEN
        PERFORM public.lock_procedure_template(NEW.id);
        PERFORM public.require_publishable_procedure_template(NEW.id);
    END IF;
    RETURN NEW;
END;
$$;
ALTER FUNCTION guard_procedure_template() OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION guard_procedure_template() FROM PUBLIC;
CREATE TRIGGER procedure_templates_guard AFTER INSERT OR UPDATE ON procedure_templates
    FOR EACH ROW EXECUTE FUNCTION guard_procedure_template();

-- Locks a template row, so a change to the template's definition and its
-- publication run one after the other.
CREATE FUNCTION lock_procedure_template(locked_template_id UUID) RETURNS VOID
LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
BEGIN
    PERFORM authz.refuse_repeatable_read();
    PERFORM template.id FROM public.procedure_templates template WHERE template.id = locked_template_id FOR NO KEY UPDATE;
END;
$$;
REVOKE ALL ON procedure_templates FROM PUBLIC, app, identity_writer;
GRANT SELECT, UPDATE (id) ON procedure_templates TO aps_writer;
REVOKE ALL ON procedure_template_slots FROM PUBLIC, app, identity_writer;
REVOKE ALL ON procedure_template_steps FROM PUBLIC, app, identity_writer;
REVOKE ALL ON procedure_template_step_edges FROM PUBLIC, app, identity_writer;
REVOKE ALL ON procedure_template_slot_bindings FROM PUBLIC, app, identity_writer;
REVOKE ALL ON procedure_template_parameters FROM PUBLIC, app, identity_writer;
REVOKE ALL ON procedure_template_step_parameters FROM PUBLIC, app, identity_writer;
GRANT SELECT ON procedure_template_slots, procedure_template_steps, procedure_template_step_edges, procedure_template_slot_bindings,
    procedure_template_parameters, procedure_template_step_parameters TO aps_writer;
ALTER FUNCTION lock_procedure_template(UUID) OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION lock_procedure_template(UUID) FROM PUBLIC;

-- Rejects a change to a row that belongs to a template which is not a draft.
-- A row whose template is gone is being removed by a cascade and passes.
CREATE FUNCTION require_draft_procedure_template(template_id UUID) RETURNS VOID LANGUAGE plpgsql SET search_path = pg_catalog, pg_temp AS $$
BEGIN
    PERFORM public.lock_procedure_template(template_id);
    IF EXISTS (SELECT 1 FROM public.procedure_templates template WHERE template.id = template_id AND template.state_id <> 'draft') THEN
        RAISE EXCEPTION 'procedure template % is not a draft, so its definition cannot change', template_id
            USING ERRCODE = 'check_violation';
    END IF;
END;
$$;
ALTER FUNCTION require_draft_procedure_template(UUID) OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION require_draft_procedure_template(UUID) FROM PUBLIC;

-- Guards a typed template subtype, whose `id` is the template id, and keeps
-- its table on the path of the template's `table_name_id`.
CREATE FUNCTION guard_procedure_template_subtype() RETURNS TRIGGER LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
BEGIN
    IF TG_OP = 'DELETE' THEN
        PERFORM public.require_draft_procedure_template(OLD.id);
        RETURN OLD;
    END IF;
    IF TG_OP = 'INSERT' THEN
        PERFORM public.require_table_path(
            (SELECT template.table_name_id FROM public.procedure_templates template WHERE template.id = NEW.id), NEW.id, TG_TABLE_NAME);
    END IF;
    PERFORM public.require_draft_procedure_template(NEW.id);
    RETURN NEW;
END;
$$;
ALTER FUNCTION guard_procedure_template_subtype() OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION guard_procedure_template_subtype() FROM PUBLIC;


-- Guards a row whose `procedure_template_id` names the template it belongs to.
CREATE FUNCTION guard_procedure_template_member() RETURNS TRIGGER LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
BEGIN
    IF TG_OP = 'DELETE' THEN
        PERFORM public.require_draft_procedure_template(OLD.procedure_template_id);
        RETURN OLD;
    END IF;
    PERFORM public.require_draft_procedure_template(NEW.procedure_template_id);
    RETURN NEW;
END;
$$;
ALTER FUNCTION guard_procedure_template_member() OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION guard_procedure_template_member() FROM PUBLIC;
CREATE TRIGGER procedure_template_slots_guard AFTER INSERT OR UPDATE OR DELETE ON procedure_template_slots
    FOR EACH ROW EXECUTE FUNCTION guard_procedure_template_member();
CREATE TRIGGER procedure_template_parameters_guard AFTER INSERT OR UPDATE OR DELETE ON procedure_template_parameters
    FOR EACH ROW EXECUTE FUNCTION guard_procedure_template_member();

-- A step's child must be published when the step is added. Published
-- templates never change, so no template can become its own ancestor.
CREATE FUNCTION guard_procedure_template_step() RETURNS TRIGGER LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
BEGIN
    IF TG_OP = 'DELETE' THEN
        PERFORM public.require_draft_procedure_template(OLD.parent_template_id);
        RETURN OLD;
    END IF;
    PERFORM public.require_draft_procedure_template(NEW.parent_template_id);
    IF (TG_OP = 'INSERT' OR NEW.child_template_id IS DISTINCT FROM OLD.child_template_id) AND NOT EXISTS (
        SELECT 1 FROM public.procedure_templates child
        WHERE child.id = NEW.child_template_id AND child.state_id = 'published'
    ) THEN
        RAISE EXCEPTION 'step % runs template %, which is not published', NEW.id, NEW.child_template_id
            USING ERRCODE = 'check_violation';
    END IF;
    RETURN NEW;
END;
$$;
ALTER FUNCTION guard_procedure_template_step() OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION guard_procedure_template_step() FROM PUBLIC;
CREATE TRIGGER procedure_template_steps_guard AFTER INSERT OR UPDATE OR DELETE ON procedure_template_steps
    FOR EACH ROW EXECUTE FUNCTION guard_procedure_template_step();

CREATE FUNCTION guard_procedure_template_step_relation() RETURNS TRIGGER LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
BEGIN
    IF TG_OP = 'DELETE' THEN
        PERFORM public.require_draft_procedure_template(OLD.parent_template_id);
        RETURN OLD;
    END IF;
    PERFORM public.require_draft_procedure_template(NEW.parent_template_id);
    RETURN NEW;
END;
$$;
ALTER FUNCTION guard_procedure_template_step_relation() OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION guard_procedure_template_step_relation() FROM PUBLIC;
CREATE TRIGGER procedure_template_step_edges_guard AFTER INSERT OR UPDATE OR DELETE ON procedure_template_step_edges
    FOR EACH ROW EXECUTE FUNCTION guard_procedure_template_step_relation();
CREATE TRIGGER procedure_template_slot_bindings_guard AFTER INSERT OR UPDATE OR DELETE ON procedure_template_slot_bindings
    FOR EACH ROW EXECUTE FUNCTION guard_procedure_template_step_relation();
CREATE TRIGGER procedure_template_step_parameters_guard AFTER INSERT OR UPDATE OR DELETE ON procedure_template_step_parameters
    FOR EACH ROW EXECUTE FUNCTION guard_procedure_template_step_relation();

-- Lists the templates the caller reads, which are those connetto grants the
-- caller and, transitively, the templates their steps run, through steps
-- outside the organizations blocking the caller.
CREATE FUNCTION readable_templates() RETURNS SETOF UUID
LANGUAGE sql STABLE SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
    WITH RECURSIVE readable (id) AS (
        SELECT granted.id FROM authz.resources_with('reader') granted (id)
        UNION
        SELECT step.child_template_id FROM public.procedure_template_steps step
        JOIN readable ON readable.id = step.parent_template_id
        WHERE NOT (step.organization_id = ANY (ARRAY(SELECT authz.blocking_organizations())))
    )
    SELECT readable.id FROM readable
$$;
ALTER FUNCTION readable_templates() OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION readable_templates() FROM PUBLIC;
GRANT EXECUTE ON FUNCTION readable_templates() TO app;

GRANT SELECT, INSERT, UPDATE (name, description, instruction, state_id), DELETE ON procedure_templates TO app;
ALTER TABLE procedure_templates ENABLE ROW LEVEL SECURITY;
ALTER TABLE procedure_templates FORCE ROW LEVEL SECURITY;
CREATE POLICY procedure_templates_select ON procedure_templates FOR SELECT TO app
    USING (id = ANY (ARRAY(SELECT public.readable_templates()))
        OR project_id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY procedure_templates_insert ON procedure_templates FOR INSERT TO app
    WITH CHECK (id = ANY (ARRAY(SELECT authz.resources_with('writer')))
        AND project_id = ANY (ARRAY(SELECT authz.resources_with('writer')))
        AND (previous_version_id IS NULL OR previous_version_id = ANY (ARRAY(SELECT authz.resources_with('reader')))));
CREATE POLICY procedure_templates_update ON procedure_templates FOR UPDATE TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('writer'))))
    WITH CHECK (id = ANY (ARRAY(SELECT authz.resources_with('writer')))
        AND project_id = ANY (ARRAY(SELECT authz.resources_with('writer'))));
CREATE POLICY procedure_templates_delete ON procedure_templates FOR DELETE TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('admin'))));
CREATE POLICY procedure_templates_archived_insert ON procedure_templates AS RESTRICTIVE FOR INSERT TO app
    WITH CHECK (NOT authz.is_archived(id) AND NOT authz.is_archived(previous_version_id));
CREATE POLICY procedure_templates_archived_update ON procedure_templates AS RESTRICTIVE FOR UPDATE TO app
    USING (NOT authz.is_archived(id)) WITH CHECK (NOT authz.is_archived(id));
CREATE POLICY procedure_templates_archived_delete ON procedure_templates AS RESTRICTIVE FOR DELETE TO app
    USING (NOT authz.is_archived(id));
CREATE POLICY procedure_templates_blocked ON procedure_templates AS RESTRICTIVE FOR ALL TO app
    USING (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))))
    WITH CHECK (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))));

GRANT SELECT, INSERT, UPDATE (name, description, asset_model_id, role_id, minimum_count, maximum_count, placed_into_slot_id, retrieved_from_slot_id), DELETE
    ON procedure_template_slots TO app;
ALTER TABLE procedure_template_slots ENABLE ROW LEVEL SECURITY;
ALTER TABLE procedure_template_slots FORCE ROW LEVEL SECURITY;
CREATE POLICY procedure_template_slots_select ON procedure_template_slots FOR SELECT TO app
    USING (procedure_template_id = ANY (ARRAY(SELECT public.readable_templates())));
CREATE POLICY procedure_template_slots_insert ON procedure_template_slots FOR INSERT TO app
    WITH CHECK (procedure_template_id = ANY (ARRAY(SELECT authz.resources_with('writer')))
        AND asset_model_id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY procedure_template_slots_update ON procedure_template_slots FOR UPDATE TO app
    USING (procedure_template_id = ANY (ARRAY(SELECT authz.resources_with('writer'))))
    WITH CHECK (procedure_template_id = ANY (ARRAY(SELECT authz.resources_with('writer')))
        AND asset_model_id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY procedure_template_slots_delete ON procedure_template_slots FOR DELETE TO app
    USING (procedure_template_id = ANY (ARRAY(SELECT authz.resources_with('writer'))));
CREATE POLICY procedure_template_slots_archived_insert ON procedure_template_slots AS RESTRICTIVE FOR INSERT TO app
    WITH CHECK (NOT authz.is_archived(procedure_template_id) AND NOT authz.is_archived(asset_model_id));
CREATE POLICY procedure_template_slots_archived_update ON procedure_template_slots AS RESTRICTIVE FOR UPDATE TO app
    USING (NOT authz.is_archived(procedure_template_id))
    WITH CHECK (NOT authz.is_archived(procedure_template_id) AND NOT authz.is_archived(asset_model_id));
CREATE POLICY procedure_template_slots_archived_delete ON procedure_template_slots AS RESTRICTIVE FOR DELETE TO app
    USING (NOT authz.is_archived(procedure_template_id));
CREATE POLICY procedure_template_slots_blocked ON procedure_template_slots AS RESTRICTIVE FOR ALL TO app
    USING (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))))
    WITH CHECK (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))));

GRANT SELECT, INSERT, UPDATE (name, description, position, optional, repeat_slot_id), DELETE ON procedure_template_steps TO app;
ALTER TABLE procedure_template_steps ENABLE ROW LEVEL SECURITY;
ALTER TABLE procedure_template_steps FORCE ROW LEVEL SECURITY;
CREATE POLICY procedure_template_steps_select ON procedure_template_steps FOR SELECT TO app
    USING (parent_template_id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY procedure_template_steps_insert ON procedure_template_steps FOR INSERT TO app
    WITH CHECK (parent_template_id = ANY (ARRAY(SELECT authz.resources_with('writer')))
        AND child_template_id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY procedure_template_steps_update ON procedure_template_steps FOR UPDATE TO app
    USING (parent_template_id = ANY (ARRAY(SELECT authz.resources_with('writer'))))
    WITH CHECK (parent_template_id = ANY (ARRAY(SELECT authz.resources_with('writer'))));
CREATE POLICY procedure_template_steps_delete ON procedure_template_steps FOR DELETE TO app
    USING (parent_template_id = ANY (ARRAY(SELECT authz.resources_with('writer'))));
CREATE POLICY procedure_template_steps_archived_insert ON procedure_template_steps AS RESTRICTIVE FOR INSERT TO app
    WITH CHECK (NOT authz.is_archived(parent_template_id) AND NOT authz.is_archived(child_template_id));
CREATE POLICY procedure_template_steps_archived_update ON procedure_template_steps AS RESTRICTIVE FOR UPDATE TO app
    USING (NOT authz.is_archived(parent_template_id)) WITH CHECK (NOT authz.is_archived(parent_template_id));
CREATE POLICY procedure_template_steps_archived_delete ON procedure_template_steps AS RESTRICTIVE FOR DELETE TO app
    USING (NOT authz.is_archived(parent_template_id));
CREATE POLICY procedure_template_steps_blocked ON procedure_template_steps AS RESTRICTIVE FOR ALL TO app
    USING (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))))
    WITH CHECK (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))));

GRANT SELECT, INSERT, DELETE ON procedure_template_step_edges TO app;
ALTER TABLE procedure_template_step_edges ENABLE ROW LEVEL SECURITY;
ALTER TABLE procedure_template_step_edges FORCE ROW LEVEL SECURITY;
CREATE POLICY procedure_template_step_edges_select ON procedure_template_step_edges FOR SELECT TO app
    USING (parent_template_id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY procedure_template_step_edges_insert ON procedure_template_step_edges FOR INSERT TO app
    WITH CHECK (parent_template_id = ANY (ARRAY(SELECT authz.resources_with('writer'))));
CREATE POLICY procedure_template_step_edges_delete ON procedure_template_step_edges FOR DELETE TO app
    USING (parent_template_id = ANY (ARRAY(SELECT authz.resources_with('writer'))));
CREATE POLICY procedure_template_step_edges_archived_insert ON procedure_template_step_edges AS RESTRICTIVE FOR INSERT TO app
    WITH CHECK (NOT authz.is_archived(parent_template_id));
CREATE POLICY procedure_template_step_edges_archived_delete ON procedure_template_step_edges AS RESTRICTIVE FOR DELETE TO app
    USING (NOT authz.is_archived(parent_template_id));
CREATE POLICY procedure_template_step_edges_blocked ON procedure_template_step_edges AS RESTRICTIVE FOR ALL TO app
    USING (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))))
    WITH CHECK (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))));

GRANT SELECT, INSERT, DELETE ON procedure_template_slot_bindings TO app;
ALTER TABLE procedure_template_slot_bindings ENABLE ROW LEVEL SECURITY;
ALTER TABLE procedure_template_slot_bindings FORCE ROW LEVEL SECURITY;
CREATE POLICY procedure_template_slot_bindings_select ON procedure_template_slot_bindings FOR SELECT TO app
    USING (parent_template_id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY procedure_template_slot_bindings_insert ON procedure_template_slot_bindings FOR INSERT TO app
    WITH CHECK (parent_template_id = ANY (ARRAY(SELECT authz.resources_with('writer')))
        AND child_template_id = ANY (ARRAY(SELECT authz.resources_with('reader')))
        AND parent_asset_model_id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY procedure_template_slot_bindings_delete ON procedure_template_slot_bindings FOR DELETE TO app
    USING (parent_template_id = ANY (ARRAY(SELECT authz.resources_with('writer'))));
CREATE POLICY procedure_template_slot_bindings_archived_insert ON procedure_template_slot_bindings AS RESTRICTIVE FOR INSERT TO app
    WITH CHECK (NOT authz.is_archived(parent_template_id));
CREATE POLICY procedure_template_slot_bindings_archived_delete ON procedure_template_slot_bindings AS RESTRICTIVE FOR DELETE TO app
    USING (NOT authz.is_archived(parent_template_id));
CREATE POLICY procedure_template_slot_bindings_blocked ON procedure_template_slot_bindings AS RESTRICTIVE FOR ALL TO app
    USING (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))))
    WITH CHECK (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))));

GRANT SELECT, INSERT, UPDATE (name, description, value_kind_id, quantity_kind_id, required, default_number, minimum_number, maximum_number, default_text, default_boolean), DELETE
    ON procedure_template_parameters TO app;
ALTER TABLE procedure_template_parameters ENABLE ROW LEVEL SECURITY;
ALTER TABLE procedure_template_parameters FORCE ROW LEVEL SECURITY;
CREATE POLICY procedure_template_parameters_select ON procedure_template_parameters FOR SELECT TO app
    USING (procedure_template_id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY procedure_template_parameters_insert ON procedure_template_parameters FOR INSERT TO app
    WITH CHECK (procedure_template_id = ANY (ARRAY(SELECT authz.resources_with('writer'))));
CREATE POLICY procedure_template_parameters_update ON procedure_template_parameters FOR UPDATE TO app
    USING (procedure_template_id = ANY (ARRAY(SELECT authz.resources_with('writer'))))
    WITH CHECK (procedure_template_id = ANY (ARRAY(SELECT authz.resources_with('writer'))));
CREATE POLICY procedure_template_parameters_delete ON procedure_template_parameters FOR DELETE TO app
    USING (procedure_template_id = ANY (ARRAY(SELECT authz.resources_with('writer'))));
CREATE POLICY procedure_template_parameters_archived_insert ON procedure_template_parameters AS RESTRICTIVE FOR INSERT TO app
    WITH CHECK (NOT authz.is_archived(procedure_template_id));
CREATE POLICY procedure_template_parameters_archived_update ON procedure_template_parameters AS RESTRICTIVE FOR UPDATE TO app
    USING (NOT authz.is_archived(procedure_template_id)) WITH CHECK (NOT authz.is_archived(procedure_template_id));
CREATE POLICY procedure_template_parameters_archived_delete ON procedure_template_parameters AS RESTRICTIVE FOR DELETE TO app
    USING (NOT authz.is_archived(procedure_template_id));
CREATE POLICY procedure_template_parameters_blocked ON procedure_template_parameters AS RESTRICTIVE FOR ALL TO app
    USING (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))))
    WITH CHECK (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))));

GRANT SELECT, INSERT, UPDATE (number_value, text_value, boolean_value), DELETE ON procedure_template_step_parameters TO app;
ALTER TABLE procedure_template_step_parameters ENABLE ROW LEVEL SECURITY;
ALTER TABLE procedure_template_step_parameters FORCE ROW LEVEL SECURITY;
CREATE POLICY procedure_template_step_parameters_select ON procedure_template_step_parameters FOR SELECT TO app
    USING (parent_template_id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY procedure_template_step_parameters_insert ON procedure_template_step_parameters FOR INSERT TO app
    WITH CHECK (parent_template_id = ANY (ARRAY(SELECT authz.resources_with('writer')))
        AND child_template_id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY procedure_template_step_parameters_update ON procedure_template_step_parameters FOR UPDATE TO app
    USING (parent_template_id = ANY (ARRAY(SELECT authz.resources_with('writer'))))
    WITH CHECK (parent_template_id = ANY (ARRAY(SELECT authz.resources_with('writer'))));
CREATE POLICY procedure_template_step_parameters_delete ON procedure_template_step_parameters FOR DELETE TO app
    USING (parent_template_id = ANY (ARRAY(SELECT authz.resources_with('writer'))));
CREATE POLICY procedure_template_step_parameters_archived_insert ON procedure_template_step_parameters AS RESTRICTIVE FOR INSERT TO app
    WITH CHECK (NOT authz.is_archived(parent_template_id));
CREATE POLICY procedure_template_step_parameters_archived_update ON procedure_template_step_parameters AS RESTRICTIVE FOR UPDATE TO app
    USING (NOT authz.is_archived(parent_template_id)) WITH CHECK (NOT authz.is_archived(parent_template_id));
CREATE POLICY procedure_template_step_parameters_archived_delete ON procedure_template_step_parameters AS RESTRICTIVE FOR DELETE TO app
    USING (NOT authz.is_archived(parent_template_id));
CREATE POLICY procedure_template_step_parameters_blocked ON procedure_template_step_parameters AS RESTRICTIVE FOR ALL TO app
    USING (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))))
    WITH CHECK (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))));

-- Records every change in connetto's `audit_events`.
CREATE TRIGGER procedure_templates_audit AFTER INSERT OR DELETE ON procedure_templates
    FOR EACH ROW EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER procedure_templates_update_audit AFTER UPDATE ON procedure_templates
    FOR EACH ROW WHEN (OLD IS DISTINCT FROM NEW) EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER procedure_template_slots_audit AFTER INSERT OR DELETE ON procedure_template_slots
    FOR EACH ROW EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER procedure_template_slots_update_audit AFTER UPDATE ON procedure_template_slots
    FOR EACH ROW WHEN (OLD IS DISTINCT FROM NEW) EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER procedure_template_steps_audit AFTER INSERT OR DELETE ON procedure_template_steps
    FOR EACH ROW EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER procedure_template_steps_update_audit AFTER UPDATE ON procedure_template_steps
    FOR EACH ROW WHEN (OLD IS DISTINCT FROM NEW) EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER procedure_template_step_edges_audit AFTER INSERT OR DELETE ON procedure_template_step_edges
    FOR EACH ROW EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER procedure_template_step_edges_update_audit AFTER UPDATE ON procedure_template_step_edges
    FOR EACH ROW WHEN (OLD IS DISTINCT FROM NEW) EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER procedure_template_slot_bindings_audit AFTER INSERT OR DELETE ON procedure_template_slot_bindings
    FOR EACH ROW EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER procedure_template_slot_bindings_update_audit AFTER UPDATE ON procedure_template_slot_bindings
    FOR EACH ROW WHEN (OLD IS DISTINCT FROM NEW) EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER procedure_template_parameters_audit AFTER INSERT OR DELETE ON procedure_template_parameters
    FOR EACH ROW EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER procedure_template_parameters_update_audit AFTER UPDATE ON procedure_template_parameters
    FOR EACH ROW WHEN (OLD IS DISTINCT FROM NEW) EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER procedure_template_step_parameters_audit AFTER INSERT OR DELETE ON procedure_template_step_parameters
    FOR EACH ROW EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER procedure_template_step_parameters_update_audit AFTER UPDATE ON procedure_template_step_parameters
    FOR EACH ROW WHEN (OLD IS DISTINCT FROM NEW) EXECUTE FUNCTION authz_write.record_audit_event();
