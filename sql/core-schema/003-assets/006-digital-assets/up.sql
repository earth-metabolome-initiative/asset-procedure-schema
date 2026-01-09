CREATE TABLE digital_assets (
    id UUID PRIMARY KEY REFERENCES assets(id) ON DELETE CASCADE,
    digital_asset_model_id UUID NOT NULL REFERENCES digital_asset_models(id) ON DELETE CASCADE,
    FOREIGN KEY (id, digital_asset_model_id) REFERENCES assets(id, model_id)
);
INSERT INTO asset_tables (id) VALUES ('digital_assets') ON CONFLICT DO NOTHING;