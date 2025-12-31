CREATE TABLE IF NOT EXISTS centrifuge_models (
	id UUID PRIMARY KEY REFERENCES physical_asset_models (id) ON DELETE CASCADE
);
CREATE TABLE IF NOT EXISTS commercial_centrifuge_models (
	id UUID PRIMARY KEY,
	centrifuge_model_id UUID NOT NULL REFERENCES centrifuge_models(id) ON DELETE CASCADE,
	FOREIGN KEY (id) REFERENCES centrifuge_models(id) ON DELETE CASCADE,
	FOREIGN KEY (id) REFERENCES commercial_products(id) ON DELETE CASCADE,
	FOREIGN KEY (id, centrifuge_model_id) REFERENCES asset_models(id, parent_model_id)
);
CREATE TABLE IF NOT EXISTS commercial_centrifuge_lots (
	id UUID PRIMARY KEY,
	FOREIGN KEY (id) REFERENCES commercial_product_lots(id) ON DELETE CASCADE,
	FOREIGN KEY (id) REFERENCES centrifuge_models(id) ON DELETE CASCADE,
	commercial_centrifuge_model_id UUID NOT NULL REFERENCES commercial_centrifuge_models(id),
	FOREIGN KEY (id, commercial_centrifuge_model_id) REFERENCES asset_models(id, parent_model_id)
);
CREATE TABLE IF NOT EXISTS centrifuges (
	id UUID PRIMARY KEY REFERENCES physical_assets (id) ON DELETE CASCADE,
	commercial_centrifuge_lot_id UUID NOT NULL REFERENCES commercial_centrifuge_lots (id),
	FOREIGN KEY (id, commercial_centrifuge_lot_id) REFERENCES assets (id, model_id)
);