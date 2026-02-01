CREATE TABLE asset_models (
	id UUID PRIMARY KEY REFERENCES namespaced_ownables(id) ON DELETE CASCADE,
	parent_model_id UUID REFERENCES asset_models(id) ON DELETE CASCADE,
	CHECK (id <> parent_model_id),
	UNIQUE (id, parent_model_id)
);
INSERT INTO table_names (id)
VALUES ('asset_models') ON CONFLICT DO NOTHING;
CREATE TABLE asset_model_ancestors (
	descendant_model_id UUID NOT NULL REFERENCES asset_models(id) ON DELETE CASCADE,
	ancestor_model_id UUID NOT NULL REFERENCES asset_models(id) ON DELETE CASCADE,
	PRIMARY KEY (descendant_model_id, ancestor_model_id)
);
CREATE TABLE asset_compatibility_rules (
	id UUID PRIMARY KEY REFERENCES ownables(id) ON DELETE CASCADE,
	left_asset_model_id UUID NOT NULL REFERENCES asset_models(id) ON DELETE CASCADE,
	right_asset_model_id UUID NOT NULL REFERENCES asset_models(id) ON DELETE CASCADE,
	UNIQUE(left_asset_model_id, right_asset_model_id),
	CHECK (left_asset_model_id <> right_asset_model_id)
);
INSERT INTO table_names (id)
VALUES ('asset_compatibility_rules') ON CONFLICT DO NOTHING;
CREATE UNIQUE INDEX unique_asset_compatibility_pair ON asset_compatibility_rules (
	LEAST(left_asset_model_id, right_asset_model_id),
	GREATEST(left_asset_model_id, right_asset_model_id)
);