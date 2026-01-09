//! Auto-generated crate for the `fractioning_procedure_templates` table.
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
/// Struct representing a row in the `fractioning_procedure_templates` table.
#[table_model(ancestors(aps_procedure_templates::procedure_templates))]
# [table_model (error = :: validation_errors :: ValidationError)]
#[table_model(default(
    aps_procedure_templates::procedure_templates::procedure_template_table_id,
    "fractioning_procedure_templates"
))]
# [diesel (table_name = fractioning_procedure_templates)]
pub struct FractioningProcedureTemplate {
    /// Field representing the `id` column in table
    /// `fractioning_procedure_templates`.
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    id: ::rosetta_uuid::Uuid,
    /// Field representing the `mass` column in table
    /// `fractioning_procedure_templates`.
    mass: f32,
    /// Field representing the `tolerance_percentage` column in table
    /// `fractioning_procedure_templates`.
    tolerance_percentage: f32,
    /// Field representing the `weighed_with_model_id` column in table
    /// `fractioning_procedure_templates`.
    #[same_as(
        aps_procedure_template_asset_models::procedure_template_asset_models::asset_model_id,
        procedure_template_weighed_with_model_id
    )]
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    weighed_with_model_id: ::rosetta_uuid::Uuid,
    /// Field representing the `procedure_template_weighed_with_model_id` column
    /// in table `fractioning_procedure_templates`.
    #[discretionary(aps_procedure_template_asset_models::procedure_template_asset_models)]
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_template_weighed_with_model_id: ::rosetta_uuid::Uuid,
    /// Field representing the `fragment_container_model_id` column in table
    /// `fractioning_procedure_templates`.
    #[same_as(
        aps_procedure_template_asset_models::procedure_template_asset_models::asset_model_id,
        procedure_template_fragment_container_model_id
    )]
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    fragment_container_model_id: ::rosetta_uuid::Uuid,
    /// Field representing the `procedure_template_fragment_container_model_id`
    /// column in table `fractioning_procedure_templates`.
    #[discretionary(aps_procedure_template_asset_models::procedure_template_asset_models)]
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_template_fragment_container_model_id: ::rosetta_uuid::Uuid,
    /// Field representing the `fragment_placed_into_model_id` column in table
    /// `fractioning_procedure_templates`.
    #[same_as(
        aps_procedure_template_asset_models::procedure_template_asset_models::asset_model_id,
        procedure_template_fragment_placed_into_model_id
    )]
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    fragment_placed_into_model_id: ::rosetta_uuid::Uuid,
    /// Field representing the
    /// `procedure_template_fragment_placed_into_model_id` column in table
    /// `fractioning_procedure_templates`.
    #[discretionary(aps_procedure_template_asset_models::procedure_template_asset_models)]
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_template_fragment_placed_into_model_id: ::rosetta_uuid::Uuid,
}
::diesel_builders::prelude::unique_index!(
    fractioning_procedure_templates::id,
    fractioning_procedure_templates::procedure_template_weighed_with_model_id
);
::diesel_builders::prelude::unique_index!(
    fractioning_procedure_templates::id,
    fractioning_procedure_templates::procedure_template_fragment_container_model_id
);
::diesel_builders::prelude::unique_index!(
    fractioning_procedure_templates::id,
    fractioning_procedure_templates::procedure_template_fragment_placed_into_model_id
);
:: diesel_builders :: prelude :: fk ! ((fractioning_procedure_templates :: id) -> (:: aps_procedure_templates :: procedure_templates :: id));
:: diesel_builders :: prelude :: fk ! ((fractioning_procedure_templates :: weighed_with_model_id) -> (:: aps_weighing_device_models :: weighing_device_models :: id));
:: diesel_builders :: prelude :: fk ! ((fractioning_procedure_templates :: fragment_container_model_id) -> (:: aps_volumetric_container_models :: volumetric_container_models :: id));
:: diesel_builders :: prelude :: fk ! ((fractioning_procedure_templates :: fragment_placed_into_model_id) -> (:: aps_volumetric_container_models :: volumetric_container_models :: id));
impl ::diesel_builders::ValidateColumn<fractioning_procedure_templates::mass>
    for <fractioning_procedure_templates::table as ::diesel_builders::TableExt>::NewValues
{
    type Error = ::validation_errors::ValidationError<&'static str>;
    #[inline]
    fn validate_column(mass: &f32) -> Result<(), Self::Error> {
        use diesel::Column;
        if mass <= &0f32 {
            return Err(validation_errors::prelude::ValidationError::strictly_greater_than_value(
                crate::fractioning_procedure_templates::mass::NAME,
                0f64,
            ));
        }
        Ok(())
    }
}
impl ::diesel_builders::ValidateColumn<fractioning_procedure_templates::tolerance_percentage>
    for <fractioning_procedure_templates::table as ::diesel_builders::TableExt>::NewValues
{
    type Error = ::validation_errors::ValidationError<&'static str>;
    #[inline]
    fn validate_column(tolerance_percentage: &f32) -> Result<(), Self::Error> {
        use diesel::Column;
        if tolerance_percentage <= &0f32 {
            return Err(validation_errors::prelude::ValidationError::strictly_greater_than_value(
                crate::fractioning_procedure_templates::tolerance_percentage::NAME,
                0f64,
            ));
        }
        if tolerance_percentage > &100f32 {
            return Err(validation_errors::prelude::ValidationError::smaller_than_value(
                crate::fractioning_procedure_templates::tolerance_percentage::NAME,
                100f64,
            ));
        }
        Ok(())
    }
}
