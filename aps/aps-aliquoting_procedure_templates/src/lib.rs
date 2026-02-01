//! Auto-generated crate for the `aliquoting_procedure_templates` table.
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
/// Struct representing a row in the `aliquoting_procedure_templates` table.
#[table_model(ancestors(
    aps_entities::entities,
    aps_ownables::ownables,
    aps_namespaced_ownables::namespaced_ownables,
    aps_procedure_templates::procedure_templates
))]
# [table_model (error = :: validation_errors :: ValidationError)]
# [diesel (belongs_to (aps_procedure_templates :: ProcedureTemplate , foreign_key = id))]
# [diesel (belongs_to (aps_volume_measuring_device_models :: VolumeMeasuringDeviceModel , foreign_key = aliquoted_with_model_id))]
# [table_model (foreign_key ((id ,) , (:: aps_procedure_templates :: procedure_templates :: id)))]
# [table_model (foreign_key ((aliquoted_from_model_id ,) , (:: aps_volumetric_container_models :: volumetric_container_models :: id)))]
# [table_model (foreign_key ((aliquoted_into_model_id ,) , (:: aps_volumetric_container_models :: volumetric_container_models :: id)))]
# [table_model (foreign_key ((aliquoted_with_model_id ,) , (:: aps_volume_measuring_device_models :: volume_measuring_device_models :: id)))]
# [table_model (foreign_key ((id , procedure_template_aliquoted_from_model_id ,) , (:: aps_reused_procedure_template_asset_models :: reused_procedure_template_asset_models :: procedure_template_id , :: aps_reused_procedure_template_asset_models :: reused_procedure_template_asset_models :: procedure_template_asset_model_id)))]
# [table_model (foreign_key ((id , procedure_template_aliquoted_into_model_id ,) , (:: aps_reused_procedure_template_asset_models :: reused_procedure_template_asset_models :: procedure_template_id , :: aps_reused_procedure_template_asset_models :: reused_procedure_template_asset_models :: procedure_template_asset_model_id)))]
# [table_model (foreign_key ((id , procedure_template_aliquoted_with_model_id ,) , (:: aps_reused_procedure_template_asset_models :: reused_procedure_template_asset_models :: procedure_template_id , :: aps_reused_procedure_template_asset_models :: reused_procedure_template_asset_models :: procedure_template_asset_model_id)))]
#[table_model(default(aps_entities::entities::table_name_id, "aliquoting_procedure_templates"))]
# [diesel (table_name = aliquoting_procedure_templates)]
pub struct AliquotingProcedureTemplate {
    /// Field representing the `id` column in table
    /// `aliquoting_procedure_templates`.
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    id: ::rosetta_uuid::Uuid,
    /// The volume in liters that should be aliquoted.
    volume: f32,
    /// Source container from which the aliquot is taken.
    #[same_as(
        aps_procedure_template_asset_models::procedure_template_asset_models::asset_model_id,
        procedure_template_aliquoted_from_model_id
    )]
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    aliquoted_from_model_id: ::rosetta_uuid::Uuid,
    /// Field representing the `procedure_template_aliquoted_from_model_id`
    /// column in table `aliquoting_procedure_templates`.
    #[discretionary(aps_procedure_template_asset_models::procedure_template_asset_models)]
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_template_aliquoted_from_model_id: ::rosetta_uuid::Uuid,
    /// Destination container to which the aliquot is transferred.
    #[same_as(
        aps_procedure_template_asset_models::procedure_template_asset_models::asset_model_id,
        procedure_template_aliquoted_into_model_id
    )]
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    aliquoted_into_model_id: ::rosetta_uuid::Uuid,
    /// Field representing the `procedure_template_aliquoted_into_model_id`
    /// column in table `aliquoting_procedure_templates`.
    #[discretionary(aps_procedure_template_asset_models::procedure_template_asset_models)]
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_template_aliquoted_into_model_id: ::rosetta_uuid::Uuid,
    /// The device used for the aliquoting procedure.
    #[same_as(
        aps_procedure_template_asset_models::procedure_template_asset_models::asset_model_id,
        procedure_template_aliquoted_with_model_id
    )]
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    aliquoted_with_model_id: ::rosetta_uuid::Uuid,
    /// Field representing the `procedure_template_aliquoted_with_model_id`
    /// column in table `aliquoting_procedure_templates`.
    #[discretionary(aps_procedure_template_asset_models::procedure_template_asset_models)]
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_template_aliquoted_with_model_id: ::rosetta_uuid::Uuid,
}
::diesel_builders::prelude::unique_index!(
    aliquoting_procedure_templates::id,
    aliquoting_procedure_templates::procedure_template_aliquoted_from_model_id
);
::diesel_builders::prelude::unique_index!(
    aliquoting_procedure_templates::id,
    aliquoting_procedure_templates::procedure_template_aliquoted_into_model_id
);
::diesel_builders::prelude::unique_index!(
    aliquoting_procedure_templates::id,
    aliquoting_procedure_templates::procedure_template_aliquoted_with_model_id
);
impl ::diesel_builders::ValidateColumn<aliquoting_procedure_templates::volume>
    for <aliquoting_procedure_templates::table as ::diesel_builders::TableExt>::NewValues
{
    type Error = ::validation_errors::ValidationError;
    #[inline]
    fn validate_column(volume: &f32) -> Result<(), Self::Error> {
        use diesel::Column;
        if volume <= &0f32 {
            return Err (:: validation_errors :: ValidationError :: strictly_greater_than_value (< crate :: aliquoting_procedure_templates :: table as :: diesel_builders :: TableExt > :: TABLE_NAME , crate :: aliquoting_procedure_templates :: volume :: NAME , 0f64)) ;
        }
        Ok(())
    }
}
impl ::diesel_builders::GetColumn<aps_entities::entities::id> for AliquotingProcedureTemplate {
    fn get_column_ref(
        &self,
    ) -> &<aliquoting_procedure_templates::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_namespaced_ownables::namespaced_ownables::id>
    for AliquotingProcedureTemplate
{
    fn get_column_ref(
        &self,
    ) -> &<aliquoting_procedure_templates::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_ownables::ownables::id> for AliquotingProcedureTemplate {
    fn get_column_ref(
        &self,
    ) -> &<aliquoting_procedure_templates::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_procedure_templates::procedure_templates::id>
    for AliquotingProcedureTemplate
{
    fn get_column_ref(
        &self,
    ) -> &<aliquoting_procedure_templates::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
