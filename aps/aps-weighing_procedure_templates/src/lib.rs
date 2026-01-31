//! Auto-generated crate for the `weighing_procedure_templates` table.
#[derive(
    Copy,
    Clone,
    Debug,
    Hash,
    Ord,
    PartialOrd,
    Eq,
    PartialEq,
    :: serde :: Serialize,
    :: serde :: Deserialize,
    :: diesel :: Queryable,
    :: diesel :: Selectable,
    :: diesel :: Identifiable,
    :: diesel :: Associations,
    :: diesel_builders :: prelude :: TableModel,
)]
/// Struct representing a row in the `weighing_procedure_templates` table.
#[table_model(ancestors(aps_ownables::ownables, aps_procedure_templates::procedure_templates))]
# [diesel (belongs_to (aps_procedure_templates :: ProcedureTemplate , foreign_key = id))]
# [diesel (belongs_to (aps_physical_asset_models :: PhysicalAssetModel , foreign_key = weighed_asset_model_id))]
# [diesel (belongs_to (aps_weighing_device_models :: WeighingDeviceModel , foreign_key = weighed_with_model_id))]
# [table_model (foreign_key ((id ,) , (:: aps_procedure_templates :: procedure_templates :: id)))]
# [table_model (foreign_key ((weighed_asset_model_id ,) , (:: aps_physical_asset_models :: physical_asset_models :: id)))]
# [table_model (foreign_key ((weighed_with_model_id ,) , (:: aps_weighing_device_models :: weighing_device_models :: id)))]
# [table_model (foreign_key ((id , procedure_template_weighed_asset_model_id ,) , (:: aps_reused_procedure_template_asset_models :: reused_procedure_template_asset_models :: procedure_template_id , :: aps_reused_procedure_template_asset_models :: reused_procedure_template_asset_models :: procedure_template_asset_model_id)))]
# [table_model (foreign_key ((id , procedure_template_weighed_with_model_id ,) , (:: aps_reused_procedure_template_asset_models :: reused_procedure_template_asset_models :: procedure_template_id , :: aps_reused_procedure_template_asset_models :: reused_procedure_template_asset_models :: procedure_template_asset_model_id)))]
#[table_model(default(aps_ownables::ownables::ownable_table_id, "weighing_procedure_templates"))]
# [diesel (table_name = weighing_procedure_templates)]
pub struct WeighingProcedureTemplate {
    /// Identifier of the weighing procedure_id template, which is also a a
    /// foreign key to the general procedure_id template table.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    id: ::rosetta_uuid::Uuid,
    /// The asset model being weighed.
    #[same_as(
        aps_procedure_template_asset_models::procedure_template_asset_models::asset_model_id,
        procedure_template_weighed_asset_model_id
    )]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    weighed_asset_model_id: ::rosetta_uuid::Uuid,
    /// The sample container model should always be an asset model that is
    /// compatible with the procedure_id template.
    #[discretionary(aps_procedure_template_asset_models::procedure_template_asset_models)]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_template_weighed_asset_model_id: ::rosetta_uuid::Uuid,
    /// The model of the instrument to be used for weighing.
    #[same_as(
        aps_procedure_template_asset_models::procedure_template_asset_models::asset_model_id,
        procedure_template_weighed_with_model_id
    )]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    weighed_with_model_id: ::rosetta_uuid::Uuid,
    /// The instrument model used for weighing should always be an asset model
    /// that is compatible with the procedure_id template.
    #[discretionary(aps_procedure_template_asset_models::procedure_template_asset_models)]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_template_weighed_with_model_id: ::rosetta_uuid::Uuid,
}
::diesel_builders::prelude::unique_index!(
    weighing_procedure_templates::id,
    weighing_procedure_templates::procedure_template_weighed_asset_model_id
);
::diesel_builders::prelude::unique_index!(
    weighing_procedure_templates::id,
    weighing_procedure_templates::procedure_template_weighed_with_model_id
);
impl ::diesel_builders::GetColumn<aps_ownables::ownables::id> for WeighingProcedureTemplate {
    fn get_column_ref(
        &self,
    ) -> &<weighing_procedure_templates::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_procedure_templates::procedure_templates::id>
    for WeighingProcedureTemplate
{
    fn get_column_ref(
        &self,
    ) -> &<weighing_procedure_templates::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
