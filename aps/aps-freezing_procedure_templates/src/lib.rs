//! Auto-generated crate for the `freezing_procedure_templates` table.
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
/// Struct representing a row in the `freezing_procedure_templates` table.
#[table_model(ancestors(aps_procedure_templates::procedure_templates))]
# [table_model (error = :: validation_errors :: ValidationError)]
#[table_model(default(
    aps_procedure_templates::procedure_templates::procedure_template_table_id,
    "freezing_procedure_templates"
))]
# [diesel (table_name = freezing_procedure_templates)]
pub struct FreezingProcedureTemplate {
    /// Field representing the `id` column in table
    /// `freezing_procedure_templates`.
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    id: ::rosetta_uuid::Uuid,
    /// Field representing the `kelvin` column in table
    /// `freezing_procedure_templates`.
    #[table_model(default = 203.15f32)]
    kelvin: f32,
    /// Field representing the `kelvin_tolerance_percentage` column in table
    /// `freezing_procedure_templates`.
    #[table_model(default = 5f32)]
    kelvin_tolerance_percentage: f32,
    /// Field representing the `duration` column in table
    /// `freezing_procedure_templates`.
    #[table_model(default = 43200f32)]
    duration: Option<f32>,
    /// Field representing the `frozen_with_model_id` column in table
    /// `freezing_procedure_templates`.
    #[same_as(
        aps_procedure_template_asset_models::procedure_template_asset_models::asset_model_id,
        procedure_template_frozen_with_model_id
    )]
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    frozen_with_model_id: ::rosetta_uuid::Uuid,
    /// Field representing the `procedure_template_frozen_with_model_id` column
    /// in table `freezing_procedure_templates`.
    #[discretionary(aps_procedure_template_asset_models::procedure_template_asset_models)]
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_template_frozen_with_model_id: ::rosetta_uuid::Uuid,
    /// Field representing the `frozen_container_model_id` column in table
    /// `freezing_procedure_templates`.
    #[same_as(
        aps_procedure_template_asset_models::procedure_template_asset_models::asset_model_id,
        procedure_template_frozen_container_model_id
    )]
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    frozen_container_model_id: ::rosetta_uuid::Uuid,
    /// Field representing the `procedure_template_frozen_container_model_id`
    /// column in table `freezing_procedure_templates`.
    #[discretionary(aps_procedure_template_asset_models::procedure_template_asset_models)]
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_template_frozen_container_model_id: ::rosetta_uuid::Uuid,
}
::diesel_builders::prelude::unique_index!(
    freezing_procedure_templates::id,
    freezing_procedure_templates::procedure_template_frozen_with_model_id
);
::diesel_builders::prelude::unique_index!(
    freezing_procedure_templates::id,
    freezing_procedure_templates::procedure_template_frozen_container_model_id
);
:: diesel_builders :: prelude :: fk ! ((freezing_procedure_templates :: id) -> (:: aps_procedure_templates :: procedure_templates :: id));
:: diesel_builders :: prelude :: fk ! ((freezing_procedure_templates :: frozen_with_model_id) -> (:: aps_freezer_models :: freezer_models :: id));
:: diesel_builders :: prelude :: fk ! ((freezing_procedure_templates :: frozen_container_model_id) -> (:: aps_volumetric_container_models :: volumetric_container_models :: id));
:: diesel_builders :: prelude :: fk ! ((freezing_procedure_templates :: frozen_with_model_id , freezing_procedure_templates :: frozen_container_model_id) -> (:: aps_asset_compatibility_rules :: asset_compatibility_rules :: left_asset_model_id , :: aps_asset_compatibility_rules :: asset_compatibility_rules :: right_asset_model_id));
:: diesel_builders :: prelude :: fk ! ((freezing_procedure_templates :: id , freezing_procedure_templates :: procedure_template_frozen_with_model_id) -> (:: aps_reused_procedure_template_asset_models :: reused_procedure_template_asset_models :: procedure_template_id , :: aps_reused_procedure_template_asset_models :: reused_procedure_template_asset_models :: procedure_template_asset_model_id));
:: diesel_builders :: prelude :: fk ! ((freezing_procedure_templates :: id , freezing_procedure_templates :: procedure_template_frozen_container_model_id) -> (:: aps_reused_procedure_template_asset_models :: reused_procedure_template_asset_models :: procedure_template_id , :: aps_reused_procedure_template_asset_models :: reused_procedure_template_asset_models :: procedure_template_asset_model_id));
impl ::diesel_builders::ValidateColumn<freezing_procedure_templates::kelvin>
    for <freezing_procedure_templates::table as ::diesel_builders::TableExt>::NewValues
{
    type Error = ::validation_errors::ValidationError;
    #[inline]
    fn validate_column(kelvin: &f32) -> Result<(), Self::Error> {
        use diesel::Column;
        if kelvin <= &0f32 {
            return Err(::validation_errors::ValidationError::strictly_greater_than_value(
                "freezing_procedure_templates",
                crate::freezing_procedure_templates::kelvin::NAME,
                0f64,
            ));
        }
        Ok(())
    }
}
impl ::diesel_builders::ValidateColumn<freezing_procedure_templates::kelvin_tolerance_percentage>
    for <freezing_procedure_templates::table as ::diesel_builders::TableExt>::NewValues
{
    type Error = ::validation_errors::ValidationError;
    #[inline]
    fn validate_column(kelvin_tolerance_percentage: &f32) -> Result<(), Self::Error> {
        use diesel::Column;
        if kelvin_tolerance_percentage <= &0f32 {
            return Err(::validation_errors::ValidationError::strictly_greater_than_value(
                "freezing_procedure_templates",
                crate::freezing_procedure_templates::kelvin_tolerance_percentage::NAME,
                0f64,
            ));
        }
        if kelvin_tolerance_percentage > &100f32 {
            return Err(::validation_errors::ValidationError::smaller_than_value(
                "freezing_procedure_templates",
                crate::freezing_procedure_templates::kelvin_tolerance_percentage::NAME,
                100f64,
            ));
        }
        Ok(())
    }
}
impl ::diesel_builders::ValidateColumn<freezing_procedure_templates::duration>
    for <freezing_procedure_templates::table as ::diesel_builders::TableExt>::NewValues
{
    type Error = ::validation_errors::ValidationError;
    #[inline]
    fn validate_column(duration: &f32) -> Result<(), Self::Error> {
        use diesel::Column;
        if duration <= &1800f32 {
            return Err(::validation_errors::ValidationError::strictly_greater_than_value(
                "freezing_procedure_templates",
                crate::freezing_procedure_templates::duration::NAME,
                1800f64,
            ));
        }
        Ok(())
    }
}
impl diesel_builders::GetColumn<aps_procedure_templates::procedure_templates::id>
    for FreezingProcedureTemplate
{
    fn get_column_ref(
        &self,
    ) -> &<freezing_procedure_templates::id as diesel_builders::Typed>::ColumnType {
        &self.id
    }
}
