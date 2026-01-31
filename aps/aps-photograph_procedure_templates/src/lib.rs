//! Auto-generated crate for the `photograph_procedure_templates` table.
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
/// Struct representing a row in the `photograph_procedure_templates` table.
#[table_model(ancestors(aps_ownables::ownables, aps_procedure_templates::procedure_templates))]
# [diesel (belongs_to (aps_procedure_templates :: ProcedureTemplate , foreign_key = id))]
# [diesel (belongs_to (aps_camera_models :: CameraModel , foreign_key = photographed_with_model_id))]
# [diesel (belongs_to (aps_physical_asset_models :: PhysicalAssetModel , foreign_key = photographed_asset_model_id))]
# [diesel (belongs_to (aps_digital_asset_models :: DigitalAssetModel , foreign_key = photograph_model_id))]
# [table_model (foreign_key ((id ,) , (:: aps_procedure_templates :: procedure_templates :: id)))]
# [table_model (foreign_key ((photographed_with_model_id ,) , (:: aps_camera_models :: camera_models :: id)))]
# [table_model (foreign_key ((photographed_asset_model_id ,) , (:: aps_physical_asset_models :: physical_asset_models :: id)))]
# [table_model (foreign_key ((photograph_model_id ,) , (:: aps_digital_asset_models :: digital_asset_models :: id)))]
# [table_model (foreign_key ((id , procedure_template_photographed_with_model_id ,) , (:: aps_reused_procedure_template_asset_models :: reused_procedure_template_asset_models :: procedure_template_id , :: aps_reused_procedure_template_asset_models :: reused_procedure_template_asset_models :: procedure_template_asset_model_id)))]
# [table_model (foreign_key ((id , procedure_template_photographed_asset_model_id ,) , (:: aps_reused_procedure_template_asset_models :: reused_procedure_template_asset_models :: procedure_template_id , :: aps_reused_procedure_template_asset_models :: reused_procedure_template_asset_models :: procedure_template_asset_model_id)))]
# [table_model (foreign_key ((id , procedure_template_photograph_model_id ,) , (:: aps_reused_procedure_template_asset_models :: reused_procedure_template_asset_models :: procedure_template_id , :: aps_reused_procedure_template_asset_models :: reused_procedure_template_asset_models :: procedure_template_asset_model_id)))]
#[table_model(default(aps_ownables::ownables::ownable_table_id, "photograph_procedure_templates"))]
# [diesel (table_name = photograph_procedure_templates)]
pub struct PhotographProcedureTemplate {
    /// Field representing the `id` column in table
    /// `photograph_procedure_templates`.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    id: ::rosetta_uuid::Uuid,
    /// The device used for photograph.
    #[same_as(
        aps_procedure_template_asset_models::procedure_template_asset_models::asset_model_id,
        procedure_template_photographed_with_model_id
    )]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    photographed_with_model_id: ::rosetta_uuid::Uuid,
    /// Field representing the `procedure_template_photographed_with_model_id`
    /// column in table `photograph_procedure_templates`.
    #[discretionary(aps_procedure_template_asset_models::procedure_template_asset_models)]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_template_photographed_with_model_id: ::rosetta_uuid::Uuid,
    /// Field representing the `photographed_asset_model_id` column in table
    /// `photograph_procedure_templates`.
    #[same_as(
        aps_procedure_template_asset_models::procedure_template_asset_models::asset_model_id,
        procedure_template_photographed_asset_model_id
    )]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    photographed_asset_model_id: ::rosetta_uuid::Uuid,
    /// Field representing the `procedure_template_photographed_asset_model_id`
    /// column in table `photograph_procedure_templates`.
    #[discretionary(aps_procedure_template_asset_models::procedure_template_asset_models)]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_template_photographed_asset_model_id: ::rosetta_uuid::Uuid,
    /// Field representing the `photograph_model_id` column in table
    /// `photograph_procedure_templates`.
    #[same_as(
        aps_procedure_template_asset_models::procedure_template_asset_models::asset_model_id,
        procedure_template_photograph_model_id
    )]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    photograph_model_id: ::rosetta_uuid::Uuid,
    /// Field representing the `procedure_template_photograph_model_id` column
    /// in table `photograph_procedure_templates`.
    #[discretionary(aps_procedure_template_asset_models::procedure_template_asset_models)]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_template_photograph_model_id: ::rosetta_uuid::Uuid,
}
::diesel_builders::prelude::unique_index!(
    photograph_procedure_templates::id,
    photograph_procedure_templates::procedure_template_photographed_with_model_id
);
::diesel_builders::prelude::unique_index!(
    photograph_procedure_templates::id,
    photograph_procedure_templates::procedure_template_photographed_asset_model_id
);
::diesel_builders::prelude::unique_index!(
    photograph_procedure_templates::id,
    photograph_procedure_templates::procedure_template_photograph_model_id
);
impl ::diesel_builders::GetColumn<aps_ownables::ownables::id> for PhotographProcedureTemplate {
    fn get_column_ref(
        &self,
    ) -> &<photograph_procedure_templates::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_procedure_templates::procedure_templates::id>
    for PhotographProcedureTemplate
{
    fn get_column_ref(
        &self,
    ) -> &<photograph_procedure_templates::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
