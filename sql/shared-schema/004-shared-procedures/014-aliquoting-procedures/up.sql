CREATE TABLE aliquoting_procedure_templates (
	id UUID PRIMARY KEY REFERENCES procedure_templates(id) ON DELETE CASCADE,
	-- The volume in liters that should be aliquoted.
	volume REAL NOT NULL CHECK (volume > 0.0),
	-- Source container from which the aliquot is taken.
	aliquoted_from_model_id UUID NOT NULL REFERENCES volumetric_container_models(id),
	procedure_template_aliquoted_from_model_id UUID NOT NULL REFERENCES procedure_template_asset_models(id),
	-- Destination container to which the aliquot is transferred.
	aliquoted_into_model_id UUID NOT NULL REFERENCES volumetric_container_models(id),
	procedure_template_aliquoted_into_model_id UUID NOT NULL REFERENCES procedure_template_asset_models(id) ON DELETE CASCADE,
	-- The device used for the aliquoting procedure.
	aliquoted_with_model_id UUID NOT NULL REFERENCES volume_measuring_device_models(id),
	procedure_template_aliquoted_with_model_id UUID NOT NULL REFERENCES procedure_template_asset_models(id) ON DELETE CASCADE,
	FOREIGN KEY (
		procedure_template_aliquoted_from_model_id,
		aliquoted_from_model_id
	) REFERENCES procedure_template_asset_models(id, asset_model_id),
	FOREIGN KEY (
		procedure_template_aliquoted_with_model_id,
		aliquoted_with_model_id
	) REFERENCES procedure_template_asset_models(id, asset_model_id),
	FOREIGN KEY (
		procedure_template_aliquoted_into_model_id,
		aliquoted_into_model_id
	) REFERENCES procedure_template_asset_models(id, asset_model_id),
	FOREIGN KEY (
		id,
		procedure_template_aliquoted_from_model_id
	) REFERENCES reused_procedure_template_asset_models(procedure_template_id, procedure_template_asset_model_id),
	FOREIGN KEY (
		id,
		procedure_template_aliquoted_into_model_id
	) REFERENCES reused_procedure_template_asset_models(procedure_template_id, procedure_template_asset_model_id),
	FOREIGN KEY (
		id,
		procedure_template_aliquoted_with_model_id
	) REFERENCES reused_procedure_template_asset_models(procedure_template_id, procedure_template_asset_model_id),
	-- We create a unique index to allow for foreign keys checking that there exist a `procedure_template_aliquoted_from_model`
	-- for the current `procedure_template`.
	UNIQUE (
		id,
		procedure_template_aliquoted_from_model_id
	),
	-- We create a unique index to allow for foreign keys checking that there exist a `procedure_template_aliquoted_into_model`
	-- for the current `procedure_template`.
	UNIQUE (
		id,
		procedure_template_aliquoted_into_model_id
	),
	-- We create a unique index to allow for foreign keys checking that there exist a `procedure_template_aliquoted_with_model`
	-- for the current `procedure_template`.
	UNIQUE (
		id,
		procedure_template_aliquoted_with_model_id
	)
);
INSERT INTO procedure_template_tables (id) VALUES ('aliquoting_procedure_templates') ON CONFLICT DO NOTHING;
CREATE TABLE aliquoting_procedures (
	id UUID PRIMARY KEY REFERENCES procedures(id) ON DELETE CASCADE,
	-- We enforce that the model of this procedure must be an aliquoting procedure template.
	aliquoting_procedure_template_id UUID NOT NULL REFERENCES aliquoting_procedure_templates(id),
	-- The identifier of the instrument used for aliquoting.
	aliquoted_with_id UUID REFERENCES volume_measuring_devices(id),
	-- The identifier of the instrument model used for aliquoting.
	aliquoted_with_model_id UUID NOT NULL REFERENCES volume_measuring_device_models(id),
	-- The procedure template asset model associated to the `aliquoted_with`.
	procedure_template_aliquoted_with_model_id UUID NOT NULL REFERENCES procedure_template_asset_models(id),
	-- The procedure asset associated to the `aliquoted_with`.
	procedure_aliquoted_with_id UUID NOT NULL REFERENCES procedure_asset_models(id) ON DELETE CASCADE,
	-- The container being aliquoted, which must be a volumetric container model.
	aliquoted_from_id UUID NOT NULL REFERENCES volumetric_containers(id),
	-- The procedure template asset model associated to the `aliquoted_from`.
	procedure_template_aliquoted_from_model_id UUID NOT NULL REFERENCES procedure_template_asset_models(id),
	-- The procedure asset associated to the `aliquoted_from`.
	procedure_aliquoted_from_id UUID NOT NULL REFERENCES procedure_asset_models(id) ON DELETE CASCADE,
	-- The container receiving the aliquot, which must be a volumetric container model.
	aliquoted_into_id UUID NOT NULL REFERENCES volumetric_containers(id),
	-- The procedure template asset model associated to the `aliquoted_into`.
	procedure_template_aliquoted_into_model_id UUID NOT NULL REFERENCES procedure_template_asset_models(id),
	-- The procedure asset associated to the `aliquoted_into`.
	procedure_aliquoted_into_id UUID NOT NULL REFERENCES procedure_asset_models(id) ON DELETE CASCADE,
	-- We enforce that the extended `procedure` has indeed the same `procedure_template`, making
	-- sure that the procedure is an aliquoting procedure without the possibility of a mistake.
	FOREIGN KEY (id, aliquoting_procedure_template_id) REFERENCES procedures(id, procedure_template_id),
	-- The procedure template asset model describing the `aliquoted_with` must be the same one
	-- as the one in the procedure template.
	FOREIGN KEY (
		aliquoting_procedure_template_id,
		procedure_template_aliquoted_with_model_id
	) REFERENCES aliquoting_procedure_templates(
		id,
		procedure_template_aliquoted_with_model_id
	),
	-- The procedure template asset model describing the `aliquoted_from` must be the same one
	-- as the one in the procedure template.
	FOREIGN KEY (
		aliquoting_procedure_template_id,
		procedure_template_aliquoted_from_model_id
	) REFERENCES aliquoting_procedure_templates(
		id,
		procedure_template_aliquoted_from_model_id
	),
	-- The procedure template asset model describing the `aliquoted_into` must be the same one
	-- as the one in the procedure template.
	FOREIGN KEY (
		aliquoting_procedure_template_id,
		procedure_template_aliquoted_into_model_id
	) REFERENCES aliquoting_procedure_templates(
		id,
		procedure_template_aliquoted_into_model_id
	),
	-- We enforce that the procedure template asset model reported in the procedure is indeed
	-- the same one associated to the procedure asset for the asset `aliquoted_with`.
	FOREIGN KEY (
		procedure_aliquoted_with_id,
		procedure_template_aliquoted_with_model_id
	) REFERENCES procedure_asset_models(id, procedure_template_asset_model_id),
	-- We enforce that the procedure template asset model reported in the procedure is indeed
	-- the same one associated to the procedure asset for the asset `aliquoted_from`.
	FOREIGN KEY (
		procedure_aliquoted_from_id,
		procedure_template_aliquoted_from_model_id
	) REFERENCES procedure_asset_models(id, procedure_template_asset_model_id),
	-- We enforce that the procedure template asset model reported in the procedure is indeed
	-- the same one associated to the procedure asset for the asset `aliquoted_into`.
	FOREIGN KEY (
		procedure_aliquoted_into_id,
		procedure_template_aliquoted_into_model_id
	) REFERENCES procedure_asset_models(id, procedure_template_asset_model_id),
	-- We enfore that the `aliquoted_with_model` asset model is correctly associated to the `aliquoted_with` procedure asset.
	FOREIGN KEY (procedure_aliquoted_with_id, aliquoted_with_model_id) REFERENCES procedure_asset_models(id, asset_model_id)
);
CREATE OR REPLACE FUNCTION aliquoting_procedure_templates_rptam_insert_fn() RETURNS TRIGGER AS $$
BEGIN
	INSERT INTO reused_procedure_template_asset_models (procedure_template_id, procedure_template_asset_model_id) VALUES (NEW.id, NEW.procedure_template_aliquoted_from_model_id) ON CONFLICT DO NOTHING;
	INSERT INTO reused_procedure_template_asset_models (procedure_template_id, procedure_template_asset_model_id) VALUES (NEW.id, NEW.procedure_template_aliquoted_into_model_id) ON CONFLICT DO NOTHING;
	INSERT INTO reused_procedure_template_asset_models (procedure_template_id, procedure_template_asset_model_id) VALUES (NEW.id, NEW.procedure_template_aliquoted_with_model_id) ON CONFLICT DO NOTHING;
RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER aliquoting_procedure_templates_rptam_insert_trigger
AFTER INSERT ON aliquoting_procedure_templates
FOR EACH ROW EXECUTE FUNCTION aliquoting_procedure_templates_rptam_insert_fn();
INSERT INTO procedure_tables (id) VALUES ('aliquoting_procedures') ON CONFLICT DO NOTHING;