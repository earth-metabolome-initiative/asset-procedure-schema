CREATE TABLE weighing_device_models (
	id UUID PRIMARY KEY REFERENCES physical_asset_models (id) ON DELETE CASCADE
);
INSERT INTO ownable_tables (id) VALUES ('weighing_device_models') ON CONFLICT DO NOTHING;

CREATE TABLE weighing_devices (
	id UUID PRIMARY KEY REFERENCES physical_assets (id) ON DELETE CASCADE,
	-- The model of the weighing device.
	weighing_device_model_id UUID NOT NULL REFERENCES weighing_device_models (id),
	-- We enforce that the extended `physical_asset` has indeed the same `physical_asset_model`, making
	-- sure that the asset is a weighing device without the possibility of a mistake.
	FOREIGN KEY (id, weighing_device_model_id) REFERENCES assets (id, model_id)
);
INSERT INTO ownable_tables (id) VALUES ('weighing_devices') ON CONFLICT DO NOTHING;