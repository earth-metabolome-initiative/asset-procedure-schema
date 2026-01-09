CREATE TABLE IF NOT EXISTS packaging_models (
    id UUID PRIMARY KEY REFERENCES physical_asset_models(id) ON DELETE CASCADE
);