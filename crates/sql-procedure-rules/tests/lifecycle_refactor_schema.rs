//! Tests for the hard-switch lifecycle/reusability refactor.

#[cfg(test)]
mod tests {
    use std::{fs, path::Path};

    use sql_traits::prelude::*;
    use sqlparser::dialect::PostgreSqlDialect;

    #[test]
    fn lifecycle_columns_live_on_physical_asset_models() {
        let db = ParserDB::from_path::<PostgreSqlDialect>(Path::new("../../"))
            .expect("Failed to parse database schema");

        let physical_asset_models = db
            .table(None, "physical_asset_models")
            .expect("physical_asset_models table should exist");
        assert!(
            physical_asset_models.column("lifecycle_class_id", &db).is_some(),
            "physical_asset_models must expose lifecycle_class_id",
        );
        assert!(
            physical_asset_models.column("recommended_max_use", &db).is_some(),
            "physical_asset_models must expose recommended_max_use",
        );

        assert!(
            db.table(None, "physical_asset_model_lifecycle_classes").is_some(),
            "physical_asset_model_lifecycle_classes must still exist",
        );
        assert!(
            db.table(None, "physical_asset_model_lifecycle_profiles").is_none(),
            "physical_asset_model_lifecycle_profiles must be removed from final schema",
        );
    }

    #[test]
    fn lifecycle_migration_contains_backfill_and_drop_steps() {
        let migration_path = Path::new(
            "../../sql/core-schema/003-ownables/002-assets/007-physical-asset-model-lifecycle-profiles/up.sql",
        );
        let sql = fs::read_to_string(migration_path)
            .expect("Failed to read lifecycle hard-switch migration");

        assert!(
            sql.contains("ALTER TABLE physical_asset_models"),
            "Expected ALTER TABLE for physical_asset_models lifecycle columns",
        );
        assert!(
            sql.contains("UPDATE physical_asset_models AS pam"),
            "Expected lifecycle backfill update from legacy profile table",
        );
        assert!(
            sql.contains("DROP TABLE physical_asset_model_lifecycle_profiles"),
            "Expected legacy lifecycle profile table drop",
        );
        assert!(
            sql.contains("physical_asset_models_recommended_max_use_requires_reusable_check"),
            "Expected physical_asset_models lifecycle enforcement constraint",
        );
        assert!(
            sql.contains("CHECK (lifecycle_class_id = 'reusable' OR recommended_max_use IS NULL)"),
            "Expected non-reusable models to enforce NULL recommended_max_use",
        );
    }
}
