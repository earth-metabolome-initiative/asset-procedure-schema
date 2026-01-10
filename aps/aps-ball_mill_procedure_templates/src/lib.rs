//! Auto-generated crate for the `ball_mill_procedure_templates` table.
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
/// Struct representing a row in the `ball_mill_procedure_templates` table.
#[table_model(ancestors(aps_procedure_templates::procedure_templates))]
# [table_model (error = :: validation_errors :: ValidationError)]
#[table_model(default(
    aps_procedure_templates::procedure_templates::procedure_template_table_id,
    "ball_mill_procedure_templates"
))]
# [diesel (table_name = ball_mill_procedure_templates)]
pub struct BallMillProcedureTemplate {
    /// Field representing the `id` column in table
    /// `ball_mill_procedure_templates`.
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    id: ::rosetta_uuid::Uuid,
    /// Field representing the `kelvin` column in table
    /// `ball_mill_procedure_templates`.
    #[table_model(default = 293.15f32)]
    kelvin: f32,
    /// Field representing the `kelvin_tolerance_percentage` column in table
    /// `ball_mill_procedure_templates`.
    #[table_model(default = 1f32)]
    kelvin_tolerance_percentage: f32,
    /// Field representing the `duration` column in table
    /// `ball_mill_procedure_templates`.
    #[table_model(default = 150f32)]
    duration: f32,
    /// Field representing the `hertz` column in table
    /// `ball_mill_procedure_templates`.
    #[table_model(default = 25f32)]
    hertz: f32,
    /// Field representing the `bead_model_id` column in table
    /// `ball_mill_procedure_templates`.
    #[same_as(
        aps_procedure_template_asset_models::procedure_template_asset_models::asset_model_id,
        procedure_template_bead_model_id
    )]
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    bead_model_id: ::rosetta_uuid::Uuid,
    /// Field representing the `procedure_template_bead_model_id` column in
    /// table `ball_mill_procedure_templates`.
    #[discretionary(aps_procedure_template_asset_models::procedure_template_asset_models)]
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_template_bead_model_id: ::rosetta_uuid::Uuid,
    /// Field representing the `bead_count` column in table
    /// `ball_mill_procedure_templates`.
    #[table_model(default = 3i16)]
    bead_count: i16,
    /// Field representing the `milled_with_model_id` column in table
    /// `ball_mill_procedure_templates`.
    #[same_as(
        aps_procedure_template_asset_models::procedure_template_asset_models::asset_model_id,
        procedure_template_milled_with_model_id
    )]
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    milled_with_model_id: ::rosetta_uuid::Uuid,
    /// Field representing the `procedure_template_milled_with_model_id` column
    /// in table `ball_mill_procedure_templates`.
    #[discretionary(aps_procedure_template_asset_models::procedure_template_asset_models)]
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_template_milled_with_model_id: ::rosetta_uuid::Uuid,
    /// Field representing the `milled_container_model_id` column in table
    /// `ball_mill_procedure_templates`.
    #[same_as(
        aps_procedure_template_asset_models::procedure_template_asset_models::asset_model_id,
        procedure_template_milled_container_model_id
    )]
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    milled_container_model_id: ::rosetta_uuid::Uuid,
    /// Field representing the `procedure_template_milled_container_model_id`
    /// column in table `ball_mill_procedure_templates`.
    #[discretionary(aps_procedure_template_asset_models::procedure_template_asset_models)]
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_template_milled_container_model_id: ::rosetta_uuid::Uuid,
}
::diesel_builders::prelude::unique_index!(
    ball_mill_procedure_templates::id,
    ball_mill_procedure_templates::procedure_template_bead_model_id
);
::diesel_builders::prelude::unique_index!(
    ball_mill_procedure_templates::id,
    ball_mill_procedure_templates::procedure_template_milled_with_model_id
);
::diesel_builders::prelude::unique_index!(
    ball_mill_procedure_templates::id,
    ball_mill_procedure_templates::procedure_template_milled_container_model_id
);
:: diesel_builders :: prelude :: fk ! ((ball_mill_procedure_templates :: id) -> (:: aps_procedure_templates :: procedure_templates :: id));
:: diesel_builders :: prelude :: fk ! ((ball_mill_procedure_templates :: bead_model_id) -> (:: aps_bead_models :: bead_models :: id));
:: diesel_builders :: prelude :: fk ! ((ball_mill_procedure_templates :: milled_with_model_id) -> (:: aps_ball_mill_machine_models :: ball_mill_machine_models :: id));
:: diesel_builders :: prelude :: fk ! ((ball_mill_procedure_templates :: milled_container_model_id) -> (:: aps_volumetric_container_models :: volumetric_container_models :: id));
:: diesel_builders :: prelude :: fk ! ((ball_mill_procedure_templates :: milled_with_model_id , ball_mill_procedure_templates :: milled_container_model_id) -> (:: aps_asset_compatibility_rules :: asset_compatibility_rules :: left_asset_model_id , :: aps_asset_compatibility_rules :: asset_compatibility_rules :: right_asset_model_id));
:: diesel_builders :: prelude :: fk ! ((ball_mill_procedure_templates :: id , ball_mill_procedure_templates :: procedure_template_bead_model_id) -> (:: aps_reused_procedure_template_asset_models :: reused_procedure_template_asset_models :: procedure_template_id , :: aps_reused_procedure_template_asset_models :: reused_procedure_template_asset_models :: procedure_template_asset_model_id));
:: diesel_builders :: prelude :: fk ! ((ball_mill_procedure_templates :: id , ball_mill_procedure_templates :: procedure_template_milled_with_model_id) -> (:: aps_reused_procedure_template_asset_models :: reused_procedure_template_asset_models :: procedure_template_id , :: aps_reused_procedure_template_asset_models :: reused_procedure_template_asset_models :: procedure_template_asset_model_id));
:: diesel_builders :: prelude :: fk ! ((ball_mill_procedure_templates :: id , ball_mill_procedure_templates :: procedure_template_milled_container_model_id) -> (:: aps_reused_procedure_template_asset_models :: reused_procedure_template_asset_models :: procedure_template_id , :: aps_reused_procedure_template_asset_models :: reused_procedure_template_asset_models :: procedure_template_asset_model_id));
:: diesel_builders :: prelude :: fk ! ((ball_mill_procedure_templates :: milled_with_model_id , ball_mill_procedure_templates :: bead_model_id) -> (:: aps_asset_compatibility_rules :: asset_compatibility_rules :: left_asset_model_id , :: aps_asset_compatibility_rules :: asset_compatibility_rules :: right_asset_model_id));
:: diesel_builders :: prelude :: fk ! ((ball_mill_procedure_templates :: bead_model_id , ball_mill_procedure_templates :: milled_container_model_id) -> (:: aps_asset_compatibility_rules :: asset_compatibility_rules :: left_asset_model_id , :: aps_asset_compatibility_rules :: asset_compatibility_rules :: right_asset_model_id));
impl ::diesel_builders::ValidateColumn<ball_mill_procedure_templates::kelvin>
    for <ball_mill_procedure_templates::table as ::diesel_builders::TableExt>::NewValues
{
    type Error = ::validation_errors::ValidationError<&'static str>;
    #[inline]
    fn validate_column(kelvin: &f32) -> Result<(), Self::Error> {
        use diesel::Column;
        if kelvin <= &0f32 {
            return Err(validation_errors::prelude::ValidationError::strictly_greater_than_value(
                crate::ball_mill_procedure_templates::kelvin::NAME,
                0f64,
            ));
        }
        Ok(())
    }
}
impl ::diesel_builders::ValidateColumn<ball_mill_procedure_templates::kelvin_tolerance_percentage>
    for <ball_mill_procedure_templates::table as ::diesel_builders::TableExt>::NewValues
{
    type Error = ::validation_errors::ValidationError<&'static str>;
    #[inline]
    fn validate_column(kelvin_tolerance_percentage: &f32) -> Result<(), Self::Error> {
        use diesel::Column;
        if kelvin_tolerance_percentage <= &0f32 {
            return Err(validation_errors::prelude::ValidationError::strictly_greater_than_value(
                crate::ball_mill_procedure_templates::kelvin_tolerance_percentage::NAME,
                0f64,
            ));
        }
        if kelvin_tolerance_percentage > &100f32 {
            return Err(validation_errors::prelude::ValidationError::smaller_than_value(
                crate::ball_mill_procedure_templates::kelvin_tolerance_percentage::NAME,
                100f64,
            ));
        }
        Ok(())
    }
}
impl ::diesel_builders::ValidateColumn<ball_mill_procedure_templates::duration>
    for <ball_mill_procedure_templates::table as ::diesel_builders::TableExt>::NewValues
{
    type Error = ::validation_errors::ValidationError<&'static str>;
    #[inline]
    fn validate_column(duration: &f32) -> Result<(), Self::Error> {
        use diesel::Column;
        if duration > &900f32 {
            return Err(validation_errors::prelude::ValidationError::smaller_than_value(
                crate::ball_mill_procedure_templates::duration::NAME,
                900f64,
            ));
        }
        if duration < &30f32 {
            return Err(validation_errors::prelude::ValidationError::greater_than_value(
                crate::ball_mill_procedure_templates::duration::NAME,
                30f64,
            ));
        }
        Ok(())
    }
}
impl ::diesel_builders::ValidateColumn<ball_mill_procedure_templates::hertz>
    for <ball_mill_procedure_templates::table as ::diesel_builders::TableExt>::NewValues
{
    type Error = ::validation_errors::ValidationError<&'static str>;
    #[inline]
    fn validate_column(hertz: &f32) -> Result<(), Self::Error> {
        use diesel::Column;
        if hertz > &50f32 {
            return Err(validation_errors::prelude::ValidationError::smaller_than_value(
                crate::ball_mill_procedure_templates::hertz::NAME,
                50f64,
            ));
        }
        if hertz < &15f32 {
            return Err(validation_errors::prelude::ValidationError::greater_than_value(
                crate::ball_mill_procedure_templates::hertz::NAME,
                15f64,
            ));
        }
        Ok(())
    }
}
impl ::diesel_builders::ValidateColumn<ball_mill_procedure_templates::bead_count>
    for <ball_mill_procedure_templates::table as ::diesel_builders::TableExt>::NewValues
{
    type Error = ::validation_errors::ValidationError<&'static str>;
    #[inline]
    fn validate_column(bead_count: &i16) -> Result<(), Self::Error> {
        use diesel::Column;
        if bead_count <= &0i16 {
            return Err(validation_errors::prelude::ValidationError::strictly_greater_than_value(
                crate::ball_mill_procedure_templates::bead_count::NAME,
                0f64,
            ));
        }
        Ok(())
    }
}
