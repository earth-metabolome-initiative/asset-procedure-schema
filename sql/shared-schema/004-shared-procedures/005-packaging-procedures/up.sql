CREATE TABLE packaging_procedure_templates (
	id UUID PRIMARY KEY REFERENCES procedure_templates(id) ON DELETE CASCADE,
	packaged_with_model_id UUID NOT NULL REFERENCES packaging_models(id),
	procedure_template_packaged_with_model_id UUID NOT NULL REFERENCES procedure_template_asset_models(id) ON DELETE CASCADE,
	sample_model_id UUID NOT NULL REFERENCES physical_asset_models(id),
	procedure_template_sample_model_id UUID NOT NULL REFERENCES procedure_template_asset_models(id) ON DELETE CASCADE,
	-- We check that the `packaged_with_model` is indeed an asset model that is compatible with the procedure_id template asset model.
	FOREIGN KEY (
		procedure_template_packaged_with_model_id,
		packaged_with_model_id
	) REFERENCES procedure_template_asset_models(id, asset_model_id),
	-- We check that the `sample_model` is indeed an asset model that is compatible with the procedure_id template asset model.
	FOREIGN KEY (
		procedure_template_sample_model_id,
		sample_model_id
	) REFERENCES procedure_template_asset_models(id, asset_model_id),
	-- The `sample_model` must be compatible with the `packaged_with_model`.
	FOREIGN KEY (packaged_with_model_id, sample_model_id) REFERENCES asset_compatibility_rules(left_asset_model_id, right_asset_model_id),
	FOREIGN KEY (
		id,
		procedure_template_packaged_with_model_id
	) REFERENCES reused_procedure_template_asset_models(procedure_template_id, procedure_template_asset_model_id),
	FOREIGN KEY (
		id,
		procedure_template_sample_model_id
	) REFERENCES reused_procedure_template_asset_models(procedure_template_id, procedure_template_asset_model_id),
	-- We create a unique index to allow for foreign keys checking that there exist a `procedure_template_packaged_with_model`
	-- for the current `procedure_template`.
	UNIQUE (
		id,
		procedure_template_packaged_with_model_id
	),
	-- We create a unique index to allow for foreign keys checking that there exist a `procedure_template_sample_model`
	-- for the current `procedure_template`.
	UNIQUE (
		id,
		procedure_template_sample_model_id
	)
);
INSERT INTO procedure_template_tables (id) VALUES ('packaging_procedure_templates') ON CONFLICT DO NOTHING;
CREATE OR REPLACE FUNCTION packaging_procedure_templates_rptam_insert_fn() RETURNS TRIGGER AS $$
BEGIN
	INSERT INTO reused_procedure_template_asset_models (procedure_template_id, procedure_template_asset_model_id) VALUES (NEW.id, NEW.procedure_template_packaged_with_model_id) ON CONFLICT DO NOTHING;
	INSERT INTO reused_procedure_template_asset_models (procedure_template_id, procedure_template_asset_model_id) VALUES (NEW.id, NEW.procedure_template_sample_model_id) ON CONFLICT DO NOTHING;
RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER packaging_procedure_templates_rptam_insert_trigger
AFTER INSERT ON packaging_procedure_templates
FOR EACH ROW EXECUTE FUNCTION packaging_procedure_templates_rptam_insert_fn();
CREATE TABLE packaging_procedures (
	-- The extended `procedure`.
	id UUID PRIMARY KEY REFERENCES procedures(id) ON DELETE CASCADE,
	-- The procedure_id template of the extended `procedure`.
	packaging_procedure_template_id UUID NOT NULL REFERENCES packaging_procedure_templates(id),
	-- The sample being packaged, which must be a physical asset.
	sample_id UUID NOT NULL REFERENCES physical_assets(id),
	-- The model of the sample being packaged, which must be a physical asset model.
	sample_model_id UUID NOT NULL REFERENCES physical_asset_models(id),
	-- The procedure_id template asset model associated to the `sample`.
	procedure_template_sample_model_id UUID NOT NULL REFERENCES procedure_template_asset_models(id),
	-- The procedure_id asset associated to the `sample`.
	procedure_sample_id UUID NOT NULL REFERENCES procedure_asset_models(id) ON DELETE CASCADE,
	-- The packaging used for packaging, which must be a packaging model.
	packaged_with_model_id UUID NOT NULL REFERENCES packaging_models(id),
	-- The procedure_id template asset model associated to the `packaged_with_model`.
	procedure_template_packaged_with_model_id UUID NOT NULL REFERENCES procedure_template_asset_models(id),
	-- The procedure_id asset associated to the `packaged_with_model`.
	procedure_packaged_with_id UUID NOT NULL REFERENCES procedure_asset_models(id) ON DELETE CASCADE,
	-- We enforce that the extended `procedure` has indeed the same `procedure_template`, making
	-- sure that the procedure_id is a packaging procedure.
	FOREIGN KEY (id, packaging_procedure_template_id) REFERENCES procedures(id, procedure_template_id),
	-- The `procedure_template_packaged_with_model` must be the same as in the `packaging_procedure_templates`.
	FOREIGN KEY (
		packaging_procedure_template_id,
		procedure_template_packaged_with_model_id
	) REFERENCES packaging_procedure_templates(
		id,
		procedure_template_packaged_with_model_id
	),
	-- The `procedure_template_sample_model` must be the same as in the `packaging_procedure_templates`.
	FOREIGN KEY (
		packaging_procedure_template_id,
		procedure_template_sample_model_id
	) REFERENCES packaging_procedure_templates(
		id,
		procedure_template_sample_model_id
	),
	-- We check that the `procedure_sample` is associated to the `sample`.
	FOREIGN KEY (sample_id, sample_model_id) REFERENCES assets(id, model_id),
	-- We check that the `procedure_packaged_with` is associated to the `packaged_with_model`.
	FOREIGN KEY (procedure_packaged_with_id, packaged_with_model_id) REFERENCES procedure_asset_models(id, asset_model_id),
	-- We check that the `procedure_sample` is indeed associated to the `procedure_template_sample_model`.
	FOREIGN KEY (
		procedure_sample_id,
		procedure_template_sample_model_id
	) REFERENCES procedure_asset_models(id, procedure_template_asset_model_id),
	-- We check that the `procedure_packaged_with` is indeed associated to the `procedure_template_packaged_with_model`.
	FOREIGN KEY (
		procedure_packaged_with_id,
		procedure_template_packaged_with_model_id
	) REFERENCES procedure_asset_models(id, procedure_template_asset_model_id),
	-- We check that the `sample` is indeed compatible with the `packaged_with_model`.
	FOREIGN KEY (packaged_with_model_id, sample_model_id) REFERENCES asset_compatibility_rules(left_asset_model_id, right_asset_model_id),
	-- We check that the `procedure_sample` is associated to the `sample_model`.
	FOREIGN KEY (procedure_sample_id, sample_model_id) REFERENCES procedure_asset_models(id, asset_model_id)
);
INSERT INTO ownable_tables (id) VALUES ('packaging_procedures') ON CONFLICT DO NOTHING;