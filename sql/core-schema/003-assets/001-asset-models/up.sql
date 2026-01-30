-- Meta-table with the unique names of asset model tables, to be referenced by asset models
-- and facilitate DAG traversal.
CREATE TABLE asset_model_tables (id TEXT PRIMARY KEY CHECK (id <> ''));
CREATE TABLE asset_models (
	id UUID PRIMARY KEY DEFAULT uuidv7(),
	asset_model_table_id TEXT DEFAULT 'asset_models' NOT NULL REFERENCES asset_model_tables(id),
	name VARCHAR(255) NOT NULL UNIQUE CHECK (name <> ''),
	description TEXT NOT NULL CHECK (description <> ''),
	parent_model_id UUID REFERENCES asset_models(id) ON DELETE CASCADE,
	creator_id UUID NOT NULL REFERENCES users(id),
	created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
	editor_id UUID NOT NULL REFERENCES users(id),
	edited_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
	CHECK (name <> description),
	CHECK (id <> parent_model_id),
	CHECK (created_at <= edited_at),
	UNIQUE (id, parent_model_id)
);
INSERT INTO asset_model_tables (id) VALUES ('asset_models') ON CONFLICT DO NOTHING;

CREATE OR REPLACE FUNCTION update_asset_models_edited_at() RETURNS TRIGGER AS $$
BEGIN
    NEW.edited_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trigger_update_asset_models_edited_at
BEFORE UPDATE ON asset_models
FOR EACH ROW EXECUTE FUNCTION update_asset_models_edited_at();

CREATE TABLE asset_model_ancestors (
	descendant_model_id UUID NOT NULL REFERENCES asset_models(id) ON DELETE CASCADE,
	ancestor_model_id UUID NOT NULL REFERENCES asset_models(id) ON DELETE CASCADE,
	PRIMARY KEY (descendant_model_id, ancestor_model_id)
);
CREATE TABLE asset_compatibility_rules (
	left_asset_model_id UUID NOT NULL REFERENCES asset_models(id) ON DELETE CASCADE,
	right_asset_model_id UUID NOT NULL REFERENCES asset_models(id) ON DELETE CASCADE,
	creator_id UUID NOT NULL REFERENCES users(id),
	created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
	PRIMARY KEY (left_asset_model_id, right_asset_model_id),
	CHECK (left_asset_model_id <> right_asset_model_id)
);
CREATE UNIQUE INDEX unique_asset_compatibility_pair ON asset_compatibility_rules (
	LEAST(left_asset_model_id, right_asset_model_id),
	GREATEST(left_asset_model_id, right_asset_model_id)
);