CREATE TABLE camera_models (
    id UUID PRIMARY KEY REFERENCES physical_asset_models(id) ON DELETE CASCADE
);
INSERT INTO table_names (id) VALUES ('camera_models') ON CONFLICT DO NOTHING;
CREATE TABLE commercial_camera_models (
	id UUID PRIMARY KEY,
	camera_model_id UUID NOT NULL REFERENCES camera_models(id) ON DELETE CASCADE,
	FOREIGN KEY (id) REFERENCES camera_models(id) ON DELETE CASCADE,
	FOREIGN KEY (id) REFERENCES commercial_products(id) ON DELETE CASCADE,
	FOREIGN KEY (id, camera_model_id) REFERENCES asset_models(id, parent_model_id)
);
INSERT INTO table_names (id) VALUES ('commercial_camera_models') ON CONFLICT DO NOTHING;
CREATE TABLE commercial_camera_lots (
	id UUID PRIMARY KEY,
	FOREIGN KEY (id) REFERENCES commercial_product_lots(id) ON DELETE CASCADE,
	FOREIGN KEY (id) REFERENCES camera_models(id) ON DELETE CASCADE,
	commercial_camera_model_id UUID NOT NULL REFERENCES commercial_camera_models(id),
	FOREIGN KEY (id, commercial_camera_model_id) REFERENCES asset_models(id, parent_model_id)
);
INSERT INTO table_names (id) VALUES ('commercial_camera_lots') ON CONFLICT DO NOTHING;
CREATE TABLE cameras (
	id UUID PRIMARY KEY REFERENCES physical_assets (id) ON DELETE CASCADE,
	commercial_camera_lot_id UUID NOT NULL REFERENCES commercial_camera_lots (id),
	FOREIGN KEY (id, commercial_camera_lot_id) REFERENCES assets (id, model_id)
);
INSERT INTO table_names (id) VALUES ('cameras') ON CONFLICT DO NOTHING;
