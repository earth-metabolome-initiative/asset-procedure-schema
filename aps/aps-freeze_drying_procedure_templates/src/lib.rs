//! Auto-generated crate for the `freeze_drying_procedure_templates` table.
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
/// Struct representing a row in the `freeze_drying_procedure_templates` table.
#[table_model(ancestors(aps_procedure_templates::procedure_templates))]
# [table_model (error = :: validation_errors :: ValidationError)]
# [diesel (belongs_to (aps_procedure_templates :: ProcedureTemplate , foreign_key = id))]
# [diesel (belongs_to (aps_freeze_dryer_models :: FreezeDryerModel , foreign_key = freeze_dried_with_model_id))]
# [diesel (belongs_to (aps_volumetric_container_models :: VolumetricContainerModel , foreign_key = freeze_dried_container_model_id))]
# [table_model (foreign_key ((id ,) , (:: aps_procedure_templates :: procedure_templates :: id)))]
# [table_model (foreign_key ((freeze_dried_with_model_id ,) , (:: aps_freeze_dryer_models :: freeze_dryer_models :: id)))]
# [table_model (foreign_key ((freeze_dried_container_model_id ,) , (:: aps_volumetric_container_models :: volumetric_container_models :: id)))]
# [table_model (foreign_key ((freeze_dried_with_model_id , freeze_dried_container_model_id ,) , (:: aps_asset_compatibility_rules :: asset_compatibility_rules :: left_asset_model_id , :: aps_asset_compatibility_rules :: asset_compatibility_rules :: right_asset_model_id)))]
# [table_model (foreign_key ((id , procedure_template_freeze_dried_with_model_id ,) , (:: aps_reused_procedure_template_asset_models :: reused_procedure_template_asset_models :: procedure_template_id , :: aps_reused_procedure_template_asset_models :: reused_procedure_template_asset_models :: procedure_template_asset_model_id)))]
# [table_model (foreign_key ((id , procedure_template_freeze_dried_container_model_id ,) , (:: aps_reused_procedure_template_asset_models :: reused_procedure_template_asset_models :: procedure_template_id , :: aps_reused_procedure_template_asset_models :: reused_procedure_template_asset_models :: procedure_template_asset_model_id)))]
#[table_model(default(
    aps_procedure_templates::procedure_templates::procedure_template_table_id,
    "freeze_drying_procedure_templates"
))]
# [diesel (table_name = freeze_drying_procedure_templates)]
pub struct FreezeDryingProcedureTemplate {
    /// Field representing the `id` column in table
    /// `freeze_drying_procedure_templates`.
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    id: ::rosetta_uuid::Uuid,
    /// The storage temperature in Kelvin.
    #[table_model(default = 203.15f32)]
    kelvin: f32,
    /// Tolerance percentage for the storage temperature.
    #[table_model(default = 5f32)]
    kelvin_tolerance_percentage: f32,
    /// We use a default of 4 Pa for the pressure in the freeze-drying chamber.
    #[table_model(default = 4f32)]
    pascal: f32,
    /// Duration in seconds. We use a default of 3 days (259200 seconds) for the
    /// freeze-drying procedure.
    #[table_model(default = 259200f32)]
    duration: f32,
    /// The device used for the freeze drying procedure.
    #[same_as(
        aps_procedure_template_asset_models::procedure_template_asset_models::asset_model_id,
        procedure_template_freeze_dried_with_model_id
    )]
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    freeze_dried_with_model_id: ::rosetta_uuid::Uuid,
    /// Field representing the `procedure_template_freeze_dried_with_model_id`
    /// column in table `freeze_drying_procedure_templates`.
    #[discretionary(aps_procedure_template_asset_models::procedure_template_asset_models)]
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_template_freeze_dried_with_model_id: ::rosetta_uuid::Uuid,
    /// The container that is being freeze_dried.
    #[same_as(
        aps_procedure_template_asset_models::procedure_template_asset_models::asset_model_id,
        procedure_template_freeze_dried_container_model_id
    )]
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    freeze_dried_container_model_id: ::rosetta_uuid::Uuid,
    /// Field representing the
    /// `procedure_template_freeze_dried_container_model_id` column in table
    /// `freeze_drying_procedure_templates`.
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
impl ::diesel_builders::ValidateColumn<freeze_drying_procedure_templates::kelvin>
    for <freeze_drying_procedure_templates::table as ::diesel_builders::TableExt>::NewValues
{
    type Error = ::validation_errors::ValidationError;
    #[inline]
    fn validate_column(kelvin: &f32) -> Result<(), Self::Error> {
        use diesel::Column;
        if kelvin <= &0f32 {
            return Err(::validation_errors::ValidationError::strictly_greater_than_value(
                "freeze_drying_procedure_templates",
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
    type Error = ::validation_errors::ValidationError;
    #[inline]
    fn validate_column(kelvin_tolerance_percentage: &f32) -> Result<(), Self::Error> {
        use diesel::Column;
        if kelvin_tolerance_percentage <= &0f32 {
            return Err(::validation_errors::ValidationError::strictly_greater_than_value(
                "freeze_drying_procedure_templates",
                crate::freeze_drying_procedure_templates::kelvin_tolerance_percentage::NAME,
                0f64,
            ));
        }
        if kelvin_tolerance_percentage > &100f32 {
            return Err(::validation_errors::ValidationError::smaller_than_value(
                "freeze_drying_procedure_templates",
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
    type Error = ::validation_errors::ValidationError;
    #[inline]
    fn validate_column(pascal: &f32) -> Result<(), Self::Error> {
        use diesel::Column;
        if pascal < &0f32 {
            return Err(::validation_errors::ValidationError::greater_than_value(
                "freeze_drying_procedure_templates",
                crate::freeze_drying_procedure_templates::pascal::NAME,
                0f64,
            ));
        }
        if pascal > &500f32 {
            return Err(::validation_errors::ValidationError::smaller_than_value(
                "freeze_drying_procedure_templates",
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
    type Error = ::validation_errors::ValidationError;
    #[inline]
    fn validate_column(duration: &f32) -> Result<(), Self::Error> {
        use diesel::Column;
        if duration < &7200f32 {
            return Err(::validation_errors::ValidationError::greater_than_value(
                "freeze_drying_procedure_templates",
                crate::freeze_drying_procedure_templates::duration::NAME,
                7200f64,
            ));
        }
        if duration > &604800f32 {
            return Err(::validation_errors::ValidationError::smaller_than_value(
                "freeze_drying_procedure_templates",
                crate::freeze_drying_procedure_templates::duration::NAME,
                604800f64,
            ));
        }
        Ok(())
    }
}
impl ::diesel_builders::GetColumn<aps_procedure_templates::procedure_templates::id>
    for FreezeDryingProcedureTemplate
{
    fn get_column_ref(
        &self,
    ) -> &<freeze_drying_procedure_templates::id as ::diesel_builders::ColumnTyped>::ColumnType
    {
        &self.id
    }
}
