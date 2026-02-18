CREATE TABLE volume_measuring_device_models (
	id UUID PRIMARY KEY REFERENCES physical_asset_models (id) ON DELETE CASCADE
);
INSERT INTO table_names (id) VALUES ('volume_measuring_device_models') ON CONFLICT DO NOTHING;

CREATE TABLE volume_measuring_devices (
	id UUID PRIMARY KEY REFERENCES physical_assets (id) ON DELETE CASCADE,
	-- The model of the aliquoting device.
	volume_measuring_device_model_id UUID NOT NULL REFERENCES volume_measuring_device_models (id),
	-- We enforce that the extended `physical_asset` has indeed the same `physical_asset_model`, making
	-- sure that the asset is a aliquoting device without the possibility of a mistake.
	FOREIGN KEY (id, volume_measuring_device_model_id) REFERENCES assets (id, model_id)
);
INSERT INTO table_names (id) VALUES ('volume_measuring_devices') ON CONFLICT DO NOTHING;