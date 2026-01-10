CREATE TABLE geopositioning_procedure_templates (
	id UUID PRIMARY KEY REFERENCES procedure_templates(id) ON DELETE CASCADE,
	-- The device used for geopositioning.
	geopositioned_with_model_id UUID NOT NULL REFERENCES geopositioning_device_models(id),
	procedure_template_geopositioned_with_model_id UUID NOT NULL REFERENCES procedure_template_asset_models(id) ON DELETE CASCADE,
	geopositioned_asset_model_id UUID NOT NULL REFERENCES physical_asset_models(id),
	procedure_template_geopositioned_asset_model_id UUID NOT NULL REFERENCES procedure_template_asset_models(id),
	-- We check that the `geopositioned_with_model` is indeed an asset model that is compatible with the procedure_id template.
	FOREIGN KEY (
		procedure_template_geopositioned_with_model_id,
		geopositioned_with_model_id
	) REFERENCES procedure_template_asset_models(id, asset_model_id),
	-- We check that the asset model is indeed a procedure_id asset model.
	FOREIGN KEY (
		procedure_template_geopositioned_asset_model_id,
		geopositioned_asset_model_id
	) REFERENCES procedure_template_asset_models(id, asset_model_id),
	FOREIGN KEY (
		id,
		procedure_template_geopositioned_with_model_id
	) REFERENCES reused_procedure_template_asset_models(procedure_template_id, procedure_template_asset_model_id),
	FOREIGN KEY (
		id,
		procedure_template_geopositioned_asset_model_id
	) REFERENCES reused_procedure_template_asset_models(procedure_template_id, procedure_template_asset_model_id),
	-- We create a unique index to allow for foreign keys checking that there exist a `procedure_template_geopositioned_with_model`
	-- for the current `procedure_template`.
	UNIQUE (
		id,
		procedure_template_geopositioned_with_model_id
	),
	-- We create a unique index to allow for foreign keys checking that there exist a `procedure_template_geopositioned_asset_model`
	-- for the current `procedure_template`.
	UNIQUE (
		id,
		procedure_template_geopositioned_asset_model_id
	)
);
INSERT INTO procedure_template_tables (id) VALUES ('geopositioning_procedure_templates') ON CONFLICT DO NOTHING;
CREATE OR REPLACE FUNCTION geopositioning_procedure_templates_rptam_insert_fn() RETURNS TRIGGER AS $$
BEGIN
	INSERT INTO reused_procedure_template_asset_models (procedure_template_id, procedure_template_asset_model_id) VALUES (NEW.id, NEW.procedure_template_geopositioned_with_model_id) ON CONFLICT DO NOTHING;
	INSERT INTO reused_procedure_template_asset_models (procedure_template_id, procedure_template_asset_model_id) VALUES (NEW.id, NEW.procedure_template_geopositioned_asset_model_id) ON CONFLICT DO NOTHING;
RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER geopositioning_procedure_templates_rptam_insert_trigger
AFTER INSERT ON geopositioning_procedure_templates
FOR EACH ROW EXECUTE FUNCTION geopositioning_procedure_templates_rptam_insert_fn();
CREATE TABLE geopositioning_procedures (
	-- Identifier of the geopositioning id, which is also a foreign key to the general procedure.
	id UUID PRIMARY KEY REFERENCES procedures(id) ON DELETE CASCADE,
	-- The template of this procedure_id should be a geopositioning procedure_id template.
	geopositioning_procedure_template_id UUID NOT NULL REFERENCES geopositioning_procedure_templates(id),
	-- The asset being geopositioned, which must be a physical asset.
	geopositioned_asset_id UUID NOT NULL REFERENCES physical_assets(id),
	-- The model of the asset being geopositioned.
	geopositioned_asset_model_id UUID NOT NULL REFERENCES physical_asset_models(id),
	-- The procedure_id template asset model associated to the `geopositioned_asset`.
	procedure_template_geopositioned_asset_model_id UUID NOT NULL REFERENCES procedure_template_asset_models(id),
	-- The procedure_id asset associated to the `geopositioned_asset`.
	procedure_geopositioned_asset_id UUID NOT NULL REFERENCES procedure_asset_models(id) ON DELETE CASCADE,
	-- The positioning device used for geopositioning. This field is optional, as the positioning device might not necessarily be tracked.
	geopositioned_with_id UUID REFERENCES geopositioning_devices(id),
	-- The model of the positioning device used for geopositioning.
	geopositioned_with_model_id UUID NOT NULL REFERENCES geopositioning_device_models(id),
	-- The procedure_id asset associated to the `geopositioned_with`.
	procedure_geopositioned_with_id UUID NOT NULL REFERENCES procedure_asset_models(id) ON DELETE CASCADE,
	-- The procedure_id template asset model associated to the `geopositioned_with_model`.
	procedure_template_geopositioned_with_model_id UUID NOT NULL REFERENCES procedure_template_asset_models(id),
	-- The latitude and longitude of the geopositioning.
	location GEOGRAPHY(POINT, 4326) NOT NULL,
	-- We enforce that the current `geopositioning` has indeed the same `geopositioning_template`.
	FOREIGN KEY (id, geopositioning_procedure_template_id) REFERENCES procedures(id, procedure_template_id),
	-- The `procedure_template_geopositioned_with_model` must be the same as in the `geopositioning_procedure_templates`.
	FOREIGN KEY (
		geopositioning_procedure_template_id,
		procedure_template_geopositioned_with_model_id
	) REFERENCES geopositioning_procedure_templates(
		id,
		procedure_template_geopositioned_with_model_id
	),
	-- The `procedure_template_geopositioned_asset_model` must be the same as in the `geopositioning_procedure_templates`.
	FOREIGN KEY (
		geopositioning_procedure_template_id,
		procedure_template_geopositioned_asset_model_id
	) REFERENCES geopositioning_procedure_templates(
		id,
		procedure_template_geopositioned_asset_model_id
	),
	-- We check that the `procedure_geopositioned_asset` has the same `procedure_template_geopositioned_asset_model`.
	FOREIGN KEY (
		procedure_geopositioned_asset_id,
		procedure_template_geopositioned_asset_model_id
	) REFERENCES procedure_asset_models(id, procedure_template_asset_model_id),
	-- We check that the `procedure_geopositioned_with` has the same `procedure_template_geopositioned_with_model`.
	FOREIGN KEY (
		procedure_geopositioned_with_id,
		procedure_template_geopositioned_with_model_id
	) REFERENCES procedure_asset_models(id, procedure_template_asset_model_id),
	-- We check that the `procedure_geopositioned_asset` is associated to the `geopositioned_asset_model`.
	FOREIGN KEY (procedure_geopositioned_asset_id, geopositioned_asset_model_id) REFERENCES procedure_asset_models(id, asset_model_id),
	-- We check that the `procedure_geopositioned_with` is associated to the `geopositioned_with_model`.
	FOREIGN KEY (procedure_geopositioned_with_id, geopositioned_with_model_id) REFERENCES procedure_asset_models(id, asset_model_id),
	-- We check that the `procedure_geopositioned_asset` is associated to the `geopositioned_asset`.
	FOREIGN KEY (geopositioned_asset_id, geopositioned_asset_model_id) REFERENCES assets(id, model_id),
	-- We check that the `procedure_geopositioned_with` is associated to the `geopositioned_with`.
	FOREIGN KEY (geopositioned_with_id, geopositioned_with_model_id) REFERENCES assets(id, model_id)
);
INSERT INTO procedure_tables (id) VALUES ('geopositioning_procedures') ON CONFLICT DO NOTHING;