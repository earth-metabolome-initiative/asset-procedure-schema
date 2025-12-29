CREATE TABLE IF NOT EXISTS asset_models (
	id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
	most_concrete_table TEXT NOT NULL,
	name VARCHAR(255) NOT NULL UNIQUE CHECK (name <> ''),
	description TEXT NOT NULL CHECK (description <> ''),
	parent_model_id UUID REFERENCES asset_models(id) ON DELETE CASCADE,
	created_by_id UUID NOT NULL REFERENCES users(id),
	created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
	updated_by_id UUID NOT NULL REFERENCES users(id),
	updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
	CHECK (name <> description),
	CHECK (id <> parent_model_id),
	CHECK (created_at <= updated_at),
	UNIQUE (id, parent_model_id)
);

CREATE TABLE IF NOT EXISTS asset_model_ancestors (
    descendant_model_id UUID NOT NULL REFERENCES asset_models(id) ON DELETE CASCADE,
    ancestor_model_id UUID NOT NULL REFERENCES asset_models(id) ON DELETE CASCADE,
    PRIMARY KEY (descendant_model_id, ancestor_model_id)
);

CREATE TABLE IF NOT EXISTS asset_compatibility_rules (
    left_asset_model_id UUID NOT NULL REFERENCES asset_models(id) ON DELETE CASCADE,
    right_asset_model_id UUID NOT NULL REFERENCES asset_models(id) ON DELETE CASCADE,
    created_by_id UUID NOT NULL REFERENCES users(id),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (left_asset_model_id, right_asset_model_id),
    CHECK (
        left_asset_model_id <> right_asset_model_id
    )
);

CREATE UNIQUE INDEX unique_asset_compatibility_pair ON asset_compatibility_rules (
    LEAST(left_asset_model_id, right_asset_model_id),
    GREATEST(left_asset_model_id, right_asset_model_id)
);