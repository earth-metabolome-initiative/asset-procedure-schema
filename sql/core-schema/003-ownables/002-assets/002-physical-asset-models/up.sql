CREATE TABLE physical_asset_models (
	id UUID PRIMARY KEY REFERENCES asset_models(id) ON DELETE CASCADE
);
INSERT INTO table_names (id) VALUES ('physical_asset_models') ON CONFLICT DO NOTHING;