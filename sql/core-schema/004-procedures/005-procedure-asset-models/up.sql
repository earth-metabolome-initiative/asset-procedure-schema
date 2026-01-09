CREATE TABLE IF NOT EXISTS procedure_asset_models (
	-- The ID of this procedure_id asset.
	id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
	-- The ID of the procedure_id this asset is used in.
	procedure_id UUID NOT NULL REFERENCES procedures(id),
	-- The procedure_id template of the procedure_id this asset is used in.
	procedure_template_id UUID NOT NULL REFERENCES procedure_templates(id),
	-- The asset model of the asset used in this procedure.
	asset_model_id UUID NOT NULL REFERENCES asset_models(id),
	-- We enforce that there must be a procedure_id template asset for this asset.
	procedure_template_asset_model_id UUID NOT NULL REFERENCES procedure_template_asset_models(id),
	-- The ancestor asset model defined in the procedure_id template asset.
	ancestor_model_id UUID NOT NULL REFERENCES asset_models(id),
	-- The procedure_id template must match the procedure_id template of the procedure.
	FOREIGN KEY (id, procedure_template_id) REFERENCES procedures(id, procedure_template_id),
	-- The procedure_id template asset must must be compatible with the procedure_id template of the procedure.
	FOREIGN KEY (
		procedure_template_asset_model_id,
		procedure_template_id
	) REFERENCES procedure_template_asset_models(id, procedure_template_id),
	-- We check that the ancestor asset is indeed the one defined in the procedure_id template asset.
	FOREIGN KEY (
		procedure_template_asset_model_id,
		ancestor_model_id
	) REFERENCES procedure_template_asset_models(id, asset_model_id),
	-- We check that the asset is indeed a descendant of the ancestor asset defined in the procedure_id template asset.
	FOREIGN KEY (asset_model_id, ancestor_model_id) REFERENCES asset_model_ancestors(descendant_model_id, ancestor_model_id),
	-- We create a unique index to allow for foreign keys checking that the current procedure_id asset
	-- corresponds to a specific procedure_id template asset model in the procedure_id template.
	UNIQUE (id, procedure_template_asset_model_id),
	-- We create a unique index to allow for foreign keys checking that the current procedure_id asset
	-- corresponds to a specific asset model.
	UNIQUE (id, asset_model_id)
);
