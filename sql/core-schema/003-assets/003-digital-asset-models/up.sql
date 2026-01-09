CREATE TABLE digital_asset_models (
    id UUID PRIMARY KEY REFERENCES asset_models(id) ON DELETE CASCADE,
    mime_type TEXT NOT NULL
);
INSERT INTO asset_model_tables (id) VALUES ('digital_asset_models') ON CONFLICT DO NOTHING;