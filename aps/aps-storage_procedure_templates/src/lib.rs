//! Auto-generated crate for the `storage_procedure_templates` table.
#[derive(
    Copy,
    Clone,
    Debug,
    PartialOrd,
    PartialEq,
    serde :: Serialize,
    serde :: Deserialize,
    diesel :: Queryable,
    diesel :: Selectable,
    diesel :: Identifiable,
    diesel_builders :: prelude :: TableModel,
)]
/// Struct representing a row in the `storage_procedure_templates` table.
#[table_model(ancestors(aps_procedure_templates::procedure_templates))]
# [table_model (error = :: validation_errors :: ValidationError)]
#[table_model(default(
    aps_procedure_templates::procedure_templates::procedure_template_table_id,
    "storage_procedure_templates"
))]
# [diesel (table_name = storage_procedure_templates)]
pub struct StorageProcedureTemplate {
    /// Field representing the `id` column in table
    /// `storage_procedure_templates`.
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    id: ::rosetta_uuid::Uuid,
    /// Field representing the `kelvin` column in table
    /// `storage_procedure_templates`.
    #[table_model(default = 293.15f32)]
    kelvin: f32,
    /// Field representing the `kelvin_tolerance_percentage` column in table
    /// `storage_procedure_templates`.
    #[table_model(default = 1f32)]
    kelvin_tolerance_percentage: f32,
    /// Field representing the `stored_into_model_id` column in table
    /// `storage_procedure_templates`.
    #[same_as(
        aps_procedure_template_asset_models::procedure_template_asset_models::asset_model_id,
        procedure_template_stored_into_model_id
    )]
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    stored_into_model_id: ::rosetta_uuid::Uuid,
    /// Field representing the `procedure_template_stored_into_model_id` column
    /// in table `storage_procedure_templates`.
    #[discretionary(aps_procedure_template_asset_models::procedure_template_asset_models)]
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_template_stored_into_model_id: ::rosetta_uuid::Uuid,
    /// Field representing the `stored_asset_model_id` column in table
    /// `storage_procedure_templates`.
    #[same_as(
        aps_procedure_template_asset_models::procedure_template_asset_models::asset_model_id,
        procedure_template_stored_asset_model_id
    )]
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    stored_asset_model_id: ::rosetta_uuid::Uuid,
    /// Field representing the `procedure_template_stored_asset_model_id` column
    /// in table `storage_procedure_templates`.
    #[discretionary(aps_procedure_template_asset_models::procedure_template_asset_models)]
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_template_stored_asset_model_id: ::rosetta_uuid::Uuid,
}
::diesel_builders::prelude::unique_index!(
    storage_procedure_templates::id,
    storage_procedure_templates::procedure_template_stored_into_model_id
);
::diesel_builders::prelude::unique_index!(
    storage_procedure_templates::id,
    storage_procedure_templates::procedure_template_stored_asset_model_id
);
:: diesel_builders :: prelude :: fk ! ((storage_procedure_templates :: id) -> (:: aps_procedure_templates :: procedure_templates :: id));
:: diesel_builders :: prelude :: fk ! ((storage_procedure_templates :: stored_into_model_id) -> (:: aps_container_models :: container_models :: id));
:: diesel_builders :: prelude :: fk ! ((storage_procedure_templates :: stored_asset_model_id) -> (:: aps_physical_asset_models :: physical_asset_models :: id));
:: diesel_builders :: prelude :: fk ! ((storage_procedure_templates :: stored_into_model_id , storage_procedure_templates :: stored_asset_model_id) -> (:: aps_asset_compatibility_rules :: asset_compatibility_rules :: left_asset_model_id , :: aps_asset_compatibility_rules :: asset_compatibility_rules :: right_asset_model_id));
impl ::diesel_builders::ValidateColumn<storage_procedure_templates::kelvin>
    for <storage_procedure_templates::table as ::diesel_builders::TableExt>::NewValues
{
    type Error = ::validation_errors::ValidationError<&'static str>;
    #[inline]
    fn validate_column(kelvin: &f32) -> Result<(), Self::Error> {
        use diesel::Column;
        if kelvin <= &0f32 {
            return Err(validation_errors::prelude::ValidationError::strictly_greater_than_value(
                crate::storage_procedure_templates::kelvin::NAME,
                0f64,
            ));
        }
        Ok(())
    }
}
impl ::diesel_builders::ValidateColumn<storage_procedure_templates::kelvin_tolerance_percentage>
    for <storage_procedure_templates::table as ::diesel_builders::TableExt>::NewValues
{
    type Error = ::validation_errors::ValidationError<&'static str>;
    #[inline]
    fn validate_column(kelvin_tolerance_percentage: &f32) -> Result<(), Self::Error> {
        use diesel::Column;
        if kelvin_tolerance_percentage <= &0f32 {
            return Err(validation_errors::prelude::ValidationError::strictly_greater_than_value(
                crate::storage_procedure_templates::kelvin_tolerance_percentage::NAME,
                0f64,
            ));
        }
        if kelvin_tolerance_percentage > &100f32 {
            return Err(validation_errors::prelude::ValidationError::smaller_than_value(
                crate::storage_procedure_templates::kelvin_tolerance_percentage::NAME,
                100f64,
            ));
        }
        Ok(())
    }
}
