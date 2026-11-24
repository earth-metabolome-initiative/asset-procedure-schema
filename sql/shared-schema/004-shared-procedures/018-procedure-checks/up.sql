-- Marks a volumetric model as relied on, after any concurrent change of its
-- volume commits, so the caller reads the volume that stays.
CREATE FUNCTION use_volumetric_model(used_model_id UUID) RETURNS VOID
LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
BEGIN
    PERFORM authz.refuse_repeatable_read();
    UPDATE public.volumetric_container_models model SET in_use = TRUE WHERE model.id = used_model_id AND NOT model.in_use;
END;
$$;
ALTER FUNCTION use_volumetric_model(UUID) OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION use_volumetric_model(UUID) FROM PUBLIC;
GRANT SELECT, UPDATE (in_use) ON volumetric_container_models TO aps_writer;

-- Lists measured deviations visible to the caller.
CREATE FUNCTION procedure_deviations(checked_procedure_id UUID) RETURNS SETOF TEXT
LANGUAGE sql STABLE SET search_path = pg_catalog, pg_temp AS $$
SELECT parameter.name FROM public.procedure_parameter_values recorded JOIN public.procedure_template_parameters parameter ON parameter.id = recorded.parameter_id WHERE recorded.procedure_id = checked_procedure_id AND (recorded.number_value < parameter.minimum_number OR recorded.number_value > parameter.maximum_number)
UNION ALL
SELECT 'pressure_pa'::text FROM public.freeze_drying_procedures measured JOIN public.freeze_drying_procedure_templates template ON template.id = measured.freeze_drying_procedure_template_id WHERE measured.id = checked_procedure_id AND abs(measured.pressure_pa - template.pressure_pa) > template.pressure_pa * template.pressure_pa_tolerance_percentage / 100.0
UNION ALL
SELECT 'speed_rpm'::text FROM public.centrifuge_procedures measured JOIN public.centrifuge_procedure_templates template ON template.id = measured.centrifuge_procedure_template_id WHERE measured.id = checked_procedure_id AND abs(measured.speed_rpm - template.speed_rpm) > template.speed_rpm * template.speed_rpm_tolerance_percentage / 100.0
UNION ALL
SELECT 'frequency_hz'::text FROM public.ball_mill_procedures measured JOIN public.ball_mill_procedure_templates template ON template.id = measured.ball_mill_procedure_template_id WHERE measured.id = checked_procedure_id AND abs(measured.frequency_hz - template.frequency_hz) > template.frequency_hz * template.frequency_hz_tolerance_percentage / 100.0
UNION ALL
SELECT 'temperature_k'::text FROM public.storage_procedures measured JOIN public.storage_procedure_templates template ON template.id = measured.storage_procedure_template_id WHERE measured.id = checked_procedure_id AND abs(measured.temperature_k - template.temperature_k) > template.temperature_k * template.temperature_tolerance_percentage / 100.0
UNION ALL
SELECT 'temperature_k'::text FROM public.freezing_procedures measured JOIN public.freezing_procedure_templates template ON template.id = measured.freezing_procedure_template_id WHERE measured.id = checked_procedure_id AND abs(measured.temperature_k - template.temperature_k) > template.temperature_k * template.temperature_tolerance_percentage / 100.0
UNION ALL
SELECT 'duration_s'::text FROM public.freezing_procedures measured JOIN public.freezing_procedure_templates template ON template.id = measured.freezing_procedure_template_id JOIN public.procedures run ON run.id = measured.id WHERE measured.id = checked_procedure_id AND abs(EXTRACT(EPOCH FROM (run.ended_at - run.started_at)) - template.duration_s) > template.duration_s * template.duration_tolerance_percentage / 100.0
UNION ALL
SELECT 'temperature_k'::text FROM public.freeze_drying_procedures measured JOIN public.freeze_drying_procedure_templates template ON template.id = measured.freeze_drying_procedure_template_id WHERE measured.id = checked_procedure_id AND abs(measured.temperature_k - template.temperature_k) > template.temperature_k * template.temperature_tolerance_percentage / 100.0
UNION ALL
SELECT 'duration_s'::text FROM public.freeze_drying_procedures measured JOIN public.freeze_drying_procedure_templates template ON template.id = measured.freeze_drying_procedure_template_id JOIN public.procedures run ON run.id = measured.id WHERE measured.id = checked_procedure_id AND abs(EXTRACT(EPOCH FROM (run.ended_at - run.started_at)) - template.duration_s) > template.duration_s * template.duration_tolerance_percentage / 100.0
UNION ALL
SELECT 'temperature_k'::text FROM public.centrifuge_procedures measured JOIN public.centrifuge_procedure_templates template ON template.id = measured.centrifuge_procedure_template_id WHERE measured.id = checked_procedure_id AND abs(measured.temperature_k - template.temperature_k) > template.temperature_k * template.temperature_tolerance_percentage / 100.0
UNION ALL
SELECT 'duration_s'::text FROM public.centrifuge_procedures measured JOIN public.centrifuge_procedure_templates template ON template.id = measured.centrifuge_procedure_template_id JOIN public.procedures run ON run.id = measured.id WHERE measured.id = checked_procedure_id AND abs(EXTRACT(EPOCH FROM (run.ended_at - run.started_at)) - template.duration_s) > template.duration_s * template.duration_tolerance_percentage / 100.0
UNION ALL
SELECT 'temperature_k'::text FROM public.ball_mill_procedures measured JOIN public.ball_mill_procedure_templates template ON template.id = measured.ball_mill_procedure_template_id WHERE measured.id = checked_procedure_id AND abs(measured.temperature_k - template.temperature_k) > template.temperature_k * template.temperature_tolerance_percentage / 100.0
UNION ALL
SELECT 'duration_s'::text FROM public.ball_mill_procedures measured JOIN public.ball_mill_procedure_templates template ON template.id = measured.ball_mill_procedure_template_id JOIN public.procedures run ON run.id = measured.id WHERE measured.id = checked_procedure_id AND abs(EXTRACT(EPOCH FROM (run.ended_at - run.started_at)) - template.duration_s) > template.duration_s * template.duration_tolerance_percentage / 100.0
UNION ALL
SELECT 'mass_kg'::text FROM public.fractioning_procedures measured JOIN public.fractioning_procedure_templates template ON template.id = measured.fractioning_procedure_template_id WHERE measured.id = checked_procedure_id AND abs(measured.mass_kg - template.mass_kg) > template.mass_kg * template.tolerance_percentage / 100.0
UNION ALL
SELECT 'volume_l'::text FROM public.pouring_procedures measured JOIN public.pouring_procedure_templates template ON template.id = measured.pouring_procedure_template_id WHERE measured.id = checked_procedure_id AND (abs(measured.volume_l - template.volume_l) > template.volume_l * template.volume_l_tolerance_percentage / 100.0 OR EXISTS (SELECT 1 FROM public.procedure_assets binding JOIN public.volumetric_container_models model ON model.id = binding.asset_model_id WHERE binding.procedure_id = measured.id AND binding.slot_id = template.poured_into_slot_id AND measured.volume_l > model.volume_l))
UNION ALL
SELECT 'volume_l'::text FROM public.aliquoting_procedures measured JOIN public.aliquoting_procedure_templates template ON template.id = measured.aliquoting_procedure_template_id WHERE measured.id = checked_procedure_id AND (abs(measured.volume_l - template.volume_l) > template.volume_l * template.volume_l_tolerance_percentage / 100.0 OR EXISTS (SELECT 1 FROM public.procedure_assets binding JOIN public.volumetric_container_models model ON model.id = binding.asset_model_id WHERE binding.procedure_id = measured.id AND binding.slot_id = template.aliquoted_into_slot_id AND measured.volume_l > model.volume_l))
UNION ALL
SELECT 'volume_l'::text FROM public.supernatant_procedures measured JOIN public.supernatant_procedure_templates template ON template.id = measured.supernatant_procedure_template_id WHERE measured.id = checked_procedure_id AND (abs(measured.volume_l - template.volume_l) > template.volume_l * template.volume_l_tolerance_percentage / 100.0 OR EXISTS (SELECT 1 FROM public.procedure_assets binding JOIN public.volumetric_container_models model ON model.id = binding.asset_model_id WHERE binding.procedure_id = measured.id AND binding.slot_id = template.supernatant_destination_slot_id AND measured.volume_l > model.volume_l))
UNION ALL
SELECT 'bead_count'::text FROM public.ball_mill_procedures measured JOIN public.ball_mill_procedure_templates template ON template.id = measured.ball_mill_procedure_template_id WHERE measured.id = checked_procedure_id AND template.bead_count <> (SELECT count(*) FROM public.procedure_assets binding WHERE binding.procedure_id = measured.id AND binding.slot_id = template.bead_slot_id)
$$;
ALTER FUNCTION procedure_deviations(UUID) OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION procedure_deviations(UUID) FROM PUBLIC;
GRANT EXECUTE ON FUNCTION procedure_deviations(UUID) TO app;

-- Requires a reason for a completed run's measured deviations.
CREATE FUNCTION check_procedure_deviations() RETURNS TRIGGER
LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
DECLARE
    bound_model_id UUID;
BEGIN
    IF EXISTS (SELECT 1 FROM public.procedures run WHERE run.id = NEW.id AND run.status_id = 'completed') THEN
        FOR bound_model_id IN SELECT DISTINCT binding.asset_model_id FROM public.procedure_assets binding
            WHERE binding.procedure_id = NEW.id ORDER BY binding.asset_model_id
        LOOP
            PERFORM public.use_volumetric_model(bound_model_id);
        END LOOP;
    END IF;
    IF EXISTS (SELECT 1 FROM public.procedures run WHERE run.id = NEW.id AND run.status_id = 'completed'
        AND (run.note IS NULL OR btrim(run.note) = '') AND EXISTS (SELECT 1 FROM public.procedure_deviations(run.id))) THEN
        RAISE EXCEPTION 'procedure % requires a note for its deviations', NEW.id USING ERRCODE = 'check_violation';
    END IF;
    RETURN NULL;
END;
$$;
ALTER FUNCTION check_procedure_deviations() OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION check_procedure_deviations() FROM PUBLIC;
CREATE CONSTRAINT TRIGGER storage_deviations AFTER INSERT OR UPDATE ON storage_procedures
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION check_procedure_deviations();
CREATE CONSTRAINT TRIGGER freezing_deviations AFTER INSERT OR UPDATE ON freezing_procedures
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION check_procedure_deviations();
CREATE CONSTRAINT TRIGGER freeze_drying_deviations AFTER INSERT OR UPDATE ON freeze_drying_procedures
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION check_procedure_deviations();
CREATE CONSTRAINT TRIGGER centrifuge_deviations AFTER INSERT OR UPDATE ON centrifuge_procedures
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION check_procedure_deviations();
CREATE CONSTRAINT TRIGGER ball_mill_deviations AFTER INSERT OR UPDATE ON ball_mill_procedures
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION check_procedure_deviations();
CREATE CONSTRAINT TRIGGER pouring_deviations AFTER INSERT OR UPDATE ON pouring_procedures
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION check_procedure_deviations();
CREATE CONSTRAINT TRIGGER aliquoting_deviations AFTER INSERT OR UPDATE ON aliquoting_procedures
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION check_procedure_deviations();
CREATE CONSTRAINT TRIGGER supernatant_deviations AFTER INSERT OR UPDATE ON supernatant_procedures
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION check_procedure_deviations();
CREATE CONSTRAINT TRIGGER fractioning_deviations AFTER INSERT OR UPDATE ON fractioning_procedures
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION check_procedure_deviations();

-- Rejects a transfer setpoint exceeding the destination model's capacity.
CREATE FUNCTION check_pouring_volume() RETURNS TRIGGER
LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
BEGIN
    PERFORM public.use_volumetric_model(NEW.poured_into_model_id);
    IF NEW.volume_l > (SELECT model.volume_l FROM public.volumetric_container_models model WHERE model.id = NEW.poured_into_model_id) THEN
        RAISE EXCEPTION 'transfer volume exceeds destination capacity' USING ERRCODE = 'check_violation';
    END IF;
    RETURN NEW;
END;
$$;
ALTER FUNCTION check_pouring_volume() OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION check_pouring_volume() FROM PUBLIC;
CREATE TRIGGER pouring_template_volume AFTER INSERT OR UPDATE ON pouring_procedure_templates
    FOR EACH ROW EXECUTE FUNCTION check_pouring_volume();

-- Rejects a transfer setpoint exceeding the destination model's capacity.
CREATE FUNCTION check_aliquoting_volume() RETURNS TRIGGER
LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
BEGIN
    PERFORM public.use_volumetric_model(NEW.aliquoted_into_model_id);
    IF NEW.volume_l > (SELECT model.volume_l FROM public.volumetric_container_models model WHERE model.id = NEW.aliquoted_into_model_id) THEN
        RAISE EXCEPTION 'transfer volume exceeds destination capacity' USING ERRCODE = 'check_violation';
    END IF;
    RETURN NEW;
END;
$$;
ALTER FUNCTION check_aliquoting_volume() OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION check_aliquoting_volume() FROM PUBLIC;
CREATE TRIGGER aliquoting_template_volume AFTER INSERT OR UPDATE ON aliquoting_procedure_templates
    FOR EACH ROW EXECUTE FUNCTION check_aliquoting_volume();

-- Rejects a transfer setpoint exceeding the destination model's capacity.
CREATE FUNCTION check_supernatant_volume() RETURNS TRIGGER
LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
BEGIN
    PERFORM public.use_volumetric_model(NEW.supernatant_destination_model_id);
    IF NEW.volume_l > (SELECT model.volume_l FROM public.volumetric_container_models model WHERE model.id = NEW.supernatant_destination_model_id) THEN
        RAISE EXCEPTION 'transfer volume exceeds destination capacity' USING ERRCODE = 'check_violation';
    END IF;
    RETURN NEW;
END;
$$;
ALTER FUNCTION check_supernatant_volume() OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION check_supernatant_volume() FROM PUBLIC;
CREATE TRIGGER supernatant_template_volume AFTER INSERT OR UPDATE ON supernatant_procedure_templates
    FOR EACH ROW EXECUTE FUNCTION check_supernatant_volume();
GRANT SELECT ON weighing_procedure_templates, weighing_procedures, storage_procedure_templates, storage_procedures, container_sealing_procedure_templates, container_sealing_procedures, disposal_procedure_templates, disposal_procedures, packaging_procedure_templates, packaging_procedures, photograph_procedure_templates, photograph_procedures, geopositioning_procedure_templates, geopositioning_procedures, freeze_drying_procedure_templates, freeze_drying_procedures, freezing_procedure_templates, freezing_procedures, fractioning_procedure_templates, fractioning_procedures, centrifuge_procedure_templates, centrifuge_procedures, ball_mill_procedure_templates, ball_mill_procedures, pouring_procedure_templates, pouring_procedures, aliquoting_procedure_templates, aliquoting_procedures, supernatant_procedure_templates, supernatant_procedures, harvesting_procedure_templates, harvesting_procedures, volumetric_container_models TO aps_writer;

CREATE CONSTRAINT TRIGGER procedures_measured_deviations AFTER INSERT OR UPDATE ON procedures
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION check_procedure_deviations();
