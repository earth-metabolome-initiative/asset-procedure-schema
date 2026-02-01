CREATE TABLE physical_assets (
    id UUID PRIMARY KEY REFERENCES assets(id) ON DELETE CASCADE,
    physical_asset_model_id UUID NOT NULL REFERENCES physical_asset_models(id) ON DELETE CASCADE,
    FOREIGN KEY (id, physical_asset_model_id) REFERENCES assets(id, model_id)
);
INSERT INTO table_names (id) VALUES ('physical_assets') ON CONFLICT DO NOTHING;