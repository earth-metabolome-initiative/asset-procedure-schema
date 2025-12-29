CREATE TABLE IF NOT EXISTS asset_models (
	id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
	most_concrete_table TEXT NOT NULL,
	name VARCHAR(255) NOT NULL UNIQUE CHECK (must_be_paragraph(name)),
	description TEXT NOT NULL CHECK (must_be_paragraph(description)),
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
