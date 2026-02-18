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

-- Legacy hard-switch SQL reference for historical DB upgrades only:
-- UPDATE physical_asset_models AS pam
-- SET
-- 	lifecycle_class_id = pamlp.lifecycle_class_id,
-- 	recommended_max_use = pamlp.recommended_max_use
-- FROM physical_asset_model_lifecycle_profiles AS pamlp
-- WHERE pam.id = pamlp.id;
--
-- DROP TRIGGER IF EXISTS physical_asset_model_lifecycle_profiles_validate_recommended_max_use_trigger
-- ON physical_asset_model_lifecycle_profiles;
-- DROP FUNCTION IF EXISTS physical_asset_model_lifecycle_profiles_validate_recommended_max_use_fn();
-- DROP TABLE physical_asset_model_lifecycle_profiles;

-- Removes stale table_names registration for the dropped lifecycle profile table.
DELETE FROM table_names WHERE id = 'physical_asset_model_lifecycle_profiles';

-- Enforces reusable/non-reusable consistency directly on physical_asset_models.
ALTER TABLE physical_asset_models
	ADD CONSTRAINT physical_asset_models_recommended_max_use_requires_reusable_check
	CHECK (lifecycle_class_id = 'reusable' OR recommended_max_use IS NULL);
