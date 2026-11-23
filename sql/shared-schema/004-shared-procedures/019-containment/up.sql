-- Locks every container model `container_model` descends from, in id order.
CREATE FUNCTION lock_container_model_ancestors(container_model UUID) RETURNS VOID
LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
DECLARE
    ancestor_identifier UUID;
BEGIN
    FOR ancestor_identifier IN
        SELECT ancestry.ancestor_model_id FROM public.asset_model_ancestors ancestry
        JOIN public.container_models container ON container.id = ancestry.ancestor_model_id
        WHERE ancestry.descendant_model_id = container_model ORDER BY ancestry.ancestor_model_id
    LOOP
        PERFORM public.lock_container_model(ancestor_identifier);
    END LOOP;
END;
$$;
ALTER FUNCTION lock_container_model_ancestors(UUID) OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION lock_container_model_ancestors(UUID) FROM PUBLIC;

-- Whether a compatibility rule covers both model families.
CREATE FUNCTION container_compatible(container_model UUID, contained_model UUID) RETURNS BOOLEAN
LANGUAGE sql STABLE SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
    SELECT EXISTS (
        SELECT 1 FROM public.container_compatibility_rules compatibility
        JOIN public.asset_model_ancestors container ON container.ancestor_model_id = compatibility.container_model_id
        JOIN public.asset_model_ancestors contained ON contained.ancestor_model_id = compatibility.contained_asset_model_id
        WHERE container.descendant_model_id = container_model AND contained.descendant_model_id = contained_model
    )
$$;
ALTER FUNCTION container_compatible(UUID, UUID) OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION container_compatible(UUID, UUID) FROM PUBLIC;

-- Requires the model of `holder_slot` to admit the model of every slot placed
-- into or retrieved from it, after locking the holder's container models.
CREATE FUNCTION require_slot_holder(holder_slot UUID) RETURNS VOID
LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
DECLARE
    holder_model UUID;
BEGIN
    SELECT holder.asset_model_id INTO holder_model FROM public.procedure_template_slots holder
    WHERE holder.id = holder_slot AND EXISTS (
        SELECT 1 FROM public.procedure_template_slots held
        WHERE held.placed_into_slot_id = holder.id OR held.retrieved_from_slot_id = holder.id
    );
    IF holder_model IS NOT NULL THEN
        PERFORM public.lock_container_model_ancestors(holder_model);
        IF EXISTS (
            SELECT 1 FROM public.procedure_template_slots held
            WHERE (held.placed_into_slot_id = holder_slot OR held.retrieved_from_slot_id = holder_slot)
                AND NOT public.container_compatible(holder_model, held.asset_model_id)
        ) THEN
            RAISE EXCEPTION 'holder slot requires compatible models' USING ERRCODE = 'foreign_key_violation';
        END IF;
    END IF;
END;
$$;
ALTER FUNCTION require_slot_holder(UUID) OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION require_slot_holder(UUID) FROM PUBLIC;

-- Checks the holders of a written slot and the slot as a holder.
CREATE FUNCTION check_slot_holders() RETURNS TRIGGER
LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
DECLARE
    placed_into_slot UUID;
    retrieved_from_slot UUID;
BEGIN
    SELECT held.placed_into_slot_id, held.retrieved_from_slot_id INTO placed_into_slot, retrieved_from_slot
    FROM public.procedure_template_slots held WHERE held.id = NEW.id;
    PERFORM public.require_slot_holder(NEW.id);
    PERFORM public.require_slot_holder(placed_into_slot);
    PERFORM public.require_slot_holder(retrieved_from_slot);
    RETURN NULL;
END;
$$;
ALTER FUNCTION check_slot_holders() OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION check_slot_holders() FROM PUBLIC;
CREATE CONSTRAINT TRIGGER procedure_template_slot_holders AFTER INSERT OR UPDATE ON procedure_template_slots
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION check_slot_holders();

-- Keeps holder slots covered when a compatibility rule changes.
CREATE FUNCTION check_rule_slot_holders() RETURNS TRIGGER
LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
BEGIN
    PERFORM public.lock_container_model_ancestors(OLD.container_model_id);
    IF EXISTS (
        SELECT 1 FROM public.procedure_template_slots held
        JOIN public.procedure_template_slots holder
            ON holder.id = held.placed_into_slot_id OR holder.id = held.retrieved_from_slot_id
        JOIN public.asset_model_ancestors ancestry ON ancestry.descendant_model_id = holder.asset_model_id
        WHERE ancestry.ancestor_model_id = OLD.container_model_id
            AND NOT public.container_compatible(holder.asset_model_id, held.asset_model_id)
    ) THEN
        RAISE EXCEPTION 'holder slot requires compatible models' USING ERRCODE = 'foreign_key_violation';
    END IF;
    RETURN NULL;
END;
$$;
ALTER FUNCTION check_rule_slot_holders() OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION check_rule_slot_holders() FROM PUBLIC;
CREATE CONSTRAINT TRIGGER compatibility_slot_holders AFTER UPDATE OR DELETE ON container_compatibility_rules
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION check_rule_slot_holders();

-- Lists every placement of a tracked asset into a tracked container.
CREATE FUNCTION asset_placements() RETURNS TABLE (procedure_id UUID, asset_id UUID, container_id UUID, started_at TIMESTAMPTZ, ended_at TIMESTAMPTZ, status_id TEXT)
LANGUAGE sql STABLE SET search_path = pg_catalog, pg_temp AS $$
SELECT run.id, contained.asset_id, destination.asset_id, run.started_at, run.ended_at, run.status_id
FROM public.procedures run
JOIN public.procedure_assets contained ON contained.procedure_id = run.id
JOIN public.procedure_template_slots slot ON slot.id = contained.slot_id
JOIN public.procedure_assets destination ON destination.procedure_id = run.id AND destination.slot_id = slot.placed_into_slot_id
WHERE contained.asset_id IS NOT NULL AND destination.asset_id IS NOT NULL
$$;
ALTER FUNCTION asset_placements() OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION asset_placements() FROM PUBLIC;

-- Derives tracked asset occupancy as half-open time intervals, one per
-- completed placement.
CREATE FUNCTION asset_containment(excluded_removal_procedure_id UUID DEFAULT NULL) RETURNS TABLE (asset_id UUID, container_id UUID, entered_at TIMESTAMPTZ, left_at TIMESTAMPTZ)
LANGUAGE sql STABLE SET search_path = pg_catalog, pg_temp AS $$
WITH placements AS (
    SELECT * FROM public.asset_placements()
), removals AS (
    SELECT placement.asset_id, placement.started_at, placement.procedure_id, placement.container_id, NULL::uuid AS retrieved_from_id
    FROM placements placement WHERE placement.status_id NOT IN ('aborted', 'skipped')
    UNION ALL
    SELECT binding.asset_id, run.started_at, run.id, NULL::uuid, NULL::uuid
    FROM public.procedure_assets binding JOIN public.procedures run ON run.id = binding.procedure_id
    WHERE binding.slot_role_id IN ('consumed', 'intermediate') AND binding.asset_id IS NOT NULL AND run.status_id NOT IN ('aborted', 'skipped')
    UNION ALL
    SELECT contained.asset_id, run.started_at, run.id, NULL::uuid, source.asset_id
    FROM public.procedures run
    JOIN public.procedure_assets contained ON contained.procedure_id = run.id
    JOIN public.procedure_template_slots slot ON slot.id = contained.slot_id
    JOIN public.procedure_assets source ON source.procedure_id = run.id AND source.slot_id = slot.retrieved_from_slot_id
    WHERE contained.asset_id IS NOT NULL AND source.asset_id IS NOT NULL AND run.status_id NOT IN ('aborted', 'skipped')
)
SELECT placement.asset_id, placement.container_id, placement.ended_at,
    (SELECT min(removal.started_at) FROM removals removal WHERE removal.asset_id = placement.asset_id
        AND removal.procedure_id <> placement.procedure_id AND removal.started_at >= placement.ended_at
        AND removal.procedure_id IS DISTINCT FROM excluded_removal_procedure_id
        AND (removal.container_id IS NULL OR removal.container_id <> placement.container_id)
        AND (removal.retrieved_from_id IS NULL OR removal.retrieved_from_id = placement.container_id))
FROM placements placement WHERE placement.status_id = 'completed'
$$;
ALTER FUNCTION asset_containment(UUID) OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION asset_containment(UUID) FROM PUBLIC;

-- Rejects excess historical occupancy in one container, whose model ancestry
-- and asset row the caller has locked. Occupancy only grows when an asset
-- enters, so the check runs at each completed placement into the container.
CREATE FUNCTION require_container_capacity(checked_container_id UUID) RETURNS VOID
LANGUAGE plpgsql SET search_path = pg_catalog, pg_temp AS $$
BEGIN
    IF EXISTS (
        SELECT 1 FROM public.asset_placements() boundary
        JOIN public.assets container ON container.id = boundary.container_id
        JOIN public.asset_model_ancestors container_ancestry ON container_ancestry.descendant_model_id = container.model_id
        JOIN public.container_compatibility_rules rule ON rule.container_model_id = container_ancestry.ancestor_model_id
        WHERE boundary.container_id = checked_container_id AND boundary.status_id = 'completed'
            AND (SELECT count(DISTINCT held.asset_id) FROM public.asset_containment() held
            JOIN public.assets asset ON asset.id = held.asset_id
            JOIN public.asset_model_ancestors ancestry ON ancestry.descendant_model_id = asset.model_id
                AND ancestry.ancestor_model_id = rule.contained_asset_model_id
            WHERE held.container_id = checked_container_id AND held.entered_at <= boundary.ended_at
                AND (held.left_at IS NULL OR held.left_at > boundary.ended_at)) > rule.quantity
    ) THEN
        RAISE EXCEPTION 'container capacity exceeded' USING ERRCODE = 'check_violation';
    END IF;
    PERFORM public.require_container_history(checked_container_id);
END;
$$;
ALTER FUNCTION require_container_capacity(UUID) OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION require_container_capacity(UUID) FROM PUBLIC;

-- Checks, in container order, every container that is or ever held an asset
-- bound to the procedure or one of the two given assets, after locking the
-- container models of all of them and then each container.
CREATE FUNCTION require_affected_container_capacity(affected_procedure_id UUID, new_asset_id UUID, old_asset_id UUID) RETURNS VOID
LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
DECLARE
    locked_model_id UUID;
    checked_container_id UUID;
BEGIN
    PERFORM authz.refuse_repeatable_read();
    PERFORM asset.id FROM public.assets asset
    WHERE asset.id IN (
        SELECT binding.asset_id FROM public.procedure_assets binding WHERE binding.procedure_id = affected_procedure_id
        UNION SELECT new_asset_id
        UNION SELECT old_asset_id
    )
    ORDER BY asset.id FOR NO KEY UPDATE;
    FOR locked_model_id IN
        WITH affected (asset_id) AS (
            SELECT binding.asset_id FROM public.procedure_assets binding
            WHERE binding.procedure_id = affected_procedure_id AND binding.asset_id IS NOT NULL
            UNION SELECT new_asset_id WHERE new_asset_id IS NOT NULL
            UNION SELECT old_asset_id WHERE old_asset_id IS NOT NULL
        )
        SELECT DISTINCT ancestry.ancestor_model_id FROM public.assets container
        JOIN public.container_models model ON model.id = container.model_id
        JOIN public.asset_model_ancestors ancestry ON ancestry.descendant_model_id = container.model_id
        JOIN public.container_models ancestor ON ancestor.id = ancestry.ancestor_model_id
        WHERE container.id IN (SELECT touched.asset_id FROM affected touched)
            OR container.id IN (SELECT placement.container_id FROM public.asset_placements() placement
                WHERE placement.status_id = 'completed' AND placement.asset_id IN (SELECT touched.asset_id FROM affected touched))
        ORDER BY ancestry.ancestor_model_id
    LOOP
        PERFORM public.lock_container_model(locked_model_id);
    END LOOP;
    FOR checked_container_id IN
        WITH affected (asset_id) AS (
            SELECT binding.asset_id FROM public.procedure_assets binding
            WHERE binding.procedure_id = affected_procedure_id AND binding.asset_id IS NOT NULL
            UNION SELECT new_asset_id WHERE new_asset_id IS NOT NULL
            UNION SELECT old_asset_id WHERE old_asset_id IS NOT NULL
        )
        SELECT container.id FROM public.assets container
        JOIN public.container_models model ON model.id = container.model_id
        WHERE container.id IN (SELECT touched.asset_id FROM affected touched)
            OR container.id IN (SELECT placement.container_id FROM public.asset_placements() placement
                WHERE placement.status_id = 'completed' AND placement.asset_id IN (SELECT touched.asset_id FROM affected touched))
        ORDER BY container.id
    LOOP
        PERFORM public.lock_asset(checked_container_id);
        PERFORM public.require_container_capacity(checked_container_id);
    END LOOP;
END;
$$;
ALTER FUNCTION require_affected_container_capacity(UUID, UUID, UUID) OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION require_affected_container_capacity(UUID, UUID, UUID) FROM PUBLIC;

-- Checks containers affected by a procedure or typed-run write.
CREATE FUNCTION check_container_capacity() RETURNS TRIGGER
LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
BEGIN
    PERFORM public.require_affected_container_capacity(COALESCE(NEW.id, OLD.id), NULL, NULL);
    RETURN NULL;
END;
$$;
ALTER FUNCTION check_container_capacity() OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION check_container_capacity() FROM PUBLIC;

-- Includes removed and rebound assets in capacity checks.
CREATE FUNCTION check_binding_container_capacity() RETURNS TRIGGER
LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
BEGIN
    PERFORM public.require_affected_container_capacity(COALESCE(NEW.procedure_id, OLD.procedure_id), NEW.asset_id, OLD.asset_id);
    RETURN NULL;
END;
$$;
ALTER FUNCTION check_binding_container_capacity() OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION check_binding_container_capacity() FROM PUBLIC;

-- Checks, in container order, every container governed by the changed
-- compatibility rule, after locking the container models of all of them and
-- then each container.
CREATE FUNCTION check_rule_container_capacity() RETURNS TRIGGER
LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
DECLARE
    changed_model_id UUID := COALESCE(NEW.container_model_id, OLD.container_model_id);
    locked_model_id UUID;
    checked_container_id UUID;
BEGIN
    PERFORM public.lock_container_model(changed_model_id);
    FOR locked_model_id IN
        SELECT DISTINCT container_ancestry.ancestor_model_id FROM public.assets container
        JOIN public.asset_model_ancestors ancestry ON ancestry.descendant_model_id = container.model_id
        JOIN public.asset_model_ancestors container_ancestry ON container_ancestry.descendant_model_id = container.model_id
        JOIN public.container_models ancestor ON ancestor.id = container_ancestry.ancestor_model_id
        WHERE ancestry.ancestor_model_id = changed_model_id
        ORDER BY container_ancestry.ancestor_model_id
    LOOP
        PERFORM public.lock_container_model(locked_model_id);
    END LOOP;
    FOR checked_container_id IN
        SELECT container.id FROM public.assets container
        JOIN public.asset_model_ancestors ancestry ON ancestry.descendant_model_id = container.model_id
        WHERE ancestry.ancestor_model_id = changed_model_id ORDER BY container.id
    LOOP
        PERFORM public.lock_asset(checked_container_id);
        PERFORM public.require_container_capacity(checked_container_id);
    END LOOP;
    RETURN NULL;
END;
$$;
ALTER FUNCTION check_rule_container_capacity() OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION check_rule_container_capacity() FROM PUBLIC;
GRANT SELECT, UPDATE (id) ON container_models TO aps_writer;
GRANT SELECT ON container_compatibility_rules TO aps_writer;

CREATE CONSTRAINT TRIGGER procedures_container_capacity AFTER INSERT OR UPDATE OR DELETE ON procedures
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION check_container_capacity();
CREATE CONSTRAINT TRIGGER procedure_assets_container_capacity AFTER INSERT OR UPDATE OR DELETE ON procedure_assets
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION check_binding_container_capacity();
CREATE CONSTRAINT TRIGGER compatibility_container_capacity AFTER INSERT OR UPDATE OR DELETE ON container_compatibility_rules
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION check_rule_container_capacity();
CREATE CONSTRAINT TRIGGER pouring_container_capacity AFTER INSERT OR UPDATE OR DELETE ON pouring_procedures
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION check_container_capacity();
CREATE CONSTRAINT TRIGGER aliquoting_container_capacity AFTER INSERT OR UPDATE OR DELETE ON aliquoting_procedures
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION check_container_capacity();
CREATE CONSTRAINT TRIGGER supernatant_container_capacity AFTER INSERT OR UPDATE OR DELETE ON supernatant_procedures
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION check_container_capacity();
CREATE CONSTRAINT TRIGGER fractioning_container_capacity AFTER INSERT OR UPDATE OR DELETE ON fractioning_procedures
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION check_container_capacity();

-- Rejects impossible retrievals, container consumption and simultaneous containment cycles.
CREATE FUNCTION require_container_history(checked_container_id UUID) RETURNS VOID
LANGUAGE plpgsql SET search_path = pg_catalog, pg_temp AS $$
BEGIN
    IF EXISTS (
        WITH transfers (procedure_id, source_sample_slot_id, source_container_slot_id) AS (
            SELECT typed.id, template.source_sample_slot_id, template.poured_from_slot_id FROM public.pouring_procedures typed JOIN public.pouring_procedure_templates template ON template.id = typed.pouring_procedure_template_id
            UNION ALL
            SELECT typed.id, template.source_sample_slot_id, template.aliquoted_from_slot_id FROM public.aliquoting_procedures typed JOIN public.aliquoting_procedure_templates template ON template.id = typed.aliquoting_procedure_template_id
            UNION ALL
            SELECT typed.id, template.source_sample_slot_id, template.stratified_source_slot_id FROM public.supernatant_procedures typed JOIN public.supernatant_procedure_templates template ON template.id = typed.supernatant_procedure_template_id
            UNION ALL
            SELECT typed.id, template.source_sample_slot_id, template.fragment_container_slot_id FROM public.fractioning_procedures typed JOIN public.fractioning_procedure_templates template ON template.id = typed.fractioning_procedure_template_id
        )
        SELECT 1 FROM transfers transfer
        JOIN public.procedures run ON run.id = transfer.procedure_id
        JOIN public.procedure_assets sample ON sample.procedure_id = run.id AND sample.slot_id = transfer.source_sample_slot_id
        JOIN public.procedure_assets source ON source.procedure_id = run.id AND source.slot_id = transfer.source_container_slot_id
        WHERE run.status_id = 'completed' AND source.asset_id = checked_container_id AND sample.asset_id IS NOT NULL
            AND NOT EXISTS (
                SELECT 1 FROM public.asset_containment(run.id) held
                WHERE held.asset_id = sample.asset_id AND held.entered_at <= run.started_at
                    AND (held.left_at IS NULL OR held.left_at > run.started_at)
                    AND held.container_id IN (
                        SELECT admitted_source.asset_id FROM public.procedure_assets admitted_source
                        WHERE admitted_source.procedure_id = run.id AND admitted_source.slot_id = transfer.source_container_slot_id
                    )
            )
    ) THEN
        RAISE EXCEPTION 'transfer source sample is not in its source container' USING ERRCODE = 'check_violation';
    END IF;
    IF EXISTS (
        SELECT 1 FROM public.procedures run
        JOIN public.procedure_assets contained ON contained.procedure_id = run.id
        JOIN public.procedure_template_slots slot ON slot.id = contained.slot_id
        JOIN public.procedure_assets source ON source.procedure_id = run.id AND source.slot_id = slot.retrieved_from_slot_id
        WHERE run.status_id = 'completed' AND source.asset_id = checked_container_id AND contained.asset_id IS NOT NULL
            AND NOT EXISTS (
                SELECT 1 FROM public.asset_containment(run.id) held
                WHERE held.asset_id = contained.asset_id AND held.container_id = source.asset_id
                    AND held.entered_at <= run.started_at AND (held.left_at IS NULL OR held.left_at > run.started_at)
            )
    ) THEN
        RAISE EXCEPTION 'retrieved asset is not in its source container' USING ERRCODE = 'check_violation';
    END IF;
    IF EXISTS (
        SELECT 1 FROM public.procedure_assets consumed
        JOIN public.procedures run ON run.id = consumed.procedure_id
        JOIN public.asset_containment() held ON held.container_id = consumed.asset_id
        WHERE consumed.asset_id = checked_container_id AND consumed.slot_role_id IN ('consumed', 'intermediate')
            AND run.status_id = 'completed' AND held.entered_at <= run.started_at
            AND (held.left_at IS NULL OR held.left_at > run.started_at)
    ) THEN
        RAISE EXCEPTION 'consumed container still holds assets' USING ERRCODE = 'check_violation';
    END IF;
    IF EXISTS (
        WITH RECURSIVE paths (container_id, entered_at, left_at, seeded) AS (
            SELECT checked_container_id, NULL::timestamptz, NULL::timestamptz, TRUE
            UNION
            SELECT held.container_id, CASE WHEN path.seeded THEN held.entered_at ELSE greatest(path.entered_at, held.entered_at) END,
                least(path.left_at, held.left_at), FALSE
            FROM paths path JOIN public.asset_containment() held ON held.asset_id = path.container_id
            WHERE (path.left_at IS NULL OR held.entered_at < path.left_at)
                AND (held.left_at IS NULL OR path.seeded OR path.entered_at < held.left_at)
        )
        SELECT 1 FROM paths path WHERE path.container_id = checked_container_id AND NOT path.seeded
    ) THEN
        RAISE EXCEPTION 'containment cycle' USING ERRCODE = 'check_violation';
    END IF;
END;
$$;
ALTER FUNCTION require_container_history(UUID) OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION require_container_history(UUID) FROM PUBLIC;
