CREATE TABLE physical_asset_models (
	id UUID PRIMARY KEY REFERENCES asset_models(id) ON DELETE CASCADE
);
INSERT INTO ownable_tables (id) VALUES ('physical_asset_models') ON CONFLICT DO NOTHING;