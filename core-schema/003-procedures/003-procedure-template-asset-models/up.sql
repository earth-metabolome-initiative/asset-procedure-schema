CREATE TABLE IF NOT EXISTS procedure_template_asset_models (
	-- Identifier of the procedure_id template asset model
	id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
	-- The name of the procedure_id template asset model
	name TEXT NOT NULL CHECK (name <> ''),
	-- Procedure template this asset model is associated with
	procedure_template_id UUID NOT NULL REFERENCES procedure_templates(id) ON DELETE CASCADE,
	-- Optional reference to a procedure_id template asset model from another procedure_id template
	-- which this procedure_id template asset model is based on
	based_on_id UUID REFERENCES procedure_template_asset_models(id),
	-- The asset model this procedure_id template asset model is associated with
	asset_model_id UUID NOT NULL REFERENCES asset_models(id) ON DELETE CASCADE,
	-- We enforce that, if based_on_id is specified, then the asset model must be the same as the one
	-- of the procedure_id template asset model it is based on.
	FOREIGN KEY (based_on_id, asset_model_id) REFERENCES procedure_template_asset_models(id, asset_model_id),
	-- The name of the procedure_id template asset model must be unique for a given procedure_id template
	-- (i.e., you cannot have two asset models with the same name for the same procedure_id template)
	UNIQUE (procedure_template_id, name),
	-- We create an index on (procedure_template_id, asset_model_id) to allow for foreign
	-- keys from the concrete procedures to check that the asset model is correctly aligned.
	UNIQUE (id, procedure_template_id),
	-- We create an index on (procedure_template_id, asset_model_id) to allow for foreign
	-- keys from the concrete procedures to check that the asset model is correctly aligned.
	UNIQUE (id, asset_model_id)
);