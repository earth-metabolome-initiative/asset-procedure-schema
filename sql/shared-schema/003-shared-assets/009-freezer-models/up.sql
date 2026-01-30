CREATE TABLE freezer_models (
	id UUID PRIMARY KEY REFERENCES physical_asset_models (id) ON DELETE CASCADE
);
INSERT INTO ownable_tables (id) VALUES ('freezer_models') ON CONFLICT DO NOTHING;
CREATE TABLE commercial_freezer_models (
	id UUID PRIMARY KEY,
	freezer_model_id UUID NOT NULL REFERENCES freezer_models(id) ON DELETE CASCADE,
	FOREIGN KEY (id) REFERENCES freezer_models(id) ON DELETE CASCADE,
	FOREIGN KEY (id) REFERENCES commercial_products(id) ON DELETE CASCADE,
	FOREIGN KEY (id, freezer_model_id) REFERENCES asset_models(id, parent_model_id)
);
INSERT INTO ownable_tables (id) VALUES ('commercial_freezer_models') ON CONFLICT DO NOTHING;
CREATE TABLE commercial_freezer_lots (
	id UUID PRIMARY KEY,
	FOREIGN KEY (id) REFERENCES commercial_product_lots(id) ON DELETE CASCADE,
	FOREIGN KEY (id) REFERENCES freezer_models(id) ON DELETE CASCADE,
	commercial_freezer_model_id UUID NOT NULL REFERENCES commercial_freezer_models(id),
	FOREIGN KEY (id, commercial_freezer_model_id) REFERENCES asset_models(id, parent_model_id)
);
INSERT INTO ownable_tables (id) VALUES ('commercial_freezer_lots') ON CONFLICT DO NOTHING;
CREATE TABLE freezers (
	id UUID PRIMARY KEY REFERENCES physical_assets (id) ON DELETE CASCADE,
	commercial_freezer_lot_id UUID NOT NULL REFERENCES commercial_freezer_lots (id),
	FOREIGN KEY (id, commercial_freezer_lot_id) REFERENCES assets (id, model_id)
);
INSERT INTO ownable_tables (id) VALUES ('freezers') ON CONFLICT DO NOTHING;