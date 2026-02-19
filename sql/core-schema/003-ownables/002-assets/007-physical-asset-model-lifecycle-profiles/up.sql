-- Catalog of allowed lifecycle classes for physical asset models.
CREATE TABLE physical_asset_model_lifecycle_classes (
	-- Stable lifecycle class identifier.
	id TEXT PRIMARY KEY CHECK (
		id <> ''
		AND length(id) < 255
	)
);
INSERT INTO table_names (id)
VALUES ('physical_asset_model_lifecycle_classes') ON CONFLICT DO NOTHING;

-- Seeds canonical lifecycle classes.
INSERT INTO physical_asset_model_lifecycle_classes (id)
VALUES ('unknown'), ('single_use'), ('reusable') ON CONFLICT DO NOTHING;

-- Adds lifecycle class referential integrity on physical_asset_models.
ALTER TABLE physical_asset_models
	ADD CONSTRAINT physical_asset_models_lifecycle_class_id_fkey
	FOREIGN KEY (lifecycle_class_id) REFERENCES physical_asset_model_lifecycle_classes(id);

-- Enforces reusable/non-reusable consistency directly on physical_asset_models.
ALTER TABLE physical_asset_models
	ADD CONSTRAINT physical_asset_models_recommended_max_use_requires_reusable_check
	CHECK (lifecycle_class_id = 'reusable' OR recommended_max_use IS NULL);
