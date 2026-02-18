CREATE TABLE weighing_device_models (
	id UUID PRIMARY KEY REFERENCES physical_asset_models (id) ON DELETE CASCADE
);
INSERT INTO table_names (id) VALUES ('weighing_device_models') ON CONFLICT DO NOTHING;

CREATE TABLE commercial_weighing_device_models (
	id UUID PRIMARY KEY,
	weighing_device_model_id UUID NOT NULL REFERENCES weighing_device_models(id) ON DELETE CASCADE,
	FOREIGN KEY (id) REFERENCES weighing_device_models(id) ON DELETE CASCADE,
	FOREIGN KEY (id) REFERENCES commercial_products(id) ON DELETE CASCADE,
	FOREIGN KEY (id, weighing_device_model_id) REFERENCES asset_models(id, parent_model_id)
);
INSERT INTO table_names (id) VALUES ('commercial_weighing_device_models') ON CONFLICT DO NOTHING;

CREATE TABLE commercial_weighing_device_lots (
	id UUID PRIMARY KEY,
	FOREIGN KEY (id) REFERENCES commercial_product_lots(id) ON DELETE CASCADE,
	FOREIGN KEY (id) REFERENCES weighing_device_models(id) ON DELETE CASCADE,
	commercial_weighing_device_model_id UUID NOT NULL REFERENCES commercial_weighing_device_models(id),
	FOREIGN KEY (id, commercial_weighing_device_model_id) REFERENCES asset_models(id, parent_model_id)
);
INSERT INTO table_names (id) VALUES ('commercial_weighing_device_lots') ON CONFLICT DO NOTHING;

CREATE TABLE weighing_devices (
	id UUID PRIMARY KEY REFERENCES physical_assets (id) ON DELETE CASCADE,
	-- The model of the weighing device.
	weighing_device_model_id UUID NOT NULL REFERENCES weighing_device_models (id),
	-- We enforce that the extended `physical_asset` has indeed the same `physical_asset_model`, making
	-- sure that the asset is a weighing device without the possibility of a mistake.
	FOREIGN KEY (id, weighing_device_model_id) REFERENCES assets (id, model_id)
);
INSERT INTO table_names (id) VALUES ('weighing_devices') ON CONFLICT DO NOTHING;
