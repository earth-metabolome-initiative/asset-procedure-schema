CREATE TABLE physical_asset_models (
	id UUID PRIMARY KEY REFERENCES asset_models(id) ON DELETE CASCADE,
	lifecycle_class_id TEXT NOT NULL DEFAULT 'unknown' CHECK (
		lifecycle_class_id <> ''
		AND length(lifecycle_class_id) < 255
	),
	recommended_max_use SMALLINT CHECK (recommended_max_use > 0)
);
INSERT INTO table_names (id) VALUES ('physical_asset_models') ON CONFLICT DO NOTHING;
