CREATE TABLE assets (
    id UUID PRIMARY KEY REFERENCES namespaced_ownables(id) ON DELETE CASCADE,
    model_id UUID NOT NULL REFERENCES asset_models(id) ON DELETE CASCADE,
    UNIQUE (id, model_id)
);
INSERT INTO table_names (id) VALUES ('assets') ON CONFLICT DO NOTHING;
