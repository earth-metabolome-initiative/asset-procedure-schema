CREATE TABLE assets (
    id UUID PRIMARY KEY DEFAULT uuidv7() REFERENCES ownables(id) ON DELETE CASCADE,
    name TEXT CHECK (name <> '' AND length(name) <= 255),
    description TEXT CHECK (description <> '' AND length(description) < 8192),
    model_id UUID NOT NULL REFERENCES asset_models(id) ON DELETE CASCADE,
    CHECK (name <> description),
    UNIQUE (id, model_id)
);
INSERT INTO ownable_tables (id) VALUES ('assets') ON CONFLICT DO NOTHING;
