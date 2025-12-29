CREATE TABLE IF NOT EXISTS assets (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    most_concrete_table TEXT NOT NULL,
    name VARCHAR(255) CHECK (must_be_paragraph(name)),
    description TEXT CHECK (must_be_paragraph(description)),
    model_id UUID NOT NULL REFERENCES asset_models(id) ON DELETE CASCADE,
    created_by_id UUID NOT NULL REFERENCES users(id),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_by_id UUID NOT NULL REFERENCES users(id),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CHECK (name <> description),
    CHECK (created_at <= updated_at),
    UNIQUE (id, model_id),
    -- Assets of different models can have the same name, but not assets of the same model.
    UNIQUE (name, model_id)
);
CREATE TABLE IF NOT EXISTS physical_assets (
    id UUID PRIMARY KEY REFERENCES assets(id) ON DELETE CASCADE,
    physical_asset_model_id INTEGER NOT NULL REFERENCES physical_asset_models(id) ON DELETE CASCADE,
    FOREIGN KEY (id, physical_asset_model_id) REFERENCES assets(id, model_id)
);