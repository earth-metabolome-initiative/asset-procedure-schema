//! Auto-generated crate for the `reused_procedure_template_asset_models` table.
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
/// Table to track reused procedure template asset models across different
/// procedure templates
# [diesel (belongs_to (aps_procedure_templates :: ProcedureTemplate , foreign_key = procedure_template_id))]
# [diesel (belongs_to (aps_procedure_template_asset_models :: ProcedureTemplateAssetModel , foreign_key = procedure_template_asset_model_id))]
#[diesel(primary_key(procedure_template_id, procedure_template_asset_model_id))]
# [table_model (foreign_key ((procedure_template_id ,) , (:: aps_procedure_templates :: procedure_templates :: id)))]
# [table_model (foreign_key ((procedure_template_asset_model_id ,) , (:: aps_procedure_template_asset_models :: procedure_template_asset_models :: id)))]
# [diesel (table_name = reused_procedure_template_asset_models)]
pub struct ReusedProcedureTemplateAssetModel {
    /// Procedure template this reused asset model is associated with
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_template_id: ::rosetta_uuid::Uuid,
    /// Identifier of the reused procedure template asset model
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_template_asset_model_id: ::rosetta_uuid::Uuid,
}
