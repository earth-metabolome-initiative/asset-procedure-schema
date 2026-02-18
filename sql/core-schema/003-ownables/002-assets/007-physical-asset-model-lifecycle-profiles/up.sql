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

-- Lifecycle metadata for physical asset models.
CREATE TABLE physical_asset_model_lifecycle_profiles (
	-- Physical asset model that this lifecycle profile describes.
	id UUID PRIMARY KEY REFERENCES physical_asset_models(id) ON DELETE CASCADE,
	-- Lifecycle class identifier independent of taxonomy folders.
	lifecycle_class_id TEXT NOT NULL CHECK (
		lifecycle_class_id <> ''
		AND length(lifecycle_class_id) < 255
	) REFERENCES physical_asset_model_lifecycle_classes(id),
	-- Suggested maximum number of uses for reusable models.
	recommended_max_use SMALLINT CHECK (recommended_max_use > 0)
);
INSERT INTO table_names (id)
VALUES ('physical_asset_model_lifecycle_profiles') ON CONFLICT DO NOTHING;

-- Validates lifecycle/recommended_max_use consistency.
CREATE OR REPLACE FUNCTION physical_asset_model_lifecycle_profiles_validate_recommended_max_use_fn() RETURNS TRIGGER AS $$
BEGIN
	IF NEW.lifecycle_class_id <> 'reusable' AND NEW.recommended_max_use IS NOT NULL THEN
		RAISE EXCEPTION 'recommended_max_use must be NULL unless lifecycle_class_id is reusable';
	END IF;
	RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Enforces lifecycle/recommended_max_use consistency on writes.
CREATE TRIGGER physical_asset_model_lifecycle_profiles_validate_recommended_max_use_trigger
BEFORE INSERT OR UPDATE ON physical_asset_model_lifecycle_profiles
FOR EACH ROW EXECUTE FUNCTION physical_asset_model_lifecycle_profiles_validate_recommended_max_use_fn();
