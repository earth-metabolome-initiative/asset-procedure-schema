CREATE TABLE IF NOT EXISTS assets (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    most_concrete_table TEXT NOT NULL,
    name VARCHAR(255) CHECK (name <> ''),
    description TEXT CHECK (description <> ''),
    model_id UUID NOT NULL REFERENCES asset_models(id) ON DELETE CASCADE,
    creator_id UUID NOT NULL REFERENCES users(id),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    editor_id UUID NOT NULL REFERENCES users(id),
    edited_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CHECK (name <> description),
    CHECK (created_at <= edited_at),
    UNIQUE (id, model_id),
    -- Assets of different models can have the same name, but not assets of the same model.
    UNIQUE (name, model_id)
);
