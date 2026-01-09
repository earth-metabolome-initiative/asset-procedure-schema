CREATE TABLE IF NOT EXISTS freeze_dryer_models (
	id UUID PRIMARY KEY REFERENCES physical_asset_models (id) ON DELETE CASCADE
);
CREATE TABLE IF NOT EXISTS commercial_freeze_dryer_models (
	id UUID PRIMARY KEY,
	freeze_dryer_model_id UUID NOT NULL REFERENCES freeze_dryer_models(id) ON DELETE CASCADE,
	FOREIGN KEY (id) REFERENCES freeze_dryer_models(id) ON DELETE CASCADE,
	FOREIGN KEY (id) REFERENCES commercial_products(id) ON DELETE CASCADE,
	FOREIGN KEY (id, freeze_dryer_model_id) REFERENCES asset_models(id, parent_model_id)
);
CREATE TABLE IF NOT EXISTS commercial_freeze_dryer_lots (
	id UUID PRIMARY KEY,
	FOREIGN KEY (id) REFERENCES commercial_product_lots(id) ON DELETE CASCADE,
	FOREIGN KEY (id) REFERENCES freeze_dryer_models(id) ON DELETE CASCADE,
	commercial_freeze_dryer_model_id UUID NOT NULL REFERENCES commercial_freeze_dryer_models(id),
	FOREIGN KEY (id, commercial_freeze_dryer_model_id) REFERENCES asset_models(id, parent_model_id)
);
CREATE TABLE IF NOT EXISTS freeze_dryers (
	id UUID PRIMARY KEY REFERENCES physical_assets (id) ON DELETE CASCADE,
	commercial_freeze_dryer_lot_id UUID NOT NULL REFERENCES commercial_freeze_dryer_lots (id),
	FOREIGN KEY (id, commercial_freeze_dryer_lot_id) REFERENCES assets (id, model_id)
);