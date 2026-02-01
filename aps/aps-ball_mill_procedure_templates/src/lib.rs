//! Auto-generated crate for the `ball_mill_procedure_templates` table.
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
/// Struct representing a row in the `ball_mill_procedure_templates` table.
#[table_model(ancestors(
    aps_entities::entities,
    aps_ownables::ownables,
    aps_procedure_templates::procedure_templates
))]
# [table_model (error = :: validation_errors :: ValidationError)]
# [diesel (belongs_to (aps_procedure_templates :: ProcedureTemplate , foreign_key = id))]
# [diesel (belongs_to (aps_bead_models :: BeadModel , foreign_key = bead_model_id))]
# [diesel (belongs_to (aps_ball_mill_machine_models :: BallMillMachineModel , foreign_key = milled_with_model_id))]
# [diesel (belongs_to (aps_volumetric_container_models :: VolumetricContainerModel , foreign_key = milled_container_model_id))]
# [table_model (foreign_key ((id ,) , (:: aps_procedure_templates :: procedure_templates :: id)))]
# [table_model (foreign_key ((bead_model_id ,) , (:: aps_bead_models :: bead_models :: id)))]
# [table_model (foreign_key ((milled_with_model_id ,) , (:: aps_ball_mill_machine_models :: ball_mill_machine_models :: id)))]
# [table_model (foreign_key ((milled_container_model_id ,) , (:: aps_volumetric_container_models :: volumetric_container_models :: id)))]
# [table_model (foreign_key ((milled_with_model_id , milled_container_model_id ,) , (:: aps_asset_compatibility_rules :: asset_compatibility_rules :: left_asset_model_id , :: aps_asset_compatibility_rules :: asset_compatibility_rules :: right_asset_model_id)))]
# [table_model (foreign_key ((id , procedure_template_bead_model_id ,) , (:: aps_reused_procedure_template_asset_models :: reused_procedure_template_asset_models :: procedure_template_id , :: aps_reused_procedure_template_asset_models :: reused_procedure_template_asset_models :: procedure_template_asset_model_id)))]
# [table_model (foreign_key ((id , procedure_template_milled_with_model_id ,) , (:: aps_reused_procedure_template_asset_models :: reused_procedure_template_asset_models :: procedure_template_id , :: aps_reused_procedure_template_asset_models :: reused_procedure_template_asset_models :: procedure_template_asset_model_id)))]
# [table_model (foreign_key ((id , procedure_template_milled_container_model_id ,) , (:: aps_reused_procedure_template_asset_models :: reused_procedure_template_asset_models :: procedure_template_id , :: aps_reused_procedure_template_asset_models :: reused_procedure_template_asset_models :: procedure_template_asset_model_id)))]
# [table_model (foreign_key ((milled_with_model_id , bead_model_id ,) , (:: aps_asset_compatibility_rules :: asset_compatibility_rules :: left_asset_model_id , :: aps_asset_compatibility_rules :: asset_compatibility_rules :: right_asset_model_id)))]
# [table_model (foreign_key ((bead_model_id , milled_container_model_id ,) , (:: aps_asset_compatibility_rules :: asset_compatibility_rules :: left_asset_model_id , :: aps_asset_compatibility_rules :: asset_compatibility_rules :: right_asset_model_id)))]
#[table_model(default(aps_entities::entities::table_name_id, "ball_mill_procedure_templates"))]
# [diesel (table_name = ball_mill_procedure_templates)]
pub struct BallMillProcedureTemplate {
    /// Field representing the `id` column in table
    /// `ball_mill_procedure_templates`.
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    id: ::rosetta_uuid::Uuid,
    /// The storage temperature in Kelvin.
    #[table_model(default = 293.15f32)]
    kelvin: f32,
    /// Tolerance percentage for the storage temperature.
    #[table_model(default = 1f32)]
    kelvin_tolerance_percentage: f32,
    /// Duration in seconds. By default, we set it to 150 seconds (2.5 minutes).
    #[table_model(default = 150f32)]
    duration: f32,
    /// The frequency in hertz at which the ball mill should operate during the
    /// procedure.
    #[table_model(default = 25f32)]
    hertz: f32,
    /// The beads model used for the procedure.
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
    /// - The count of beads used in the procedure.
    #[table_model(default = 3i16)]
    bead_count: i16,
    /// The device used for the ball mill procedure.
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
    /// The container that is being milled.
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
impl ::diesel_builders::ValidateColumn<ball_mill_procedure_templates::kelvin>
    for <ball_mill_procedure_templates::table as ::diesel_builders::TableExt>::NewValues
{
    type Error = ::validation_errors::ValidationError;
    #[inline]
    fn validate_column(kelvin: &f32) -> Result<(), Self::Error> {
        use diesel::Column;
        if kelvin <= &0f32 {
            return Err (:: validation_errors :: ValidationError :: strictly_greater_than_value (< crate :: ball_mill_procedure_templates :: table as :: diesel_builders :: TableExt > :: TABLE_NAME , crate :: ball_mill_procedure_templates :: kelvin :: NAME , 0f64)) ;
        }
        Ok(())
    }
}
impl ::diesel_builders::ValidateColumn<ball_mill_procedure_templates::kelvin_tolerance_percentage>
    for <ball_mill_procedure_templates::table as ::diesel_builders::TableExt>::NewValues
{
    type Error = ::validation_errors::ValidationError;
    #[inline]
    fn validate_column(kelvin_tolerance_percentage: &f32) -> Result<(), Self::Error> {
        use diesel::Column;
        if kelvin_tolerance_percentage <= &0f32 {
            return Err (:: validation_errors :: ValidationError :: strictly_greater_than_value (< crate :: ball_mill_procedure_templates :: table as :: diesel_builders :: TableExt > :: TABLE_NAME , crate :: ball_mill_procedure_templates :: kelvin_tolerance_percentage :: NAME , 0f64)) ;
        }
        if kelvin_tolerance_percentage > &100f32 {
            return Err (:: validation_errors :: ValidationError :: smaller_than_value (< crate :: ball_mill_procedure_templates :: table as :: diesel_builders :: TableExt > :: TABLE_NAME , crate :: ball_mill_procedure_templates :: kelvin_tolerance_percentage :: NAME , 100f64)) ;
        }
        Ok(())
    }
}
impl ::diesel_builders::ValidateColumn<ball_mill_procedure_templates::duration>
    for <ball_mill_procedure_templates::table as ::diesel_builders::TableExt>::NewValues
{
    type Error = ::validation_errors::ValidationError;
    #[inline]
    fn validate_column(duration: &f32) -> Result<(), Self::Error> {
        use diesel::Column;
        if duration > &900f32 {
            return Err (:: validation_errors :: ValidationError :: smaller_than_value (< crate :: ball_mill_procedure_templates :: table as :: diesel_builders :: TableExt > :: TABLE_NAME , crate :: ball_mill_procedure_templates :: duration :: NAME , 900f64)) ;
        }
        if duration < &30f32 {
            return Err (:: validation_errors :: ValidationError :: greater_than_value (< crate :: ball_mill_procedure_templates :: table as :: diesel_builders :: TableExt > :: TABLE_NAME , crate :: ball_mill_procedure_templates :: duration :: NAME , 30f64)) ;
        }
        Ok(())
    }
}
impl ::diesel_builders::ValidateColumn<ball_mill_procedure_templates::hertz>
    for <ball_mill_procedure_templates::table as ::diesel_builders::TableExt>::NewValues
{
    type Error = ::validation_errors::ValidationError;
    #[inline]
    fn validate_column(hertz: &f32) -> Result<(), Self::Error> {
        use diesel::Column;
        if hertz > &50f32 {
            return Err (:: validation_errors :: ValidationError :: smaller_than_value (< crate :: ball_mill_procedure_templates :: table as :: diesel_builders :: TableExt > :: TABLE_NAME , crate :: ball_mill_procedure_templates :: hertz :: NAME , 50f64)) ;
        }
        if hertz < &15f32 {
            return Err (:: validation_errors :: ValidationError :: greater_than_value (< crate :: ball_mill_procedure_templates :: table as :: diesel_builders :: TableExt > :: TABLE_NAME , crate :: ball_mill_procedure_templates :: hertz :: NAME , 15f64)) ;
        }
        Ok(())
    }
}
impl ::diesel_builders::ValidateColumn<ball_mill_procedure_templates::bead_count>
    for <ball_mill_procedure_templates::table as ::diesel_builders::TableExt>::NewValues
{
    type Error = ::validation_errors::ValidationError;
    #[inline]
    fn validate_column(bead_count: &i16) -> Result<(), Self::Error> {
        use diesel::Column;
        if bead_count <= &0i16 {
            return Err (:: validation_errors :: ValidationError :: strictly_greater_than_value (< crate :: ball_mill_procedure_templates :: table as :: diesel_builders :: TableExt > :: TABLE_NAME , crate :: ball_mill_procedure_templates :: bead_count :: NAME , 0f64)) ;
        }
        Ok(())
    }
}
impl ::diesel_builders::GetColumn<aps_entities::entities::id> for BallMillProcedureTemplate {
    fn get_column_ref(
        &self,
    ) -> &<ball_mill_procedure_templates::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_ownables::ownables::id> for BallMillProcedureTemplate {
    fn get_column_ref(
        &self,
    ) -> &<ball_mill_procedure_templates::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_procedure_templates::procedure_templates::id>
    for BallMillProcedureTemplate
{
    fn get_column_ref(
        &self,
    ) -> &<ball_mill_procedure_templates::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
