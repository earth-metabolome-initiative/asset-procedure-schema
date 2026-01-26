//! Auto-generated crate for the `disposal_procedures` table.
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
/// Struct representing a row in the `disposal_procedures` table.
#[table_model(ancestors(aps_procedures::procedures))]
# [diesel (belongs_to (aps_physical_assets :: PhysicalAsset , foreign_key = disposed_asset_id))]
# [diesel (belongs_to (aps_physical_asset_models :: PhysicalAssetModel , foreign_key = disposed_asset_model_id))]
# [diesel (belongs_to (aps_procedure_template_asset_models :: ProcedureTemplateAssetModel , foreign_key = procedure_template_disposed_asset_model_id))]
# [table_model (foreign_key ((id ,) , (:: aps_procedures :: procedures :: id)))]
# [table_model (foreign_key ((disposal_procedure_template_id ,) , (:: aps_disposal_procedure_templates :: disposal_procedure_templates :: id)))]
# [table_model (foreign_key ((disposed_asset_id ,) , (:: aps_physical_assets :: physical_assets :: id)))]
# [table_model (foreign_key ((disposed_asset_model_id ,) , (:: aps_physical_asset_models :: physical_asset_models :: id)))]
# [table_model (foreign_key ((procedure_template_disposed_asset_model_id ,) , (:: aps_procedure_template_asset_models :: procedure_template_asset_models :: id)))]
#[table_model(default(aps_procedures::procedures::procedure_table_id, "disposal_procedures"))]
# [diesel (table_name = disposal_procedures)]
pub struct DisposalProcedure {
    /// Field representing the `id` column in table `disposal_procedures`.
    #[same_as(aps_procedures::procedures::id)]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    id: ::rosetta_uuid::Uuid,
    /// The model of the procedure.
    #[same_as(aps_procedures::procedures::procedure_template_id)]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    disposal_procedure_template_id: ::rosetta_uuid::Uuid,
    /// The disposed asset is the one that is being disposed_asset_id of.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    disposed_asset_id: Option<::rosetta_uuid::Uuid>,
    /// The model of the disposed asset.
    #[same_as(
        aps_procedure_asset_models::procedure_asset_models::asset_model_id,
        procedure_disposed_asset_id
    )]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    disposed_asset_model_id: ::rosetta_uuid::Uuid,
    /// The procedure_id template asset model associated to the
    /// `disposed_asset`.
    #[same_as(
        aps_procedure_asset_models::procedure_asset_models::procedure_template_asset_model_id,
        procedure_disposed_asset_id
    )]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_template_disposed_asset_model_id: ::rosetta_uuid::Uuid,
    /// The procedure_id asset associated to the `disposed_asset`.
    #[discretionary(aps_procedure_asset_models::procedure_asset_models)]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_disposed_asset_id: ::rosetta_uuid::Uuid,
}
impl ::diesel_builders::GetColumn<aps_procedures::procedures::id> for DisposalProcedure {
    fn get_column_ref(
        &self,
    ) -> &<disposal_procedures::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl procedure_like::ProcedureTableLike for disposal_procedures::table {
    type ProcedureTemplateTable =
        aps_disposal_procedure_templates::disposal_procedure_templates::table;
}
