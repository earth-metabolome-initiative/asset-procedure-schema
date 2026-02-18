CREATE TABLE volume_measuring_device_models (
	id UUID PRIMARY KEY REFERENCES physical_asset_models (id) ON DELETE CASCADE
);
INSERT INTO table_names (id) VALUES ('volume_measuring_device_models') ON CONFLICT DO NOTHING;

CREATE TABLE commercial_volume_measuring_device_models (
	id UUID PRIMARY KEY,
	volume_measuring_device_model_id UUID NOT NULL REFERENCES volume_measuring_device_models(id) ON DELETE CASCADE,
	FOREIGN KEY (id) REFERENCES volume_measuring_device_models(id) ON DELETE CASCADE,
	FOREIGN KEY (id) REFERENCES commercial_products(id) ON DELETE CASCADE,
	FOREIGN KEY (id, volume_measuring_device_model_id) REFERENCES asset_models(id, parent_model_id)
);
INSERT INTO table_names (id) VALUES ('commercial_volume_measuring_device_models') ON CONFLICT DO NOTHING;

CREATE TABLE commercial_volume_measuring_device_lots (
	id UUID PRIMARY KEY,
	FOREIGN KEY (id) REFERENCES commercial_product_lots(id) ON DELETE CASCADE,
	FOREIGN KEY (id) REFERENCES volume_measuring_device_models(id) ON DELETE CASCADE,
	commercial_volume_measuring_device_model_id UUID NOT NULL REFERENCES commercial_volume_measuring_device_models(id),
	FOREIGN KEY (id, commercial_volume_measuring_device_model_id) REFERENCES asset_models(id, parent_model_id)
);
INSERT INTO table_names (id) VALUES ('commercial_volume_measuring_device_lots') ON CONFLICT DO NOTHING;

CREATE TABLE volume_measuring_devices (
	id UUID PRIMARY KEY REFERENCES physical_assets (id) ON DELETE CASCADE,
	-- The model of the aliquoting device.
	volume_measuring_device_model_id UUID NOT NULL REFERENCES volume_measuring_device_models (id),
	-- We enforce that the extended `physical_asset` has indeed the same `physical_asset_model`, making
	-- sure that the asset is a aliquoting device without the possibility of a mistake.
	FOREIGN KEY (id, volume_measuring_device_model_id) REFERENCES assets (id, model_id)
);
INSERT INTO table_names (id) VALUES ('volume_measuring_devices') ON CONFLICT DO NOTHING;
