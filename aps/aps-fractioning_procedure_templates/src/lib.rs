//! Auto-generated crate for the `fractioning_procedure_templates` table.
#[derive(
    Copy,
    Clone,
    Debug,
    PartialOrd,
    PartialEq,
    :: serde :: Serialize,
    :: serde :: Deserialize,
    :: diesel :: Queryable,
    :: diesel :: Selectable,
    :: diesel :: Identifiable,
    :: diesel :: Associations,
    :: diesel_builders :: prelude :: TableModel,
)]
/// Struct representing a row in the `fractioning_procedure_templates` table.
#[table_model(ancestors(aps_procedure_templates::procedure_templates))]
# [table_model (error = :: validation_errors :: ValidationError)]
# [diesel (belongs_to (aps_procedure_templates :: ProcedureTemplate , foreign_key = id))]
# [diesel (belongs_to (aps_weighing_device_models :: WeighingDeviceModel , foreign_key = weighed_with_model_id))]
# [table_model (foreign_key ((id ,) , (:: aps_procedure_templates :: procedure_templates :: id)))]
# [table_model (foreign_key ((weighed_with_model_id ,) , (:: aps_weighing_device_models :: weighing_device_models :: id)))]
# [table_model (foreign_key ((fragment_container_model_id ,) , (:: aps_volumetric_container_models :: volumetric_container_models :: id)))]
# [table_model (foreign_key ((fragment_placed_into_model_id ,) , (:: aps_volumetric_container_models :: volumetric_container_models :: id)))]
# [table_model (foreign_key ((id , procedure_template_weighed_with_model_id ,) , (:: aps_reused_procedure_template_asset_models :: reused_procedure_template_asset_models :: procedure_template_id , :: aps_reused_procedure_template_asset_models :: reused_procedure_template_asset_models :: procedure_template_asset_model_id)))]
# [table_model (foreign_key ((id , procedure_template_fragment_container_model_id ,) , (:: aps_reused_procedure_template_asset_models :: reused_procedure_template_asset_models :: procedure_template_id , :: aps_reused_procedure_template_asset_models :: reused_procedure_template_asset_models :: procedure_template_asset_model_id)))]
# [table_model (foreign_key ((id , procedure_template_fragment_placed_into_model_id ,) , (:: aps_reused_procedure_template_asset_models :: reused_procedure_template_asset_models :: procedure_template_id , :: aps_reused_procedure_template_asset_models :: reused_procedure_template_asset_models :: procedure_template_asset_model_id)))]
#[table_model(default(
    aps_procedure_templates::procedure_templates::procedure_template_table_id,
    "fractioning_procedure_templates"
))]
# [diesel (table_name = fractioning_procedure_templates)]
pub struct FractioningProcedureTemplate {
    /// Identifier of the fractioning procedure_id template, which is also a
    /// foreign key to the general procedure_id template.
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    id: ::rosetta_uuid::Uuid,
    /// Mass in kilograms. Expected amount of the fraction to be collected.
    mass: f32,
    /// The tolerance percentage of the fraction mass.
    tolerance_percentage: f32,
    /// The model of the scale used to measure the fraction mass.
    #[same_as(
        aps_procedure_template_asset_models::procedure_template_asset_models::asset_model_id,
        procedure_template_weighed_with_model_id
    )]
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    weighed_with_model_id: ::rosetta_uuid::Uuid,
    /// The model of the instrument used for weighing should always be an asset
    /// model that is compatible with the procedure_id template.
    #[discretionary(aps_procedure_template_asset_models::procedure_template_asset_models)]
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_template_weighed_with_model_id: ::rosetta_uuid::Uuid,
    /// Source container model from which the fraction is taken.
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
    /// Destination container model to which the fraction is transferred.
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
impl ::diesel_builders::ValidateColumn<fractioning_procedure_templates::mass>
    for <fractioning_procedure_templates::table as ::diesel_builders::TableExt>::NewValues
{
    type Error = ::validation_errors::ValidationError;
    #[inline]
    fn validate_column(mass: &f32) -> Result<(), Self::Error> {
        use diesel::Column;
        if mass <= &0f32 {
            return Err(::validation_errors::ValidationError::strictly_greater_than_value(
                "fractioning_procedure_templates",
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
    type Error = ::validation_errors::ValidationError;
    #[inline]
    fn validate_column(tolerance_percentage: &f32) -> Result<(), Self::Error> {
        use diesel::Column;
        if tolerance_percentage <= &0f32 {
            return Err(::validation_errors::ValidationError::strictly_greater_than_value(
                "fractioning_procedure_templates",
                crate::fractioning_procedure_templates::tolerance_percentage::NAME,
                0f64,
            ));
        }
        if tolerance_percentage > &100f32 {
            return Err(::validation_errors::ValidationError::smaller_than_value(
                "fractioning_procedure_templates",
                crate::fractioning_procedure_templates::tolerance_percentage::NAME,
                100f64,
            ));
        }
        Ok(())
    }
}
impl ::diesel_builders::GetColumn<aps_procedure_templates::procedure_templates::id>
    for FractioningProcedureTemplate
{
    fn get_column_ref(
        &self,
    ) -> &<fractioning_procedure_templates::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
