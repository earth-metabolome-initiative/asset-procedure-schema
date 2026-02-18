CREATE TABLE packaging_models (
    id UUID PRIMARY KEY REFERENCES physical_asset_models(id) ON DELETE CASCADE
);
INSERT INTO table_names (id) VALUES ('packaging_models') ON CONFLICT DO NOTHING;