CREATE TABLE IF NOT EXISTS digital_asset_models (
    id UUID PRIMARY KEY REFERENCES asset_models(id) ON DELETE CASCADE,
    mime_type TEXT NOT NULL
);