-- Whether a compatibility rule of an organization not blocking the caller
-- covers both model families.
CREATE FUNCTION container_compatible(container_model UUID, contained_model UUID) RETURNS BOOLEAN
LANGUAGE sql STABLE SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
    SELECT EXISTS (
        SELECT 1 FROM public.container_compatibility_rules compatibility
        JOIN public.asset_model_ancestors container ON container.ancestor_model_id = compatibility.container_model_id
        JOIN public.asset_model_ancestors contained ON contained.ancestor_model_id = compatibility.contained_asset_model_id
        WHERE container.descendant_model_id = container_model AND contained.descendant_model_id = contained_model
          AND NOT (compatibility.organization_id = ANY (ARRAY(SELECT authz.blocking_organizations())))
    )
$$;
ALTER FUNCTION container_compatible(UUID, UUID) OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION container_compatible(UUID, UUID) FROM PUBLIC;

-- Marks every compatibility rule of an organization not blocking the caller
-- covering both model families as relied on, after any concurrent change of
-- those rules commits, so the caller reads the rules that stay.
CREATE FUNCTION use_compatibility_rules(container_model UUID, contained_model UUID) RETURNS VOID
LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
BEGIN
    PERFORM authz.refuse_repeatable_read();
    UPDATE public.container_compatibility_rules compatibility SET in_use = TRUE
    WHERE NOT compatibility.in_use AND compatibility.id IN (
        SELECT matching.id FROM public.container_compatibility_rules matching
        JOIN public.asset_model_ancestors container ON container.ancestor_model_id = matching.container_model_id
        JOIN public.asset_model_ancestors contained ON contained.ancestor_model_id = matching.contained_asset_model_id
        WHERE container.descendant_model_id = container_model AND contained.descendant_model_id = contained_model
          AND NOT (matching.organization_id = ANY (ARRAY(SELECT authz.blocking_organizations())))
    );
END;
$$;
ALTER FUNCTION use_compatibility_rules(UUID, UUID) OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION use_compatibility_rules(UUID, UUID) FROM PUBLIC;
GRANT SELECT, UPDATE (in_use) ON container_compatibility_rules TO aps_writer;

-- Requires the model of `holder_slot` to admit the model of every slot placed
-- into or retrieved from it, relying on the rules that admit them.
CREATE FUNCTION require_slot_holder(holder_slot UUID) RETURNS VOID
LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
DECLARE
    holder_model UUID;
    held_model UUID;
BEGIN
    SELECT holder.asset_model_id INTO holder_model FROM public.procedure_template_slots holder WHERE holder.id = holder_slot;
    FOR held_model IN
        SELECT DISTINCT held.asset_model_id FROM public.procedure_template_slots held
        WHERE held.placed_into_slot_id = holder_slot OR held.retrieved_from_slot_id = holder_slot
        ORDER BY held.asset_model_id
    LOOP
        PERFORM public.use_compatibility_rules(holder_model, held_model);
        IF NOT public.container_compatible(holder_model, held_model) THEN
            RAISE EXCEPTION 'holder slot requires compatible models' USING ERRCODE = 'foreign_key_violation';
        END IF;
    END LOOP;
END;
$$;
ALTER FUNCTION require_slot_holder(UUID) OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION require_slot_holder(UUID) FROM PUBLIC;

-- Checks the holders of a written slot and the slot as a holder.
CREATE FUNCTION check_slot_holders() RETURNS TRIGGER
LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
BEGIN
    PERFORM public.require_slot_holder(NEW.id);
    IF NEW.placed_into_slot_id IS NOT NULL THEN
        PERFORM public.require_slot_holder(NEW.placed_into_slot_id);
    END IF;
    IF NEW.retrieved_from_slot_id IS NOT NULL THEN
        PERFORM public.require_slot_holder(NEW.retrieved_from_slot_id);
    END IF;
    RETURN NULL;
END;
$$;
ALTER FUNCTION check_slot_holders() OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION check_slot_holders() FROM PUBLIC;
CREATE CONSTRAINT TRIGGER procedure_template_slot_holders AFTER INSERT ON procedure_template_slots
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION check_slot_holders();
CREATE CONSTRAINT TRIGGER procedure_template_slot_holders_update AFTER UPDATE OF asset_model_id, placed_into_slot_id, retrieved_from_slot_id ON procedure_template_slots
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW WHEN (OLD.asset_model_id IS DISTINCT FROM NEW.asset_model_id OR OLD.placed_into_slot_id IS DISTINCT FROM NEW.placed_into_slot_id OR OLD.retrieved_from_slot_id IS DISTINCT FROM NEW.retrieved_from_slot_id) EXECUTE FUNCTION check_slot_holders();

-- How one bound asset moves relative to another bound in the same procedure.
CREATE TABLE asset_move_kinds (
    -- Kind name, `into` for a placement, `from` for a retrieval and `source`
    -- for a transfer drawing from a sample in a container.
    id TEXT PRIMARY KEY CHECK (id <> '' AND length(id) <= 63)
);
INSERT INTO asset_move_kinds (id) VALUES ('into'), ('from'), ('source');

-- One row per pair of bindings in one procedure where the asset of one goes
-- into the asset of the other, comes from it, or is a transfer's source sample
-- in it. Trusted triggers rewrite the rows from the procedure's own bindings,
-- slots and typed row, and keep the procedure's status and times on them.
CREATE TABLE asset_moves (
    -- Binding of the moving asset.
    contained_binding_id UUID NOT NULL,
    -- Binding of the container.
    holder_binding_id UUID NOT NULL,
    -- How the asset moves.
    kind_id TEXT NOT NULL REFERENCES asset_move_kinds (id),
    -- connetto organization owning the procedure.
    organization_id UUID NOT NULL,
    -- Procedure binding both assets.
    procedure_id UUID NOT NULL,
    -- Moving asset.
    asset_id UUID NOT NULL REFERENCES assets (id),
    -- Container.
    container_id UUID NOT NULL REFERENCES assets (id),
    -- Status of the procedure.
    status_id TEXT NOT NULL REFERENCES procedure_statuses (id),
    -- When the procedure started.
    started_at TIMESTAMPTZ,
    -- When the procedure ended, the time a completed placement's asset enters
    -- its container.
    ended_at TIMESTAMPTZ,
    PRIMARY KEY (contained_binding_id, holder_binding_id, kind_id),
    FOREIGN KEY (contained_binding_id, organization_id) REFERENCES procedure_assets (id, organization_id) ON DELETE CASCADE,
    FOREIGN KEY (holder_binding_id, organization_id) REFERENCES procedure_assets (id, organization_id) ON DELETE CASCADE
);
CREATE INDEX ON asset_moves (holder_binding_id);
CREATE INDEX ON asset_moves (kind_id);
CREATE INDEX ON asset_moves (procedure_id);
CREATE INDEX ON asset_moves (asset_id);
CREATE INDEX ON asset_moves (container_id);
CREATE INDEX ON asset_moves (status_id);
INSERT INTO table_names (id) VALUES ('asset_move_kinds'), ('asset_moves');

-- One row per completed placement and compatibility rule its asset counts
-- against in its container, from the time it entered.
CREATE TABLE container_occupancy (
    -- Binding of the placed asset.
    contained_binding_id UUID NOT NULL,
    -- Binding of the container.
    holder_binding_id UUID NOT NULL,
    -- Rule the placed asset counts against.
    rule_id UUID NOT NULL REFERENCES container_compatibility_rules (id),
    -- connetto organization owning the placement.
    organization_id UUID NOT NULL,
    -- Container.
    container_id UUID NOT NULL REFERENCES assets (id),
    -- When the placement completed.
    entered_at TIMESTAMPTZ NOT NULL,
    PRIMARY KEY (contained_binding_id, holder_binding_id, rule_id),
    FOREIGN KEY (contained_binding_id, organization_id) REFERENCES procedure_assets (id, organization_id) ON DELETE CASCADE,
    FOREIGN KEY (holder_binding_id, organization_id) REFERENCES procedure_assets (id, organization_id) ON DELETE CASCADE
);
CREATE INDEX ON container_occupancy (holder_binding_id);
CREATE INDEX ON container_occupancy (rule_id);
CREATE INDEX ON container_occupancy (container_id);
INSERT INTO table_names (id) VALUES ('container_occupancy');

-- One row per completed placement and later procedure that took its asset
-- out of the container, by consuming it, placing it elsewhere or retrieving
-- it. The asset left at the earliest of them. A row outlives the removal, so
-- an aborted or deleted removal still took the asset out.
CREATE TABLE move_exits (
    -- Binding of the placed asset.
    contained_binding_id UUID NOT NULL,
    -- Binding of the container.
    holder_binding_id UUID NOT NULL,
    -- Binding through which the removal took the asset out.
    removal_binding_id UUID NOT NULL,
    -- connetto organization owning the placement.
    organization_id UUID NOT NULL,
    -- Container the asset left.
    container_id UUID NOT NULL REFERENCES assets (id),
    -- Procedure that took the asset out.
    removal_procedure_id UUID NOT NULL,
    -- When the removal started.
    left_at TIMESTAMPTZ NOT NULL,
    PRIMARY KEY (contained_binding_id, holder_binding_id, removal_binding_id),
    FOREIGN KEY (contained_binding_id, organization_id) REFERENCES procedure_assets (id, organization_id) ON DELETE CASCADE,
    FOREIGN KEY (holder_binding_id, organization_id) REFERENCES procedure_assets (id, organization_id) ON DELETE CASCADE
);
CREATE INDEX ON move_exits (holder_binding_id);
CREATE INDEX ON move_exits (removal_binding_id);
CREATE INDEX ON move_exits (container_id);
CREATE INDEX ON move_exits (removal_procedure_id);
INSERT INTO table_names (id) VALUES ('move_exits');
REVOKE ALL ON asset_moves, container_occupancy, move_exits FROM PUBLIC, app, identity_writer;
GRANT SELECT, INSERT, UPDATE, DELETE ON asset_moves TO aps_writer;
GRANT SELECT, INSERT ON container_occupancy, move_exits TO aps_writer;
REVOKE ALL ON asset_move_kinds FROM PUBLIC, app, identity_writer;
GRANT SELECT ON asset_move_kinds TO aps_writer;

-- Lists the assets whose enclosures the caller reads, those of the caller's
-- ledgers and, transitively, the containers their completed placements put
-- them into, through placements outside the organizations blocking the
-- caller.
CREATE FUNCTION enclosing_assets() RETURNS SETOF UUID
LANGUAGE sql STABLE SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
    WITH RECURSIVE enclosing (id) AS (
        SELECT ledger.id FROM public.ledger_assets() ledger (id)
        UNION
        SELECT placement.container_id FROM public.asset_moves placement
        JOIN enclosing ON enclosing.id = placement.asset_id
        WHERE placement.kind_id = 'into' AND placement.status_id = 'completed'
          AND NOT (placement.organization_id = ANY (ARRAY(SELECT authz.blocking_organizations())))
    )
    SELECT enclosing.id FROM enclosing
$$;
ALTER FUNCTION enclosing_assets() OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION enclosing_assets() FROM PUBLIC;
GRANT EXECUTE ON FUNCTION enclosing_assets() TO app;

-- Lists the bindings of the placements whose exits the caller reads through
-- the enclosures of the assets placed, outside the organizations blocking the
-- caller.
CREATE FUNCTION enclosing_placements() RETURNS SETOF UUID
LANGUAGE sql STABLE SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
    SELECT placement.contained_binding_id FROM public.asset_moves placement
    WHERE placement.kind_id = 'into' AND placement.asset_id IN (SELECT public.enclosing_assets())
      AND NOT (placement.organization_id = ANY (ARRAY(SELECT authz.blocking_organizations())))
$$;
ALTER FUNCTION enclosing_placements() OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION enclosing_placements() FROM PUBLIC;
GRANT EXECUTE ON FUNCTION enclosing_placements() TO app;

-- When the asset of a completed placement left its container, ignoring the
-- removal `ignored_procedure_id`, or infinity while it is there.
CREATE FUNCTION placement_exit(placed_binding_id UUID, holding_binding_id UUID, ignored_procedure_id UUID DEFAULT NULL)
RETURNS TIMESTAMPTZ LANGUAGE sql STABLE SET search_path = pg_catalog, pg_temp AS $$
    SELECT coalesce(min(departure.left_at), 'infinity'::timestamptz) FROM public.move_exits departure
    WHERE departure.contained_binding_id = placed_binding_id AND departure.holder_binding_id = holding_binding_id
      AND departure.removal_procedure_id IS DISTINCT FROM ignored_procedure_id
$$;
ALTER FUNCTION placement_exit(UUID, UUID, UUID) OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION placement_exit(UUID, UUID, UUID) FROM PUBLIC;

-- Rewrites the moves of a procedure from its bindings, slots and typed row.
CREATE FUNCTION refresh_asset_moves(moved_procedure_id UUID) RETURNS VOID
LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
BEGIN
    DELETE FROM public.asset_moves moved WHERE moved.procedure_id = moved_procedure_id;
    INSERT INTO public.asset_moves (contained_binding_id, holder_binding_id, kind_id, organization_id, procedure_id, asset_id,
        container_id, status_id, started_at, ended_at)
    WITH pairs (contained_slot_id, holder_slot_id, kind_id) AS (
        SELECT slot.id, slot.placed_into_slot_id, 'into' FROM public.procedure_template_slots slot
        JOIN public.procedures run ON run.procedure_template_id = slot.procedure_template_id
        WHERE run.id = moved_procedure_id AND slot.placed_into_slot_id IS NOT NULL
        UNION ALL
        SELECT slot.id, slot.retrieved_from_slot_id, 'from' FROM public.procedure_template_slots slot
        JOIN public.procedures run ON run.procedure_template_id = slot.procedure_template_id
        WHERE run.id = moved_procedure_id AND slot.retrieved_from_slot_id IS NOT NULL
        UNION ALL
        SELECT template.source_sample_slot_id, template.poured_from_slot_id, 'source' FROM public.pouring_procedures typed
        JOIN public.pouring_procedure_templates template ON template.id = typed.pouring_procedure_template_id
        WHERE typed.id = moved_procedure_id
        UNION ALL
        SELECT template.source_sample_slot_id, template.aliquoted_from_slot_id, 'source' FROM public.aliquoting_procedures typed
        JOIN public.aliquoting_procedure_templates template ON template.id = typed.aliquoting_procedure_template_id
        WHERE typed.id = moved_procedure_id
        UNION ALL
        SELECT template.source_sample_slot_id, template.stratified_source_slot_id, 'source' FROM public.supernatant_procedures typed
        JOIN public.supernatant_procedure_templates template ON template.id = typed.supernatant_procedure_template_id
        WHERE typed.id = moved_procedure_id
        UNION ALL
        SELECT template.source_sample_slot_id, template.fragment_container_slot_id, 'source' FROM public.fractioning_procedures typed
        JOIN public.fractioning_procedure_templates template ON template.id = typed.fractioning_procedure_template_id
        WHERE typed.id = moved_procedure_id
    )
    SELECT contained.id, holder.id, pair.kind_id, run.organization_id, run.id, contained.asset_id, holder.asset_id,
        run.status_id, run.started_at, run.ended_at
    FROM pairs pair
    JOIN public.procedures run ON run.id = moved_procedure_id
    JOIN public.procedure_assets contained ON contained.procedure_id = run.id AND contained.slot_id = pair.contained_slot_id
    JOIN public.procedure_assets holder ON holder.procedure_id = run.id AND holder.slot_id = pair.holder_slot_id
    WHERE contained.asset_id IS NOT NULL AND holder.asset_id IS NOT NULL
    ON CONFLICT DO NOTHING;
END;
$$;
ALTER FUNCTION refresh_asset_moves(UUID) OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION refresh_asset_moves(UUID) FROM PUBLIC;

-- Records, for every binding through which a started procedure takes an
-- asset out of a container, that the asset left each container its earlier
-- completed placements put it into, at the procedure's start.
CREATE FUNCTION record_removal_exits(removal_procedure UUID) RETURNS VOID
LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
DECLARE
    removed_asset_id UUID;
BEGIN
    IF NOT EXISTS (SELECT 1 FROM public.procedures run WHERE run.id = removal_procedure AND run.started_at IS NOT NULL) THEN
        RETURN;
    END IF;
    FOR removed_asset_id IN
        SELECT used.asset_id FROM public.asset_uses used WHERE used.procedure_id = removal_procedure AND used.consumes
        UNION
        SELECT moved.asset_id FROM public.asset_moves moved
        WHERE moved.procedure_id = removal_procedure AND moved.kind_id IN ('into', 'from')
        ORDER BY 1
    LOOP
        PERFORM public.lock_ledger_asset(removed_asset_id);
    END LOOP;
    INSERT INTO public.move_exits (contained_binding_id, holder_binding_id, removal_binding_id, organization_id, container_id,
        removal_procedure_id, left_at)
    WITH removals (binding_id, asset_id) AS (
        SELECT used.id, used.asset_id FROM public.asset_uses used WHERE used.procedure_id = removal_procedure AND used.consumes
        UNION
        SELECT moved.contained_binding_id, moved.asset_id FROM public.asset_moves moved
        WHERE moved.procedure_id = removal_procedure AND moved.kind_id IN ('into', 'from')
    )
    SELECT placement.contained_binding_id, placement.holder_binding_id, removal.binding_id, placement.organization_id,
        placement.container_id, run.id, run.started_at
    FROM removals removal
    JOIN public.procedures run ON run.id = removal_procedure
    JOIN public.asset_moves placement ON placement.asset_id = removal.asset_id
    WHERE placement.kind_id = 'into' AND placement.status_id = 'completed' AND placement.procedure_id <> run.id
      AND placement.ended_at <= run.started_at
    ON CONFLICT DO NOTHING;
END;
$$;
ALTER FUNCTION record_removal_exits(UUID) OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION record_removal_exits(UUID) FROM PUBLIC;

-- Enters the assets a completed procedure placed into their containers: one
-- occupancy row per rule each counts against, relied on from then, and an
-- exit for every procedure that took the asset out since it entered.
CREATE FUNCTION record_placements(placing_procedure UUID) RETURNS VOID
LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
DECLARE
    placement RECORD;
    locked_asset_id UUID;
BEGIN
    FOR placement IN
        SELECT moved.contained_binding_id, moved.holder_binding_id, moved.organization_id, moved.asset_id, moved.container_id,
            moved.ended_at, contained.asset_model_id AS contained_model_id, holder.asset_model_id AS holder_model_id
        FROM public.asset_moves moved
        JOIN public.procedure_assets contained ON contained.id = moved.contained_binding_id
        JOIN public.procedure_assets holder ON holder.id = moved.holder_binding_id
        WHERE moved.procedure_id = placing_procedure AND moved.kind_id = 'into'
        ORDER BY moved.contained_binding_id, moved.holder_binding_id
    LOOP
        FOR locked_asset_id IN SELECT unnest(ARRAY[placement.asset_id, placement.container_id]) ORDER BY 1 LOOP
            PERFORM public.lock_ledger_asset(locked_asset_id);
        END LOOP;
        PERFORM public.use_compatibility_rules(placement.holder_model_id, placement.contained_model_id);
        INSERT INTO public.container_occupancy (contained_binding_id, holder_binding_id, rule_id, organization_id, container_id, entered_at)
        SELECT placement.contained_binding_id, placement.holder_binding_id, compatibility.id, placement.organization_id,
            placement.container_id, placement.ended_at
        FROM public.container_compatibility_rules compatibility
        JOIN public.asset_model_ancestors container ON container.ancestor_model_id = compatibility.container_model_id
        JOIN public.asset_model_ancestors contained ON contained.ancestor_model_id = compatibility.contained_asset_model_id
        WHERE container.descendant_model_id = placement.holder_model_id
          AND contained.descendant_model_id = placement.contained_model_id
          AND NOT (compatibility.organization_id = ANY (ARRAY(SELECT authz.blocking_organizations())))
        ON CONFLICT DO NOTHING;
        INSERT INTO public.move_exits (contained_binding_id, holder_binding_id, removal_binding_id, organization_id, container_id,
            removal_procedure_id, left_at)
        WITH removals (binding_id, procedure_id, started_at) AS (
            SELECT used.id, used.procedure_id, used.started_at FROM public.asset_uses used
            WHERE used.asset_id = placement.asset_id AND used.consumes
            UNION
            SELECT moved.contained_binding_id, moved.procedure_id, moved.started_at FROM public.asset_moves moved
            WHERE moved.asset_id = placement.asset_id AND moved.kind_id IN ('into', 'from')
        )
        SELECT placement.contained_binding_id, placement.holder_binding_id, removal.binding_id, placement.organization_id,
            placement.container_id, removal.procedure_id, removal.started_at
        FROM removals removal
        WHERE removal.procedure_id <> placing_procedure AND removal.started_at >= placement.ended_at
        ON CONFLICT DO NOTHING;
    END LOOP;
END;
$$;
ALTER FUNCTION record_placements(UUID) OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION record_placements(UUID) FROM PUBLIC;

-- Keeps the moves of a procedure equal to its bindings, records the exits its
-- removals take, and refuses changing the asset of a binding that took one.
CREATE FUNCTION record_binding_moves() RETURNS TRIGGER
LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
BEGIN
    IF TG_OP <> 'INSERT' AND EXISTS (SELECT 1 FROM public.procedures run WHERE run.id = OLD.procedure_id)
        AND EXISTS (SELECT 1 FROM public.move_exits departure WHERE departure.removal_binding_id = OLD.id) THEN
        RAISE EXCEPTION 'binding % recorded its asset leaving a container, so its asset cannot change', OLD.id
            USING ERRCODE = 'check_violation';
    END IF;
    IF TG_OP = 'DELETE' THEN
        IF EXISTS (SELECT 1 FROM public.procedures run WHERE run.id = OLD.procedure_id) THEN
            PERFORM public.refresh_asset_moves(OLD.procedure_id);
        END IF;
        RETURN NULL;
    END IF;
    PERFORM public.refresh_asset_moves(NEW.procedure_id);
    PERFORM public.record_removal_exits(NEW.procedure_id);
    RETURN NULL;
END;
$$;
ALTER FUNCTION record_binding_moves() OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION record_binding_moves() FROM PUBLIC;
CREATE TRIGGER procedure_assets_whereabouts AFTER INSERT OR DELETE ON procedure_assets
    FOR EACH ROW EXECUTE FUNCTION record_binding_moves();
CREATE TRIGGER procedure_assets_whereabouts_update AFTER UPDATE OF asset_id ON procedure_assets
    FOR EACH ROW WHEN (OLD.asset_id IS DISTINCT FROM NEW.asset_id) EXECUTE FUNCTION record_binding_moves();

-- Keeps the moves of a transfer equal to its typed row.
CREATE FUNCTION record_transfer_moves() RETURNS TRIGGER
LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
BEGIN
    IF EXISTS (SELECT 1 FROM public.procedures run WHERE run.id = COALESCE(NEW.id, OLD.id)) THEN
        PERFORM public.refresh_asset_moves(COALESCE(NEW.id, OLD.id));
    END IF;
    RETURN NULL;
END;
$$;
ALTER FUNCTION record_transfer_moves() OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION record_transfer_moves() FROM PUBLIC;
CREATE TRIGGER pouring_whereabouts AFTER INSERT OR UPDATE OF id, pouring_procedure_template_id, status_id, volume_l OR DELETE ON pouring_procedures
    FOR EACH ROW EXECUTE FUNCTION record_transfer_moves();
CREATE TRIGGER aliquoting_whereabouts AFTER INSERT OR UPDATE OF id, aliquoting_procedure_template_id, status_id, volume_l OR DELETE ON aliquoting_procedures
    FOR EACH ROW EXECUTE FUNCTION record_transfer_moves();
CREATE TRIGGER supernatant_whereabouts AFTER INSERT OR UPDATE OF id, supernatant_procedure_template_id, status_id, volume_l OR DELETE ON supernatant_procedures
    FOR EACH ROW EXECUTE FUNCTION record_transfer_moves();
CREATE TRIGGER fractioning_whereabouts AFTER INSERT OR UPDATE OF id, fractioning_procedure_template_id, status_id, mass_kg OR DELETE ON fractioning_procedures
    FOR EACH ROW EXECUTE FUNCTION record_transfer_moves();

-- Copies a procedure's status and times into its moves, records the exits
-- of its removals once it starts and its placements once it completes, and
-- refuses moving the start of a procedure that took an asset out later.
CREATE FUNCTION record_procedure_moves() RETURNS TRIGGER
LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
BEGIN
    IF NEW.started_at > OLD.started_at
        AND EXISTS (SELECT 1 FROM public.move_exits departure WHERE departure.removal_procedure_id = NEW.id) THEN
        RAISE EXCEPTION 'procedure % recorded an asset leaving a container, so it cannot start later', NEW.id
            USING ERRCODE = 'check_violation';
    END IF;
    UPDATE public.asset_moves SET status_id = NEW.status_id, started_at = NEW.started_at, ended_at = NEW.ended_at
    WHERE procedure_id = NEW.id;
    IF NEW.started_at IS DISTINCT FROM OLD.started_at THEN
        PERFORM public.record_removal_exits(NEW.id);
    END IF;
    IF NEW.status_id = 'completed' AND OLD.status_id <> 'completed' THEN
        PERFORM public.record_placements(NEW.id);
    END IF;
    RETURN NULL;
END;
$$;
ALTER FUNCTION record_procedure_moves() OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION record_procedure_moves() FROM PUBLIC;
CREATE TRIGGER procedures_whereabouts AFTER UPDATE OF status_id, started_at, ended_at ON procedures
    FOR EACH ROW WHEN (OLD.status_id IS DISTINCT FROM NEW.status_id OR OLD.started_at IS DISTINCT FROM NEW.started_at
        OR OLD.ended_at IS DISTINCT FROM NEW.ended_at)
    EXECUTE FUNCTION record_procedure_moves();

-- Rejects a completed retrieval or transfer of an asset that was not in the
-- container it names when the procedure started, ignoring the exit the
-- procedure itself took.
CREATE FUNCTION require_asset_whereabouts(checked_asset_id UUID) RETURNS VOID
LANGUAGE plpgsql SET search_path = pg_catalog, pg_temp AS $$
DECLARE
    misplaced_kind TEXT;
BEGIN
    SELECT moved.kind_id INTO misplaced_kind FROM public.asset_moves moved
    WHERE moved.asset_id = checked_asset_id AND moved.kind_id IN ('from', 'source') AND moved.status_id = 'completed'
      AND NOT EXISTS (
          SELECT 1 FROM public.asset_moves placement
          WHERE placement.asset_id = moved.asset_id AND placement.container_id = moved.container_id
            AND placement.kind_id = 'into' AND placement.status_id = 'completed' AND placement.procedure_id <> moved.procedure_id
            AND placement.ended_at <= moved.started_at
            AND public.placement_exit(placement.contained_binding_id, placement.holder_binding_id, moved.procedure_id) > moved.started_at
      )
    LIMIT 1;
    IF misplaced_kind = 'from' THEN
        RAISE EXCEPTION 'retrieved asset is not in its source container' USING ERRCODE = 'check_violation';
    ELSIF misplaced_kind = 'source' THEN
        RAISE EXCEPTION 'transfer source sample is not in its source container' USING ERRCODE = 'check_violation';
    END IF;
END;
$$;
ALTER FUNCTION require_asset_whereabouts(UUID) OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION require_asset_whereabouts(UUID) FROM PUBLIC;

-- Rejects a completed placement that overfills its container under a rule,
-- puts an asset into a container consumed while it is there, or closes a
-- cycle of containers enclosing one another at the same time.
CREATE FUNCTION require_placement(placed_binding_id UUID, holding_binding_id UUID) RETURNS VOID
LANGUAGE plpgsql SET search_path = pg_catalog, pg_temp AS $$
DECLARE
    placed_asset_id UUID;
    holding_container_id UUID;
    entered TIMESTAMPTZ;
    exited TIMESTAMPTZ;
BEGIN
    SELECT placement.asset_id, placement.container_id, placement.ended_at
    INTO placed_asset_id, holding_container_id, entered
    FROM public.asset_moves placement
    WHERE placement.contained_binding_id = placed_binding_id AND placement.holder_binding_id = holding_binding_id
      AND placement.kind_id = 'into';
    exited := public.placement_exit(placed_binding_id, holding_binding_id);
    IF EXISTS (
        WITH held (rule_id, entered_at, left_at) AS (
            SELECT occupied.rule_id, occupied.entered_at,
                public.placement_exit(occupied.contained_binding_id, occupied.holder_binding_id)
            FROM public.container_occupancy occupied WHERE occupied.container_id = holding_container_id
        )
        SELECT 1 FROM held boundary
        JOIN public.container_compatibility_rules compatibility ON compatibility.id = boundary.rule_id
        WHERE boundary.entered_at >= entered AND boundary.entered_at < exited
          AND compatibility.quantity < (
              SELECT count(*) FROM held counted
              WHERE counted.rule_id = boundary.rule_id AND counted.entered_at <= boundary.entered_at
                AND counted.left_at > boundary.entered_at
          )
    ) THEN
        RAISE EXCEPTION 'container capacity exceeded' USING ERRCODE = 'check_violation';
    END IF;
    IF EXISTS (
        SELECT 1 FROM public.asset_uses consumption
        WHERE consumption.asset_id = holding_container_id AND consumption.consumes AND consumption.status_id = 'completed'
          AND consumption.started_at >= entered AND consumption.started_at < exited
    ) THEN
        RAISE EXCEPTION 'consumed container still holds assets' USING ERRCODE = 'check_violation';
    END IF;
    IF EXISTS (
        WITH RECURSIVE visible (id) AS MATERIALIZED (
            SELECT public.enclosing_assets()
        ), climb (asset_id, entered_at, left_at) AS (
            SELECT holding_container_id, entered, exited
            UNION
            SELECT outer_placement.container_id, greatest(climb.entered_at, outer_placement.ended_at),
                least(climb.left_at, public.placement_exit(outer_placement.contained_binding_id, outer_placement.holder_binding_id))
            FROM climb
            JOIN public.asset_moves outer_placement ON outer_placement.asset_id = climb.asset_id
            WHERE outer_placement.kind_id = 'into' AND outer_placement.status_id = 'completed'
              AND outer_placement.asset_id IN (SELECT visible.id FROM visible)
              AND outer_placement.ended_at < climb.left_at
              AND public.placement_exit(outer_placement.contained_binding_id, outer_placement.holder_binding_id) > climb.entered_at
        )
        SELECT 1 FROM climb WHERE climb.asset_id = placed_asset_id
    ) THEN
        RAISE EXCEPTION 'containment cycle' USING ERRCODE = 'check_violation';
    END IF;
END;
$$;
ALTER FUNCTION require_placement(UUID, UUID) OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION require_placement(UUID, UUID) FROM PUBLIC;

-- Checks a completed move at commit.
CREATE FUNCTION check_asset_move() RETURNS TRIGGER
LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
DECLARE
    locked_asset_id UUID;
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM public.asset_moves moved
        WHERE moved.contained_binding_id = NEW.contained_binding_id AND moved.holder_binding_id = NEW.holder_binding_id
          AND moved.kind_id = NEW.kind_id AND moved.status_id = 'completed'
    ) THEN
        RETURN NULL;
    END IF;
    FOR locked_asset_id IN SELECT unnest(ARRAY[NEW.asset_id, NEW.container_id]) ORDER BY 1 LOOP
        PERFORM public.lock_ledger_asset(locked_asset_id);
    END LOOP;
    IF NEW.kind_id = 'into' THEN
        PERFORM public.require_placement(NEW.contained_binding_id, NEW.holder_binding_id);
    ELSE
        PERFORM public.require_asset_whereabouts(NEW.asset_id);
    END IF;
    RETURN NULL;
END;
$$;
ALTER FUNCTION check_asset_move() OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION check_asset_move() FROM PUBLIC;
CREATE CONSTRAINT TRIGGER asset_moves_checked AFTER INSERT OR UPDATE ON asset_moves
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW WHEN (NEW.status_id = 'completed') EXECUTE FUNCTION check_asset_move();

-- Rechecks at commit the retrievals and transfers of an asset that left a
-- container.
CREATE FUNCTION check_move_exit() RETURNS TRIGGER
LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
DECLARE
    departed_asset_id UUID;
BEGIN
    SELECT placement.asset_id INTO departed_asset_id FROM public.asset_moves placement
    WHERE placement.contained_binding_id = NEW.contained_binding_id AND placement.holder_binding_id = NEW.holder_binding_id
      AND placement.kind_id = 'into';
    IF departed_asset_id IS NOT NULL THEN
        PERFORM public.lock_ledger_asset(departed_asset_id);
        PERFORM public.require_asset_whereabouts(departed_asset_id);
    END IF;
    RETURN NULL;
END;
$$;
ALTER FUNCTION check_move_exit() OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION check_move_exit() FROM PUBLIC;
CREATE CONSTRAINT TRIGGER move_exits_checked AFTER INSERT ON move_exits
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION check_move_exit();

-- Rejects at commit consuming a container while it holds an asset.
CREATE FUNCTION check_consumed_container() RETURNS TRIGGER
LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
BEGIN
    PERFORM public.lock_ledger_asset(NEW.asset_id);
    IF EXISTS (
        SELECT 1 FROM public.asset_uses consumption
        JOIN public.container_occupancy occupied ON occupied.container_id = consumption.asset_id
        WHERE consumption.id = NEW.id AND consumption.status_id = 'completed'
          AND occupied.entered_at <= consumption.started_at
          AND public.placement_exit(occupied.contained_binding_id, occupied.holder_binding_id) > consumption.started_at
    ) THEN
        RAISE EXCEPTION 'consumed container still holds assets' USING ERRCODE = 'check_violation';
    END IF;
    RETURN NULL;
END;
$$;
ALTER FUNCTION check_consumed_container() OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION check_consumed_container() FROM PUBLIC;
CREATE CONSTRAINT TRIGGER asset_uses_consumed_container AFTER INSERT OR UPDATE ON asset_uses
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW WHEN (NEW.consumes AND NEW.status_id = 'completed')
    EXECUTE FUNCTION check_consumed_container();

ALTER TABLE asset_move_kinds ENABLE ROW LEVEL SECURITY;
ALTER TABLE asset_move_kinds FORCE ROW LEVEL SECURITY;
CREATE POLICY asset_move_kinds_select ON asset_move_kinds FOR SELECT TO app USING (true);
GRANT SELECT ON asset_move_kinds TO app;

GRANT SELECT ON asset_moves TO app;
ALTER TABLE asset_moves ENABLE ROW LEVEL SECURITY;
ALTER TABLE asset_moves FORCE ROW LEVEL SECURITY;
CREATE POLICY asset_moves_select ON asset_moves FOR SELECT TO app
    USING (asset_id = ANY (ARRAY(SELECT public.enclosing_assets())));
CREATE POLICY asset_moves_blocked ON asset_moves AS RESTRICTIVE FOR ALL TO app
    USING (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))))
    WITH CHECK (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))));

GRANT SELECT ON container_occupancy TO app;
ALTER TABLE container_occupancy ENABLE ROW LEVEL SECURITY;
ALTER TABLE container_occupancy FORCE ROW LEVEL SECURITY;
CREATE POLICY container_occupancy_select ON container_occupancy FOR SELECT TO app
    USING (container_id = ANY (ARRAY(SELECT public.ledger_assets())));
CREATE POLICY container_occupancy_blocked ON container_occupancy AS RESTRICTIVE FOR ALL TO app
    USING (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))))
    WITH CHECK (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))));

GRANT SELECT ON move_exits TO app;
ALTER TABLE move_exits ENABLE ROW LEVEL SECURITY;
ALTER TABLE move_exits FORCE ROW LEVEL SECURITY;
CREATE POLICY move_exits_select ON move_exits FOR SELECT TO app
    USING (container_id = ANY (ARRAY(SELECT public.ledger_assets()))
        OR contained_binding_id = ANY (ARRAY(SELECT public.enclosing_placements())));
CREATE POLICY move_exits_blocked ON move_exits AS RESTRICTIVE FOR ALL TO app
    USING (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))))
    WITH CHECK (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))));
