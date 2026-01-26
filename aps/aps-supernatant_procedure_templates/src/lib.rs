//! Auto-generated crate for the `supernatant_procedure_templates` table.
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
/// Struct representing a row in the `supernatant_procedure_templates` table.
#[table_model(ancestors(aps_procedure_templates::procedure_templates))]
# [table_model (error = :: validation_errors :: ValidationError)]
# [diesel (belongs_to (aps_procedure_templates :: ProcedureTemplate , foreign_key = id))]
# [diesel (belongs_to (aps_volume_measuring_device_models :: VolumeMeasuringDeviceModel , foreign_key = transferred_with_model_id))]
# [table_model (foreign_key ((id ,) , (:: aps_procedure_templates :: procedure_templates :: id)))]
# [table_model (foreign_key ((stratified_source_model_id ,) , (:: aps_volumetric_container_models :: volumetric_container_models :: id)))]
# [table_model (foreign_key ((supernatant_destination_model_id ,) , (:: aps_volumetric_container_models :: volumetric_container_models :: id)))]
# [table_model (foreign_key ((transferred_with_model_id ,) , (:: aps_volume_measuring_device_models :: volume_measuring_device_models :: id)))]
# [table_model (foreign_key ((id , procedure_template_stratified_source_model_id ,) , (:: aps_reused_procedure_template_asset_models :: reused_procedure_template_asset_models :: procedure_template_id , :: aps_reused_procedure_template_asset_models :: reused_procedure_template_asset_models :: procedure_template_asset_model_id)))]
# [table_model (foreign_key ((id , procedure_template_supernatant_destination_model_id ,) , (:: aps_reused_procedure_template_asset_models :: reused_procedure_template_asset_models :: procedure_template_id , :: aps_reused_procedure_template_asset_models :: reused_procedure_template_asset_models :: procedure_template_asset_model_id)))]
# [table_model (foreign_key ((id , procedure_template_transferred_with_model_id ,) , (:: aps_reused_procedure_template_asset_models :: reused_procedure_template_asset_models :: procedure_template_id , :: aps_reused_procedure_template_asset_models :: reused_procedure_template_asset_models :: procedure_template_asset_model_id)))]
#[table_model(default(
    aps_procedure_templates::procedure_templates::procedure_template_table_id,
    "supernatant_procedure_templates"
))]
# [diesel (table_name = supernatant_procedure_templates)]
pub struct SupernatantProcedureTemplate {
    /// Field representing the `id` column in table
    /// `supernatant_procedure_templates`.
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    id: ::rosetta_uuid::Uuid,
    /// Volume in liters. The amount that should be transferred.
    volume: f32,
    /// The source container from which the supernatant is taken.
    #[same_as(
        aps_procedure_template_asset_models::procedure_template_asset_models::asset_model_id,
        procedure_template_stratified_source_model_id
    )]
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    stratified_source_model_id: ::rosetta_uuid::Uuid,
    /// Field representing the `procedure_template_stratified_source_model_id`
    /// column in table `supernatant_procedure_templates`.
    #[discretionary(aps_procedure_template_asset_models::procedure_template_asset_models)]
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_template_stratified_source_model_id: ::rosetta_uuid::Uuid,
    /// Destination container to which the supernatant is transferred.
    #[same_as(
        aps_procedure_template_asset_models::procedure_template_asset_models::asset_model_id,
        procedure_template_supernatant_destination_model_id
    )]
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    supernatant_destination_model_id: ::rosetta_uuid::Uuid,
    /// Field representing the
    /// `procedure_template_supernatant_destination_model_id` column in table
    /// `supernatant_procedure_templates`.
    #[discretionary(aps_procedure_template_asset_models::procedure_template_asset_models)]
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_template_supernatant_destination_model_id: ::rosetta_uuid::Uuid,
    /// The device used for the aliquoting procedure.
    #[same_as(
        aps_procedure_template_asset_models::procedure_template_asset_models::asset_model_id,
        procedure_template_transferred_with_model_id
    )]
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    transferred_with_model_id: ::rosetta_uuid::Uuid,
    /// Field representing the `procedure_template_transferred_with_model_id`
    /// column in table `supernatant_procedure_templates`.
    #[discretionary(aps_procedure_template_asset_models::procedure_template_asset_models)]
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_template_transferred_with_model_id: ::rosetta_uuid::Uuid,
}
::diesel_builders::prelude::unique_index!(
    supernatant_procedure_templates::id,
    supernatant_procedure_templates::procedure_template_stratified_source_model_id
);
::diesel_builders::prelude::unique_index!(
    supernatant_procedure_templates::id,
    supernatant_procedure_templates::procedure_template_supernatant_destination_model_id
);
::diesel_builders::prelude::unique_index!(
    supernatant_procedure_templates::id,
    supernatant_procedure_templates::procedure_template_transferred_with_model_id
);
impl ::diesel_builders::ValidateColumn<supernatant_procedure_templates::volume>
    for <supernatant_procedure_templates::table as ::diesel_builders::TableExt>::NewValues
{
    type Error = ::validation_errors::ValidationError;
    #[inline]
    fn validate_column(volume: &f32) -> Result<(), Self::Error> {
        use diesel::Column;
        if volume <= &0f32 {
            return Err(::validation_errors::ValidationError::strictly_greater_than_value(
                "supernatant_procedure_templates",
                crate::supernatant_procedure_templates::volume::NAME,
                0f64,
            ));
        }
        Ok(())
    }
}
impl ::diesel_builders::GetColumn<aps_procedure_templates::procedure_templates::id>
    for SupernatantProcedureTemplate
{
    fn get_column_ref(
        &self,
    ) -> &<supernatant_procedure_templates::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
