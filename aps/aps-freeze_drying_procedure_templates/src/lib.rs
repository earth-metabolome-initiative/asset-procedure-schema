//! Auto-generated crate for the `freeze_drying_procedure_templates` table.
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
/// Undocumented table
#[table_model(ancestors(aps_procedure_templates::procedure_templates))]
# [table_model (error = :: validation_errors :: ValidationError)]
# [diesel (table_name = freeze_drying_procedure_templates)]
pub struct FreezeDryingProcedureTemplate {
    /// Undocumented column
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    id: ::rosetta_uuid::Uuid,
    /// Undocumented column
    #[table_model(default = 203.15f32)]
    kelvin: f32,
    /// Undocumented column
    #[table_model(default = 5f32)]
    kelvin_tolerance_percentage: f32,
    /// Undocumented column
    #[table_model(default = 4f32)]
    pascal: f32,
    /// Undocumented column
    #[table_model(default = 259200f32)]
    duration: f32,
    /// Undocumented column
    #[same_as(
        aps_procedure_template_asset_models::procedure_template_asset_models::asset_model_id,
        procedure_template_freeze_dried_with_model_id
    )]
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    freeze_dried_with_model_id: ::rosetta_uuid::Uuid,
    /// Undocumented column
    #[discretionary(aps_procedure_template_asset_models::procedure_template_asset_models)]
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_template_freeze_dried_with_model_id: ::rosetta_uuid::Uuid,
    /// Undocumented column
    #[same_as(
        aps_procedure_template_asset_models::procedure_template_asset_models::asset_model_id,
        procedure_template_freeze_dried_container_model_id
    )]
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    freeze_dried_container_model_id: ::rosetta_uuid::Uuid,
    /// Undocumented column
    #[discretionary(aps_procedure_template_asset_models::procedure_template_asset_models)]
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_template_freeze_dried_container_model_id: ::rosetta_uuid::Uuid,
}
::diesel_builders::prelude::unique_index!(
    freeze_drying_procedure_templates::id,
    freeze_drying_procedure_templates::procedure_template_freeze_dried_with_model_id
);
::diesel_builders::prelude::unique_index!(
    freeze_drying_procedure_templates::id,
    freeze_drying_procedure_templates::procedure_template_freeze_dried_container_model_id
);
:: diesel_builders :: prelude :: fk ! ((freeze_drying_procedure_templates :: id) -> (:: aps_procedure_templates :: procedure_templates :: id));
:: diesel_builders :: prelude :: fk ! ((freeze_drying_procedure_templates :: freeze_dried_with_model_id) -> (:: aps_freeze_dryer_models :: freeze_dryer_models :: id));
:: diesel_builders :: prelude :: fk ! ((freeze_drying_procedure_templates :: freeze_dried_container_model_id) -> (:: aps_volumetric_container_models :: volumetric_container_models :: id));
:: diesel_builders :: prelude :: fk ! ((freeze_drying_procedure_templates :: freeze_dried_with_model_id , freeze_drying_procedure_templates :: freeze_dried_container_model_id) -> (:: aps_asset_compatibility_rules :: asset_compatibility_rules :: left_asset_model_id , :: aps_asset_compatibility_rules :: asset_compatibility_rules :: right_asset_model_id));
impl ::diesel_builders::ValidateColumn<freeze_drying_procedure_templates::kelvin>
    for <freeze_drying_procedure_templates::table as ::diesel_builders::TableExt>::NewValues
{
    type Error = ::validation_errors::ValidationError<&'static str>;
    #[inline]
    fn validate_column(kelvin: &f32) -> Result<(), Self::Error> {
        use diesel::Column;
        if kelvin <= &0f32 {
            return Err(validation_errors::prelude::ValidationError::strictly_greater_than_value(
                crate::freeze_drying_procedure_templates::kelvin::NAME,
                0f64,
            ));
        }
        Ok(())
    }
}
impl
    ::diesel_builders::ValidateColumn<
        freeze_drying_procedure_templates::kelvin_tolerance_percentage,
    > for <freeze_drying_procedure_templates::table as ::diesel_builders::TableExt>::NewValues
{
    type Error = ::validation_errors::ValidationError<&'static str>;
    #[inline]
    fn validate_column(kelvin_tolerance_percentage: &f32) -> Result<(), Self::Error> {
        use diesel::Column;
        if kelvin_tolerance_percentage <= &0f32 {
            return Err(validation_errors::prelude::ValidationError::strictly_greater_than_value(
                crate::freeze_drying_procedure_templates::kelvin_tolerance_percentage::NAME,
                0f64,
            ));
        }
        if kelvin_tolerance_percentage > &100f32 {
            return Err(validation_errors::prelude::ValidationError::smaller_than_value(
                crate::freeze_drying_procedure_templates::kelvin_tolerance_percentage::NAME,
                100f64,
            ));
        }
        Ok(())
    }
}
impl ::diesel_builders::ValidateColumn<freeze_drying_procedure_templates::pascal>
    for <freeze_drying_procedure_templates::table as ::diesel_builders::TableExt>::NewValues
{
    type Error = ::validation_errors::ValidationError<&'static str>;
    #[inline]
    fn validate_column(pascal: &f32) -> Result<(), Self::Error> {
        use diesel::Column;
        if pascal < &0f32 {
            return Err(validation_errors::prelude::ValidationError::greater_than_value(
                crate::freeze_drying_procedure_templates::pascal::NAME,
                0f64,
            ));
        }
        if pascal > &500f32 {
            return Err(validation_errors::prelude::ValidationError::smaller_than_value(
                crate::freeze_drying_procedure_templates::pascal::NAME,
                500f64,
            ));
        }
        Ok(())
    }
}
impl ::diesel_builders::ValidateColumn<freeze_drying_procedure_templates::duration>
    for <freeze_drying_procedure_templates::table as ::diesel_builders::TableExt>::NewValues
{
    type Error = ::validation_errors::ValidationError<&'static str>;
    #[inline]
    fn validate_column(duration: &f32) -> Result<(), Self::Error> {
        use diesel::Column;
        if duration < &7200f32 {
            return Err(validation_errors::prelude::ValidationError::greater_than_value(
                crate::freeze_drying_procedure_templates::duration::NAME,
                7200f64,
            ));
        }
        if duration > &604800f32 {
            return Err(validation_errors::prelude::ValidationError::smaller_than_value(
                crate::freeze_drying_procedure_templates::duration::NAME,
                604800f64,
            ));
        }
        Ok(())
    }
}
