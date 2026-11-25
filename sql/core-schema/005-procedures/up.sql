-- A procedure is one execution of a published template. A composite template
-- runs as a parent procedure whose children each execute one of its steps,
-- once, or once per asset of the step's repeat slot.
CREATE TABLE procedures (
    -- Identifier of the connetto ownable this procedure extends.
    id UUID PRIMARY KEY REFERENCES ownables (id) ON DELETE CASCADE,
    -- connetto organization owning the row.
    organization_id UUID NOT NULL,
    -- connetto project holding the procedure.
    project_id UUID NOT NULL,
    -- Most concrete APS table holding this procedure.
    table_name_id TEXT NOT NULL REFERENCES table_names (id),
    -- Template the procedure executes.
    procedure_template_id UUID NOT NULL REFERENCES procedure_templates (id),
    -- Lifecycle status, `planned`, `in_progress`, `completed`, `aborted` or
    -- `skipped`. Only a step is skipped, and skipping a non-optional step
    -- records a deviation whose reason is in `note`.
    status_id TEXT NOT NULL DEFAULT 'planned' REFERENCES procedure_statuses (id),
    -- Procedure whose step this procedure executes.
    parent_procedure_id UUID,
    -- Template of the parent procedure.
    parent_template_id UUID,
    -- The step of the parent template this procedure executes.
    step_id UUID,
    -- Top-level procedure of the run this procedure belongs to, itself for a
    -- procedure without a parent.
    root_procedure_id UUID NOT NULL,
    -- Parent binding of the repeat slot this run of a repeated step is for.
    repeat_procedure_asset_id UUID,
    -- When the procedure is planned to happen.
    planned_for TIMESTAMPTZ,
    -- When the procedure started, by the clock of the device that recorded
    -- it, which uploads the change later when offline. A time entered by
    -- hand may lie earlier, and none lies beyond the clock tolerance ahead.
    started_at TIMESTAMPTZ CHECK (started_at <= now() + interval '5 minutes'),
    -- When the procedure completed, was aborted or was skipped, by the same
    -- device clock and under the same bound as `started_at`.
    ended_at TIMESTAMPTZ CHECK (ended_at <= now() + interval '5 minutes'),
    -- connetto user who performed the procedure.
    performed_by_id UUID REFERENCES users (id),
    -- Free-text note, editable in every status.
    note TEXT CHECK (note <> '' AND length(note) <= 65536),
    CHECK (parent_procedure_id <> id),
    CHECK ((parent_procedure_id IS NULL) = (step_id IS NULL)),
    CHECK ((parent_procedure_id IS NULL) = (parent_template_id IS NULL)),
    CHECK (repeat_procedure_asset_id IS NULL OR step_id IS NOT NULL),
    CHECK (status_id <> 'planned' OR (started_at IS NULL AND ended_at IS NULL)),
    CHECK (status_id <> 'in_progress' OR (started_at IS NOT NULL AND ended_at IS NULL)),
    CHECK (status_id <> 'completed' OR (started_at IS NOT NULL AND ended_at IS NOT NULL)),
    CHECK (status_id <> 'aborted' OR ended_at IS NOT NULL),
    CHECK (status_id <> 'skipped' OR (step_id IS NOT NULL AND started_at IS NULL AND ended_at IS NOT NULL)),
    CHECK ((parent_procedure_id IS NULL) = (root_procedure_id = id)),
    CHECK (ended_at >= started_at),
    CHECK (status_id NOT IN ('in_progress', 'completed') OR performed_by_id IS NOT NULL),
    UNIQUE (id, procedure_template_id, organization_id),
    UNIQUE (id, procedure_template_id, status_id, organization_id),
    UNIQUE (id, root_procedure_id, organization_id),
    UNIQUE (id, parent_procedure_id, organization_id),
    UNIQUE (id, project_id, organization_id),
    FOREIGN KEY (parent_procedure_id, parent_template_id, organization_id)
        REFERENCES procedures (id, procedure_template_id, organization_id) ON DELETE CASCADE,
    FOREIGN KEY (parent_procedure_id, root_procedure_id, organization_id)
        REFERENCES procedures (id, root_procedure_id, organization_id) ON DELETE CASCADE,
    -- A child runs in its parent's project. Moving a run moves its whole tree in one transaction.
    FOREIGN KEY (parent_procedure_id, project_id, organization_id)
        REFERENCES procedures (id, project_id, organization_id) DEFERRABLE INITIALLY DEFERRED,
    FOREIGN KEY (step_id, parent_template_id, procedure_template_id)
        REFERENCES procedure_template_steps (id, parent_template_id, child_template_id),
    FOREIGN KEY (id, project_id) REFERENCES ownables (id, project_id) ON UPDATE CASCADE ON DELETE CASCADE,
    FOREIGN KEY (organization_id, id) REFERENCES resources (organization_id, id) ON DELETE CASCADE
);
CREATE INDEX ON procedures (procedure_template_id);
CREATE INDEX ON procedures (parent_procedure_id, project_id);
CREATE INDEX ON procedures (root_procedure_id);
CREATE INDEX ON procedures (performed_by_id);
CREATE INDEX ON procedures (repeat_procedure_asset_id);
-- A step runs at most once under a parent, and a repeated step at most once
-- per bound asset of its repeat slot, aborted attempts aside.
CREATE UNIQUE INDEX procedures_step_once ON procedures (parent_procedure_id, project_id, step_id)
    WHERE repeat_procedure_asset_id IS NULL AND status_id <> 'aborted';
CREATE UNIQUE INDEX procedures_repeat_once ON procedures (parent_procedure_id, project_id, step_id, repeat_procedure_asset_id)
    WHERE repeat_procedure_asset_id IS NOT NULL AND status_id <> 'aborted';
CREATE INDEX ON procedures (status_id);
CREATE INDEX ON procedures (step_id, parent_template_id, procedure_template_id);
CREATE INDEX ON procedures (table_name_id);
INSERT INTO table_names (id) VALUES ('procedures');
INSERT INTO table_name_ancestors (descendant_table_name_id, ancestor_table_name_id) VALUES ('procedures', 'procedures');
INSERT INTO table_name_instances (id, instance_table_name_id) VALUES ('procedure_templates', 'procedures');

-- The only place a procedure names an asset. `asset_id` stays NULL when the
-- asset's model is known but its identity is not tracked. The status is a
-- copy of the procedure's, kept by the cascade, so closed procedures can be
-- guarded and aborted or skipped ones excluded from the uniqueness of production and
-- consumption. A row filling a child slot its step binds names the parent's
-- row, and only rows without one count towards that uniqueness.
CREATE TABLE procedure_assets (
    -- Identifier of the binding.
    id UUID PRIMARY KEY,
    -- connetto organization owning the procedure.
    organization_id UUID NOT NULL,
    -- Procedure binding the asset.
    procedure_id UUID NOT NULL,
    -- Template of the procedure.
    procedure_template_id UUID NOT NULL,
    -- Status of the procedure, kept equal by the cascade.
    status_id TEXT NOT NULL,
    -- Slot the asset fills.
    slot_id UUID NOT NULL,
    -- Model of the slot.
    slot_asset_model_id UUID NOT NULL,
    -- Role of the slot.
    slot_role_id TEXT NOT NULL,
    -- Model of the asset, equal to or below the slot's.
    asset_model_id UUID NOT NULL,
    -- Tracked asset, NULL when only its model is known.
    asset_id UUID,
    -- Parent procedure's binding this row inherits through the step's slot binding.
    parent_procedure_asset_id UUID,
    -- Parent of the procedure, set exactly on a row that inherits a parent binding.
    parent_procedure_id UUID,
    CHECK ((parent_procedure_asset_id IS NULL) = (parent_procedure_id IS NULL)),
    UNIQUE (procedure_id, slot_id, asset_id),
    UNIQUE (id, procedure_id, asset_model_id, organization_id),
    UNIQUE (id, organization_id),
    UNIQUE (id, organization_id, asset_id, procedure_id, status_id),
    FOREIGN KEY (procedure_id, procedure_template_id, status_id, organization_id)
        REFERENCES procedures (id, procedure_template_id, status_id, organization_id) ON DELETE CASCADE
        DEFERRABLE INITIALLY DEFERRED,
    FOREIGN KEY (slot_id, procedure_template_id, slot_asset_model_id, slot_role_id)
        REFERENCES procedure_template_slots (id, procedure_template_id, asset_model_id, role_id),
    FOREIGN KEY (asset_model_id, slot_asset_model_id)
        REFERENCES asset_model_ancestors (descendant_model_id, ancestor_model_id),
    FOREIGN KEY (asset_id, asset_model_id) REFERENCES assets (id, model_id),
    FOREIGN KEY (procedure_id, parent_procedure_id, organization_id) REFERENCES procedures (id, parent_procedure_id, organization_id),
    FOREIGN KEY (parent_procedure_asset_id, parent_procedure_id, asset_model_id, organization_id)
        REFERENCES procedure_assets (id, procedure_id, asset_model_id, organization_id),
    FOREIGN KEY (organization_id, procedure_id) REFERENCES resources (organization_id, id) ON DELETE CASCADE
);
CREATE INDEX ON procedure_assets (asset_id);
CREATE INDEX ON procedure_assets (slot_id);
CREATE INDEX ON procedure_assets (parent_procedure_asset_id);
CREATE UNIQUE INDEX procedure_assets_produced_once ON procedure_assets (asset_id)
    WHERE slot_role_id IN ('produced', 'intermediate') AND parent_procedure_asset_id IS NULL
      AND status_id NOT IN ('aborted', 'skipped');
CREATE UNIQUE INDEX procedure_assets_consumed_once ON procedure_assets (asset_id)
    WHERE slot_role_id IN ('consumed', 'intermediate') AND parent_procedure_asset_id IS NULL
      AND status_id NOT IN ('aborted', 'skipped');
CREATE INDEX ON procedure_assets (asset_model_id, slot_asset_model_id);
INSERT INTO table_names (id) VALUES ('procedure_assets');

-- The value a procedure records for a parameter its template declares. The
-- status is a copy of the procedure's, kept by the cascade.
CREATE TABLE procedure_parameter_values (
    -- Procedure recording the value.
    procedure_id UUID NOT NULL,
    -- Parameter of the procedure's template.
    parameter_id UUID NOT NULL,
    -- Template of the procedure.
    procedure_template_id UUID NOT NULL,
    -- Status of the procedure, kept equal by the cascade.
    status_id TEXT NOT NULL,
    -- Kind of the parameter's value.
    value_kind_id TEXT NOT NULL,
    -- connetto organization owning the procedure.
    organization_id UUID NOT NULL,
    -- Value of a number parameter, in the parameter's unit.
    number_value DOUBLE PRECISION CHECK (number_value > '-Infinity'::double precision AND number_value < 'Infinity'::double precision),
    -- Value of a text parameter.
    text_value TEXT CHECK (text_value <> '' AND length(text_value) <= 8192),
    -- Value of a boolean parameter.
    boolean_value BOOLEAN,
    PRIMARY KEY (procedure_id, parameter_id),
    CHECK ((number_value IS NOT NULL) = (value_kind_id = 'number')),
    CHECK ((text_value IS NOT NULL) = (value_kind_id = 'text')),
    CHECK ((boolean_value IS NOT NULL) = (value_kind_id = 'boolean')),
    FOREIGN KEY (procedure_id, procedure_template_id, status_id, organization_id)
        REFERENCES procedures (id, procedure_template_id, status_id, organization_id) ON DELETE CASCADE
        DEFERRABLE INITIALLY DEFERRED,
    FOREIGN KEY (parameter_id, procedure_template_id, value_kind_id)
        REFERENCES procedure_template_parameters (id, procedure_template_id, value_kind_id),
    FOREIGN KEY (organization_id, procedure_id) REFERENCES resources (organization_id, id) ON DELETE CASCADE
);
CREATE INDEX ON procedure_parameter_values (parameter_id);
INSERT INTO table_names (id) VALUES ('procedure_parameter_values');

-- Locks a procedure row, so the checks that read the procedure's bindings,
-- values or children run one after the writes to them.
CREATE FUNCTION lock_procedure(locked_procedure_id UUID) RETURNS VOID
LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
BEGIN
    PERFORM authz.refuse_repeatable_read();
    PERFORM procedure.id FROM public.procedures procedure WHERE procedure.id = locked_procedure_id FOR NO KEY UPDATE;
END;
$$;
REVOKE ALL ON procedures FROM PUBLIC, app, identity_writer;
GRANT SELECT, UPDATE (id) ON procedures TO aps_writer;
REVOKE ALL ON procedure_assets FROM PUBLIC, app, identity_writer;
REVOKE ALL ON procedure_parameter_values FROM PUBLIC, app, identity_writer;
GRANT SELECT ON procedure_assets, procedure_parameter_values, memberships TO aps_writer;
ALTER FUNCTION lock_procedure(UUID) OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION lock_procedure(UUID) FROM PUBLIC;

-- Whether every required run of a step under a parent has closed as completed
-- or skipped: one run, or one per parent binding of the step's repeat slot.
CREATE FUNCTION step_runs_closed(parent_id UUID, checked_step_id UUID) RETURNS BOOLEAN
LANGUAGE sql STABLE SET search_path = pg_catalog, pg_temp AS $$
    SELECT CASE WHEN step.repeat_slot_id IS NULL THEN EXISTS (
        SELECT 1 FROM public.procedures run
        WHERE run.parent_procedure_id = parent_id AND run.step_id = step.id AND run.status_id IN ('completed', 'skipped')
    ) ELSE NOT EXISTS (
        SELECT 1 FROM public.procedure_assets repeated
        WHERE repeated.procedure_id = parent_id AND repeated.slot_id = step.repeat_slot_id
          AND NOT EXISTS (
              SELECT 1 FROM public.procedures run
              WHERE run.parent_procedure_id = parent_id AND run.step_id = step.id
                AND run.repeat_procedure_asset_id = repeated.id AND run.status_id IN ('completed', 'skipped')
          )
    ) END
    FROM public.procedure_template_steps step WHERE step.id = checked_step_id
$$;
ALTER FUNCTION step_runs_closed(UUID, UUID) OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION step_runs_closed(UUID, UUID) FROM PUBLIC;

-- Rejects starting a step whose predecessor under the same parent has not
-- closed all its runs. An optional predecessor without a run in progress
-- counts as skipped.
CREATE FUNCTION require_predecessors_closed(procedure_id UUID, parent_id UUID, current_step_id UUID)
RETURNS VOID LANGUAGE plpgsql SET search_path = pg_catalog, pg_temp AS $$
BEGIN
    IF EXISTS (
        SELECT 1 FROM public.procedure_template_step_edges edge
        JOIN public.procedure_template_steps predecessor ON predecessor.id = edge.predecessor_step_id
            AND predecessor.organization_id = edge.organization_id
        WHERE edge.successor_step_id = current_step_id
          AND NOT public.step_runs_closed(parent_id, predecessor.id)
          AND (NOT predecessor.optional OR EXISTS (
              SELECT 1 FROM public.procedures sibling
              WHERE sibling.parent_procedure_id = parent_id
                AND sibling.step_id = edge.predecessor_step_id
                AND sibling.status_id = 'in_progress'
          ))
    ) THEN
        RAISE EXCEPTION 'procedure % starts before a predecessor step closed', procedure_id
            USING ERRCODE = 'check_violation';
    END IF;
END;
$$;
ALTER FUNCTION require_predecessors_closed(UUID, UUID, UUID) OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION require_predecessors_closed(UUID, UUID, UUID) FROM PUBLIC;

CREATE FUNCTION require_no_open_children(procedure_id UUID) RETURNS VOID LANGUAGE plpgsql SET search_path = pg_catalog, pg_temp AS $$
BEGIN
    IF EXISTS (
        SELECT 1 FROM public.procedures child
        WHERE child.parent_procedure_id = procedure_id AND child.status_id IN ('planned', 'in_progress')
    ) THEN
        RAISE EXCEPTION 'procedure % has children that are still open', procedure_id
            USING ERRCODE = 'check_violation';
    END IF;
END;
$$;
ALTER FUNCTION require_no_open_children(UUID) OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION require_no_open_children(UUID) FROM PUBLIC;

-- Rejects a skipped non-optional step without the reason in its note.
CREATE FUNCTION require_skip_reason(procedure_id UUID, skipped_step_id UUID, reason TEXT) RETURNS VOID
LANGUAGE plpgsql SET search_path = pg_catalog, pg_temp AS $$
BEGIN
    IF (reason IS NULL OR btrim(reason) = '') AND EXISTS (
        SELECT 1 FROM public.procedure_template_steps step WHERE step.id = skipped_step_id AND NOT step.optional
    ) THEN
        RAISE EXCEPTION 'procedure % skips a required step without a reason in its note', procedure_id
            USING ERRCODE = 'check_violation';
    END IF;
END;
$$;
ALTER FUNCTION require_skip_reason(UUID, UUID, TEXT) OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION require_skip_reason(UUID, UUID, TEXT) FROM PUBLIC;

-- Rejects running a template that is neither `published` nor, under a
-- parent, `deprecated`.
CREATE FUNCTION require_executable_template(procedure_id UUID, template_id UUID, under_parent BOOLEAN) RETURNS VOID
LANGUAGE plpgsql SET search_path = pg_catalog, pg_temp AS $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM public.procedure_templates template
        WHERE template.id = template_id
          AND (template.state_id = 'published' OR (template.state_id = 'deprecated' AND under_parent))
    ) THEN
        RAISE EXCEPTION 'procedure % cannot execute template % in its state', procedure_id, template_id
            USING ERRCODE = 'check_violation';
    END IF;
END;
$$;
ALTER FUNCTION require_executable_template(UUID, UUID, BOOLEAN) OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION require_executable_template(UUID, UUID, BOOLEAN) FROM PUBLIC;

-- Rejects an update naming as performer anyone but the caller, before the
-- users key can tell an existing user from a missing one. The insert policy
-- already confines a new row's performer to the caller.
CREATE FUNCTION require_caller_performer() RETURNS TRIGGER
LANGUAGE plpgsql SET search_path = pg_catalog, pg_temp AS $$
BEGIN
    IF NEW.performed_by_id IS DISTINCT FROM authz.caller_id() THEN
        RAISE EXCEPTION 'procedure % names performer %, who is not the caller', NEW.id, NEW.performed_by_id
            USING ERRCODE = 'check_violation';
    END IF;
    RETURN NEW;
END;
$$;
ALTER FUNCTION require_caller_performer() OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION require_caller_performer() FROM PUBLIC;

CREATE FUNCTION guard_procedure() RETURNS TRIGGER LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
DECLARE
    parent_status TEXT;
    repeat_slot UUID;
    template_table TEXT;
    instance_table TEXT;
BEGIN
    IF TG_OP = 'DELETE' THEN
        IF OLD.parent_procedure_id IS NOT NULL THEN
            PERFORM public.lock_procedure(OLD.parent_procedure_id);
            SELECT parent.status_id INTO parent_status FROM public.procedures parent WHERE parent.id = OLD.parent_procedure_id;
            IF parent_status IN ('completed', 'aborted', 'skipped') THEN
                RAISE EXCEPTION 'parent procedure % is %, so its step runs cannot be deleted', OLD.parent_procedure_id, parent_status
                    USING ERRCODE = 'check_violation';
            END IF;
        END IF;
        RETURN OLD;
    END IF;
    IF TG_OP = 'INSERT' THEN
        PERFORM public.require_table_path(NEW.table_name_id, NEW.id, TG_TABLE_NAME);
        SELECT template.table_name_id INTO template_table FROM public.procedure_templates template
        WHERE template.id = NEW.procedure_template_id;
        instance_table := public.instance_table_name(template_table);
        IF instance_table IS DISTINCT FROM NEW.table_name_id THEN
            RAISE EXCEPTION 'procedure % does not name its template instance table', NEW.id USING ERRCODE = 'check_violation';
        END IF;
        IF NEW.status_id NOT IN ('planned', 'in_progress') THEN
            RAISE EXCEPTION 'procedure % must be inserted as planned or in_progress', NEW.id
                USING ERRCODE = 'check_violation';
        END IF;
        PERFORM public.require_executable_template(NEW.id, NEW.procedure_template_id, NEW.parent_procedure_id IS NOT NULL);
        IF NEW.parent_procedure_id IS NOT NULL THEN
            PERFORM public.lock_procedure(NEW.parent_procedure_id);
            SELECT parent.status_id INTO parent_status FROM public.procedures parent WHERE parent.id = NEW.parent_procedure_id;
            IF parent_status IS NULL OR parent_status IN ('completed', 'aborted', 'skipped')
                OR (parent_status = 'planned' AND NEW.status_id <> 'planned') THEN
                RAISE EXCEPTION 'procedure % cannot be % under a parent that is %', NEW.id, NEW.status_id, parent_status
                    USING ERRCODE = 'check_violation';
            END IF;
            SELECT step.repeat_slot_id INTO repeat_slot FROM public.procedure_template_steps step WHERE step.id = NEW.step_id;
            IF (repeat_slot IS NULL AND NEW.repeat_procedure_asset_id IS NOT NULL)
                OR (repeat_slot IS NOT NULL AND NOT EXISTS (
                    SELECT 1 FROM public.procedure_assets repeated
                    WHERE repeated.id = NEW.repeat_procedure_asset_id
                      AND repeated.procedure_id = NEW.parent_procedure_id
                      AND repeated.slot_id = repeat_slot
                )) THEN
                RAISE EXCEPTION 'procedure % must name a parent binding of its step''s repeat slot, and only then', NEW.id
                    USING ERRCODE = 'check_violation';
            END IF;
            IF NEW.status_id = 'in_progress' THEN
                PERFORM public.require_predecessors_closed(NEW.id, NEW.parent_procedure_id, NEW.step_id);
            END IF;
        END IF;
        RETURN NEW;
    END IF;

    IF OLD.status_id IN ('completed', 'aborted', 'skipped') THEN
        IF NEW.status_id IS DISTINCT FROM OLD.status_id
            OR NEW.planned_for IS DISTINCT FROM OLD.planned_for
            OR NEW.started_at IS DISTINCT FROM OLD.started_at
            OR NEW.ended_at IS DISTINCT FROM OLD.ended_at
            OR NEW.performed_by_id IS DISTINCT FROM OLD.performed_by_id THEN
            RAISE EXCEPTION 'procedure % is %, so only its note can change', NEW.id, OLD.status_id
                USING ERRCODE = 'check_violation';
        END IF;
        IF NEW.status_id = 'skipped' THEN
            PERFORM public.require_skip_reason(NEW.id, NEW.step_id, NEW.note);
        END IF;
        RETURN NEW;
    END IF;

    IF NEW.status_id = OLD.status_id THEN
        RETURN NEW;
    END IF;

    IF OLD.status_id = 'planned' AND NEW.status_id = 'in_progress' THEN
        PERFORM public.require_executable_template(NEW.id, NEW.procedure_template_id, NEW.parent_procedure_id IS NOT NULL);
        IF NEW.parent_procedure_id IS NOT NULL THEN
            PERFORM public.lock_procedure(NEW.parent_procedure_id);
            IF NOT EXISTS (
                SELECT 1 FROM public.procedures parent
                WHERE parent.id = NEW.parent_procedure_id AND parent.status_id = 'in_progress'
            ) THEN
                RAISE EXCEPTION 'procedure % starts while its parent is not in progress', NEW.id
                    USING ERRCODE = 'check_violation';
            END IF;
            PERFORM public.require_predecessors_closed(NEW.id, NEW.parent_procedure_id, NEW.step_id);
        END IF;
        RETURN NEW;
    END IF;

    IF NEW.status_id = 'skipped' THEN
        IF OLD.status_id <> 'planned' THEN
            RAISE EXCEPTION 'procedure % is %, and only a planned step can be skipped', NEW.id, OLD.status_id
                USING ERRCODE = 'check_violation';
        END IF;
        PERFORM public.require_no_open_children(NEW.id);
        PERFORM public.require_skip_reason(NEW.id, NEW.step_id, NEW.note);
        RETURN NEW;
    END IF;

    IF NEW.status_id = 'aborted' THEN
        PERFORM public.require_no_open_children(NEW.id);
        RETURN NEW;
    END IF;

    IF OLD.status_id = 'in_progress' AND NEW.status_id = 'completed' THEN
        PERFORM public.require_no_open_children(NEW.id);
        IF EXISTS (
            SELECT 1 FROM public.procedure_template_slots slot
            WHERE slot.procedure_template_id = NEW.procedure_template_id
              AND slot.minimum_count > (
                  SELECT count(*) FROM public.procedure_assets binding
                  WHERE binding.procedure_id = NEW.id AND binding.slot_id = slot.id
              )
        ) THEN
            RAISE EXCEPTION 'procedure % completes with a slot below its minimum count', NEW.id
                USING ERRCODE = 'check_violation';
        END IF;
        IF EXISTS (
            SELECT 1 FROM public.procedure_template_parameters parameter
            WHERE parameter.procedure_template_id = NEW.procedure_template_id AND parameter.required
              AND NOT EXISTS (
                  SELECT 1 FROM public.procedure_parameter_values recorded
                  WHERE recorded.procedure_id = NEW.id AND recorded.parameter_id = parameter.id
              )
        ) THEN
            RAISE EXCEPTION 'procedure % completes without a required parameter value', NEW.id
                USING ERRCODE = 'check_violation';
        END IF;
        IF EXISTS (
            SELECT 1 FROM public.procedure_template_steps step
            WHERE step.parent_template_id = NEW.procedure_template_id
              AND NOT step.optional
              AND NOT public.step_runs_closed(NEW.id, step.id)
        ) THEN
            RAISE EXCEPTION 'procedure % completes with a required step neither completed nor skipped', NEW.id
                USING ERRCODE = 'check_violation';
        END IF;
        IF EXISTS (
            SELECT 1 FROM public.procedure_assets parent_binding
            JOIN public.procedure_template_slot_bindings fulfillment
                ON fulfillment.parent_template_id = NEW.procedure_template_id
                AND fulfillment.parent_slot_id = parent_binding.slot_id
                AND fulfillment.child_role_id IN ('produced', 'consumed')
            WHERE parent_binding.procedure_id = NEW.id
              AND parent_binding.slot_role_id IN ('produced', 'consumed', 'intermediate')
              AND NOT EXISTS (
                  SELECT 1 FROM public.procedure_assets child_binding
                  JOIN public.procedures child ON child.id = child_binding.procedure_id
                  WHERE child_binding.parent_procedure_asset_id = parent_binding.id
                    AND child_binding.slot_id = fulfillment.child_slot_id
                    AND child.parent_procedure_id = NEW.id
                    AND child.step_id = fulfillment.step_id
                    AND child.status_id = 'completed'
              )
        ) THEN
            RAISE EXCEPTION 'procedure % completes with an unfulfilled asset binding', NEW.id
                USING ERRCODE = 'check_violation';
        END IF;
        RETURN NEW;
    END IF;

    RAISE EXCEPTION 'procedure % cannot go from % to %', NEW.id, OLD.status_id, NEW.status_id
        USING ERRCODE = 'check_violation';
END;
$$;
ALTER FUNCTION guard_procedure() OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION guard_procedure() FROM PUBLIC;
CREATE TRIGGER procedures_guard AFTER INSERT OR UPDATE OR DELETE ON procedures
    FOR EACH ROW EXECUTE FUNCTION guard_procedure();
CREATE TRIGGER procedures_performer BEFORE UPDATE OF performed_by_id, status_id ON procedures
    FOR EACH ROW WHEN ((NEW.performed_by_id IS DISTINCT FROM OLD.performed_by_id AND NEW.performed_by_id IS NOT NULL)
        OR (OLD.status_id = 'planned' AND NEW.status_id = 'in_progress'))
    EXECUTE FUNCTION require_caller_performer();

-- Writes a procedure's new status into its bindings and parameter values,
-- whose deferred foreign keys hold them equal to it at commit. The
-- `procedures_status_copy` triggers sort after `procedures_guard`, so a refused
-- status change is refused by the procedure's own guard first.
CREATE FUNCTION copy_procedure_status() RETURNS TRIGGER LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
BEGIN
    UPDATE public.procedure_assets SET status_id = NEW.status_id WHERE procedure_id = NEW.id;
    UPDATE public.procedure_parameter_values SET status_id = NEW.status_id WHERE procedure_id = NEW.id;
    RETURN NULL;
END;
$$;
GRANT UPDATE (status_id) ON procedure_assets, procedure_parameter_values TO aps_writer;
ALTER FUNCTION copy_procedure_status() OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION copy_procedure_status() FROM PUBLIC;
CREATE TRIGGER procedures_status_copy AFTER UPDATE OF status_id ON procedures
    FOR EACH ROW WHEN (OLD.status_id IS DISTINCT FROM NEW.status_id) EXECUTE FUNCTION copy_procedure_status();

-- Rejects, wherever both times are known, a procedure starting before its
-- parent started or ending after its parent ended, and a run starting before
-- a run of a predecessor step under the same parent ended. An aborted
-- predecessor run closes nothing and is ignored.
CREATE FUNCTION require_nested_times(checked_id UUID) RETURNS VOID LANGUAGE plpgsql SET search_path = pg_catalog, pg_temp AS $$
DECLARE
    inner_id UUID;
    outer_id UUID;
BEGIN
    SELECT child.id, parent.id INTO inner_id, outer_id
    FROM public.procedures child
    JOIN public.procedures parent ON parent.id = child.parent_procedure_id AND parent.organization_id = child.organization_id
    WHERE (child.id = checked_id OR parent.id = checked_id) AND child.started_at < parent.started_at
    LIMIT 1;
    IF inner_id IS NOT NULL THEN
        RAISE EXCEPTION 'procedure % starts before its parent % started', inner_id, outer_id
            USING ERRCODE = 'check_violation';
    END IF;
    SELECT child.id, parent.id INTO inner_id, outer_id
    FROM public.procedures child
    JOIN public.procedures parent ON parent.id = child.parent_procedure_id AND parent.organization_id = child.organization_id
    WHERE (child.id = checked_id OR parent.id = checked_id) AND child.ended_at > parent.ended_at
    LIMIT 1;
    IF inner_id IS NOT NULL THEN
        RAISE EXCEPTION 'procedure % ends after its parent % ended', inner_id, outer_id
            USING ERRCODE = 'check_violation';
    END IF;
    SELECT successor.id, predecessor.id INTO inner_id, outer_id
    FROM public.procedures successor
    JOIN public.procedure_template_step_edges edge
        ON edge.successor_step_id = successor.step_id AND edge.parent_template_id = successor.parent_template_id
    JOIN public.procedures predecessor
        ON predecessor.parent_procedure_id = successor.parent_procedure_id AND predecessor.step_id = edge.predecessor_step_id
        AND predecessor.organization_id = successor.organization_id
    WHERE (successor.id = checked_id OR predecessor.id = checked_id)
      AND predecessor.status_id <> 'aborted' AND successor.started_at < predecessor.ended_at
    LIMIT 1;
    IF inner_id IS NOT NULL THEN
        RAISE EXCEPTION 'procedure % starts before its predecessor run % ended', inner_id, outer_id
            USING ERRCODE = 'check_violation';
    END IF;
END;
$$;
ALTER FUNCTION require_nested_times(UUID) OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION require_nested_times(UUID) FROM PUBLIC;

-- Checks the times of a procedure against its parent, its children and its
-- sibling runs at commit, after locking the parent that every sibling's and
-- child's check locks too.
CREATE FUNCTION check_procedure_times() RETURNS TRIGGER LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
BEGIN
    IF NEW.parent_procedure_id IS NOT NULL THEN
        PERFORM public.lock_procedure(NEW.parent_procedure_id);
    END IF;
    PERFORM public.require_nested_times(NEW.id);
    RETURN NULL;
END;
$$;
ALTER FUNCTION check_procedure_times() OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION check_procedure_times() FROM PUBLIC;
CREATE CONSTRAINT TRIGGER procedures_times AFTER INSERT OR UPDATE OF started_at, ended_at ON procedures
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION check_procedure_times();

-- Rejects bindings on closed procedures, enforces `maximum_count`, and makes
-- a row filling a child slot its step binds inherit the parent's row of the
-- bound parent slot, the repeat binding itself for a repeated step's repeat
-- slot. A row other rows or runs inherit keeps its asset and stays. A run in
-- progress or completed that binds an asset keeps its history and is aborted
-- instead of deleted.
CREATE FUNCTION guard_procedure_asset() RETURNS TRIGGER LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
DECLARE
    bound_parent_slot UUID;
    repeat_slot UUID;
    parent_id UUID;
    repeat_row UUID;
BEGIN
    IF TG_OP = 'DELETE' THEN
        PERFORM public.lock_procedure(OLD.procedure_id);
        IF EXISTS (SELECT 1 FROM public.procedures procedure WHERE procedure.id = OLD.procedure_id) THEN
            IF OLD.status_id IN ('completed', 'aborted', 'skipped') THEN
                RAISE EXCEPTION 'procedure % is %, so its assets cannot change', OLD.procedure_id, OLD.status_id
                    USING ERRCODE = 'check_violation';
            END IF;
            IF EXISTS (SELECT 1 FROM public.procedures run WHERE run.repeat_procedure_asset_id = OLD.id) THEN
                RAISE EXCEPTION 'binding % has runs of a repeated step', OLD.id USING ERRCODE = 'check_violation';
            END IF;
        ELSIF OLD.asset_id IS NOT NULL AND OLD.status_id IN ('in_progress', 'completed') THEN
            RAISE EXCEPTION 'procedure % is % and uses an asset, so it cannot be deleted', OLD.procedure_id, OLD.status_id
                USING ERRCODE = 'check_violation';
        END IF;
        RETURN OLD;
    END IF;
    PERFORM public.lock_procedure(NEW.procedure_id);
    IF TG_OP = 'UPDATE' THEN
        IF OLD.status_id IN ('completed', 'aborted', 'skipped') THEN
            RAISE EXCEPTION 'procedure % is %, so its assets cannot change', OLD.procedure_id, OLD.status_id
                USING ERRCODE = 'check_violation';
        END IF;
        IF NEW.status_id IS DISTINCT FROM OLD.status_id THEN
            RETURN NEW;
        END IF;
        IF NEW.asset_id IS DISTINCT FROM OLD.asset_id AND (
            EXISTS (SELECT 1 FROM public.procedure_assets inheriting WHERE inheriting.parent_procedure_asset_id = OLD.id)
            OR EXISTS (SELECT 1 FROM public.procedures run WHERE run.repeat_procedure_asset_id = OLD.id)
        ) THEN
            RAISE EXCEPTION 'binding % is inherited by a step, so its asset cannot change', OLD.id
                USING ERRCODE = 'check_violation';
        END IF;
    ELSIF NEW.status_id IN ('completed', 'aborted', 'skipped') THEN
        RAISE EXCEPTION 'procedure % is %, so its assets cannot change', NEW.procedure_id, NEW.status_id
            USING ERRCODE = 'check_violation';
    END IF;
    IF EXISTS (
        SELECT 1 FROM public.procedure_template_slots slot
        WHERE slot.id = NEW.slot_id
          AND slot.maximum_count <= (
              SELECT count(*) FROM public.procedure_assets binding
              WHERE binding.procedure_id = NEW.procedure_id AND binding.slot_id = NEW.slot_id AND binding.id <> NEW.id
          )
    ) THEN
        RAISE EXCEPTION 'procedure % binds more assets than slot % admits', NEW.procedure_id, NEW.slot_id
            USING ERRCODE = 'check_violation';
    END IF;
    SELECT binding.parent_slot_id, step.repeat_slot_id, child.parent_procedure_id, child.repeat_procedure_asset_id
        INTO bound_parent_slot, repeat_slot, parent_id, repeat_row
        FROM public.procedures child
        JOIN public.procedure_template_slot_bindings binding
            ON binding.step_id = child.step_id AND binding.child_slot_id = NEW.slot_id
        JOIN public.procedure_template_steps step ON step.id = child.step_id
        WHERE child.id = NEW.procedure_id;
    IF bound_parent_slot IS NOT NULL THEN
        PERFORM public.lock_procedure(parent_id);
    END IF;
    IF bound_parent_slot IS NULL THEN
        IF NEW.parent_procedure_asset_id IS NOT NULL THEN
            RAISE EXCEPTION 'procedure % inherits a binding for slot %, which its step does not bind', NEW.procedure_id, NEW.slot_id
                USING ERRCODE = 'check_violation';
        END IF;
    ELSIF NOT EXISTS (
        SELECT 1 FROM public.procedure_assets parent_asset
        WHERE parent_asset.id = NEW.parent_procedure_asset_id
          AND parent_asset.procedure_id = parent_id
          AND parent_asset.slot_id = bound_parent_slot
          AND parent_asset.asset_id IS NOT DISTINCT FROM NEW.asset_id
    ) OR (bound_parent_slot = repeat_slot AND NEW.parent_procedure_asset_id IS DISTINCT FROM repeat_row) THEN
        RAISE EXCEPTION 'procedure % must bind the asset its parent binds to slot %', NEW.procedure_id, NEW.slot_id
            USING ERRCODE = 'check_violation';
    END IF;
    RETURN NEW;
END;
$$;
ALTER FUNCTION guard_procedure_asset() OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION guard_procedure_asset() FROM PUBLIC;
CREATE TRIGGER procedure_assets_guard AFTER INSERT OR UPDATE OR DELETE ON procedure_assets
    FOR EACH ROW EXECUTE FUNCTION guard_procedure_asset();

-- One row per binding naming an asset, kept equal to the binding and its
-- procedure by trusted triggers, so an asset's readers check its lifetime
-- without reading the procedures that used it.
CREATE TABLE asset_uses (
    -- Binding naming the asset.
    id UUID PRIMARY KEY,
    -- connetto organization owning the procedure.
    organization_id UUID NOT NULL,
    -- Asset the binding names.
    asset_id UUID NOT NULL,
    -- Procedure binding the asset.
    procedure_id UUID NOT NULL,
    -- Identifiers of the procedure's ancestors and the procedure, from the
    -- root down, each followed by `/`, so a use nested in another has the
    -- other's lineage as prefix.
    lineage TEXT NOT NULL CHECK (lineage <> '' AND length(lineage) <= 4096),
    -- Whether the binding produces the asset.
    produces BOOLEAN NOT NULL,
    -- Whether the binding consumes the asset.
    consumes BOOLEAN NOT NULL,
    -- Status of the procedure.
    status_id TEXT NOT NULL,
    -- When the procedure started.
    started_at TIMESTAMPTZ,
    -- When the procedure ended.
    ended_at TIMESTAMPTZ,
    FOREIGN KEY (id, organization_id, asset_id, procedure_id, status_id)
        REFERENCES procedure_assets (id, organization_id, asset_id, procedure_id, status_id) ON DELETE CASCADE
        DEFERRABLE INITIALLY DEFERRED
);
CREATE INDEX ON asset_uses (asset_id);
CREATE INDEX ON asset_uses (procedure_id);
CREATE INDEX ON asset_uses (status_id);
INSERT INTO table_names (id) VALUES ('asset_uses');
REVOKE ALL ON asset_uses FROM PUBLIC, app, identity_writer;
GRANT SELECT, INSERT, UPDATE, DELETE ON asset_uses TO aps_writer;

-- Lists the assets whose ledgers the caller reads, which are the assets the
-- caller reads and those bound by procedures the caller reads, outside the
-- organizations blocking the caller.
CREATE FUNCTION ledger_assets() RETURNS SETOF UUID
LANGUAGE sql STABLE SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
    SELECT readable.id FROM authz.resources_with('reader') readable (id)
    UNION
    SELECT binding.asset_id FROM public.procedure_assets binding
    WHERE binding.asset_id IS NOT NULL AND binding.procedure_id IN (SELECT authz.resources_with('reader'))
      AND NOT (binding.organization_id = ANY (ARRAY(SELECT authz.blocking_organizations())))
$$;
ALTER FUNCTION ledger_assets() OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION ledger_assets() FROM PUBLIC;
GRANT EXECUTE ON FUNCTION ledger_assets() TO app;

-- Serialises the checks of one asset's ledger without reading its row.
CREATE FUNCTION lock_ledger_asset(locked_asset_id UUID) RETURNS VOID
LANGUAGE plpgsql SET search_path = pg_catalog, pg_temp AS $$
BEGIN
    PERFORM authz.refuse_repeatable_read();
    PERFORM pg_advisory_xact_lock(hashtextextended(locked_asset_id::text, 0));
END;
$$;
ALTER FUNCTION lock_ledger_asset(UUID) OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION lock_ledger_asset(UUID) FROM PUBLIC;

-- The lineage of a procedure, as `asset_uses.lineage` holds it. The walk
-- stops after 111 procedures, whose 37-character segments already exceed the
-- 4096 characters `asset_uses.lineage` admits.
CREATE FUNCTION procedure_lineage(lineage_procedure_id UUID) RETURNS TEXT
LANGUAGE sql STABLE SET search_path = pg_catalog, pg_temp AS $$
    WITH RECURSIVE line (id, parent_id, depth) AS (
        SELECT run.id, run.parent_procedure_id, 0 FROM public.procedures run WHERE run.id = lineage_procedure_id
        UNION ALL
        SELECT run.id, run.parent_procedure_id, line.depth + 1
        FROM public.procedures run JOIN line ON run.id = line.parent_id
        WHERE line.depth < 110
    )
    SELECT string_agg(line.id::text || '/', '' ORDER BY line.depth DESC) FROM line
$$;
ALTER FUNCTION procedure_lineage(UUID) OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION procedure_lineage(UUID) FROM PUBLIC;

-- Rewrites the use of a written binding.
CREATE FUNCTION record_asset_use() RETURNS TRIGGER LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
BEGIN
    IF TG_OP = 'UPDATE' THEN
        DELETE FROM public.asset_uses used WHERE used.id = OLD.id;
    END IF;
    IF NEW.asset_id IS NOT NULL THEN
        INSERT INTO public.asset_uses
            (id, organization_id, asset_id, procedure_id, lineage, produces, consumes, status_id, started_at, ended_at)
        SELECT NEW.id, NEW.organization_id, NEW.asset_id, NEW.procedure_id, public.procedure_lineage(run.id),
            NEW.slot_role_id IN ('produced', 'intermediate'), NEW.slot_role_id IN ('consumed', 'intermediate'),
            run.status_id, run.started_at, run.ended_at
        FROM public.procedures run WHERE run.id = NEW.procedure_id;
    END IF;
    RETURN NULL;
END;
$$;
ALTER FUNCTION record_asset_use() OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION record_asset_use() FROM PUBLIC;
CREATE TRIGGER procedure_assets_use AFTER INSERT ON procedure_assets
    FOR EACH ROW EXECUTE FUNCTION record_asset_use();
CREATE TRIGGER procedure_assets_use_update AFTER UPDATE OF asset_id ON procedure_assets
    FOR EACH ROW WHEN (OLD.asset_id IS DISTINCT FROM NEW.asset_id) EXECUTE FUNCTION record_asset_use();

-- Copies a procedure's status and times into the uses of its bindings.
CREATE FUNCTION copy_procedure_use_times() RETURNS TRIGGER LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
BEGIN
    UPDATE public.asset_uses SET status_id = NEW.status_id, started_at = NEW.started_at, ended_at = NEW.ended_at
    WHERE procedure_id = NEW.id;
    RETURN NULL;
END;
$$;
ALTER FUNCTION copy_procedure_use_times() OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION copy_procedure_use_times() FROM PUBLIC;
CREATE TRIGGER procedures_use_times AFTER UPDATE OF status_id, started_at, ended_at ON procedures
    FOR EACH ROW WHEN (OLD.status_id IS DISTINCT FROM NEW.status_id OR OLD.started_at IS DISTINCT FROM NEW.started_at
        OR OLD.ended_at IS DISTINCT FROM NEW.ended_at)
    EXECUTE FUNCTION copy_procedure_use_times();

-- Rejects uses of an asset outside its life. Among uses whose procedures are
-- neither aborted nor skipped and not nested in one another, a use starts no
-- earlier than the producing one ended and ends no later than the consuming
-- one started. An unknown time counts against the started procedure.
CREATE FUNCTION require_asset_lifetime(checked_asset_id UUID) RETURNS VOID LANGUAGE plpgsql SET search_path = pg_catalog, pg_temp AS $$
BEGIN
    IF EXISTS (
        SELECT 1 FROM public.asset_uses edge
        JOIN public.asset_uses other ON other.asset_id = edge.asset_id AND other.procedure_id <> edge.procedure_id
        WHERE edge.asset_id = checked_asset_id
          AND edge.status_id NOT IN ('aborted', 'skipped') AND other.status_id NOT IN ('aborted', 'skipped')
          AND ((edge.produces AND other.started_at IS NOT NULL
                    AND (edge.ended_at IS NULL OR other.started_at < edge.ended_at))
               OR (edge.consumes AND edge.started_at IS NOT NULL AND other.started_at IS NOT NULL
                    AND (other.ended_at IS NULL OR other.ended_at > edge.started_at)))
          AND other.lineage NOT LIKE edge.lineage || '%' AND edge.lineage NOT LIKE other.lineage || '%'
    ) THEN
        RAISE EXCEPTION 'asset % is used outside the time between its production and its consumption', checked_asset_id
            USING ERRCODE = 'check_violation';
    END IF;
END;
$$;
ALTER FUNCTION require_asset_lifetime(UUID) OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION require_asset_lifetime(UUID) FROM PUBLIC;

CREATE FUNCTION check_asset_use_lifetime() RETURNS TRIGGER LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
BEGIN
    PERFORM public.lock_ledger_asset(NEW.asset_id);
    PERFORM public.require_asset_lifetime(NEW.asset_id);
    RETURN NULL;
END;
$$;
ALTER FUNCTION check_asset_use_lifetime() OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION check_asset_use_lifetime() FROM PUBLIC;
CREATE CONSTRAINT TRIGGER asset_uses_lifetime AFTER INSERT OR UPDATE ON asset_uses
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION check_asset_use_lifetime();

-- Guards a typed procedure subtype, which carries a copy of `status_id` held
-- equal to its procedure by a deferred `FOREIGN KEY (id, ..., status_id)
-- REFERENCES procedures (...)`, and keeps its table on the path of the
-- procedure's `table_name_id`.
CREATE FUNCTION guard_procedure_subtype() RETURNS TRIGGER LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
BEGIN
    IF TG_OP = 'DELETE' THEN
        IF OLD.status_id IN ('completed', 'aborted', 'skipped')
            AND EXISTS (SELECT 1 FROM public.procedures procedure WHERE procedure.id = OLD.id) THEN
            RAISE EXCEPTION 'procedure % is %, so it cannot change', OLD.id, OLD.status_id
                USING ERRCODE = 'check_violation';
        END IF;
        RETURN OLD;
    END IF;
    IF TG_OP = 'INSERT' THEN
        PERFORM public.require_table_path(
            (SELECT procedure.table_name_id FROM public.procedures procedure WHERE procedure.id = NEW.id), NEW.id, TG_TABLE_NAME);
    END IF;
    IF (TG_OP = 'INSERT' AND NEW.status_id IN ('completed', 'aborted', 'skipped'))
        OR (TG_OP = 'UPDATE' AND OLD.status_id IN ('completed', 'aborted', 'skipped')) THEN
        RAISE EXCEPTION 'procedure % is %, so it cannot change', NEW.id, NEW.status_id
            USING ERRCODE = 'check_violation';
    END IF;
    RETURN NEW;
END;
$$;
ALTER FUNCTION guard_procedure_subtype() OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION guard_procedure_subtype() FROM PUBLIC;


-- Rejects recording a parameter value on a closed procedure.
CREATE FUNCTION guard_procedure_parameter_value() RETURNS TRIGGER LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
BEGIN
    IF TG_OP = 'DELETE' THEN
        PERFORM public.lock_procedure(OLD.procedure_id);
        IF OLD.status_id IN ('completed', 'aborted', 'skipped')
            AND EXISTS (SELECT 1 FROM public.procedures procedure WHERE procedure.id = OLD.procedure_id) THEN
            RAISE EXCEPTION 'procedure % is %, so its parameter values cannot change', OLD.procedure_id, OLD.status_id
                USING ERRCODE = 'check_violation';
        END IF;
        RETURN OLD;
    END IF;
    PERFORM public.lock_procedure(NEW.procedure_id);
    IF (TG_OP = 'INSERT' AND NEW.status_id IN ('completed', 'aborted', 'skipped'))
        OR (TG_OP = 'UPDATE' AND OLD.status_id IN ('completed', 'aborted', 'skipped')) THEN
        RAISE EXCEPTION 'procedure % is %, so its parameter values cannot change', NEW.procedure_id, NEW.status_id
            USING ERRCODE = 'check_violation';
    END IF;
    RETURN NEW;
END;
$$;
ALTER FUNCTION guard_procedure_parameter_value() OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION guard_procedure_parameter_value() FROM PUBLIC;
CREATE TRIGGER procedure_parameter_values_guard AFTER INSERT OR UPDATE OR DELETE ON procedure_parameter_values
    FOR EACH ROW EXECUTE FUNCTION guard_procedure_parameter_value();

-- Lists number parameters recorded outside their admissible range.
CREATE FUNCTION procedure_parameter_deviations(checked_procedure_id UUID) RETURNS SETOF TEXT
LANGUAGE sql STABLE SET search_path = pg_catalog, pg_temp AS $$
    SELECT parameter.name
    FROM public.procedure_parameter_values recorded
    JOIN public.procedure_template_parameters parameter ON parameter.id = recorded.parameter_id
    WHERE recorded.procedure_id = checked_procedure_id
      AND (recorded.number_value < parameter.minimum_number OR recorded.number_value > parameter.maximum_number)
$$;
ALTER FUNCTION procedure_parameter_deviations(UUID) OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION procedure_parameter_deviations(UUID) FROM PUBLIC;
GRANT EXECUTE ON FUNCTION procedure_parameter_deviations(UUID) TO app;

CREATE FUNCTION check_procedure_parameter_deviations() RETURNS TRIGGER
LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
BEGIN
    IF EXISTS (
        SELECT 1 FROM public.procedures run WHERE run.id = NEW.id AND run.status_id = 'completed'
          AND (run.note IS NULL OR btrim(run.note) = '')
          AND EXISTS (SELECT 1 FROM public.procedure_parameter_deviations(run.id))
    ) THEN
        RAISE EXCEPTION 'procedure % requires a note for its deviations', NEW.id USING ERRCODE = 'check_violation';
    END IF;
    RETURN NULL;
END;
$$;
ALTER FUNCTION check_procedure_parameter_deviations() OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION check_procedure_parameter_deviations() FROM PUBLIC;
CREATE CONSTRAINT TRIGGER procedures_parameter_deviations AFTER INSERT OR UPDATE ON procedures
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION check_procedure_parameter_deviations();

GRANT SELECT, INSERT, UPDATE (status_id, planned_for, started_at, ended_at, performed_by_id, note), DELETE
    ON procedures TO app;
ALTER TABLE procedures ENABLE ROW LEVEL SECURITY;
ALTER TABLE procedures FORCE ROW LEVEL SECURITY;
CREATE POLICY procedures_select ON procedures FOR SELECT TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('reader')))
        OR project_id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY procedures_insert ON procedures FOR INSERT TO app
    WITH CHECK (id = ANY (ARRAY(SELECT authz.resources_with('writer')))
        AND project_id = ANY (ARRAY(SELECT authz.resources_with('writer')))
        AND procedure_template_id = ANY (ARRAY(SELECT authz.resources_with('reader')))
        AND (parent_procedure_id IS NULL OR parent_procedure_id = ANY (ARRAY(SELECT authz.resources_with('writer'))))
        AND (parent_template_id IS NULL OR parent_template_id = ANY (ARRAY(SELECT authz.resources_with('reader'))))
        AND (performed_by_id IS NULL OR performed_by_id = (SELECT authz.caller_id())));
CREATE POLICY procedures_update ON procedures FOR UPDATE TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('writer')))
        AND procedure_template_id = ANY (ARRAY(SELECT authz.resources_with('reader'))))
    WITH CHECK (id = ANY (ARRAY(SELECT authz.resources_with('writer')))
        AND project_id = ANY (ARRAY(SELECT authz.resources_with('writer'))));
CREATE POLICY procedures_delete ON procedures FOR DELETE TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('admin')))
        AND (parent_procedure_id IS NULL OR parent_procedure_id = ANY (ARRAY(SELECT authz.resources_with('reader')))));
CREATE POLICY procedures_archived_insert ON procedures AS RESTRICTIVE FOR INSERT TO app
    WITH CHECK (NOT authz.is_archived(id) AND NOT authz.is_archived(procedure_template_id)
        AND NOT authz.is_archived(parent_procedure_id));
CREATE POLICY procedures_archived_update ON procedures AS RESTRICTIVE FOR UPDATE TO app
    USING (NOT authz.is_archived(id)) WITH CHECK (NOT authz.is_archived(id));
CREATE POLICY procedures_archived_delete ON procedures AS RESTRICTIVE FOR DELETE TO app
    USING (NOT authz.is_archived(id));
CREATE POLICY procedures_blocked ON procedures AS RESTRICTIVE FOR ALL TO app
    USING (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))))
    WITH CHECK (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))));

GRANT SELECT, INSERT, UPDATE (asset_id, asset_model_id), DELETE ON procedure_assets TO app;
ALTER TABLE procedure_assets ENABLE ROW LEVEL SECURITY;
ALTER TABLE procedure_assets FORCE ROW LEVEL SECURITY;
CREATE POLICY procedure_assets_select ON procedure_assets FOR SELECT TO app
    USING (procedure_id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY procedure_assets_insert ON procedure_assets FOR INSERT TO app
    WITH CHECK (procedure_id = ANY (ARRAY(SELECT authz.resources_with('writer')))
        AND asset_model_id = ANY (ARRAY(SELECT authz.resources_with('reader')))
        AND (asset_id IS NULL OR asset_id = ANY (ARRAY(SELECT authz.resources_with('writer'))))
        AND procedure_template_id = ANY (ARRAY(SELECT authz.resources_with('reader')))
        AND (parent_procedure_id IS NULL OR parent_procedure_id = ANY (ARRAY(SELECT authz.resources_with('writer')))));
CREATE POLICY procedure_assets_update ON procedure_assets FOR UPDATE TO app
    USING (procedure_id = ANY (ARRAY(SELECT authz.resources_with('writer')))
        AND procedure_template_id = ANY (ARRAY(SELECT authz.resources_with('reader')))
        AND (asset_id IS NULL OR asset_id = ANY (ARRAY(SELECT authz.resources_with('reader')))))
    WITH CHECK (procedure_id = ANY (ARRAY(SELECT authz.resources_with('writer')))
        AND asset_model_id = ANY (ARRAY(SELECT authz.resources_with('reader')))
        AND (asset_id IS NULL OR asset_id = ANY (ARRAY(SELECT authz.resources_with('writer')))));
CREATE POLICY procedure_assets_delete ON procedure_assets FOR DELETE TO app
    USING (procedure_id = ANY (ARRAY(SELECT authz.resources_with('writer')))
        AND (asset_id IS NULL OR asset_id = ANY (ARRAY(SELECT authz.resources_with('reader')))));
CREATE POLICY procedure_assets_archived_insert ON procedure_assets AS RESTRICTIVE FOR INSERT TO app
    WITH CHECK (NOT authz.is_archived(procedure_id) AND NOT authz.is_archived(asset_model_id) AND NOT authz.is_archived(asset_id));
CREATE POLICY procedure_assets_archived_update ON procedure_assets AS RESTRICTIVE FOR UPDATE TO app
    USING (NOT authz.is_archived(procedure_id))
    WITH CHECK (NOT authz.is_archived(procedure_id) AND NOT authz.is_archived(asset_model_id) AND NOT authz.is_archived(asset_id));
CREATE POLICY procedure_assets_archived_delete ON procedure_assets AS RESTRICTIVE FOR DELETE TO app
    USING (NOT authz.is_archived(procedure_id));
CREATE POLICY procedure_assets_blocked ON procedure_assets AS RESTRICTIVE FOR ALL TO app
    USING (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))))
    WITH CHECK (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))));

GRANT SELECT, INSERT, UPDATE (number_value, text_value, boolean_value), DELETE ON procedure_parameter_values TO app;
ALTER TABLE procedure_parameter_values ENABLE ROW LEVEL SECURITY;
ALTER TABLE procedure_parameter_values FORCE ROW LEVEL SECURITY;
CREATE POLICY procedure_parameter_values_select ON procedure_parameter_values FOR SELECT TO app
    USING (procedure_id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY procedure_parameter_values_insert ON procedure_parameter_values FOR INSERT TO app
    WITH CHECK (procedure_id = ANY (ARRAY(SELECT authz.resources_with('writer')))
        AND procedure_template_id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY procedure_parameter_values_update ON procedure_parameter_values FOR UPDATE TO app
    USING (procedure_id = ANY (ARRAY(SELECT authz.resources_with('writer')))
        AND procedure_template_id = ANY (ARRAY(SELECT authz.resources_with('reader'))))
    WITH CHECK (procedure_id = ANY (ARRAY(SELECT authz.resources_with('writer'))));
CREATE POLICY procedure_parameter_values_delete ON procedure_parameter_values FOR DELETE TO app
    USING (procedure_id = ANY (ARRAY(SELECT authz.resources_with('writer'))));
CREATE POLICY procedure_parameter_values_archived_insert ON procedure_parameter_values AS RESTRICTIVE FOR INSERT TO app
    WITH CHECK (NOT authz.is_archived(procedure_id));
CREATE POLICY procedure_parameter_values_archived_update ON procedure_parameter_values AS RESTRICTIVE FOR UPDATE TO app
    USING (NOT authz.is_archived(procedure_id)) WITH CHECK (NOT authz.is_archived(procedure_id));
CREATE POLICY procedure_parameter_values_archived_delete ON procedure_parameter_values AS RESTRICTIVE FOR DELETE TO app
    USING (NOT authz.is_archived(procedure_id));
CREATE POLICY procedure_parameter_values_blocked ON procedure_parameter_values AS RESTRICTIVE FOR ALL TO app
    USING (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))))
    WITH CHECK (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))));

GRANT SELECT ON asset_uses TO app;
ALTER TABLE asset_uses ENABLE ROW LEVEL SECURITY;
ALTER TABLE asset_uses FORCE ROW LEVEL SECURITY;
CREATE POLICY asset_uses_select ON asset_uses FOR SELECT TO app
    USING (asset_id = ANY (ARRAY(SELECT public.ledger_assets())));
CREATE POLICY asset_uses_blocked ON asset_uses AS RESTRICTIVE FOR ALL TO app
    USING (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))))
    WITH CHECK (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))));

-- Lists the models the caller reads through connetto grants, procedures the
-- caller reads that bind them, and slots of templates the caller reads.
CREATE FUNCTION named_models() RETURNS SETOF UUID
LANGUAGE sql STABLE SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
    SELECT readable.id FROM authz.resources_with('reader') readable (id)
    UNION
    SELECT binding.asset_model_id FROM public.procedure_assets binding
    WHERE binding.procedure_id IN (SELECT authz.resources_with('reader'))
    UNION
    SELECT slot.asset_model_id FROM public.procedure_template_slots slot
    WHERE slot.procedure_template_id IN (SELECT public.readable_templates())
$$;
ALTER FUNCTION named_models() OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION named_models() FROM PUBLIC;

-- Lists the models the caller reads, which are the named models with all
-- their ancestors.
CREATE FUNCTION readable_models() RETURNS SETOF UUID
LANGUAGE sql STABLE SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
    SELECT ancestry.ancestor_model_id FROM public.asset_model_ancestors ancestry
    WHERE ancestry.descendant_model_id IN (SELECT public.named_models())
$$;
ALTER FUNCTION readable_models() OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION readable_models() FROM PUBLIC;
GRANT EXECUTE ON FUNCTION readable_models() TO app;

CREATE POLICY asset_models_select ON asset_models FOR SELECT TO app
    USING (id = ANY (ARRAY(SELECT public.readable_models()))
        OR project_id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY asset_model_ancestors_select ON asset_model_ancestors FOR SELECT TO app
    USING (descendant_model_id = ANY (ARRAY(SELECT public.readable_models())));

-- Records every change in connetto's `audit_events`.
CREATE TRIGGER procedures_audit AFTER INSERT OR DELETE ON procedures
    FOR EACH ROW EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER procedures_update_audit AFTER UPDATE ON procedures
    FOR EACH ROW WHEN (OLD IS DISTINCT FROM NEW) EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER procedure_assets_audit AFTER INSERT OR DELETE ON procedure_assets
    FOR EACH ROW EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER procedure_assets_update_audit AFTER UPDATE ON procedure_assets
    FOR EACH ROW WHEN (OLD IS DISTINCT FROM NEW) EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER procedure_parameter_values_audit AFTER INSERT OR DELETE ON procedure_parameter_values
    FOR EACH ROW EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER procedure_parameter_values_update_audit AFTER UPDATE ON procedure_parameter_values
    FOR EACH ROW WHEN (OLD IS DISTINCT FROM NEW) EXECUTE FUNCTION authz_write.record_audit_event();
