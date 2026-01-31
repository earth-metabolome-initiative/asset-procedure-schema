//! Auto-generated crate for the `centrifuge_procedure_templates` table.
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
/// Struct representing a row in the `centrifuge_procedure_templates` table.
#[table_model(ancestors(aps_ownables::ownables, aps_procedure_templates::procedure_templates))]
# [table_model (error = :: validation_errors :: ValidationError)]
# [diesel (belongs_to (aps_procedure_templates :: ProcedureTemplate , foreign_key = id))]
# [diesel (belongs_to (aps_centrifuge_models :: CentrifugeModel , foreign_key = centrifuged_with_model_id))]
# [diesel (belongs_to (aps_volumetric_container_models :: VolumetricContainerModel , foreign_key = centrifuged_container_model_id))]
# [table_model (foreign_key ((id ,) , (:: aps_procedure_templates :: procedure_templates :: id)))]
# [table_model (foreign_key ((centrifuged_with_model_id ,) , (:: aps_centrifuge_models :: centrifuge_models :: id)))]
# [table_model (foreign_key ((centrifuged_container_model_id ,) , (:: aps_volumetric_container_models :: volumetric_container_models :: id)))]
# [table_model (foreign_key ((centrifuged_with_model_id , centrifuged_container_model_id ,) , (:: aps_asset_compatibility_rules :: asset_compatibility_rules :: left_asset_model_id , :: aps_asset_compatibility_rules :: asset_compatibility_rules :: right_asset_model_id)))]
# [table_model (foreign_key ((id , procedure_template_centrifuged_with_model_id ,) , (:: aps_reused_procedure_template_asset_models :: reused_procedure_template_asset_models :: procedure_template_id , :: aps_reused_procedure_template_asset_models :: reused_procedure_template_asset_models :: procedure_template_asset_model_id)))]
# [table_model (foreign_key ((id , procedure_template_centrifuged_container_model_id ,) , (:: aps_reused_procedure_template_asset_models :: reused_procedure_template_asset_models :: procedure_template_id , :: aps_reused_procedure_template_asset_models :: reused_procedure_template_asset_models :: procedure_template_asset_model_id)))]
#[table_model(default(aps_ownables::ownables::ownable_table_id, "centrifuge_procedure_templates"))]
# [diesel (table_name = centrifuge_procedure_templates)]
pub struct CentrifugeProcedureTemplate {
    /// Field representing the `id` column in table
    /// `centrifuge_procedure_templates`.
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    id: ::rosetta_uuid::Uuid,
    /// The storage temperature in Kelvin.
    #[table_model(default = 293.15f32)]
    kelvin: f32,
    /// Tolerance percentage for the storage temperature.
    #[table_model(default = 1f32)]
    kelvin_tolerance_percentage: f32,
    /// Duration in seconds that the centrifuge should be used for the
    /// procedure.
    #[table_model(default = 120f32)]
    duration: f32,
    /// The RPMs (rotations per minute) of the centrifuge.
    #[table_model(default = 13000f32)]
    rotation_per_minute: f32,
    /// The device used for the centrifuge procedure.
    #[same_as(
        aps_procedure_template_asset_models::procedure_template_asset_models::asset_model_id,
        procedure_template_centrifuged_with_model_id
    )]
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    centrifuged_with_model_id: ::rosetta_uuid::Uuid,
    /// Field representing the `procedure_template_centrifuged_with_model_id`
    /// column in table `centrifuge_procedure_templates`.
    #[discretionary(aps_procedure_template_asset_models::procedure_template_asset_models)]
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_template_centrifuged_with_model_id: ::rosetta_uuid::Uuid,
    /// The container that is being centrifuged.
    #[same_as(
        aps_procedure_template_asset_models::procedure_template_asset_models::asset_model_id,
        procedure_template_centrifuged_container_model_id
    )]
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    centrifuged_container_model_id: ::rosetta_uuid::Uuid,
    /// The centrifuged container model should allways be an asset model that is
    /// compatible with the procedure_id template.
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
impl ::diesel_builders::ValidateColumn<centrifuge_procedure_templates::kelvin>
    for <centrifuge_procedure_templates::table as ::diesel_builders::TableExt>::NewValues
{
    type Error = ::validation_errors::ValidationError;
    #[inline]
    fn validate_column(kelvin: &f32) -> Result<(), Self::Error> {
        use diesel::Column;
        if kelvin <= &0f32 {
            return Err (:: validation_errors :: ValidationError :: strictly_greater_than_value (< crate :: centrifuge_procedure_templates :: table as :: diesel_builders :: TableExt > :: TABLE_NAME , crate :: centrifuge_procedure_templates :: kelvin :: NAME , 0f64)) ;
        }
        Ok(())
    }
}
impl ::diesel_builders::ValidateColumn<centrifuge_procedure_templates::kelvin_tolerance_percentage>
    for <centrifuge_procedure_templates::table as ::diesel_builders::TableExt>::NewValues
{
    type Error = ::validation_errors::ValidationError;
    #[inline]
    fn validate_column(kelvin_tolerance_percentage: &f32) -> Result<(), Self::Error> {
        use diesel::Column;
        if kelvin_tolerance_percentage <= &0f32 {
            return Err (:: validation_errors :: ValidationError :: strictly_greater_than_value (< crate :: centrifuge_procedure_templates :: table as :: diesel_builders :: TableExt > :: TABLE_NAME , crate :: centrifuge_procedure_templates :: kelvin_tolerance_percentage :: NAME , 0f64)) ;
        }
        if kelvin_tolerance_percentage > &100f32 {
            return Err (:: validation_errors :: ValidationError :: smaller_than_value (< crate :: centrifuge_procedure_templates :: table as :: diesel_builders :: TableExt > :: TABLE_NAME , crate :: centrifuge_procedure_templates :: kelvin_tolerance_percentage :: NAME , 100f64)) ;
        }
        Ok(())
    }
}
impl ::diesel_builders::ValidateColumn<centrifuge_procedure_templates::duration>
    for <centrifuge_procedure_templates::table as ::diesel_builders::TableExt>::NewValues
{
    type Error = ::validation_errors::ValidationError;
    #[inline]
    fn validate_column(duration: &f32) -> Result<(), Self::Error> {
        use diesel::Column;
        if duration < &30f32 {
            return Err (:: validation_errors :: ValidationError :: greater_than_value (< crate :: centrifuge_procedure_templates :: table as :: diesel_builders :: TableExt > :: TABLE_NAME , crate :: centrifuge_procedure_templates :: duration :: NAME , 30f64)) ;
        }
        if duration > &1800f32 {
            return Err (:: validation_errors :: ValidationError :: smaller_than_value (< crate :: centrifuge_procedure_templates :: table as :: diesel_builders :: TableExt > :: TABLE_NAME , crate :: centrifuge_procedure_templates :: duration :: NAME , 1800f64)) ;
        }
        Ok(())
    }
}
impl ::diesel_builders::ValidateColumn<centrifuge_procedure_templates::rotation_per_minute>
    for <centrifuge_procedure_templates::table as ::diesel_builders::TableExt>::NewValues
{
    type Error = ::validation_errors::ValidationError;
    #[inline]
    fn validate_column(rotation_per_minute: &f32) -> Result<(), Self::Error> {
        use diesel::Column;
        if rotation_per_minute < &5000f32 {
            return Err (:: validation_errors :: ValidationError :: greater_than_value (< crate :: centrifuge_procedure_templates :: table as :: diesel_builders :: TableExt > :: TABLE_NAME , crate :: centrifuge_procedure_templates :: rotation_per_minute :: NAME , 5000f64)) ;
        }
        if rotation_per_minute > &30000f32 {
            return Err (:: validation_errors :: ValidationError :: smaller_than_value (< crate :: centrifuge_procedure_templates :: table as :: diesel_builders :: TableExt > :: TABLE_NAME , crate :: centrifuge_procedure_templates :: rotation_per_minute :: NAME , 30000f64)) ;
        }
        Ok(())
    }
}
impl ::diesel_builders::GetColumn<aps_ownables::ownables::id> for CentrifugeProcedureTemplate {
    fn get_column_ref(
        &self,
    ) -> &<centrifuge_procedure_templates::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_procedure_templates::procedure_templates::id>
    for CentrifugeProcedureTemplate
{
    fn get_column_ref(
        &self,
    ) -> &<centrifuge_procedure_templates::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
