CREATE TABLE geopositioning_device_models (
	id UUID PRIMARY KEY REFERENCES physical_asset_models (id) ON DELETE CASCADE
);
INSERT INTO ownable_tables (id) VALUES ('geopositioning_device_models') ON CONFLICT DO NOTHING;
CREATE TABLE commercial_geopositioning_device_models (
	id UUID PRIMARY KEY,
	geopositioning_device_model_id UUID NOT NULL REFERENCES geopositioning_device_models(id) ON DELETE CASCADE,
	FOREIGN KEY (id) REFERENCES geopositioning_device_models(id) ON DELETE CASCADE,
	FOREIGN KEY (id) REFERENCES commercial_products(id) ON DELETE CASCADE,
	FOREIGN KEY (id, geopositioning_device_model_id) REFERENCES asset_models(id, parent_model_id)
);
CREATE TABLE commercial_geopositioning_device_lots (
	id UUID PRIMARY KEY,
	FOREIGN KEY (id) REFERENCES commercial_product_lots(id) ON DELETE CASCADE,
	FOREIGN KEY (id) REFERENCES geopositioning_device_models(id) ON DELETE CASCADE,
	commercial_geopositioning_device_model_id UUID NOT NULL REFERENCES commercial_geopositioning_device_models(id),
	FOREIGN KEY (id, commercial_geopositioning_device_model_id) REFERENCES asset_models(id, parent_model_id)
);
CREATE TABLE geopositioning_devices (
	id UUID PRIMARY KEY REFERENCES physical_assets (id) ON DELETE CASCADE,
	commercial_geopositioning_device_lot_id UUID NOT NULL REFERENCES commercial_geopositioning_device_lots (id),
	FOREIGN KEY (id, commercial_geopositioning_device_lot_id) REFERENCES assets (id, model_id)
);