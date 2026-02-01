CREATE TABLE digital_asset_models (
    id UUID PRIMARY KEY REFERENCES asset_models(id) ON DELETE CASCADE,
    mime_type TEXT NOT NULL CHECK (mime_type <> '' AND length(mime_type) <= 255)
);
INSERT INTO table_names (id) VALUES ('digital_asset_models') ON CONFLICT DO NOTHING;