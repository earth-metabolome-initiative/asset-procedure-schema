//! Auto-generated crate for the `centrifuge_procedure_templates` table.
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
# [diesel (table_name = centrifuge_procedure_templates)]
pub struct CentrifugeProcedureTemplate {
    /// Undocumented column
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    id: ::rosetta_uuid::Uuid,
    /// Undocumented column
    #[table_model(default = 293.15f32)]
    kelvin: f32,
    /// Undocumented column
    #[table_model(default = 1f32)]
    kelvin_tolerance_percentage: f32,
    /// Undocumented column
    #[table_model(default = 120f32)]
    duration: f32,
    /// Undocumented column
    #[table_model(default = 13000f32)]
    rotation_per_minute: f32,
    /// Undocumented column
    #[same_as(
        aps_procedure_template_asset_models::procedure_template_asset_models::asset_model_id,
        procedure_template_centrifuged_with_model_id
    )]
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    centrifuged_with_model_id: ::rosetta_uuid::Uuid,
    /// Undocumented column
    #[discretionary(aps_procedure_template_asset_models::procedure_template_asset_models)]
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_template_centrifuged_with_model_id: ::rosetta_uuid::Uuid,
    /// Undocumented column
    #[same_as(
        aps_procedure_template_asset_models::procedure_template_asset_models::asset_model_id,
        procedure_template_centrifuged_container_model_id
    )]
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    centrifuged_container_model_id: ::rosetta_uuid::Uuid,
    /// Undocumented column
    #[discretionary(aps_procedure_template_asset_models::procedure_template_asset_models)]
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_template_centrifuged_container_model_id: ::rosetta_uuid::Uuid,
}
::diesel_builders::prelude::unique_index!(
    centrifuge_procedure_templates::id,
    centrifuge_procedure_templates::procedure_template_centrifuged_with_model_id
);
::diesel_builders::prelude::unique_index!(
    centrifuge_procedure_templates::id,
    centrifuge_procedure_templates::procedure_template_centrifuged_container_model_id
);
:: diesel_builders :: prelude :: fk ! ((centrifuge_procedure_templates :: id) -> (:: aps_procedure_templates :: procedure_templates :: id));
:: diesel_builders :: prelude :: fk ! ((centrifuge_procedure_templates :: centrifuged_with_model_id) -> (:: aps_centrifuge_models :: centrifuge_models :: id));
:: diesel_builders :: prelude :: fk ! ((centrifuge_procedure_templates :: centrifuged_container_model_id) -> (:: aps_volumetric_container_models :: volumetric_container_models :: id));
:: diesel_builders :: prelude :: fk ! ((centrifuge_procedure_templates :: centrifuged_with_model_id , centrifuge_procedure_templates :: centrifuged_container_model_id) -> (:: aps_asset_compatibility_rules :: asset_compatibility_rules :: left_asset_model_id , :: aps_asset_compatibility_rules :: asset_compatibility_rules :: right_asset_model_id));
impl ::diesel_builders::ValidateColumn<centrifuge_procedure_templates::kelvin>
    for <centrifuge_procedure_templates::table as ::diesel_builders::TableExt>::NewValues
{
    type Error = ::validation_errors::ValidationError<&'static str>;
    #[inline]
    fn validate_column(kelvin: &f32) -> Result<(), Self::Error> {
        use diesel::Column;
        if kelvin <= &0f32 {
            return Err(validation_errors::prelude::ValidationError::strictly_greater_than_value(
                crate::centrifuge_procedure_templates::kelvin::NAME,
                0f64,
            ));
        }
        Ok(())
    }
}
impl ::diesel_builders::ValidateColumn<centrifuge_procedure_templates::kelvin_tolerance_percentage>
    for <centrifuge_procedure_templates::table as ::diesel_builders::TableExt>::NewValues
{
    type Error = ::validation_errors::ValidationError<&'static str>;
    #[inline]
    fn validate_column(kelvin_tolerance_percentage: &f32) -> Result<(), Self::Error> {
        use diesel::Column;
        if kelvin_tolerance_percentage <= &0f32 {
            return Err(validation_errors::prelude::ValidationError::strictly_greater_than_value(
                crate::centrifuge_procedure_templates::kelvin_tolerance_percentage::NAME,
                0f64,
            ));
        }
        if kelvin_tolerance_percentage > &100f32 {
            return Err(validation_errors::prelude::ValidationError::smaller_than_value(
                crate::centrifuge_procedure_templates::kelvin_tolerance_percentage::NAME,
                100f64,
            ));
        }
        Ok(())
    }
}
impl ::diesel_builders::ValidateColumn<centrifuge_procedure_templates::duration>
    for <centrifuge_procedure_templates::table as ::diesel_builders::TableExt>::NewValues
{
    type Error = ::validation_errors::ValidationError<&'static str>;
    #[inline]
    fn validate_column(duration: &f32) -> Result<(), Self::Error> {
        use diesel::Column;
        if duration < &30f32 {
            return Err(validation_errors::prelude::ValidationError::greater_than_value(
                crate::centrifuge_procedure_templates::duration::NAME,
                30f64,
            ));
        }
        if duration > &1800f32 {
            return Err(validation_errors::prelude::ValidationError::smaller_than_value(
                crate::centrifuge_procedure_templates::duration::NAME,
                1800f64,
            ));
        }
        Ok(())
    }
}
impl ::diesel_builders::ValidateColumn<centrifuge_procedure_templates::rotation_per_minute>
    for <centrifuge_procedure_templates::table as ::diesel_builders::TableExt>::NewValues
{
    type Error = ::validation_errors::ValidationError<&'static str>;
    #[inline]
    fn validate_column(rotation_per_minute: &f32) -> Result<(), Self::Error> {
        use diesel::Column;
        if rotation_per_minute < &5000f32 {
            return Err(validation_errors::prelude::ValidationError::greater_than_value(
                crate::centrifuge_procedure_templates::rotation_per_minute::NAME,
                5000f64,
            ));
        }
        if rotation_per_minute > &30000f32 {
            return Err(validation_errors::prelude::ValidationError::smaller_than_value(
                crate::centrifuge_procedure_templates::rotation_per_minute::NAME,
                30000f64,
            ));
        }
        Ok(())
    }
}
